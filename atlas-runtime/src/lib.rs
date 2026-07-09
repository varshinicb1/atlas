pub mod loader;
pub mod decision;
pub mod verify;
pub mod compliance;
pub mod reasoner;

use loader::AtlasBundle;
use std::collections::HashMap;

pub use decision::{DecisionContext, DecisionEngine, DecisionResult};

pub struct Runtime {
    bundles: HashMap<String, AtlasBundle>,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            bundles: HashMap::new(),
        }
    }

    pub fn load(&mut self, path: &std::path::Path) -> Result<String, anyhow::Error> {
        log::info!("Loading bundle from {}", path.display());
        let bundle = loader::AtlasBundle::from_file(path)?;
        let name = bundle.ir.meta.source_manifest
            .first()
            .map(|s| s.source_id.clone())
            .unwrap_or_else(|| {
                path.file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown")
                    .to_string()
            });
        self.bundles.insert(name.clone(), bundle);
        Ok(name)
    }

    pub fn load_all(&mut self, dir: &std::path::Path) -> Result<usize, anyhow::Error> {
        let mut count = 0;
        if dir.is_dir() {
            for entry in std::fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("atlas") {
                    self.load(&path)?;
                    count += 1;
                }
            }
        }
        Ok(count)
    }

    pub fn get(&self, name: &str) -> Option<&AtlasBundle> {
        self.bundles.get(name)
    }

    pub fn get_workflows(&self, bundle_name: &str) -> Option<&Vec<atlas_ir::Workflow>> {
        self.bundles.get(bundle_name).map(|b| &b.ir.workflows)
    }

    pub fn solve<'a>(&'a self, bundle_name: &str, query: &str) -> Result<SolveResult<'a>, anyhow::Error> {
        let bundle = self.bundles.get(bundle_name)
            .ok_or_else(|| anyhow::anyhow!("Bundle '{}' not loaded", bundle_name))?;
        let q = query.to_lowercase();

        log::debug!("Solving query '{}' against bundle '{}'", query, bundle_name);

        let mut seen = std::collections::HashSet::new();
        let mut nodes = Vec::new();

        let direct: Vec<&atlas_ir::Node> = bundle.ir.search(&q);
        for node in &direct {
            if seen.insert(node.id.clone()) {
                nodes.push(*node);
            }
        }

        if let Some(seed) = direct.first() {
            for n in bundle.ir.subgraph(&seed.id, 1) {
                if seen.insert(n.id.clone()) {
                    nodes.push(n);
                }
            }
        }

        let mut has_exact_symbol = false;
        if let Some(ref indices) = bundle.ir.indices {
            has_exact_symbol = indices.symbol_index.contains_key(&q);
        }

        let has_embedding = if nodes.len() < 3 {
            let emb_results = self.search_embedding(bundle, query, 5);
            for (idx, _score) in &emb_results {
                if idx < &bundle.ir.nodes.len() {
                    let node = &bundle.ir.nodes[*idx];
                    if seen.insert(node.id.clone()) {
                        nodes.push(node);
                    }
                }
            }
            !emb_results.is_empty()
        } else {
            false
        };

        let has_subgraph_only = nodes.len() > direct.len() && !has_exact_symbol && !has_embedding;

        let confidence = if has_exact_symbol {
            0.95
        } else if has_embedding {
            0.7
        } else if has_subgraph_only {
            0.5
        } else if !direct.is_empty() {
            0.95
        } else {
            0.0
        };

        Ok(SolveResult {
            query: query.to_string(),
            bundle: bundle_name.to_string(),
            nodes,
            total_matches: direct.len(),
            confidence,
        })
    }

    fn search_embedding(&self, bundle: &AtlasBundle, query: &str, top_k: usize) -> Vec<(usize, f32)> {
        let indices = match bundle.ir.indices {
            Some(ref idx) => idx,
            None => return Vec::new(),
        };
        if indices.embeddings.is_empty() {
            return Vec::new();
        }
        let query_emb = atlas_ir::embedding::compute(
            &[query.to_string()],
            atlas_ir::embedding::EMBEDDING_DIM,
        );
        atlas_ir::embedding::nearest(&indices.embeddings, &query_emb[0], top_k)
    }

    pub fn decide(&self, bundle_name: &str, query: &str, context: Option<&HashMap<String, String>>) -> Result<Option<DecisionResult>, anyhow::Error> {
        let bundle = self.bundles.get(bundle_name)
            .ok_or_else(|| anyhow::anyhow!("Bundle '{}' not loaded", bundle_name))?;

        let mut ctx = DecisionContext::new(query);
        if let Some(kvs) = context {
            for (k, v) in kvs {
                ctx.set(k, v);
            }
        }

        let matching = DecisionEngine::find_matching(&bundle.ir.decision_trees, &ctx);
        for tree in matching {
            if let Some(result) = DecisionEngine::walk(tree, &ctx) {
                return Ok(Some(result));
            }
        }
        Ok(None)
    }

    pub fn verify(&self, bundle_name: &str, artifact: Option<&str>) -> Result<verify::VerificationReport, anyhow::Error> {
        let bundle = self.bundles.get(bundle_name)
            .ok_or_else(|| anyhow::anyhow!("Bundle '{}' not loaded", bundle_name))?;
        log::info!("Running verification on bundle '{}'", bundle_name);
        let verifier = verify::Verifier::new(&bundle.ir);
        Ok(verifier.verify(artifact))
    }

    pub fn verify_with_policy(&self, bundle_name: &str, policy: &str) -> Result<verify::VerificationReport, anyhow::Error> {
        let bundle = self.bundles.get(bundle_name)
            .ok_or_else(|| anyhow::anyhow!("Bundle '{}' not loaded", bundle_name))?;
        log::info!("Running {} compliance check on bundle '{}'", policy, bundle_name);
        let compliance = compliance::ComplianceChecker::new(&bundle.ir);
        let checks = compliance.check_policy(policy);
        let passed = checks.iter().all(|c| c.passed || c.severity == "warn");
        if !passed {
            log::warn!("{} compliance check failed for bundle '{}'", policy, bundle_name);
        }
        Ok(verify::VerificationReport { passed, checks })
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SolveResult<'a> {
    pub query: String,
    pub bundle: String,
    pub nodes: Vec<&'a atlas_ir::Node>,
    pub total_matches: usize,
    pub confidence: f32,
}
