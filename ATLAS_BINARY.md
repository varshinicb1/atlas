# Atlas Binary (`.atlas`)

The `.atlas` file is the compiled, loadable artifact of the Engineering IR.
Design targets (OS-grade, not web-grade):

- **Instant load** via memory mapping (no full parse).
- **Partial / lazy loading** of sections and subgraphs.
- **Versioned** schema + content checksums.
- **Incremental updates** (patch sections without rewrite).
- **Compression** (per-section, usually Zstd).
- **Graph traversal** supported directly over mmap'd structures.

It sits alongside `.apk`, `.exe`, `.class`, `.pt`, `.gguf` as a *knowledge*
artifact.

## 1. File layout

```
┌─────────────────────────────────────────────┐
│ Magic: "ATLAS\0" (6 bytes)                   │
├─────────────────────────────────────────────┤
│ Header (fixed)                               │
│  - schema_major, schema_minor                │
│  - flags (endian, compressed, signed)        │
│  - section_count                             │
│  - toc_offset, toc_length                    │
│  - content_checksum (blake3)                 │
│  - total_length                              │
├─────────────────────────────────────────────┤
│ Section 0: Metadata (package, version, manifest)│
│ Section 1: Nodes (CSR / columnar)            │
│ Section 2: Edges (CSR per edge type)         │
│ Section 3: Decision Trees                    │
│ Section 4: Examples                          │
│ Section 5: Failure Modes                     │
│ Section 6: Verification Rules                │
│ Section 7: Embeddings (HNSW)                 │
│ Section 8: Indices (symbol/type/version)     │
│ Section 9: Provenance                        │
├─────────────────────────────────────────────┤
│ Table of Contents (TOC)                       │
│  per section: id, offset, length, comp, hash │
├─────────────────────────────────────────────┤
│ Footer                                        │
│  - TOC hash, signature (ed25519, opt)        │
└─────────────────────────────────────────────┘
```

## 2. Header details

| Field | Size | Notes |
| --- | --- | --- |
| magic | 6 | `41 54 4C 41 53 00` |
| schema_major | u16 | Runtime refuses higher major |
| schema_minor | u16 | |
| flags | u32 | bit0 little-endian, bit1 zstd, bit2 signed |
| section_count | u16 | |
| toc_offset | u64 | absolute |
| toc_length | u64 | |
| content_checksum | 32 | blake3 of all sections |
| total_length | u64 | |

## 3. Columnar node storage (Section 1)

Nodes stored column-wise for cache-friendly traversal:

```
node_id[]      // fixed-width stable ids (u64 offset into string pool)
node_kind[]    // u8 enum
node_name_off[]// into string pool
node_version[]
node_status[]  // u8
node_conf[]    // f32
node_prop_off[]// into provenance section
node_attr_off[]// into attr blob (kind-specific optional fields, msgpack)
string_pool     // deduped strings, mmap-shared
```

Edges (Section 2) follow CSR as in `GRAPH_MODEL.md`, one sub-array per edge type
+ reverse arrays.

## 4. Loading semantics

1. `mmap()` whole file (read-only). No copy.
2. Validate magic + schema_major + content_checksum.
3. Read TOC; sections remain lazily paged by the OS until touched.
4. `expand(node)` triggers page faults only for the subgraph's sections.
5. If `flags.signed`, verify footer signature against trusted registry key.

## 5. Compression

- Per-section Zstd at a level tuned at compile time (default 3).
- Compressed sections are inflated on first access into an anonymous mmap
  region; the on-disk compressed bytes stay mmap'd for checksum.

## 6. Incremental update

A `.atlas.patch` carries:

```
{ base_checksum, added_nodes[], changed_nodes[], removed_ids[], new_edges[] }
```

Applied by the Compiler/runtime without full rebuild. Patch checksum chained
into content_checksum.

## 7. Determinism

Same IR + same compiler version → byte-identical `.atlas` **except** header
timestamps and checksums. CI enforces reproducible builds.

## 8. Size budget (targets)

| Bundle | Nodes | Target size |
| --- | --- | --- |
| `flutter@3.24` | ~40k | < 25 MB (zstd) |
| `supabase` | ~8k | < 6 MB |
| Private repo (mid) | ~5k | < 4 MB |

Embeddings are the dominant cost; stored as `f16` quantized HNSW.
