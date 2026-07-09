# Runtime

The Atlas Runtime (Rust) replaces RAG. It loads `.atlas` binaries and answers
engineering problems by navigating the knowledge graph — the LLM is **one** stage,
not the system.

## 1. Query pipeline

```
Problem (text)
   │
   ▼
┌────────────────┐
│ Intent Detection│  classify: lookup? decision? code-gen? verify?
└────────────────┘
   ▼
┌────────────────────┐
│ Problem Classifier │  domain + constraints (offline? sync? lang? version?)
└────────────────────┘
   ▼
┌────────────────┐
│ Graph Navigator │  symbol lookup + embedding NN + traversal (GRAPH_MODEL)
└────────────────┘
   ▼
┌──────────────────────┐
│ Knowledge Resolver    │  collect nodes/APIs/examples into a context set
└──────────────────────┘
   ▼
┌────────────┐
│ Planner     │  decompose the problem into steps
└────────────┘
   ▼
┌────────────┐
│ Reasoner    │  LLM: synthesize using ONLY the resolved knowledge
└────────────┘
   ▼
┌────────────┐
│ Code Generator│  emit code/answer constrained to graph facts
└────────────┘
   ▼
┌────────────┐
│ Verifier    │  compile/lint/API-exists/version checks (SECURITY §verify)
└────────────┘
   ▼
┌────────────┐
│ Response    │  answer + provenance + confidence
└────────────┘
```

## 2. Memory hierarchy (CPU-cache model)

Resolves from the cheapest, most-trusted level first:

```
L0 Registers       conversation context (active node ids)
L1 Working Memory  current project graph slice (mmap'd subgraph)
L2 Atlas Binary    loaded .atlas (mmap)
L3 Indexed Docs    RocksDB/SQLite on-disk indices
L4 MCP Servers     live adapters (Flutter/GitHub/Firebase)
L5 Internet        last resort; never trusted for facts
```

The Runtime measures "cache hits" and prefers L0–L2 to avoid expensive MCP/net
calls. A `trace` shows which level answered each sub-query.

## 3. Knowledge Resolver rules

- The Reasoner receives **only** nodes fetched from the graph, each tagged with
  `provenance`.
- The Code Generator may reference **only** APIs present in the resolved set.
- Unknown API → Runtime refuses and returns nearest known candidates; never
  fabricates.

## 4. Decision Engine integration

If `Intent Detection` matches a `DecisionTree.trigger`, the Runtime short-circuits
to `decision_walk` (no LLM), returning a recommendation + rationale. See
`DECISION_ENGINE.md`.

## 5. Verifier (non-negotiable)

Every generated artifact passes:

| Check | Source of truth |
| --- | --- |
| API exists | graph node + provenance |
| Version match | node.version vs project constraint |
| Compiles | optional `verification_rules` command |
| Lints | optional command |
| Docs match | node vs official doc hash |
| Breaking changes | node.breaking_changes |

Failing `ERROR`-severity checks → answer marked `UNVERIFIED`, not shipped.

## 6. Runtime API (Rust core, exposed to SDK/MCP)

```
atlas_runtime_load(path)            -> Handle
atlas_solve(handle, problem)        -> Answer{ text, nodes[], confidence, provenance[] }
atlas_navigate(handle, node, depth) -> Subgraph
atlas_decide(handle, problem)       -> Recommendation
atlas_verify(handle, artifact)      -> VerificationReport
```

## 7. Determinism & safety

- Given the same `.atlas` + same problem + same pinned model, the structural
  navigation (everything except the LLM Reasoner step) is deterministic.
- The LLM step is logged with the exact resolved context, so any answer is
  replayable and auditable.
