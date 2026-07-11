use atlas_ir::EngineeringIR;
use serde::{Deserialize, Serialize};

const MAGIC: &[u8; 6] = b"ATLAS\0";
const SCHEMA_MAJOR: u16 = 0;
const SCHEMA_MINOR: u16 = 1;
pub const HEADER_SIZE: u64 = 64;

pub fn write_binary(ir: &EngineeringIR, path: &std::path::Path) -> Result<(), anyhow::Error> {
    let sections = serialize_sections(ir)?;
    let checksum = compute_checksum(&sections);
    let section_start = HEADER_SIZE;

    let mut section_data = Vec::new();
    let mut toc_entries = Vec::new();
    let mut offset = section_start;

    for (id, compressed, hash) in &sections {
        toc_entries.push(TocEntry {
            name: id.clone(),
            offset,
            length: compressed.len() as u64,
            checksum: *hash,
        });
        section_data.extend_from_slice(compressed);
        offset += compressed.len() as u64;
    }

    let toc_offset = offset;
    let mut toc_buf = Vec::new();
    for entry in &toc_entries {
        toc_buf.extend_from_slice(entry.name.as_bytes());
        toc_buf.push(0);
        toc_buf.extend_from_slice(&entry.offset.to_le_bytes());
        toc_buf.extend_from_slice(&entry.length.to_le_bytes());
        toc_buf.extend_from_slice(&entry.checksum);
    }
    let toc_hash = blake3::hash(&toc_buf);

    let mut buf = Vec::new();
    buf.extend_from_slice(MAGIC);
    buf.extend_from_slice(&SCHEMA_MAJOR.to_le_bytes());
    buf.extend_from_slice(&SCHEMA_MINOR.to_le_bytes());
    let flags: u32 = 0b001;
    buf.extend_from_slice(&flags.to_le_bytes());
    buf.extend_from_slice(&(sections.len() as u16).to_le_bytes());
    buf.extend_from_slice(&toc_offset.to_le_bytes());
    buf.extend_from_slice(&checksum);
    buf.extend_from_slice(&0u64.to_le_bytes());
    buf.extend_from_slice(&section_data);
    buf.extend_from_slice(&toc_buf);
    buf.extend_from_slice(toc_hash.as_bytes());

    std::fs::write(path, &buf)?;
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TocEntry {
    name: String,
    offset: u64,
    length: u64,
    checksum: [u8; 32],
}

#[allow(clippy::type_complexity)]
fn serialize_sections(ir: &EngineeringIR) -> Result<Vec<(String, Vec<u8>, [u8; 32])>, anyhow::Error> {
    let mut sections = Vec::new();
    let json = serde_json::to_vec(&ir.meta)?;
    let hash = blake3::hash(&json);
    sections.push(("meta".into(), compress(&json), *hash.as_bytes()));

    let json = serde_json::to_vec(&ir.ontology)?;
    let hash = blake3::hash(&json);
    sections.push(("ontology".into(), compress(&json), *hash.as_bytes()));

    let json = serde_json::to_vec(&ir.nodes)?;
    let hash = blake3::hash(&json);
    sections.push(("nodes".into(), compress(&json), *hash.as_bytes()));

    let json = serde_json::to_vec(&ir.edges)?;
    let hash = blake3::hash(&json);
    sections.push(("edges".into(), compress(&json), *hash.as_bytes()));

    let json = serde_json::to_vec(&ir.decision_trees)?;
    let hash = blake3::hash(&json);
    sections.push(("decision_trees".into(), compress(&json), *hash.as_bytes()));

    let json = serde_json::to_vec(&ir.examples)?;
    let hash = blake3::hash(&json);
    sections.push(("examples".into(), compress(&json), *hash.as_bytes()));

    let json = serde_json::to_vec(&ir.failure_modes)?;
    let hash = blake3::hash(&json);
    sections.push(("failure_modes".into(), compress(&json), *hash.as_bytes()));

    let json = serde_json::to_vec(&ir.workflows)?;
    let hash = blake3::hash(&json);
    sections.push(("workflows".into(), compress(&json), *hash.as_bytes()));

    let json = serde_json::to_vec(&ir.verification_rules)?;
    let hash = blake3::hash(&json);
    sections.push(("verification_rules".into(), compress(&json), *hash.as_bytes()));

    if let Some(ref indices) = ir.indices {
        let json = serde_json::to_vec(indices)?;
        let hash = blake3::hash(&json);
        sections.push(("indices".into(), compress(&json), *hash.as_bytes()));
    }

    Ok(sections)
}

fn compress(data: &[u8]) -> Vec<u8> {
    zstd::encode_all(std::io::Cursor::new(data), 3).unwrap_or_else(|_| data.to_vec())
}

fn compute_checksum(sections: &[(String, Vec<u8>, [u8; 32])]) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new();
    for (_, data, _) in sections {
        hasher.update(data);
    }
    *hasher.finalize().as_bytes()
}

