use crate::verify::VerificationCheck;
use atlas_ir::EngineeringIR;

pub struct ComplianceChecker<'a> {
    ir: &'a EngineeringIR,
}

impl<'a> ComplianceChecker<'a> {
    pub fn new(ir: &'a EngineeringIR) -> Self {
        Self { ir }
    }

    pub fn check_policy(&self, policy: &str) -> Vec<VerificationCheck> {
        let checks = match policy {
            "eu-ai-act" => self.check_eu_ai_act(),
            "soc2" | "soc-2" => self.check_soc2(),
            "hipaa" => self.check_hipaa(),
            _ => vec![VerificationCheck {
                name: "Unknown policy".into(),
                passed: false,
                message: format!("Unknown compliance policy: {}. Supported: eu-ai-act, soc2, hipaa", policy),
                severity: "error".into(),
            }],
        };
        for c in &checks {
            if !c.passed {
                log::warn!("Compliance check '{}' failed: {}", c.name, c.message);
            }
        }
        checks
    }

    /// EU AI Act compliance checks (effective August 2, 2026)
    /// Requirements:
    ///   - Art. 13: Transparency — AI decisions must be explainable
    ///   - Art. 12: Traceability — decisions must be logged and replayable
    ///   - Art. 15: Accuracy — outputs must be verifiable against ground truth
    ///   - Art. 14: Human oversight — critical decisions allow human intervention
    fn check_eu_ai_act(&self) -> Vec<VerificationCheck> {
        vec![
            // Art. 13: Every node should have a description for explainability
            self.check_descriptions(),
            // Art. 15: Decision trees should have terminal rationales
            self.check_decision_tree_rationales(),
            // Art. 12: Provenance tracking
            self.check_provenance(),
            // Art. 14: Decision trees should have agent instructions for human handoff
            self.check_agent_instructions(),
            // Art. 13: Edge types should be semantically meaningful
            self.check_edge_semantics(),
            // General: Node ID format consistency
            self.check_id_format(),
        ]
    }

    /// SOC 2 compliance: security, availability, processing integrity, confidentiality, privacy
    /// Meaningful check: ALL nodes must have provenance AND descriptions AND at least one edge exists.
    /// This ensures audit trails (provenance), explainability (descriptions), and connectivity (edges).
    fn check_soc2(&self) -> Vec<VerificationCheck> {
        let with_prov = self.ir.nodes.iter().filter(|n| !n.provenance.is_empty()).count();
        let with_desc = self.ir.nodes.iter().filter(|n| n.description.is_some()).count();
        vec![
            VerificationCheck {
                name: "SOC2: Audit trail completeness (provenance)".into(),
                passed: with_prov == self.ir.nodes.len(),
                message: format!("{}/{} nodes have provenance for audit trails", with_prov, self.ir.nodes.len()),
                severity: "error".into(),
            },
            VerificationCheck {
                name: "SOC2: Processing integrity (descriptions)".into(),
                passed: with_desc == self.ir.nodes.len(),
                message: format!("{}/{} nodes have descriptions for processing integrity", with_desc, self.ir.nodes.len()),
                severity: "error".into(),
            },
            VerificationCheck {
                name: "SOC2: Knowledge graph connectivity".into(),
                passed: !self.ir.edges.is_empty(),
                message: format!("{} edges connect the knowledge graph", self.ir.edges.len()),
                severity: "error".into(),
            },
        ]
    }

    /// HIPAA compliance: healthcare data privacy and security
    /// Meaningful check: ALL failure modes must have a fix (no unresolved vulnerabilities).
    fn check_hipaa(&self) -> Vec<VerificationCheck> {
        let with_fix = self.ir.failure_modes.iter().filter(|f| !f.fix.trim().is_empty()).count();
        vec![
            self.check_provenance(),
            VerificationCheck {
                name: "HIPAA: Vulnerability remediation (failure mode fixes)".into(),
                passed: self.ir.failure_modes.is_empty() || with_fix == self.ir.failure_modes.len(),
                message: format!("{}/{} failure modes have fixes for vulnerability remediation", with_fix, self.ir.failure_modes.len()),
                severity: "error".into(),
            },
        ]
    }

