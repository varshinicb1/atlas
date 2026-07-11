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
            symbol.insert(node.name.to_lowercase(), node.id.clone());
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

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_meta() -> Meta {
        Meta {
            schema_version: "0.1.0".into(),
            generator: "test".into(),
            created_at: 0,
            source_manifest: Vec::new(),
        }
    }

    fn ref_() -> SourceRef {
        SourceRef {
            source_id: "s".into(),
            locator: "l".into(),
            line: None,
            hash: "h".into(),
        }
    }

    fn node(id: &str, kind: NodeKind, name: &str) -> Node {
        Node {
            id: id.to_string(),
            kind,
            name: name.to_string(),
            version: None,
            category: None,
            provenance: vec![ref_()],
            confidence: 1.0,
            status: NodeStatus::Verified,
            description: Some(format!("description for {}", name)),
            attributes: serde_json::json!({}),
        }
    }

    fn edge(src: &str, dst: &str, t: EdgeType) -> Edge {
        Edge {
            id: Edge::new_id(),
            src: src.into(),
            dst: dst.into(),
            edge_type: t,
            weight: 1.0,
            provenance: ref_(),
        }
    }

    #[test]
    fn test_new_has_default_ontology() {
        let ir = EngineeringIR::new(empty_meta());
        assert!(!ir.ontology.node_kinds.is_empty());
        assert!(ir.ontology.node_kinds.contains(&"Concept".to_string()));
        assert!(ir.ontology.edge_types.contains(&"DEPENDS_ON".to_string()));
        assert!(ir.nodes.is_empty());
        assert!(ir.edges.is_empty());
    }

    #[test]
    fn test_add_and_find_node() {
        let mut ir = EngineeringIR::new(empty_meta());
        ir.add_node(node("a", NodeKind::Concept, "Alpha"));
        assert_eq!(ir.nodes.len(), 1);
        assert!(ir.find_node("a").is_some());
        assert!(ir.find_node("missing").is_none());
    }

    #[test]
    fn test_neighbors_filtered() {
        let mut ir = EngineeringIR::new(empty_meta());
        ir.add_node(node("a", NodeKind::Concept, "A"));
        ir.add_node(node("b", NodeKind::Concept, "B"));
        ir.add_node(node("c", NodeKind::Concept, "C"));
        ir.add_edge(edge("a", "b", EdgeType::DependsOn));
        ir.add_edge(edge("a", "c", EdgeType::Uses));

        let all = ir.neighbors("a", None);
        assert_eq!(all.len(), 2);
        let dep = ir.neighbors("a", Some(EdgeType::DependsOn));
        assert_eq!(dep.len(), 1);
        assert_eq!(dep[0].id, "b");
        // successors is an alias for neighbors
        let succ = ir.successors("a", None);
        assert_eq!(succ.len(), 2);
    }

    #[test]
    fn test_predecessors() {
        let mut ir = EngineeringIR::new(empty_meta());
        ir.add_node(node("a", NodeKind::Concept, "A"));
        ir.add_node(node("b", NodeKind::Concept, "B"));
        ir.add_edge(edge("a", "b", EdgeType::PartOf));
        let preds = ir.predecessors("b", None);
        assert_eq!(preds.len(), 1);
        assert_eq!(preds[0].id, "a");
        let preds_filtered = ir.predecessors("b", Some(EdgeType::DependsOn));
        assert!(preds_filtered.is_empty());
    }

    #[test]
    fn test_subgraph_depth() {
        let mut ir = EngineeringIR::new(empty_meta());
        for (id, name) in [("a", "A"), ("b", "B"), ("c", "C"), ("d", "D")] {
            ir.add_node(node(id, NodeKind::Concept, name));
        }
        ir.add_edge(edge("a", "b", EdgeType::DependsOn));
        ir.add_edge(edge("b", "c", EdgeType::DependsOn));
        ir.add_edge(edge("c", "d", EdgeType::DependsOn));

        let sg = ir.subgraph("a", 1);
        let ids: Vec<&str> = sg.iter().map(|n| n.id.as_str()).collect();
        assert!(ids.contains(&"a"));
        assert!(ids.contains(&"b"));
        assert!(!ids.contains(&"c"));
        assert!(!ids.contains(&"d"));
    }

    #[test]
    fn test_subgraph_full() {
        let mut ir = EngineeringIR::new(empty_meta());
        for (id, name) in [("a", "A"), ("b", "B"), ("c", "C")] {
            ir.add_node(node(id, NodeKind::Concept, name));
        }
        ir.add_edge(edge("a", "b", EdgeType::DependsOn));
        ir.add_edge(edge("b", "c", EdgeType::DependsOn));
        let sg = ir.subgraph("a", 10);
        assert_eq!(sg.len(), 3);
    }

    #[test]
    fn test_search_by_name_and_id() {
        let mut ir = EngineeringIR::new(empty_meta());
        ir.add_node(node("concept:alpha", NodeKind::Concept, "AlphaThing"));
        ir.add_node(node("concept:beta", NodeKind::Concept, "BetaThing"));

        let by_name = ir.search("alpha");
        assert_eq!(by_name.len(), 1);
        assert_eq!(by_name[0].id, "concept:alpha");

        let by_id = ir.search("concept:beta");
        assert_eq!(by_id.len(), 1);
        assert_eq!(by_id[0].id, "concept:beta");

        assert!(ir.search("zzz").is_empty());
    }

    #[test]
    fn test_search_uses_symbol_index_case_insensitive() {
        let mut ir = EngineeringIR::new(empty_meta());
        ir.add_node(node("concept:x", NodeKind::Concept, "Xylophone"));
        ir.indices = Some(Indices::build(&ir));
        let r = ir.search("xylophone");
        assert_eq!(r.len(), 1);
        assert_eq!(r[0].id, "concept:x");
        // Original-case query must also match.
        assert_eq!(ir.search("Xylophone").len(), 1);
    }

    #[test]
    fn test_indices_build() {
        let mut ir = EngineeringIR::new(empty_meta());
        ir.add_node(node("concept:a", NodeKind::Concept, "Alpha"));
        ir.add_node(node("api:b", NodeKind::API, "Beta"));
        ir.workflows.push(Workflow {
            id: "workflow:w".into(),
            name: "W".into(),
            description: "d".into(),
            steps: Vec::new(),
        });
        let idx = Indices::build(&ir);
        assert_eq!(idx.symbol_index.get("alpha"), Some(&"concept:a".to_string()));
        assert_eq!(idx.type_index.get("Concept"), Some(&vec!["concept:a".to_string()]));
        assert_eq!(idx.workflow_index.get("workflow:w"), Some(&"W".to_string()));
        assert_eq!(idx.embeddings.len(), 2);
        assert_eq!(idx.embeddings[0].len(), 128);
    }

    #[test]
    fn test_ontology_default() {
        let o = Ontology::default();
        assert!(o.node_kinds.len() >= 13);
        assert!(o.edge_types.len() >= 13);
        assert!(o.node_kinds.contains(&"Workflow".to_string()));
        assert!(o.edge_types.contains(&"HAS_WORKFLOW".to_string()));
    }

    #[test]
    fn test_nodekind_edgekind_display() {
        assert_eq!(format!("{}", NodeKind::API), "API");
        assert_eq!(format!("{}", NodeKind::FailureMode), "FailureMode");
        assert_eq!(format!("{}", EdgeType::DependsOn), "DependsOn");
        assert_eq!(format!("{}", EdgeType::HasWorkflow), "HasWorkflow");
    }

    #[test]
    fn test_edge_new_id_unique() {
        let a = Edge::new_id();
        let b = Edge::new_id();
        assert_ne!(a, b);
    }

    #[test]
    fn test_serde_roundtrip() {
        let mut ir = EngineeringIR::new(empty_meta());
        ir.add_node(node("concept:a", NodeKind::Concept, "Alpha"));
        ir.add_edge(edge("concept:a", "concept:b", EdgeType::DependsOn));
        let json = serde_json::to_string(&ir).unwrap();
        let back: EngineeringIR = serde_json::from_str(&json).unwrap();
        assert_eq!(back.nodes.len(), 1);
        assert_eq!(back.edges.len(), 1);
        assert_eq!(back.edges[0].edge_type, EdgeType::DependsOn);
    }
}
