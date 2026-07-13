pub mod decision;

use atlas_ir::{
    DecisionTree, Edge, EdgeType, Example, FailureMode, Node, NodeKind, NodeStatus, SourceRef, Workflow, WorkflowStep,
};
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct Frontmatter {
    #[serde(default)]
    kind: String,
    #[serde(default)]
    id: String,
    #[serde(default)]
    name: String,
    #[serde(default)]
    version: Option<String>,
    #[serde(default)]
    category: Option<String>,
    #[serde(default)]
    dependencies: Vec<String>,
    #[serde(default)]
    concepts: Vec<ConceptEntry>,
    #[serde(default)]
    apis: Vec<ApiEntry>,
    #[serde(default)]
    examples: Vec<ExampleEntry>,
    #[serde(default)]
    failures: Vec<FailureEntry>,
    #[serde(default)]
    workflows: Vec<WorkflowEntry>,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    extends: Vec<String>,
    #[serde(default)]
    implements: Vec<String>,
    #[serde(default)]
    uses: Vec<String>,
    #[serde(default)]
    part_of: Option<String>,
    #[serde(default)]
    solves: Vec<String>,
    #[serde(default)]
    alternatives: Vec<String>,
    #[serde(default)]
    purpose: Option<String>,
    #[serde(default)]
    problem_solved: Option<String>,
    #[serde(default)]
    install: Option<String>,
    #[serde(default)]
    signature: Option<String>,
    #[serde(default)]
    returns: Option<String>,
    #[serde(default)]
    since: Option<String>,
    #[serde(default)]
    decision_tree: Option<serde_yaml::Value>,
}

