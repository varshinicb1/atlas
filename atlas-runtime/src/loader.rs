use atlas_ir::{EngineeringIR, Indices};
use std::path::Path;

struct ParsedTocEntry {
    name: String,
    offset: u64,
    length: u64,
    checksum: [u8; 32],
}

fn parse_toc(data: &[u8], toc_offset: u64, count: u16) -> Result<Vec<ParsedTocEntry>, anyhow::Error> {
    let mut off = toc_offset as usize;
    let mut entries = Vec::new();
    for _ in 0..count {
        let name_end = data[off..].iter().position(|&b| b == 0)
            .ok_or_else(|| anyhow::anyhow!("Invalid TOC entry (no null terminator)"))?;
        let name = std::str::from_utf8(&data[off..off + name_end])?.to_string();
        off += name_end + 1;

        let offset = u64::from_le_bytes([
            data[off], data[off + 1], data[off + 2], data[off + 3],
            data[off + 4], data[off + 5], data[off + 6], data[off + 7],
        ]);
        off += 8;

        let length = u64::from_le_bytes([
            data[off], data[off + 1], data[off + 2], data[off + 3],
            data[off + 4], data[off + 5], data[off + 6], data[off + 7],
        ]);
        off += 8;

        let checksum: [u8; 32] = {
            let mut c = [0u8; 32];
            c.copy_from_slice(&data[off..off + 32]);
            c
        };
        off += 32;

        entries.push(ParsedTocEntry { name, offset, length, checksum });
    }
    Ok(entries)
}

pub struct AtlasBundle {
    pub ir: EngineeringIR,
    pub path: String,
}

