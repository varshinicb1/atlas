pub mod frontends;
pub mod binary;

use atlas_ir::{EngineeringIR, Meta, Node, Edge, SourceEntry, Workflow};
use std::collections::HashMap;
use std::path::Path;

pub struct Compiler {
    ir: EngineeringIR,
    workflows: Vec<Workflow>,
    source_hashes: HashMap<String, String>,
}

impl Compiler {
    pub fn new() -> Self {
        let meta = Meta {
            schema_version: "0.1.0".into(),
            generator: "atlas-compiler/0.1.0".into(),
            created_at: chrono::Utc::now().timestamp_millis(),
            source_manifest: Vec::new(),
        };
        Self {
            ir: EngineeringIR::new(meta),
            workflows: Vec::new(),
            source_hashes: HashMap::new(),
        }
    }

    pub fn add_source(&mut self, source_id: &str, path: &Path, content: Option<&str>) -> Result<(), anyhow::Error> {
        let content = match content {
            Some(c) => c.to_string(),
            None => std::fs::read_to_string(path)?,
        };
        let hash = blake3::hash(content.as_bytes()).to_hex().to_string();
        self.source_hashes.insert(source_id.to_string(), hash.clone());
        self.ir.meta.source_manifest.push(SourceEntry {
            source_id: source_id.to_string(),
            path: path.to_string_lossy().to_string(),
            hash,
        });
        Ok(())
    }

    pub fn add_node(&mut self, node: Node) -> Result<(), anyhow::Error> {
        if self.ir.nodes.iter().any(|n| n.id == node.id) {
            anyhow::bail!("Node with id '{}' already exists", node.id);
        }
        self.ir.add_node(node);
        Ok(())
    }

    pub fn add_edge(&mut self, edge: Edge) -> Result<(), anyhow::Error> {
        if self.ir.edges.iter().any(|e| e.id == edge.id) {
            anyhow::bail!("Edge with id '{}' already exists", edge.id);
        }
        self.ir.add_edge(edge);
        Ok(())
    }

    pub fn add_example(&mut self, example: atlas_ir::Example) {
        self.ir.examples.push(example);
    }

    pub fn add_failure(&mut self, failure: atlas_ir::FailureMode) {
        self.ir.failure_modes.push(failure);
    }

    pub fn add_workflow(&mut self, wf: Workflow) {
        self.workflows.push(wf);
    }

    pub fn add_decision_trees(&mut self, trees: Vec<atlas_ir::DecisionTree>) {
        self.ir.decision_trees.extend(trees);
    }

    pub fn build(&mut self) -> Result<EngineeringIR, anyhow::Error> {
        self.ir.workflows = std::mem::take(&mut self.workflows);
        self.ir.indices = Some(atlas_ir::Indices::build(&self.ir));
        Ok(self.ir.clone())
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}
