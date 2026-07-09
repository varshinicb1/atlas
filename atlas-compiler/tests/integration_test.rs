#[cfg(test)]
mod tests {
    use atlas_compiler::Compiler;
    use atlas_compiler::frontends::MdDocument;
    use std::path::Path;

    #[test]
    fn test_empty_compile_produces_valid_empty_bundle() {
        let mut compiler = Compiler::new();
        let result = compiler.build();
        assert!(result.is_ok(), "Empty build should produce a valid (empty) bundle");
        let ir = result.unwrap();
        assert!(ir.nodes.is_empty(), "Empty build should have zero nodes");
        assert!(ir.edges.is_empty(), "Empty build should have zero edges");
    }

    #[test]
    fn test_minimal_md_parse() {
        let path = Path::new("../packages/typescript_7_migration.md");
        if path.exists() {
            let doc = MdDocument::parse("test", path);
            assert!(doc.is_ok(), "Should parse existing knowledge package");
        }
    }

    #[test]
    fn test_duplicate_node_id_rejected() {
        use atlas_ir::{Node, NodeKind, NodeStatus};
        let mut compiler = Compiler::new();
        let node = Node {
            id: "test:dup".into(),
            kind: NodeKind::Concept,
            name: "Test".into(),
            version: None,
            category: None,
            provenance: vec![],
            confidence: 1.0,
            status: NodeStatus::Verified,
            description: None,
            attributes: serde_json::json!({}),
        };
        assert!(compiler.add_node(node.clone()).is_ok());
        assert!(compiler.add_node(node).is_err(), "Duplicate node ID should error");
    }
}
