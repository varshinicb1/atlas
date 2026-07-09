# Architecture

Atlas is one system with four language zones. Each zone is chosen for the
workload it performs, not for fashion:

```
                    Atlas Studio  (Next.js + React + Tauri)
                              │
                    Atlas SDK  (Python)
                              │
   ═══════════════════════════╪══════════════════════════
                    Atlas Runtime  (Rust)
   ═══════════════════════════╪══════════════════════════
     Graph Engine │ Knowledge │ Cache │ Search │ Binary │
     Engine       │ Engine    │Engine │Engine │ Loader │
                  │           │       │       │        │
              Decision Engine │ Verifier │ Compiler │ Package Mgr
   ═══════════════════════════╪══════════════════════════
                        .atlas binaries
   ═══════════════════════════╪══════════════════════════
     GitHub │ Docs │ RFCs │ OpenAPI │ MCP │ Source │ Registries
```

## 1. Why this split

| Zone | Language | Why |
| --- | --- | --- |
| Compiler | **Rust** | LLVM-class work: parsing, graph build, compression, incremental compile, memory-mapped output. |
| Runtime | **Rust** | Systems work: graph traversal, cache hierarchy, binary parsing, lazy loading, scheduling. |
| CLI | **Rust** | Tiny static binary, fast (`atlas …`). Like `ripgrep`/`uv`. |
| Package Manager | **Rust** | Resolution, fetching, verification — same shape as `cargo`/`pnpm`. |
| Studio | **TypeScript** | Interactive graphs/maps/editors. Same stack as VS Code, Obsidian, Figma. |
| Diagram engine | **TypeScript** | D3, Cytoscape, React Flow, Mermaid, Excalidraw. |
| MCP server | **TypeScript** | Almost all MCP servers today are TS/Node. |
| SDK | **Python** | Ergonomic wrapper; calls into the Rust runtime via FFI/grpc. |
| Orchestrator | **Python** | LangGraph, PydanticAI, vLLM, PyTorch — the AI ecosystem already lives here. |

**Storage:** not PostgreSQL initially. Use **RocksDB** (key–value, embedded) +
**SQLite** (relational indices) + **memory-mapped `.atlas` files**. LLVM doesn't
use PostgreSQL; neither does Atlas's hot path.

## 2. Inter-zone contracts

- **Compiler → Runtime:** the `.atlas` binary (defined in `ATLAS_BINARY.md`). The
  only contract that matters. Both sides are Rust, so the loader is zero-copy.
- **Runtime → SDK:** a C ABI (`atlas.h`) exposed via `cdylib`. Python binds it
  with `pyo3`.
- **Runtime → MCP:** the Runtime is wrapped by a TypeScript MCP server that calls
  the Rust core over stdin/stdout or a local socket.
- **SDK/Orchestrator → Runtime:** the Orchestrator (Python) calls `atlas.solve()`
  and feeds the returned structured knowledge to the reasoning model.

## 3. Runtime sub-engines

| Engine | Responsibility |
| --- | --- |
| **Graph Engine** | Node/edge storage, typed traversal, adjacency indices. |
| **Knowledge Engine** | Concept/API/example resolution, ontology queries. |
| **Cache Engine** | CPU-cache-like hierarchy (working → project → bundle → docs). |
| **Search Engine** | Hybrid: exact symbol lookup + embedding nearest-neighbour. |
| **Binary Loader** | Memory-map `.atlas`, lazy section expansion, checksums. |
| **Decision Engine** | Problem → recommendation via decision trees. |
| **Verifier** | Compile/lint/API-existence/version checks on generated output. |
| **Compiler** (hosted in runtime for incremental) | Re-compile small deltas without a full rebuild. |
| **Package Manager** | Resolve/fetch/verify bundles. |

## 4. Memory hierarchy (like a CPU)

```
 Registers        → conversation context (active nodes)
 L1 Working Mem   → current project graph slice
 L2 Atlas Binary  → loaded .atlas (mmap)
 L3 Indexed Docs  → on-disk indices (RocksDB/SQLite)
 L4 MCP Servers   → live adapters (Flutter/GitHub/Firebase)
 L5 Internet      → last resort, never trusted for facts
```

The Runtime always resolves from the smallest, cheapest, most-trusted level
first and only descends when necessary.

## 5. Determinism & versioning

- Every `.atlas` carries a **schema version**, a **content checksum**, and a
  **source manifest** (every input file + its hash).
- Same manifest → identical IR. Reproducible builds are a hard requirement.
- Multiple versions of the same package coexist; the Runtime selects by the
  project's pinned constraint.

## 6. Failure model

- Unknown API → **refuse**, do not guess. Return "not in graph" with candidate
  nearest nodes.
- Broken edge / missing provenance → node marked `UNVERIFIED`, excluded from
  authoritative answers.
- Checksum mismatch on load → binary rejected; Package Manager re-fetches.
