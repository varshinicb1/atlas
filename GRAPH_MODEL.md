# Graph Model

The Engineering IR is a **typed property graph**. This document specifies how
nodes and edges are stored, indexed, and traversed by the Runtime.

## 1. Graph as storage

```
G = (V, E)
V : set of nodes (see DATA_SCHEMA.md)
E : set of typed, directed, weighted edges with provenance
```

The graph is **not** a general-purpose RDF triple store. It is a closed-world,
schema-validated graph: only the node kinds and edge types declared in the IR
ontology exist. This keeps traversal fast and deterministic.

## 2. Adjacency model

Primary storage is **compressed sparse row (CSR)** per edge type:

```
For each edge type T:
  edge_src[]   // source node id (sorted)
  edge_dst[]   // destination node id
  edge_attr[]  // (weight, provenance_id) packed
```

- Outgoing lookup for a node = binary search in `edge_src`.
- Per-edge-type CSR lets the Runtime load only the relationship kinds it needs
  (lazy expansion, `ATLAS_BINARY.md`).
- Reverse edges are generated at compile time (`rev_T`) for cheap inbound
  queries.

## 3. Indices

| Index | Backing | Use |
| --- | --- | --- |
| Symbol index | RocksDB (string→node id) | `flutter.widgets.Riverpod` exact lookup |
| Type index | bitmap per kind | "all `Function` nodes" |
| Embedding index | HNSW (in-binary) | semantic nearest-neighbour for fuzzy queries |
| Version index | SQLite | resolve `flutter@3.24` constraints |
| Provenance index | map source id → node ids | traceability / audit |

## 4. Traversal primitives (Runtime API)

```
neighbors(node, edge_type, direction) -> [node]
shortest_path(a, b, edge_types)       -> [node]   // for "how does X relate to Y"
subgraph(seed, depth, kinds)           -> Graph    // for Studio zoom
expand(node)                           -> subnodes  // lazy load on demand
filter(kind, predicate)                -> [node]
decision_walk(tree, context)           -> terminal // Decision Engine
```

## 5. The canonical Flutter example

```
flutter@3.24 (Package)
 └─ PART_OF → widgets (Module)
       └─ PART_OF → StateManagement (Concept)
             └─ SOLVES → "reactive UI state"
             ├─ ALTERNATIVE_TO → Provider
             └─ HAS_EXAMPLE → riverpod_counter
       └─ PART_OF → Riverpod (Package)
             ├─ EXTENDS → Notifier
             │     ├─ EXTENDS → AsyncNotifier
             │     │     └─ HAS_API → watch()
             │     └─ HAS_API → build()
             ├─ PART_OF → ConsumerWidget
             │     └─ USES → BuildContext
             │           └─ PART_OF → WidgetTree
             │                 └─ PART_OF → RenderObject
             └─ HAS_FAILURE → riverpod_provider_disposed
```

Every hop above is a typed edge the Runtime can navigate without an LLM.

## 6. Consistency invariants

- Every `edge.src` and `edge.dst` must reference an existing node (dangling
  edges rejected at compile/validate time).
- `weight ∈ [0,1]`.
- No edge without `provenance`.
- `COMPATIBLE_WITH` edges must reference versioned nodes.

## 7. Merge semantics

When bundles combine (e.g. `flutter` + `supabase`), nodes with identical ids
merge; edges union; provenance lists concatenate; conflicting attributes are
flagged `UNRESOLVED` and excluded from authoritative answers until reconciled.