pub fn is_atlas_file(path: &std::path::Path) -> bool {
    std::fs::read(path)
        .ok()
        .and_then(|d| d.get(..6).map(|b| b == MAGIC))
        .unwrap_or(false)
}

#[allow(clippy::type_complexity)]
pub fn read_header(data: &[u8]) -> Result<(u16, u16, u32, u16, u64, [u8; 32]), anyhow::Error> {
    if data.len() < 6 || &data[..6] != MAGIC {
        anyhow::bail!("Not a valid .atlas file");
    }
    let mut off = 6;
    let schema_major = u16::from_le_bytes([data[off], data[off + 1]]);
    off += 2;
    let schema_minor = u16::from_le_bytes([data[off], data[off + 1]]);
    off += 2;
    let flags = u32::from_le_bytes([data[off], data[off + 1], data[off + 2], data[off + 3]]);
    off += 4;
    let section_count = u16::from_le_bytes([data[off], data[off + 1]]);
    off += 2;
    let toc_offset = u64::from_le_bytes([
        data[off], data[off + 1], data[off + 2], data[off + 3],
        data[off + 4], data[off + 5], data[off + 6], data[off + 7],
    ]);
    off += 8;
    let mut checksum = [0u8; 32];
    checksum.copy_from_slice(&data[off..off + 32]);
    Ok((schema_major, schema_minor, flags, section_count, toc_offset, checksum))
}

#[cfg(test)]
mod tests {
    use super::*;
    use atlas_ir::{EngineeringIR, Meta, Node, NodeKind, NodeStatus, SourceRef, Edge, EdgeType};

    fn sample_ir() -> EngineeringIR {
        let mut ir = EngineeringIR::new(Meta {
            schema_version: "0.1.0".into(),
            generator: "test".into(),
            created_at: 0,
            source_manifest: Vec::new(),
        });
        ir.add_node(Node {
            id: "concept:a".into(),
            kind: NodeKind::Concept,
            name: "Alpha".into(),
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
            description: Some("desc".into()),
            attributes: serde_json::json!({}),
        });
        ir.add_edge(Edge {
            id: "e1".into(),
            src: "concept:a".into(),
            dst: "concept:b".into(),
            edge_type: EdgeType::DependsOn,
            weight: 1.0,
            provenance: SourceRef {
                source_id: "s".into(),
                locator: "l".into(),
                line: None,
                hash: "h".into(),
            },
        });
        ir.indices = Some(atlas_ir::Indices::build(&ir));
        ir
    }

    #[test]
    fn test_write_and_header_roundtrip() {
        let ir = sample_ir();
        let path = std::env::temp_dir().join("atlas_bin_test_write.atlas");
        write_binary(&ir, &path).unwrap();
        assert!(is_atlas_file(&path));

        let data = std::fs::read(&path).unwrap();
        let (major, minor, flags, count, _toc, _checksum) = read_header(&data).unwrap();
        assert_eq!(major, 0);
        assert_eq!(minor, 1);
        assert_eq!(flags, 0b001);
        // meta, ontology, nodes, edges, decision_trees, examples, failure_modes,
        // workflows, verification_rules, indices => 10 sections
        assert_eq!(count, 10);
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_is_atlas_file_rejects_non_atlas() {
        let path = std::env::temp_dir().join("atlas_bin_test_not.atlas");
        std::fs::write(&path, b"not an atlas file at all").unwrap();
        assert!(!is_atlas_file(&path));
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_read_header_rejects_bad_magic() {
        let data = b"NOTATLASdata".to_vec();
        assert!(read_header(&data).is_err());
    }
}
