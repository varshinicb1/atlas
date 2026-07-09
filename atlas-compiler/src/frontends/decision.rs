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
