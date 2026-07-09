# Engineering Intermediate Representation (IR)

The Engineering IR is the canonical, language-agnostic representation of
engineering knowledge. It is the single target every frontend (Markdown, HTML,
PDF, GitHub, source code, OpenAPI, video transcript, MCP server) compiles into,
and the single source the `.atlas` binary is generated from.

It is **not** Markdown, HTML, or "JSON docs." It is a typed, machine-traversable
**property graph** with decision trees and verification rules attached.

## 1. Design goals

1. **Frontend-agnostic.** Any source lowers into the same IR.
2. **Round-trippable.** IR → `.atlas` → IR is lossless for structure.
3. **Queryable.** Every relationship is a typed edge, not prose.
4. **Verifiable.** Every node carries provenance + verification hooks.
5. **Composable.** Bundles merge by node identity.

## 2. Top-level structure

```
EngineeringIR
├── meta            # schema version, generator, timestamps, source manifest
├── ontology        # vocabulary: node kinds, edge types, taxonomies
├── nodes[]         # typed entities (see DATA_SCHEMA.md)
├── edges[]         # typed relationships between nodes
├── decisionTrees[] # problem → recommendation graphs
├── examples[]      # canonical code/usage examples (referenced by nodes)
├── failureModes[]  # known errors + fixes (referenced by nodes)
├── verification[]  # rules used by the Verifier at runtime
└── indices         # precomputed lookup tables (symbol, embedding)
```

## 3. Node kinds

| Kind | Meaning |
| --- | --- |
| `Concept` | An idea/abstraction (e.g. "State Management") |
| `Package` | A distributable library/framework (e.g. `flutter`) |
| `Module` | A grouping within a package |
| `Class` | A type/class |
| `Function` | A free or member function/method |
| `API` | A public surface element (endpoint, widget, CLI flag) |
| `Protocol` | A wire/spec protocol (HTTP, gRPC, OpenAPI op) |
| `Example` | A runnable/illustrative snippet |
| `FailureMode` | A known error + remediation |
| `Decision` | A node in a decision tree |
| `Architecture` | A structural pattern/role |
| `Alternative` | A competing approach (with tradeoffs) |

## 4. Edge types (typed, directed)

| Edge | Semantics |
| --- | --- |
| `DEPENDS_ON` | compile/runtime dependency |
| `IMPLEMENTS` | class implements protocol/interface |
| `PART_OF` | child belongs to parent (module→package) |
| `USES` | concept/API consumes another |
| `EXTENDS` | inheritance / subclassing |
| `REFERENCES` | loose documentation reference |
| `ALTERNATIVE_TO` | competing approach |
| `SOLVES` | concept/package solves a problem |
| `HAS_EXAMPLE` | node → example |
| `HAS_FAILURE` | node → failure mode |
| `MIGRATES_TO` | upgrade path (v3 → v4) |
| `COMPATIBLE_WITH` | version/platform compatibility |

Every edge has a `weight` (confidence 0..1) and `provenance` (source id + line).

## 5. Decision tree shape

```
DecisionTree
├── id
├── trigger         # problem signature this tree answers
├── root: DecisionNode
└── nodes[]: DecisionNode
      ├── id
      ├── question        # "Needs offline sync?"
      ├── type            # boolean | categorical | lookup
      ├── branches[]      # { condition, next: nodeId | terminal }
      └── terminal?       # recommendation (package/API set) + rationale
```

The Decision Engine (runtime) walks these without LLM involvement when the
problem matches a trigger.

## 6. Verification rule shape

```
VerificationRule
├── id
├── target           # node id or edge type
├── kind             # API_EXISTS | COMPILES | LINTS | VERSION_MATCH | DOCS_MATCH
├── command          # how to check (e.g. "dart analyze", "exists_in_graph")
└── severity         # ERROR | WARN
```

## 7. Ontology & extensibility

The ontology section defines the closed set of node kinds and edge types for a
schema version. Custom domains add **taxonomy tags** (not new kinds) to stay
forward-compatible. Schema version bumps when kinds/edges change.

## 8. Serialization of the IR itself

The IR is authored/debugged as **ZSON** (typed JSON with schema) during
compilation, then lowered to the binary form in `ATLAS_BINARY.md`. Tooling can
emit IR as pretty JSON for inspection (`atlas compile --emit-ir`).

## 9. Identity & merging

- Node id = `kind:domain/package@version/path` (stable, content-addressed
  optional suffix). Enables cross-bundle merge and dedupe.
- Two nodes with same id from different sources → merged, provenance stacked.
