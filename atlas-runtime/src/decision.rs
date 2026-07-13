use atlas_ir::{DecisionTree, DecisionNode};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct DecisionContext {
    pub values: HashMap<String, String>,
    pub query: String,
    pub tags: Vec<String>,
}

impl DecisionContext {
    pub fn new(query: &str) -> Self {
        Self {
            values: HashMap::new(),
            query: query.to_string(),
            tags: Vec::new(),
        }
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.values.insert(key.to_string(), value.to_string());
    }

    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }
}

#[derive(Debug, Clone)]
pub struct DecisionResult {
    pub tree_id: String,
    pub recommendations: Vec<atlas_ir::RecommendationItem>,
    pub rationale: String,
    pub agent_instructions: Option<String>,
    pub path: Vec<String>,
}

pub struct DecisionEngine;

impl DecisionEngine {
    pub fn find_matching<'a>(
        trees: &'a [DecisionTree],
        context: &DecisionContext,
    ) -> Vec<&'a DecisionTree> {
        let q = context.query.to_lowercase();
        trees
            .iter()
            .filter(|t| {
                let tag_match = if t.trigger.tags.is_empty() {
                    true
                } else {
                    t.trigger.tags.iter().any(|tag| {
                        q.contains(&tag.to_lowercase())
                            || context.tags.iter().any(|ct| ct == tag)
                            || context.values.values().any(|v| v == tag)
                    })
                };
                tag_match
            })
            .collect()
    }

    pub fn walk(tree: &DecisionTree, context: &DecisionContext) -> Option<DecisionResult> {
        let node_map: HashMap<&str, &DecisionNode> =
            tree.nodes.iter().map(|n| (n.id.as_str(), n)).collect();

        let mut current_id = tree.root.as_str();
        let mut path = vec![current_id.to_string()];

        loop {
            let node = node_map.get(current_id)?;
            if let Some(ref terminal) = node.terminal {
                return Some(DecisionResult {
                    tree_id: tree.id.clone(),
                    recommendations: terminal.recommendation.clone(),
                    rationale: terminal.rationale.clone(),
                    agent_instructions: terminal.agent_instructions.clone(),
                    path,
                });
            }

            let mut matched = false;
            for branch in &node.branches {
                if evaluate_condition(&branch.condition, context) {
                    current_id = &branch.next;
                    path.push(current_id.to_string());
                    matched = true;
                    break;
                }
            }

            if !matched {
                return None;
            }
        }
    }
}

fn evaluate_condition(condition: &str, context: &DecisionContext) -> bool {
    let parts: Vec<&str> = condition.splitn(2, '=').collect();
    if parts.len() != 2 {
        return false;
    }
    let key = parts[0].trim();
    let value = parts[1].trim();
    context.values.get(key).is_some_and(|v| v == value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use atlas_ir::{DecisionBranch, DecisionNode, DecisionTerminal, DecisionTree, DecisionTrigger, RecommendationItem};

    fn tree() -> DecisionTree {
        DecisionTree {
            id: "t".into(),
            trigger: DecisionTrigger {
                intent: Some("choose".into()),
                domain: None,
                tags: vec!["rust".into()],
            },
            root: "start".into(),
            nodes: vec![
                DecisionNode {
                    id: "start".into(),
                    question: Some("use rust?".into()),
                    node_type: Some("boolean".into()),
                    branches: vec![
                        DecisionBranch { condition: "lang=rust".into(), next: "yes".into() },
                        DecisionBranch { condition: "lang=go".into(), next: "no".into() },
                    ],
                    terminal: None,
                },
                DecisionNode {
                    id: "yes".into(),
                    question: None,
                    node_type: None,
                    branches: vec![],
                    terminal: Some(DecisionTerminal {
                        recommendation: vec![RecommendationItem { node_id: "concept:rust".into(), confidence: 0.9 }],
                        rationale: "rust good".into(),
                        agent_instructions: Some("cargo".into()),
                    }),
                },
                DecisionNode {
                    id: "no".into(),
                    question: None,
                    node_type: None,
                    branches: vec![],
                    terminal: Some(DecisionTerminal {
                        recommendation: vec![],
                        rationale: "go".into(),
                        agent_instructions: None,
                    }),
                },
            ],
        }
    }

    #[test]
    fn test_evaluate_condition() {
        let mut ctx = DecisionContext::new("q");
        ctx.set("lang", "rust");
        assert!(evaluate_condition("lang=rust", &ctx));
        assert!(!evaluate_condition("lang=go", &ctx));
        assert!(!evaluate_condition("malformed", &ctx));
    }

    #[test]
    fn test_find_matching_by_tag() {
        let trees = vec![tree()];
        let m = DecisionContext::new("please use rust here");
        let matched = DecisionEngine::find_matching(&trees, &m);
        assert_eq!(matched.len(), 1);

        let none = DecisionContext::new("use python instead");
        let not_matched = DecisionEngine::find_matching(&trees, &none);
        assert!(not_matched.is_empty());

        // context tags also count
        let mut tagged = DecisionContext::new("x");
        tagged.tags.push("rust".into());
        assert_eq!(DecisionEngine::find_matching(&trees, &tagged).len(), 1);
    }

    #[test]
    fn test_walk_branches() {
        let t = tree();
        let mut rust_ctx = DecisionContext::new("q");
        rust_ctx.set("lang", "rust");
        let r = DecisionEngine::walk(&t, &rust_ctx).unwrap();
        assert_eq!(r.tree_id, "t");
        assert_eq!(r.path, vec!["start".to_string(), "yes".to_string()]);
        assert_eq!(r.rationale, "rust good");
        assert_eq!(r.recommendations[0].node_id, "concept:rust");

        let mut go_ctx = DecisionContext::new("q");
        go_ctx.set("lang", "go");
        let r2 = DecisionEngine::walk(&t, &go_ctx).unwrap();
        assert_eq!(r2.path.last().unwrap(), "no");

        let mut no_ctx = DecisionContext::new("q");
        no_ctx.set("lang", "python");
        assert!(DecisionEngine::walk(&t, &no_ctx).is_none());
    }

    #[test]
    fn test_decision_context_with_tag() {
        let ctx = DecisionContext::new("q").with_tag("t1").with_tag("t2");
        assert_eq!(ctx.tags, vec!["t1".to_string(), "t2".to_string()]);
    }
}

