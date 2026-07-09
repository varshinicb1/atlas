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
