#[cfg(test)]
mod tests {
    use atlas_compiler::binary;
    use atlas_compiler::frontends::decision::DecisionParser;
    use atlas_compiler::frontends::MdDocument;
    use atlas_compiler::Compiler;
    use atlas_knowledge::reasoner::{Reasoner, ReasonContext, TemplateReasoner};
    use atlas_knowledge::Runtime;

    const DEMO_MD: &str = r#"---
kind: Package
id: package:demo
name: Demo Package
version: "1.0.0"
description: A demo package for runtime tests
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

    const DEC_YAML: &str = r#"id: demo_decision
trigger:
  intent: choose
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
      rationale: "Rust is a good fit."
      agent_instructions: "use cargo"
  - id: terminal_go
    terminal:
      recommendation:
        - node_id: "concept:go"
          confidence: 0.7
      rationale: "Go is simpler."
"#;

    fn tmp_path(tag: &str) -> std::path::PathBuf {
        static C: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let n = C.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        std::env::temp_dir().join(format!("atlas_rt_{}_{}_{}.atlas", tag, std::process::id(), n))
    }

    fn make_bundle() -> (Runtime, String, std::path::PathBuf) {
        let mut c = Compiler::new();
        let doc = MdDocument::parse_from_str("demo", std::path::Path::new("demo.md"), DEMO_MD).unwrap();
        let (nodes, edges, examples, failures, workflows) = doc.into_parts();
        for n in nodes {
            c.add_node(n).unwrap();
        }
        for e in edges {
            c.add_edge(e).unwrap();
        }
        for ex in examples {
            c.add_example(ex);
        }
        for f in failures {
            c.add_failure(f);
        }
        for w in workflows {
            c.add_workflow(w);
        }
        let trees = DecisionParser::parse_multi(DEC_YAML).unwrap();
        c.add_decision_trees(trees);
        let ir = c.build().unwrap();
        let path = tmp_path("bundle");
        binary::write_binary(&ir, &path).unwrap();
        let mut rt = Runtime::new();
        let name = rt.load(&path).unwrap();
        (rt, name, path)
    }

    #[test]
    fn test_runtime_load_and_get() {
        let (rt, name, path) = make_bundle();
        let bundle = rt.get(&name).expect("bundle loaded");
        assert_eq!(bundle.ir.nodes.len(), 4);
        assert!(rt.get_workflows(&name).is_some());
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_runtime_solve_match() {
        let (rt, name, path) = make_bundle();
        let res = rt.solve(&name, "Widget").unwrap();
        assert_eq!(res.total_matches, 1);
        assert!(res.confidence > 0.9);
        assert!(res.nodes.iter().any(|n| n.id == "concept:widget"));
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_runtime_solve_no_match() {
        let (rt, name, path) = make_bundle();
        let res = rt.solve(&name, "zzzzznotpresent").unwrap();
        assert_eq!(res.total_matches, 0);
        assert_eq!(res.confidence, 0.0);
        assert!(res.nodes.is_empty());
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_runtime_solve_embedding_fallback() {
        let mut c = Compiler::new();
        let md = "---\nkind: Concept\nid: concept:riverpod\nname: Riverpod\n---\nRiverpod is a reactive state management library for Flutter.\n";
        let doc = MdDocument::parse_from_str("rp", std::path::Path::new("rp.md"), md).unwrap();
        let (nodes, edges, _ex, _f, _w) = doc.into_parts();
        for n in nodes {
            c.add_node(n).unwrap();
        }
        for e in edges {
            c.add_edge(e).unwrap();
        }
        let ir = c.build().unwrap();
        let path = tmp_path("emb");
        binary::write_binary(&ir, &path).unwrap();
        let mut rt = Runtime::new();
        let name = rt.load(&path).unwrap();
        // query shares no name/id substring but matches via embedding
        let res = rt.solve(&name, "state management library").unwrap();
        assert!(res.confidence > 0.0);
        assert!(res.nodes.iter().any(|n| n.id == "concept:riverpod"));
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_runtime_decide() {
        let (rt, name, path) = make_bundle();
        let mut ctx = std::collections::HashMap::new();
        ctx.insert("lang".to_string(), "rust".to_string());
        let res = rt.decide(&name, "please use rust", Some(&ctx)).unwrap();
        assert!(res.is_some());
        let r = res.unwrap();
        assert_eq!(r.tree_id, "demo_decision");
        assert_eq!(r.path.last().unwrap(), "terminal_rust");
        assert_eq!(r.recommendations[0].node_id, "concept:rust");
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_runtime_decide_no_match() {
        let (rt, name, path) = make_bundle();
        let ctx = std::collections::HashMap::new();
        let res = rt.decide(&name, "use python please", Some(&ctx)).unwrap();
        assert!(res.is_none());
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_runtime_verify() {
        let (rt, name, path) = make_bundle();
        let report = rt.verify(&name, None).unwrap();
        assert!(report.passed, "checks: {:?}", report.checks.iter().map(|c| &c.message).collect::<Vec<_>>());
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_runtime_verify_artifact_unknown() {
        let (rt, name, path) = make_bundle();
        let report = rt.verify(&name, Some("call unknown::func()")).unwrap();
        assert!(!report.passed);
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_runtime_verify_with_policy() {
        let (rt, name, path) = make_bundle();
        for policy in ["eu-ai-act", "soc2", "hipaa"] {
            let report = rt.verify_with_policy(&name, policy).unwrap();
            assert!(report.passed, "{policy} failed");
        }
        let unknown = rt.verify_with_policy(&name, "nist").unwrap();
        assert!(!unknown.passed);
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_runtime_load_all() {
        let (_rt, _name, path) = make_bundle();
        let dir = std::env::temp_dir().join(format!("atlas_rt_dir_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let target = dir.join("demo.atlas");
        std::fs::copy(&path, &target).unwrap();
        let _ = std::fs::remove_file(&path);

        let mut rt2 = Runtime::new();
        let count = rt2.load_all(&dir).unwrap();
        assert_eq!(count, 1);
        let _ = std::fs::remove_file(&target);
        let _ = std::fs::remove_dir(&dir);
    }

    #[test]
    fn test_runtime_reasoner_via_solve() {
        let (rt, name, path) = make_bundle();
        let solve = rt.solve(&name, "Widget").unwrap();
        let decide = rt.decide(&name, "use rust", Some(&std::collections::HashMap::new())).ok().and_then(|r| r);
        let ctx = ReasonContext {
            query: "Widget",
            bundle: &solve.bundle,
            confidence: solve.confidence,
            nodes: &solve.nodes,
            total_matches: solve.total_matches,
            decide_result: decide.as_ref(),
        };
        let answer = TemplateReasoner.reason("Widget", &ctx).unwrap();
        assert!(answer.contains("## Answer for: Widget"));
        let _ = std::fs::remove_file(&path);
    }
}
