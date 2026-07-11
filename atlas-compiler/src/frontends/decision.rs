use atlas_ir::{
    DecisionBranch, DecisionNode, DecisionTerminal, DecisionTree,
    DecisionTrigger, RecommendationItem,
};
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct DecisionFile {
    id: String,
    trigger: TriggerDef,
    root: String,
    nodes: Vec<DecisionNodeDef>,
}

#[derive(Debug, Deserialize)]
struct TriggerDef {
    #[serde(default)]
    intent: Option<String>,
    #[serde(default)]
    domain: Option<String>,
    #[serde(default)]
    tags: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct DecisionNodeDef {
    id: String,
    #[serde(default)]
    question: Option<String>,
    #[serde(rename = "node_type")]
    #[serde(default)]
    node_type: Option<String>,
    #[serde(default)]
    branches: Vec<BranchDef>,
    #[serde(default)]
    terminal: Option<TerminalDef>,
}

#[derive(Debug, Deserialize)]
struct BranchDef {
    condition: String,
    next: String,
}

#[derive(Debug, Deserialize)]
struct TerminalDef {
    recommendation: Vec<RecDef>,
    rationale: String,
    #[serde(default)]
    agent_instructions: Option<String>,
}

#[derive(Debug, Deserialize)]
struct RecDef {
    node_id: String,
    confidence: f32,
}

pub struct DecisionParser;

impl DecisionParser {
    pub fn parse_file(path: &Path) -> Result<Vec<DecisionTree>, anyhow::Error> {
        let content = std::fs::read_to_string(path)?;
        Self::parse_multi(&content)
    }

    pub fn parse_multi(content: &str) -> Result<Vec<DecisionTree>, anyhow::Error> {
        let mut trees = Vec::new();
        for doc in content.split("---") {
            let doc = doc.trim();
            if doc.is_empty() {
                continue;
            }
            let df: DecisionFile = serde_yaml::from_str(doc)?;
            trees.push(DecisionTree {
                id: df.id,
                trigger: DecisionTrigger {
                    intent: df.trigger.intent,
                    domain: df.trigger.domain,
                    tags: df.trigger.tags,
                },
                root: df.root,
                nodes: df
                    .nodes
                    .into_iter()
                    .map(|n| DecisionNode {
                        id: n.id,
                        question: n.question,
                        node_type: n.node_type,
                        branches: n
                            .branches
                            .into_iter()
                            .map(|b| DecisionBranch {
                                condition: b.condition,
                                next: b.next,
                            })
                            .collect(),
                        terminal: n.terminal.map(|t| DecisionTerminal {
                            recommendation: t
                                .recommendation
                                .into_iter()
                                .map(|r| RecommendationItem {
                                    node_id: r.node_id,
                                    confidence: r.confidence,
                                })
                                .collect(),
                            rationale: t.rationale,
                            agent_instructions: t.agent_instructions,
                        }),
                    })
                    .collect(),
            });
        }
        Ok(trees)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TREE_YAML: &str = r#"id: my_tree
trigger:
  intent: choose_solution
  domain: web
  tags: [rust]
root: start
nodes:
  - id: start
    question: "use rust?"
    node_type: boolean
    branches:
      - condition: "lang=rust"
        next: terminal_rust
      - condition: "lang=go"
        next: terminal_go
  - id: terminal_rust
    terminal:
      recommendation:
        - node_id: "concept:rust"
          confidence: 0.9
      rationale: "Rust is good"
      agent_instructions: "use cargo"
  - id: terminal_go
    terminal:
      recommendation:
        - node_id: "concept:go"
          confidence: 0.7
      rationale: "Go is simple"
"#;

    #[test]
    fn test_parse_single_tree() {
        let trees = DecisionParser::parse_multi(TREE_YAML).unwrap();
        assert_eq!(trees.len(), 1);
        let t = &trees[0];
        assert_eq!(t.id, "my_tree");
        assert_eq!(t.root, "start");
        assert_eq!(t.nodes.len(), 3);
        assert_eq!(t.trigger.tags, vec!["rust".to_string()]);
        assert_eq!(t.trigger.intent.as_deref(), Some("choose_solution"));
        assert_eq!(t.trigger.domain.as_deref(), Some("web"));

        let start = t.nodes.iter().find(|n| n.id == "start").unwrap();
        assert_eq!(start.branches.len(), 2);
        assert!(start.terminal.is_none());

        let rust = t.nodes.iter().find(|n| n.id == "terminal_rust").unwrap();
        let term = rust.terminal.as_ref().unwrap();
        assert_eq!(term.recommendation[0].node_id, "concept:rust");
        assert!((term.recommendation[0].confidence - 0.9).abs() < 1e-6);
        assert_eq!(term.rationale, "Rust is good");
        assert_eq!(term.agent_instructions.as_deref(), Some("use cargo"));
    }

    #[test]
    fn test_parse_multiple_docs() {
        let doc_a = "id: a\nroot: r\ntrigger:\n  tags: [x]\nnodes:\n  - id: r\n    terminal:\n      recommendation: []\n      rationale: ra\n";
        let doc_b = "id: b\nroot: s\ntrigger:\n  tags: [y]\nnodes:\n  - id: s\n    terminal:\n      recommendation: []\n      rationale: rb\n";
        let combined = format!("{}\n---\n{}", doc_a, doc_b);
        let trees = DecisionParser::parse_multi(&combined).unwrap();
        assert_eq!(trees.len(), 2);
        assert_eq!(trees[0].id, "a");
        assert_eq!(trees[1].id, "b");
    }

    #[test]
    fn test_parse_invalid_yaml_errors() {
        let bad = "id: [unclosed\nroot: r\n";
        assert!(DecisionParser::parse_multi(bad).is_err());
    }

    #[test]
    fn test_parse_empty_returns_no_trees() {
        assert!(DecisionParser::parse_multi("   \n  \n").unwrap().is_empty());
    }
}
