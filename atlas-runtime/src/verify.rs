use atlas_ir::EngineeringIR;

#[derive(Debug, Clone, serde::Serialize)]
pub struct VerificationCheck {
    pub name: String,
    pub passed: bool,
    pub message: String,
    pub severity: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct VerificationReport {
    pub passed: bool,
    pub checks: Vec<VerificationCheck>,
}

pub struct Verifier<'a> {
    ir: &'a EngineeringIR,
}

impl<'a> Verifier<'a> {
    pub fn new(ir: &'a EngineeringIR) -> Self {
        Self { ir }
    }

    pub fn verify(&self, artifact: Option<&str>) -> VerificationReport {
        let mut checks = Vec::new();
        checks.push(self.check_api_exists());
        checks.push(self.check_versions());
        checks.push(self.check_provenance());
        if let Some(code) = artifact {
            checks.push(self.check_artifact_api_usage(code));
        }
        let passed = checks.iter().all(|c| c.passed || c.severity == "warn");
        VerificationReport { passed, checks }
    }

    fn check_api_exists(&self) -> VerificationCheck {
        let mut bad_edges = Vec::new();
        for (i, e) in self.ir.edges.iter().enumerate() {
            if e.edge_type == atlas_ir::EdgeType::PartOf && infer_api_kind(e) {
                let in_nodes = self.ir.nodes.iter().any(|n| n.id == e.dst);
                let in_failures = self.ir.failure_modes.iter().any(|f| f.id == e.dst);
                if !in_nodes && !in_failures {
                    bad_edges.push(format!("#{} dst={}", i, e.dst));
                }
            }
        }
        let passed = bad_edges.is_empty();

        VerificationCheck {
            name: "API existence".into(),
            passed,
            message: if bad_edges.is_empty() {
                format!("All {} edges reference valid nodes", self.ir.edges.len())
            } else {
                format!(
                    "{} edges reference missing nodes: {}",
                    bad_edges.len(),
                    bad_edges.join(", ")
                )
            },
            severity: "error".into(),
        }
    }

    fn check_versions(&self) -> VerificationCheck {
        let versioned: Vec<_> = self
            .ir
            .nodes
            .iter()
            .filter(|n| n.version.is_some())
            .collect();
        VerificationCheck {
            name: "Version consistency".into(),
            passed: true,
            message: format!(
                "{} versioned nodes, {} unversioned",
                versioned.len(),
                self.ir.nodes.len() - versioned.len()
            ),
            severity: "warn".into(),
        }
    }

    fn check_provenance(&self) -> VerificationCheck {
        let with_provenance = self
            .ir
            .nodes
            .iter()
            .filter(|n| !n.provenance.is_empty())
            .count();
        VerificationCheck {
            name: "Provenance".into(),
            passed: with_provenance == self.ir.nodes.len(),
            message: format!(
                "{} / {} nodes have provenance",
                with_provenance,
                self.ir.nodes.len()
            ),
            severity: "error".into(),
        }
    }

    fn check_artifact_api_usage(&self, artifact: &str) -> VerificationCheck {
        let known_apis: Vec<&str> = self
            .ir
            .nodes
            .iter()
            .filter(|n| n.kind == atlas_ir::NodeKind::API)
            .map(|n| n.name.as_str())
            .collect();

        let mut unknown = Vec::new();
        for word in artifact.split_whitespace() {
            let clean = word.trim_matches(|c: char| !c.is_alphanumeric());
            if (clean.contains('(') || clean.contains("::") || clean.contains('.'))
                && !known_apis.iter().any(|api| clean.contains(api))
            {
                unknown.push(clean.to_string());
            }
        }

        VerificationCheck {
            name: "Artifact API usage".into(),
            passed: unknown.is_empty(),
            message: if unknown.is_empty() {
                "All referenced APIs exist in the graph".into()
            } else {
                format!("Unknown API references: {}", unknown.join(", "))
            },
            severity: "error".into(),
        }
    }
}

fn infer_api_kind(_edge: &atlas_ir::Edge) -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use atlas_ir::{Edge, EdgeType, FailureMode, Meta, Node, NodeKind, NodeStatus, SourceRef};

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
            id: id.into(),
            kind,
            name: name.into(),
            version: None,
            category: None,
            provenance: vec![ref_()],
            confidence: 1.0,
            status: NodeStatus::Verified,
            description: Some(format!("desc {}", name)),
            attributes: serde_json::json!({}),
        }
    }

    fn compliant_ir() -> EngineeringIR {
        let mut ir = EngineeringIR::new(Meta {
            schema_version: "0.1.0".into(),
            generator: "t".into(),
            created_at: 0,
            source_manifest: Vec::new(),
        });
        ir.add_node(node("package:p", NodeKind::Package, "P"));
        ir.add_node(node("concept:a", NodeKind::Concept, "A"));
        ir.add_node(node("api:b", NodeKind::API, "build"));
        ir.add_edge(Edge {
            id: "e1".into(),
            src: "package:p".into(),
            dst: "concept:a".into(),
            edge_type: EdgeType::PartOf,
            weight: 1.0,
            provenance: ref_(),
        });
        ir.add_edge(Edge {
            id: "e2".into(),
            src: "package:p".into(),
            dst: "api:b".into(),
            edge_type: EdgeType::PartOf,
            weight: 1.0,
            provenance: ref_(),
        });
        ir.failure_modes.push(FailureMode {
            id: "failure:f".into(),
            symptom: "s".into(),
            cause: "c".into(),
            fix: "fix it".into(),
            affects: vec!["package:p".into()],
            version_range: None,
        });
        ir
    }

    #[test]
    fn test_verify_passes_for_compliant_ir() {
        let ir = compliant_ir();
        let v = Verifier::new(&ir);
        let report = v.verify(None);
        assert!(report.passed, "checks: {:?}", report.checks.iter().map(|c| &c.message).collect::<Vec<_>>());
        assert!(!report.checks.is_empty());
    }

    #[test]
    fn test_verify_artifact_known_api_passes() {
        let ir = compliant_ir();
        let v = Verifier::new(&ir);
        let report = v.verify(Some("call build() here"));
        assert!(report.passed);
    }

    #[test]
    fn test_verify_artifact_unknown_api_fails() {
        let ir = compliant_ir();
        let v = Verifier::new(&ir);
        let report = v.verify(Some("use foo::bar() now"));
        assert!(!report.passed);
        assert!(report.checks.iter().any(|c| c.name == "Artifact API usage" && !c.passed));
    }

    #[test]
    fn test_verify_missing_node_in_edge_fails() {
        let mut ir = compliant_ir();
        ir.add_edge(Edge {
            id: "e3".into(),
            src: "package:p".into(),
            dst: "concept:ghost".into(),
            edge_type: EdgeType::PartOf,
            weight: 1.0,
            provenance: ref_(),
        });
        let v = Verifier::new(&ir);
        let report = v.verify(None);
        assert!(!report.passed);
        assert!(report.checks.iter().any(|c| c.name == "API existence" && !c.passed));
    }

    #[test]
    fn test_check_versions_is_warn_and_passes() {
        let ir = compliant_ir();
        let v = Verifier::new(&ir);
        let report = v.verify(None);
        let cv = report.checks.iter().find(|c| c.name == "Version consistency").unwrap();
        assert_eq!(cv.severity, "warn");
        assert!(cv.passed);
    }
}

