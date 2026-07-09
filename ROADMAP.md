# Roadmap

Phased delivery plan. Each phase produces a shippable, usable slice.

---

## Phase 0 — Foundation (weeks 1-6)

**Goal: single package compile + load + answer one question. Prove the thesis.**

- [ ] Engineering IR schema (ZSON) finalized.
- [ ] Rust project scaffold: `compiler/`, `runtime/`, `cli/`.
- [ ] `md_frontend` parser: converts structured Markdown docs into IR nodes.
- [ ] `openapi_frontend` parser: OpenAPI spec → API/Protocol nodes.
- [ ] IR Builder + Validator (dangling-edge / provenance checks).
- [ ] `.atlas` binary serializer (header, nodes section, edges section, zstd).
- [ ] `.atlas` loader (mmap, header validate, lazy expand).
- [ ] `atlas solve` pipeline stub: intent → graph nav → hardcoded "only graph facts" output.
- [ ] **Demo:** compile `flutter/riverpod` docs into `.atlas`; answer "What class should I use
      if I need AsyncNotifier?" by graph traversal only, 0 model calls.

---

## Phase 1 — Runtime + Decision Engine (weeks 7-12)

**Goal: replace RAG for a small domain. 1B model solves Flutter state problems.**

- [ ] Full query pipeline (Intent → Classify → Navigate → Resolve → Reason → Verify → Response).
- [ ] Decision tree format + Decision Engine (LLM-free fast path).
- [ ] Embedding index (HNSW) + hybrid search (symbol + NN).
- [ ] Memory hierarchy: L0–L3 (conversation → working → bundle → indexed).
- [ ] Verifier: API-exists, version-match, compile-check (via shell command).
- [ ] Python SDK: `atlas.solve()`, `atlas.decide()`, `atlas.load()`.
- [ ] CLI: `atlas install`, `atlas solve`, `atlas decide`.
- [ ] 1B model integration (Phi-3-mini or similar) as the Reasoner.
- [ ] **Demo:** "Solve offline synced state in Flutter" → 0 fabricated APIs.
- [ ] Eval suite: 200 Flutter questions; ≥ 95% factual precision.

---

## Phase 2 — Studio + MCP (weeks 13-18)

**Goal: visual knowledge maps + agent integration.**

- [ ] Atlas Studio scaffold (Next.js + React + Tauri).
- [ ] Interactive graph viewer (Cytoscape / React Flow): zoom from Packages → Concepts → APIs.
- [ ] Node search + filter in UI.
- [ ] Diagram engine: auto-layout architectural diagrams from graph.
- [ ] MCP server with `atlas_search`, `atlas_solve`, `atlas_decide`, `atlas_navigate`.
- [ ] Editor/authoring mode: inspect IR, fix edges, tag nodes.
- [ ] **Demo:** Install Flutter atlas, open Studio, zoom from Riverpod → AsyncNotifier → `watch()`.
- [ ] **Demo:** Agent (Claude/Cursor) uses MCP tools to answer a 5-step engineering problem.

---

## Phase 3 — Ecosystem (weeks 19-30)

**Goal: community can publish, find, and install packages.**

- [ ] Atlas Package Registry (R2-backed, signed index).
- [ ] `atlas publish`, `atlas install` from registry.
- [ ] Signing + supply-chain verification.
- [ ] 10 reference packages: `flutter`, `supabase`, `react`, `next`, `tailwind`,
      `fastapi`, `go`, `postgres`, `kubernetes`, `aws-sdk`.
- [ ] Decision tree authoring: CLI and Studio tools.
- [ ] CI integration: `atlas verify` in GitHub Actions.
- [ ] **Demo:** `atlas install flutter@3.24 && atlas install supabase && atlas solve "auth flow"`.

---

## Phase 4 — Platform (beyond week 30)

- [ ] Incremental compiler (`.atlas.patch`).
- [ ] Private registries + on-prem.
- [ ] IDE plugins (VS Code, JetBrains).
- [ ] `atlas doctor` diagnostics. (planned)
- [ ] Community frontends (PDF, video transcript, RFC → IR). (planned)
- [ ] LSP integration. (planned)
- [ ] 20+ published packages. (planned)
- [ ] Performance: sub-ms decision tree walk; 10K node traversal < 50 µs. (planned)