#[derive(Debug, Deserialize)]
struct ConceptEntry {
    name: String,
    id: String,
    #[serde(default)]
    description: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ApiEntry {
    name: String,
    id: String,
    #[serde(default)]
    signature: Option<String>,
    #[serde(default)]
    returns: Option<String>,
    #[serde(default)]
    description: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ExampleEntry {
    id: String,
    #[serde(default)]
    language: String,
    #[serde(default)]
    #[allow(dead_code)]
    description: Option<String>,
}

#[derive(Debug, Deserialize)]
struct FailureEntry {
    id: String,
    #[serde(default)]
    symptom: Option<String>,
    #[serde(default)]
    cause: Option<String>,
    #[serde(default)]
    fix: Option<String>,
}

#[derive(Debug, Deserialize)]
struct WorkflowEntry {
    name: String,
    id: String,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    steps: Vec<WorkflowStepEntry>,
}

#[derive(Debug, Deserialize)]
struct WorkflowStepEntry {
    order: u32,
    action: String,
}

pub struct MdDocument {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    examples: Vec<Example>,
    failures: Vec<FailureMode>,
    workflows: Vec<Workflow>,
    decision_trees: Vec<DecisionTree>,
}

impl MdDocument {
    pub fn parse(source_id: &str, path: &Path) -> Result<Self, anyhow::Error> {
        let content = std::fs::read_to_string(path)?;
        Self::parse_from_str(source_id, path, &content)
    }

    pub fn parse_from_str(source_id: &str, path: &Path, content: &str) -> Result<Self, anyhow::Error> {
        let hash = blake3::hash(content.as_bytes()).to_hex().to_string();
        let make_ref = |locator: &str, line: Option<u32>| -> SourceRef {
            SourceRef {
                source_id: source_id.to_string(),
                locator: locator.to_string(),
                line,
                hash: hash.clone(),
            }
        };

        let parts: Vec<&str> = content.splitn(3, "---").collect();
        if parts.len() < 3 {
            anyhow::bail!("No YAML frontmatter found in {}", path.display());
        }

        let fm: Frontmatter = serde_yaml::from_str(parts[1])?;
        let body = parts.get(2).unwrap_or(&"").trim();
        let kind = parse_kind(&fm.kind);

        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        let mut examples = Vec::new();
        let mut failures = Vec::new();
        let mut workflows = Vec::new();

        for dep in &fm.dependencies {
            edges.push(Edge {
                id: Edge::new_id(),
                src: fm.id.clone(),
                dst: dep.clone(),
                edge_type: EdgeType::DependsOn,
                weight: 1.0,
                provenance: make_ref("frontmatter.dependencies", None),
            });
        }

        for target in &fm.extends {
            edges.push(Edge {
                id: Edge::new_id(),
                src: fm.id.clone(),
                dst: target.clone(),
                edge_type: EdgeType::Extends,
                weight: 1.0,
                provenance: make_ref("frontmatter.extends", None),
            });
        }

        for target in &fm.implements {
            edges.push(Edge {
                id: Edge::new_id(),
                src: fm.id.clone(),
                dst: target.clone(),
                edge_type: EdgeType::Implements,
                weight: 1.0,
                provenance: make_ref("frontmatter.implements", None),
            });
        }

        for target in &fm.uses {
            edges.push(Edge {
                id: Edge::new_id(),
                src: fm.id.clone(),
                dst: target.clone(),
                edge_type: EdgeType::Uses,
                weight: 1.0,
                provenance: make_ref("frontmatter.uses", None),
            });
        }

        if let Some(parent) = &fm.part_of {
            edges.push(Edge {
                id: Edge::new_id(),
                src: fm.id.clone(),
                dst: parent.clone(),
                edge_type: EdgeType::PartOf,
                weight: 1.0,
                provenance: make_ref("frontmatter.part_of", None),
            });
        }

        for target in &fm.solves {
            edges.push(Edge {
                id: Edge::new_id(),
                src: fm.id.clone(),
                dst: target.clone(),
                edge_type: EdgeType::Solves,
                weight: 1.0,
                provenance: make_ref("frontmatter.solves", None),
            });
        }

        for target in &fm.alternatives {
            edges.push(Edge {
                id: Edge::new_id(),
                src: fm.id.clone(),
                dst: target.clone(),
                edge_type: EdgeType::AlternativeTo,
                weight: 0.8,
                provenance: make_ref("frontmatter.alternatives", None),
            });
        }

        for concept in &fm.concepts {
            nodes.push(Node {
                id: concept.id.clone(),
                kind: NodeKind::Concept,
                name: concept.name.clone(),
                version: None,
                category: None,
                provenance: vec![make_ref(&format!("concept:{}", concept.name), None)],
                confidence: 1.0,
                status: NodeStatus::Verified,
                description: concept.description.clone(),
                attributes: serde_json::json!({}),
            });
            edges.push(Edge {
                id: Edge::new_id(),
                src: fm.id.clone(),
                dst: concept.id.clone(),
                edge_type: EdgeType::PartOf,
                weight: 1.0,
                provenance: make_ref(&format!("concept:{}", concept.name), None),
            });
        }

        for api in &fm.apis {
            nodes.push(Node {
                id: api.id.clone(),
                kind: NodeKind::API,
                name: api.name.clone(),
                version: fm.version.clone(),
                category: None,
                provenance: vec![make_ref(&format!("api:{}", api.name), None)],
                confidence: 1.0,
                status: NodeStatus::Verified,
                description: api.description.clone(),
                attributes: serde_json::json!({
                    "signature": api.signature,
                    "returns": api.returns,
                }),
            });
            edges.push(Edge {
                id: Edge::new_id(),
                src: fm.id.clone(),
                dst: api.id.clone(),
                edge_type: EdgeType::PartOf,
                weight: 1.0,
                provenance: make_ref(&format!("api:{}", api.name), None),
            });
        }

        for ex in &fm.examples {
            let example_id = ex.id.clone();
            examples.push(Example {
                id: example_id.clone(),
                language: ex.language.clone(),
                code: String::new(),
                runnable: true,
                expected_output: None,
                references_node: Some(fm.id.clone()),
            });
            edges.push(Edge {
                id: Edge::new_id(),
                src: fm.id.clone(),
                dst: example_id,
                edge_type: EdgeType::HasExample,
                weight: 1.0,
                provenance: make_ref("frontmatter.examples", None),
            });
        }

        for fe in &fm.failures {
            let fail_id = fe.id.clone();
            failures.push(FailureMode {
                id: fail_id.clone(),
                symptom: fe.symptom.clone().unwrap_or_default(),
                cause: fe.cause.clone().unwrap_or_default(),
                fix: fe.fix.clone().unwrap_or_default(),
                affects: vec![fm.id.clone()],
                version_range: fm.version.clone(),
            });
            edges.push(Edge {
                id: Edge::new_id(),
                src: fm.id.clone(),
                dst: fail_id,
                edge_type: EdgeType::HasFailure,
                weight: 1.0,
                provenance: make_ref("frontmatter.failures", None),
            });
        }

        for wf in &fm.workflows {
            let workflow_id = wf.id.clone();
            nodes.push(Node {
                id: workflow_id.clone(),
                kind: NodeKind::Workflow,
                name: wf.name.clone(),
                version: None,
                category: None,
                provenance: vec![make_ref(&format!("workflow:{}", wf.name), None)],
                confidence: 1.0,
                status: NodeStatus::Verified,
                description: wf.description.clone(),
                attributes: serde_json::json!({}),
            });
            edges.push(Edge {
                id: Edge::new_id(),
                src: fm.id.clone(),
                dst: workflow_id.clone(),
                edge_type: EdgeType::PartOf,
                weight: 1.0,
                provenance: make_ref(&format!("workflow:{}", wf.name), None),
            });
            workflows.push(Workflow {
                id: workflow_id,
                name: wf.name.clone(),
                description: wf.description.clone().unwrap_or_default(),
                steps: wf.steps.iter().map(|s| WorkflowStep {
                    order: s.order,
                    action: s.action.clone(),
                }).collect(),
            });
        }

        let body_desc = if body.len() > 500 {
            format!("{}...", &body[..500])
        } else {
            body.to_string()
        };

        let mut attributes = serde_json::json!({
            "purpose": fm.purpose,
            "problem_solved": fm.problem_solved,
            "install": fm.install,
            "signature": fm.signature,
            "returns": fm.returns,
            "since": fm.since,
        });

        if let Some(ref desc) = fm.description {
            attributes["description"] = serde_json::json!(desc);
        } else {
            attributes["description"] = serde_json::json!(body_desc);
        }

        let id = if fm.id.is_empty() {
            anyhow::bail!("Missing 'id' in frontmatter of {}", path.display());
        } else {
            fm.id.clone()
        };

        let main_node = Node {
            id,
            kind,
            name: fm.name.clone(),
            version: fm.version,
            category: fm.category,
            provenance: vec![make_ref("frontmatter", None)],
            confidence: 1.0,
            status: NodeStatus::Verified,
            description: fm.description.or({
                if body_desc.is_empty() {
                    None
                } else {
                    Some(body_desc)
                }
            }),
            attributes,
        };

        nodes.insert(0, main_node);

        let decision_trees = if let Some(ref dt_value) = fm.decision_tree {
            let yaml_str = serde_yaml::to_string(dt_value)?;
            crate::frontends::decision::DecisionParser::parse_multi(&yaml_str)?
        } else {
            Vec::new()
        };

        Ok(Self {
            nodes,
            edges,
            examples,
            failures,
            workflows,
            decision_trees,
        })
    }

    #[allow(clippy::type_complexity)]
    pub fn into_parts(self) -> (Vec<Node>, Vec<Edge>, Vec<Example>, Vec<FailureMode>, Vec<Workflow>) {
        (self.nodes, self.edges, self.examples, self.failures, self.workflows)
    }

    pub fn decision_trees(&self) -> &[DecisionTree] {
        &self.decision_trees
    }
}

fn parse_kind(s: &str) -> NodeKind {
    match s.to_lowercase().as_str() {
        "concept" => NodeKind::Concept,
        "package" => NodeKind::Package,
        "module" => NodeKind::Module,
        "class" => NodeKind::Class,
        "function" => NodeKind::Function,
        "api" => NodeKind::API,
        "protocol" => NodeKind::Protocol,
        "example" => NodeKind::Example,
        "failure" | "failuremode" => NodeKind::FailureMode,
        "decision" => NodeKind::Decision,
        "architecture" => NodeKind::Architecture,
        "alternative" => NodeKind::Alternative,
        _ => NodeKind::Concept,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    const PKG_MD: &str = r#"---
kind: Package
id: package:demo
name: Demo Package
version: "1.0.0"
description: A demo package for tests
concepts:
  - name: Widget
    id: concept:widget
    description: A UI widget
apis:
  - name: build()
    id: api:build
    signature: "Widget build()"
    returns: Widget
    description: builds a widget
workflows:
  - name: Create
    id: workflow:create
    description: create workflow
    steps:
      - order: 1
        action: "do thing"
failures:
  - id: failure:broken
    symptom: breaks
    cause: bad
    fix: fix it
examples:
  - id: example:one
    language: rust
---
Body text describing the package.
"#;

    #[test]
    fn test_parse_kind_variants() {
        assert_eq!(parse_kind("api"), NodeKind::API);
        assert_eq!(parse_kind("FailureMode"), NodeKind::FailureMode);
        assert_eq!(parse_kind("failuremode"), NodeKind::FailureMode);
        assert_eq!(parse_kind("decision"), NodeKind::Decision);
        assert_eq!(parse_kind("CLASS"), NodeKind::Class);
        assert_eq!(parse_kind("unknown_thing"), NodeKind::Concept);
    }

    #[test]
    fn test_parse_from_str_package() {
        let doc = MdDocument::parse_from_str("demo", Path::new("demo.md"), PKG_MD).unwrap();
        let (nodes, edges, examples, failures, workflows) = doc.into_parts();

        // main + concept + api + workflow nodes
        assert_eq!(nodes.len(), 4);
        assert_eq!(nodes[0].kind, NodeKind::Package);
        assert_eq!(nodes[0].name, "Demo Package");
        assert_eq!(nodes[0].version.as_deref(), Some("1.0.0"));

        // concept + api + workflow nodes present
        let kinds: Vec<NodeKind> = nodes.iter().map(|n| n.kind.clone()).collect();
        assert!(kinds.contains(&NodeKind::Concept));
        assert!(kinds.contains(&NodeKind::API));
        assert!(kinds.contains(&NodeKind::Workflow));

        // edges: concept+api+workflow PartOf, example HasExample, failure HasFailure
        assert_eq!(edges.len(), 5);
        let edge_types: Vec<EdgeType> = edges.iter().map(|e| e.edge_type.clone()).collect();
        assert!(edge_types.contains(&EdgeType::PartOf));
        assert!(edge_types.contains(&EdgeType::HasExample));
        assert!(edge_types.contains(&EdgeType::HasFailure));

        assert_eq!(examples.len(), 1);
        assert_eq!(failures.len(), 1);
        assert_eq!(workflows.len(), 1);
        assert_eq!(workflows[0].steps.len(), 1);
    }

    #[test]
    fn test_parse_from_str_kind_api() {
        let md = "---\nkind: API\nid: api:thing\nname: Thing\n---\nbody\n";
        let doc = MdDocument::parse_from_str("t", Path::new("t.md"), md).unwrap();
        let (nodes, _edges, _ex, _f, _w) = doc.into_parts();
        assert_eq!(nodes[0].kind, NodeKind::API);
        assert_eq!(nodes[0].name, "Thing");
    }

    #[test]
    fn test_parse_no_frontmatter_errors() {
        let r = MdDocument::parse_from_str("x", Path::new("x.md"), "no frontmatter at all");
        assert!(r.is_err());
    }

    #[test]
    fn test_parse_missing_id_errors() {
        let md = "---\nkind: Package\nname: NoId\n---\nbody\n";
        let r = MdDocument::parse_from_str("x", Path::new("x.md"), md);
        assert!(r.is_err());
    }

    #[test]
    fn test_parse_dependencies_and_extends_edges() {
        let md = r#"---
kind: Package
id: package:child
name: Child
dependencies: [package:parent]
extends: [package:base]
uses: [api:other]
alternatives: [package:alt]
---
body
"#;
        let doc = MdDocument::parse_from_str("c", Path::new("c.md"), md).unwrap();
        let (_nodes, edges, _ex, _f, _w) = doc.into_parts();
        let types: Vec<EdgeType> = edges.iter().map(|e| e.edge_type.clone()).collect();
        assert!(types.contains(&EdgeType::DependsOn));
        assert!(types.contains(&EdgeType::Extends));
        assert!(types.contains(&EdgeType::Uses));
        assert!(types.contains(&EdgeType::AlternativeTo));
    }
}
