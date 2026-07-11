#[cfg(test)]
mod tests {
    use atlas_compiler::Compiler;
    use atlas_compiler::binary;
    use atlas_ir::{Edge, EdgeType, Node, NodeKind, NodeStatus, SourceRef};

    fn node(id: &str, name: &str) -> Node {
        Node {
            id: id.to_string(),
            kind: NodeKind::Concept,
            name: name.to_string(),
            version: None,
            category: None,
            provenance: vec![SourceRef {
                source_id: "s".into(),
                locator: "l".into(),
                line: None,
                hash: "h".into(),
            }],
            confidence: 1.0,
            status: NodeStatus::Verified,
            description: None,
            attributes: serde_json::json!({}),
        }
    }

    #[test]
    fn test_duplicate_edge_id_rejected() {
        let mut c = Compiler::new();
        let edge = Edge {
            id: "e:dup".into(),
            src: "a".into(),
            dst: "b".into(),
            edge_type: EdgeType::DependsOn,
            weight: 1.0,
            provenance: SourceRef {
                source_id: "s".into(),
                locator: "l".into(),
                line: None,
                hash: "h".into(),
            },
        };
        assert!(c.add_edge(edge.clone()).is_ok());
        assert!(c.add_edge(edge).is_err());
    }

    #[test]
    fn test_add_source_records_manifest() {
        let mut c = Compiler::new();
        let r = c.add_source("src1", std::path::Path::new("src1.md"), Some("hello world"));
        assert!(r.is_ok());
        let ir = c.build().unwrap();
        assert_eq!(ir.meta.source_manifest.len(), 1);
        assert_eq!(ir.meta.source_manifest[0].source_id, "src1");
        // build must have produced indices with embeddings
        assert!(ir.indices.is_some());
        let idx = ir.indices.unwrap();
        assert!(idx.embeddings.is_empty()); // no nodes yet
    }

    #[test]
    fn test_build_with_nodes_produces_embeddings() {
        let mut c = Compiler::new();
        c.add_node(node("concept:a", "Alpha")).unwrap();
        c.add_node(node("concept:b", "Beta")).unwrap();
        let ir = c.build().unwrap();
        let idx = ir.indices.expect("indices built");
        assert_eq!(idx.embeddings.len(), 2);
        assert_eq!(idx.embeddings[0].len(), atlas_ir::embedding::EMBEDDING_DIM);
        assert_eq!(idx.symbol_index.get("alpha"), Some(&"concept:a".to_string()));
    }

    #[test]
    fn test_write_then_is_atlas_file() {
        let mut c = Compiler::new();
        c.add_node(node("concept:a", "Alpha")).unwrap();
        let ir = c.build().unwrap();
        let path = std::env::temp_dir().join("atlas_compiler_itest.atlas");
        binary::write_binary(&ir, &path).unwrap();
        assert!(binary::is_atlas_file(&path));
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_compiler_default_new() {
        let mut c = Compiler::default();
        let ir = c.build().unwrap();
        assert!(ir.nodes.is_empty());
    }
}
