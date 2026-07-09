# Compiler

The Atlas Compiler (Rust) lowers heterogeneous engineering sources into the
Engineering IR and then into a `.atlas` binary. It is Atlas's LLVM: deterministic,
incremental, and frontend-agnostic.

## 1. Pipeline

```
Raw Source
   │
   ▼
┌─────────────┐
│ Parser      │  per-source frontend (Markdown, OpenAPI, GitHub, PDF, src)
└─────────────┘
   ▼
┌─────────────┐
│ Normalizer   │  canonical encodings, dedupe, unicode, stable ids
└─────────────┘
   ▼
┌──────────────────┐
│ Semantic Extractor│  LLM-assisted: pull Concepts/APIs/params from prose
└──────────────────┘
   ▼
┌──────────────────────┐
│ Relationship Extractor│  derive typed edges (DEPENDS_ON, SOLVES, …)
└──────────────────────┘
   ▼
┌────────────────┐
│ Ontology Mapper │  map to closed node-kind / edge-type vocabulary
└────────────────┘
   ▼
┌────────────────────┐
│ Knowledge Validator │  schema check, dangling-edge check, provenance check
└────────────────────┘
   ▼
┌────────────────┐
│ Reference Tracker│  resolve every REFERENCES to a real node or drop
└────────────────────┘
   ▼
┌────────────────┐
│ Version Manager │  stamp versions, build COMPATIBLE_WITH / MIGRATES_TO
└────────────────┘
   ▼
┌────────────┐
│ IR Builder  │  emit EngineeringIR (ZSON)
└────────────┘
   ▼
┌────────────────┐
│ Binary Writer   │  serialize → .atlas (ATLAS_BINARY.md)
└────────────────┘
```

## 2. Frontends (parsers)

| Source | Frontend | Notes |
| --- | --- | --- |
| Markdown/docs | `md_frontend` | tree-sitter / pulldown-cmark |
| OpenAPI/Swagger | `openapi_frontend` | spec → API/Protocol nodes |
| GitHub repo | `repo_frontend` | tree-sitter per language → Class/Function/API |
| PDF / papers | `pdf_frontend` | textract + layout → Concept nodes |
| RFC | `rfc_frontend` | section parser |
| MCP server | `mcp_frontend` | introspect tools → API nodes |
| Video transcript | `transcript_frontend` | ASR text → Concept/Example |

Each frontend outputs **unstructured candidate nodes**; the Semantic Extractor
(shown below) structures them.

## 3. Semantic Extractor — the only LLM step in compilation

Unlike the runtime, the compiler *may* use a model to lift prose → structure
(parameters, signatures, relationships). Critical rule: **extraction is
proposal; validation is authority.** Every extracted fact is checked against the
source (`Reference Tracker`) and marked `UNVERIFIED` until confirmed. No model
output becomes `VERIFIED` without a source anchor.

## 4. Determinism

- LLM extraction is run with fixed temperature=0 and pinned model version; its
  output is cached keyed by source hash. Recompiles reuse cache → identical IR.
- All maps serialized in sorted order. Checksums computed last.

## 5. Incremental compilation

- Source manifest hashes each input. Only changed inputs re-run their frontend
  + downstream stages; unchanged node ids are preserved. Produces a `.atlas.patch`.

## 6. CLI surface

```bash
atlas compile ./flutter-docs --out flutter@3.24.atlas
atlas compile --emit-ir ./out/ir.zson      # inspect IR
atlas compile --incremental --patch out.patch
atlas compile --verify                       # run Knowledge Validator only
```

## 7. Output contract

The Compiler's only hard contract is the `.atlas` binary + its embedded
`content_checksum`. The Runtime trusts that checksum (after signature check).
