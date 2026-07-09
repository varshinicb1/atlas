pub mod embedding;

use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};

static EDGE_ID_COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineeringIR {
    pub meta: Meta,
    pub ontology: Ontology,
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub decision_trees: Vec<DecisionTree>,
    pub examples: Vec<Example>,
    pub failure_modes: Vec<FailureMode>,
    pub workflows: Vec<Workflow>,
    pub verification_rules: Vec<VerificationRule>,
    pub indices: Option<Indices>,
}

impl EngineeringIR {
    pub fn new(meta: Meta) -> Self {
        Self {
            meta,
            ontology: Ontology::default(),
            nodes: Vec::new(),
            edges: Vec::new(),
            decision_trees: Vec::new(),
            examples: Vec::new(),
            failure_modes: Vec::new(),
            workflows: Vec::new(),
            verification_rules: Vec::new(),
            indices: None,
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
    }

    pub fn add_edge(&mut self, edge: Edge) {
        self.edges.push(edge);
    }

    pub fn find_node(&self, id: &str) -> Option<&Node> {
        self.nodes.iter().find(|n| n.id == id)
    }

    pub fn neighbors(&self, node_id: &str, edge_type: Option<EdgeType>) -> Vec<&Node> {
        let target_ids: Vec<&str> = self
            .edges
            .iter()
            .filter(|e| {
                e.src == node_id && edge_type.as_ref().is_none_or(|t| e.edge_type == *t)
            })
            .map(|e| e.dst.as_str())
            .collect();
        self.nodes
            .iter()
            .filter(|n| target_ids.contains(&n.id.as_str()))
            .collect()
    }

    pub fn successors(&self, node_id: &str, edge_type: Option<EdgeType>) -> Vec<&Node> {
        self.neighbors(node_id, edge_type)
    }

    pub fn predecessors(&self, node_id: &str, edge_type: Option<EdgeType>) -> Vec<&Node> {
        let src_ids: Vec<&str> = self
            .edges
            .iter()
            .filter(|e| {
                e.dst == node_id && edge_type.as_ref().is_none_or(|t| e.edge_type == *t)
            })
            .map(|e| e.src.as_str())
            .collect();
        self.nodes
            .iter()
            .filter(|n| src_ids.contains(&n.id.as_str()))
            .collect()
    }

    pub fn subgraph(&self, seed: &str, depth: u32) -> Vec<&Node> {
        let mut visited = std::collections::HashSet::new();
        let mut result = Vec::new();
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((seed, 0u32));
        while let Some((id, d)) = queue.pop_front() {
            if d > depth || !visited.insert(id) {
                continue;
            }
            if let Some(node) = self.find_node(id) {
                result.push(node);
            }
            for edge in &self.edges {
                if edge.src == id {
                    queue.push_back((&edge.dst, d + 1));
                }
            }
        }
        result
    }

    pub fn search(&self, query: &str) -> Vec<&Node> {
        let q = query.to_lowercase();
        let mut matched_ids = std::collections::HashSet::new();
        let mut results = Vec::new();

        if let Some(ref indices) = self.indices {
            if let Some(id) = indices.symbol_index.get(&q) {
                if let Some(node) = self.find_node(id) {
                    if matched_ids.insert(node.id.as_str()) {
                        results.push(node);
                    }
                }
            }
        }

        for node in &self.nodes {
            if (node.name.to_lowercase().contains(&q) || node.id.to_lowercase().contains(&q))
                && matched_ids.insert(node.id.as_str())
            {
                results.push(node);
            }
        }

        results
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meta {
    pub schema_version: String,
    pub generator: String,
    pub created_at: i64,
    pub source_manifest: Vec<SourceEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceEntry {
    pub source_id: String,
    pub path: String,
    pub hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ontology {
    pub node_kinds: Vec<String>,
    pub edge_types: Vec<String>,
}

impl Default for Ontology {
    fn default() -> Self {
        Self {
            node_kinds: vec![
                "Concept".into(),
                "Package".into(),
                "Module".into(),
                "Class".into(),
                "Function".into(),
                "API".into(),
                "Protocol".into(),
                "Example".into(),
                "FailureMode".into(),
                "Decision".into(),
                "Architecture".into(),
                "Alternative".into(),
                "Workflow".into(),
            ],
            edge_types: vec![
                "DEPENDS_ON".into(),
                "IMPLEMENTS".into(),
                "PART_OF".into(),
                "USES".into(),
                "EXTENDS".into(),
                "REFERENCES".into(),
                "ALTERNATIVE_TO".into(),
                "SOLVES".into(),
                "HAS_EXAMPLE".into(),
                "HAS_FAILURE".into(),
                "MIGRATES_TO".into(),
                "COMPATIBLE_WITH".into(),
                "HAS_WORKFLOW".into(),
            ],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub kind: NodeKind,
    pub name: String,
    pub version: Option<String>,
    pub category: Option<String>,
    pub provenance: Vec<SourceRef>,
    pub confidence: f32,
    pub status: NodeStatus,
    pub description: Option<String>,
    pub attributes: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NodeKind {
    Concept,
    Package,
    Module,
    Class,
    Function,
    API,
    Protocol,
    Example,
    FailureMode,
    Decision,
    Architecture,
    Alternative,
    Workflow,
}

impl std::fmt::Display for NodeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceRef {
    pub source_id: String,
    pub locator: String,
    pub line: Option<u32>,
    pub hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeStatus {
    Verified,
    Unverified,
    Unresolved,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    pub id: String,
    pub src: String,
    pub dst: String,
    pub edge_type: EdgeType,
    pub weight: f32,
    pub provenance: SourceRef,
}

impl Edge {
    pub fn new_id() -> String {
        format!("e{}", EDGE_ID_COUNTER.fetch_add(1, Ordering::SeqCst))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum EdgeType {
    DependsOn,
    Implements,
    PartOf,
    Uses,
    Extends,
    References,
    AlternativeTo,
    Solves,
    HasExample,
    HasFailure,
    MigratesTo,
    CompatibleWith,
    HasWorkflow,
}

impl std::fmt::Display for EdgeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionTree {
    pub id: String,
    pub trigger: DecisionTrigger,
    pub root: String,
    pub nodes: Vec<DecisionNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionTrigger {
    pub intent: Option<String>,
    pub domain: Option<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionNode {
    pub id: String,
    pub question: Option<String>,
    pub node_type: Option<String>,
    pub branches: Vec<DecisionBranch>,
    pub terminal: Option<DecisionTerminal>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionBranch {
    pub condition: String,
    pub next: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionTerminal {
    pub recommendation: Vec<RecommendationItem>,
    pub rationale: String,
    pub agent_instructions: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationItem {
    pub node_id: String,
    pub confidence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Example {
    pub id: String,
    pub language: String,
    pub code: String,
    pub runnable: bool,
    pub expected_output: Option<String>,
    pub references_node: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailureMode {
    pub id: String,
    pub symptom: String,
    pub cause: String,
    pub fix: String,
    pub affects: Vec<String>,
    pub version_range: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStep {
    pub order: u32,
    pub action: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    pub id: String,
    pub name: String,
    pub description: String,
    pub steps: Vec<WorkflowStep>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationRule {
    pub id: String,
    pub target: String,
    pub kind: String,
    pub command: Option<String>,
    pub severity: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Indices {
    pub symbol_index: std::collections::HashMap<String, String>,
    pub type_index: std::collections::HashMap<String, Vec<String>>,
    pub workflow_index: std::collections::HashMap<String, String>,
    pub embeddings: Vec<crate::embedding::Embedding>,
}

impl Indices {
    pub fn build(ir: &EngineeringIR) -> Self {
        let mut symbol = std::collections::HashMap::new();
        let mut types: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();
        for node in &ir.nodes {
            symbol.insert(node.name.clone(), node.id.clone());
            types
                .entry(node.kind.to_string())
                .or_default()
                .push(node.id.clone());
        }

        let texts: Vec<String> = ir
            .nodes
            .iter()
            .map(|n| {
                let desc = n.description.as_deref().unwrap_or("");
                format!("{} {} {}", n.name, desc, n.category.as_deref().unwrap_or(""))
            })
            .collect();
        let embeddings = crate::embedding::compute(&texts, crate::embedding::EMBEDDING_DIM);

        let mut wf_index = std::collections::HashMap::new();
        for wf in &ir.workflows {
            wf_index.insert(wf.id.clone(), wf.name.clone());
        }

        Self {
            symbol_index: symbol,
            type_index: types,
            workflow_index: wf_index,
            embeddings,
        }
    }
}