impl AtlasBundle {
    pub fn from_file(path: &Path) -> Result<Self, anyhow::Error> {
        let data = std::fs::read(path)?;
        if data.len() < 6 || &data[..6] != b"ATLAS\0" {
            anyhow::bail!("Not a valid .atlas file: {}", path.display());
        }

        let (_, _, _, section_count, toc_offset, _) = atlas_compiler::binary::read_header(&data)
            .map_err(|e| anyhow::anyhow!("Header parse error: {}", e))?;

        let toc = parse_toc(&data, toc_offset, section_count)?;

        let mut ir = EngineeringIR::new(atlas_ir::Meta {
            schema_version: "0.1.0".into(),
            generator: "atlas-knowledge".into(),
            created_at: 0,
            source_manifest: Vec::new(),
        });

        let mut errors: Vec<String> = Vec::new();

        for entry in &toc {
            let start = entry.offset as usize;
            let end = start + entry.length as usize;
            if end > data.len() {
                anyhow::bail!("Section {} exceeds file bounds", entry.name);
            }
            let compressed = &data[start..end];

            let decompressed = match zstd::decode_all(std::io::Cursor::new(compressed)) {
                Ok(d) => d,
                Err(_) => compressed.to_vec(),
            };

            let computed = blake3::hash(&decompressed);
            if computed.as_bytes() != &entry.checksum {
                anyhow::bail!(
                    "Checksum mismatch for section '{}'",
                    entry.name
                );
            }

            match entry.name.as_str() {
                "meta" => match serde_json::from_slice(&decompressed) {
                    Ok(m) => ir.meta = m,
                    Err(e) => errors.push(format!("meta section: {e}")),
                },
                "ontology" => match serde_json::from_slice(&decompressed) {
                    Ok(o) => ir.ontology = o,
                    Err(e) => errors.push(format!("ontology section: {e}")),
                },
                "nodes" => match serde_json::from_slice(&decompressed) {
                    Ok(n) => ir.nodes = n,
                    Err(e) => errors.push(format!("nodes section: {e}")),
                },
                "edges" => match serde_json::from_slice(&decompressed) {
                    Ok(e) => ir.edges = e,
                    Err(e) => errors.push(format!("edges section: {e}")),
                },
                "decision_trees" => match serde_json::from_slice(&decompressed) {
                    Ok(t) => ir.decision_trees = t,
                    Err(e) => errors.push(format!("decision_trees section: {e}")),
                },
                "examples" => match serde_json::from_slice(&decompressed) {
                    Ok(ex) => ir.examples = ex,
                    Err(e) => errors.push(format!("examples section: {e}")),
                },
                "failure_modes" => match serde_json::from_slice(&decompressed) {
                    Ok(f) => ir.failure_modes = f,
                    Err(e) => errors.push(format!("failure_modes section: {e}")),
                },
                "workflows" => match serde_json::from_slice(&decompressed) {
                    Ok(w) => ir.workflows = w,
                    Err(e) => errors.push(format!("workflows section: {e}")),
                },
                "verification_rules" => match serde_json::from_slice(&decompressed) {
                    Ok(r) => ir.verification_rules = r,
                    Err(e) => errors.push(format!("verification_rules section: {e}")),
                },
                "indices" => match serde_json::from_slice::<Indices>(&decompressed) {
                    Ok(idx) => ir.indices = Some(idx),
                    Err(e) => errors.push(format!("indices section: {e}")),
                },
                _ => {}
            }
        }

        if !errors.is_empty() {
            anyhow::bail!(
                "Binary load had {} error(s): {}",
                errors.len(),
                errors.join("; ")
            );
        }

        if ir.nodes.is_empty() {
            anyhow::bail!("No nodes found in atlas binary (corrupt or empty)");
        }

        log::info!(
            "Loaded bundle from {} ({} nodes, {} edges, {} decision trees)",
            path.display(),
            ir.nodes.len(),
            ir.edges.len(),
            ir.decision_trees.len(),
        );

        Ok(Self {
            ir,
            path: path.to_string_lossy().to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use atlas_compiler::binary;
    use atlas_ir::{Edge, EdgeType, EngineeringIR, Meta, Node, NodeKind, NodeStatus, SourceRef};

    fn ref_() -> SourceRef {
        SourceRef {
            source_id: "s".into(),
            locator: "l".into(),
            line: None,
            hash: "h".into(),
        }
    }

    fn ir_with_node() -> EngineeringIR {
        let mut ir = EngineeringIR::new(Meta {
            schema_version: "0.1.0".into(),
            generator: "t".into(),
            created_at: 0,
            source_manifest: Vec::new(),
        });
        ir.add_node(Node {
            id: "concept:a".into(),
            kind: NodeKind::Concept,
            name: "Alpha".into(),
            version: None,
            category: None,
            provenance: vec![ref_()],
            confidence: 1.0,
            status: NodeStatus::Verified,
            description: Some("desc".into()),
            attributes: serde_json::json!({}),
        });
        ir.add_edge(Edge {
            id: "e1".into(),
            src: "concept:a".into(),
            dst: "concept:b".into(),
            edge_type: EdgeType::DependsOn,
            weight: 1.0,
            provenance: ref_(),
        });
        ir.indices = Some(atlas_ir::Indices::build(&ir));
        ir
    }

    fn ir_empty() -> EngineeringIR {
        let mut ir = EngineeringIR::new(Meta {
            schema_version: "0.1.0".into(),
            generator: "t".into(),
            created_at: 0,
            source_manifest: Vec::new(),
        });
        ir.indices = Some(atlas_ir::Indices::build(&ir));
        ir
    }

    #[test]
    fn test_from_file_roundtrip() {
        let ir = ir_with_node();
        let path = std::env::temp_dir().join("atlas_loader_test.atlas");
        binary::write_binary(&ir, &path).unwrap();
        let bundle = AtlasBundle::from_file(&path).unwrap();
        assert_eq!(bundle.ir.nodes.len(), 1);
        assert_eq!(bundle.ir.edges.len(), 1);
        assert_eq!(bundle.ir.nodes[0].id, "concept:a");
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_from_file_rejects_bad_magic() {
        let path = std::env::temp_dir().join("atlas_loader_bad.atlas");
        std::fs::write(&path, b"this is not an atlas file").unwrap();
        assert!(AtlasBundle::from_file(&path).is_err());
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_from_file_rejects_empty_nodes() {
        let ir = ir_empty();
        let path = std::env::temp_dir().join("atlas_loader_empty.atlas");
        binary::write_binary(&ir, &path).unwrap();
        assert!(AtlasBundle::from_file(&path).is_err());
        let _ = std::fs::remove_file(&path);
    }
}