    fn check_descriptions(&self) -> VerificationCheck {
        let with_desc = self.ir.nodes.iter().filter(|n| n.description.is_some()).count();
        VerificationCheck {
            name: "EU AI Act Art. 13: Explainability (descriptions)".into(),
            passed: with_desc >= self.ir.nodes.len() / 2,
            message: format!("{}/{} nodes have descriptions for explainability", with_desc, self.ir.nodes.len()),
            severity: "error".into(),
        }
    }

    fn check_decision_tree_rationales(&self) -> VerificationCheck {
        let trees = &self.ir.decision_trees;
        if trees.is_empty() {
            return VerificationCheck {
                name: "EU AI Act Art. 15: Decision tree rationales".into(),
                passed: true,
                message: "No decision trees to validate (not applicable)".into(),
                severity: "warn".into(),
            };
        }
        let mut total_terminals = 0;
        let mut with_rationale = 0;
        for tree in trees {
            for node in &tree.nodes {
                if let Some(ref terminal) = node.terminal {
                    total_terminals += 1;
                    if !terminal.rationale.trim().is_empty() && terminal.rationale != "TODO: Explain why this recommendation is correct."
                        && terminal.rationale != "TODO: Explain the alternative."
                    {
                        with_rationale += 1;
                    }
                }
            }
        }
        let passed = total_terminals == 0 || with_rationale >= total_terminals / 2;
        VerificationCheck {
            name: "EU AI Act Art. 15: Accuracy (terminal rationales)".into(),
            passed,
            message: format!("{}/{} terminal nodes have rationales", with_rationale, total_terminals),
            severity: "error".into(),
        }
    }

    fn check_provenance(&self) -> VerificationCheck {
        let with_prov = self.ir.nodes.iter().filter(|n| !n.provenance.is_empty()).count();
        VerificationCheck {
            name: "EU AI Act Art. 12: Traceability (provenance)".into(),
            passed: with_prov == self.ir.nodes.len(),
            message: format!("{}/{} nodes have provenance tracking", with_prov, self.ir.nodes.len()),
            severity: "error".into(),
        }
    }

    fn check_agent_instructions(&self) -> VerificationCheck {
        let trees = &self.ir.decision_trees;
        let mut total_terminals = 0;
        let mut with_instructions = 0;
        for tree in trees {
            for node in &tree.nodes {
                if let Some(ref terminal) = node.terminal {
                    total_terminals += 1;
                    if terminal.agent_instructions.is_some() {
                        with_instructions += 1;
                    }
                }
            }
        }
        let passed = total_terminals == 0 || with_instructions >= total_terminals / 2;
        VerificationCheck {
            name: "EU AI Act Art. 14: Human oversight (agent instructions)".into(),
            passed,
            message: format!("{}/{} terminal nodes have agent instructions for human handoff", with_instructions, total_terminals),
            severity: "warn".into(),
        }
    }

    fn check_edge_semantics(&self) -> VerificationCheck {
        let semantic_count = self.ir.edges.iter().filter(|e| {
            matches!(e.edge_type, atlas_ir::EdgeType::DependsOn | atlas_ir::EdgeType::Extends | atlas_ir::EdgeType::PartOf)
        }).count();
        VerificationCheck {
            name: "EU AI Act Art. 13: Relationship semantics".into(),
            passed: semantic_count >= self.ir.edges.len() / 2,
            message: format!("{}/{} edges have semantic types (DependsOn, Extends, PartOf)", semantic_count, self.ir.edges.len()),
            severity: "warn".into(),
        }
    }

    fn check_id_format(&self) -> VerificationCheck {
        let valid = self.ir.nodes.iter().filter(|n| n.id.contains(':')).count();
        VerificationCheck {
            name: "ID format convention".into(),
            passed: valid >= self.ir.nodes.len() / 2,
            message: format!("{}/{} nodes use namespaced IDs (e.g., concept:foo)", valid, self.ir.nodes.len()),
            severity: "warn".into(),
        }
    }
}
