# Atlas Knowledge Package QA Report

Date: 2026-07-11  
Audit scope: 12 knowledge packages, 6 with decision trees, CLI `atlas.exe`  
Method: Compile `.md` → `.atlas` bundles under `qa/knowledge/`, run `solve`/`decide`/`verify`/`reason/dump`

---

## Executive Summary

| Severity | Count |
|----------|-------|
| P0 (wrong/unsafe/dangling) | 6 |
| P1 (frequent misses, silent empty) | 10 |
| P2 (quality) | 5 |
| P3 (nit) | 3 |
| **Total** | **24** |

### Answer Quality Grades

| Package | Grade | Notes |
|---------|-------|-------|
| atlas | B | Good coverage, all edges resolve internally |
| flutter_core | C | "stateful widget" miss, missing `concept:stateful_widget` via space query, `package:flutter` typed as Concept |
| riverpod | D | 5 dangling cross-package edges; 7 concept nodes have null descriptions |
| rust_patterns | B+ | Good solve results; 3 dangling DependsOn edges (tokio, serde, serde_json) |
| python_patterns | B | Sound content; 3 dangling AlternativeTo edges |
| typescript_7_migration | C+ | Many dangling edges use descriptive strings, not node IDs |
| typescript_nextjs | C | 12 dangling edges use plain names instead of IDs |
| flutter_w_decisions | B | Good decisions, but context key collision across trees |
| react_patterns | B | Clean edges overall; 2 dangling DependsOn |
| go_patterns | B | 4 dangling edges |
| docker_kubernetes | B | 5 dangling AlternativeTo/DependsOn |
| postgres_patterns | B+ | 4 dangling edges |
| cloudflare_workers | B | 4 dangling edges |

---

## P0 Issues

### KNOW-01 — Riverpod: 5 dangling cross-package edges

**Package**: `riverpod.atlas`  
**Severity**: P0

**Exact repro**: Any edge referencing an external package that is not a node in the bundle.

```
atlas dump --bundle qa\knowledge\riverpod.atlas --json
```

**Actual**: 5 edges target node IDs that do not exist in the bundle:

| Edge | Src | Dst | Type |
|------|-----|-----|------|
| e0 | package:flutter/riverpod | **package:flutter/widgets** | DependsOn |
| e3 | package:flutter/riverpod | **problem:flutter/reactive_state** | Solves |
| e4 | package:flutter/riverpod | **package:flutter/provider** | AlternativeTo |
| e5 | package:flutter/riverpod | **package:flutter/bloc** | AlternativeTo |
| e6 | package:flutter/riverpod | **package:flutter/get_it** | AlternativeTo |

**Expected**: These edges should reference nodes that exist either in-package or resolvable across a multi-bundle runtime. Currently they are dead links — any downstream tooling consuming the graph will have dangling references.

**Suggested fix**: Either add `package:flutter_core` (renamed to `package:flutter/widgets`) as a dependency node, or change the riverpod source markdown to reference the actual compiled package IDs. The `flutter.yaml` decision tree also references `package:pub.dev/supabase_flutter` etc. which are not compiled nodes.

---

### KNOW-02 — typescript_nextjs: 12 dangling edges using plain names instead of node IDs

**Package**: `typescript_nextjs.atlas`  
**Severity**: P0

**Exact repro**:
```
atlas dump --bundle qa\knowledge\typescript_nextjs.atlas --json
```

**Actual**: 12 edges use plain-text names/sentences as `dst` instead of scoped node IDs.

Examples:
- `e0: React [DependsOn]` — no node with id `"React"`
- `e1: Node.js [DependsOn]`
- `e3: App Router [Uses]` — should be `concept:app-router`
- `e4: Server Components [Uses]` — should be `concept:server-components`
- `e9: full-stack web development with React and type safety [Solves]`
- `e10/e11/e12/e13: Remix, SvelteKit, Nuxt 3, Astro [AlternativeTo]`

**Expected**: Every `dst` should be a node ID that exists in the compiled graph. Descriptive sentences should not be used as edge targets.

**Suggested fix**: Rewrite edges to use proper IDs:
```yaml
# Instead of:
#   DependsOn: React
#   DependsOn: Node.js
dependencies:
  - concept:react/hooks
  - concept:node/runtime
```

---

### KNOW-03 — rust_patterns: 3 dangling DependsOn edges

**Package**: `rust_patterns.atlas`  
**Severity**: P0

**Exact repro**: `atlas dump --bundle qa\knowledge\rust_patterns.atlas --json`

**Actual**:
- `e0: tokio [DependsOn]` — no node `"tokio"`
- `e1: serde [DependsOn]` — no node `"serde"`
- `e2: serde_json [DependsOn]` — no node `"serde_json"`
- `e8: memory-safe systems programming with zero-cost abstractions [Solves]` — sentence as ID

**Expected**: DependsOn targets should be scoped node IDs like `package:tokio` or `concept:rust/tokio`.

---

### KNOW-04 — typescript_7_migration: 7 dangling edges using descriptive text as node IDs

**Package**: `typescript_7_migration.atlas`  
**Severity**: P0

**Actual**: Several edges use descriptive strings instead of node IDs.

| Edge | Dst |
|------|-----|
| e0 | TypeScript 5.x or 6.x |
| e1 | TypeScript |
| e2 | Project Corsa |
| e8 | TypeScript 7 migration planning and execution |
| e9 | Continue using TypeScript 5.x/6.x (lose performance gains) |
| e10 | Use bun tsc or alternatives (less compatible) |
| e5 | Module Resolution Deprecation |

---

### KNOW-05 — General: Many packages have `Solves`/`AlternativeTo` edges pointing to descriptions, not node IDs

**Packages**: `cloudflare_workers`, `docker_kubernetes`, `go_patterns`, `postgres_patterns`, `python_patterns`, `react_patterns`  
**Severity**: P0

**Pattern**: Every compiled-from-`.md` package uses the `problem_solved` and `dependencies` YAML frontmatter to generate `Solves` and `AlternativeTo` edges, but the values are plain text strings rather than scoped node IDs.

**Example for `cloudflare_workers.atlas`**:
```
e10: globally distributed, low-latency serverless applications without managing servers [Solves]
e11: AWS Lambda@Edge for the AWS ecosystem [AlternativeTo]
```

**Suggested fix**: The compiler should either (a) auto-generate scoped IDs from the problem statement, or (b) the source `.md` files should specify `problem_solved: "problem:cf/serverless"` and use proper IDs for alternatives.

---

### KNOW-06 — Decide: Context key `answer` collides across tree levels, causing silent null

**Packages**: `py_w_decisions.atlas`, `flutter_w_decisions.atlas`, `react_w_decisions.atlas`, `ts7_w_decisions.atlas`  
**Severity**: P0

**Exact repro**:
```
atlas decide --bundle qa\knowledge\flutter_w_decisions.atlas "flutter widget" -c answer=true -c answer=external --json
# Returns null
```

**Root cause**: The evaluate_condition function performs exact match on `context.values.get(key)`. When a decision tree uses the same key name (`answer`) at multiple nesting levels, passing context values for different levels overwrites each other because the CLI stores context in a flat `HashMap<String, String>`. In the above example, `answer=external` overwrites `answer=true`, so the root node (`widget_state_node`) condition `answer=true` fails.

**Affected trees**:
- `flutter_widget_choice`: uses `answer` for 3 different questions
- `flutter_state_management`: same
- `react_state_location`: same
- `react_server_vs_client`: same
- `python_sync_vs_async`: same
- `ts7_migration_path`: same (also uses `answer` at multiple levels)

**Expected**: The user should be able to provide context values for all levels without collision. Or the decision tree should use distinct key names (e.g., `needs_state`, `needs_builder`, `state_source`).

**Suggested fix**: Re-key the decision tree conditions to use semantically meaningful names per node, e.g.:
```yaml
# Instead of:
branches:
  - condition: "answer=true"

# Use:
branches:
  - condition: "needs_mutable_state=true"
```

---

## P1 Issues

### KNOW-07 — "stateful widget" (with space) fails to find `concept:stateful_widget`

**Package**: `flutter_core.atlas`  
**Severity**: P1

**Exact repro**:
```
atlas solve --bundle qa\knowledge\flutter_core.atlas "stateful widget"
```

**Actual**: Returns `MediaQuery.of`, `RenderObject`, `BuildContext`, `Key`, `StatelessWidget` — none of which is `StatefulWidget`.

**Expected**: `StatefulWidget` should be in the top 3 results. The symbol `stateful_widget` exists, but the embedding + symbol search does not match `"stateful widget"` (space) to `"stateful_widget"` (underscore).

**Contrast**: `atlas solve --bundle qa\knowledge\flutter_core.atlas "stateful_widget"` — works perfectly (total_matches: 1).

---

### KNOW-08 — "serde json" fails to find serialization concept

**Package**: `rust_patterns.atlas`  
**Severity**: P1

**Exact repro**:
```
atlas solve --bundle qa\knowledge\rust_patterns.atlas "serde json"
```

**Actual**: Returns `Result`, `Error Handling`, `Arc/Mutex`, `Option` — but NOT `concept:rust/serde`.

**Expected**: The concept "Serialization" (serde) should be a top result.

---

### KNOW-09 — "navigation" on flutter_core returns only 1 irrelevant node

**Package**: `flutter_core.atlas`  
**Severity**: P1

**Exact repro**:
```
atlas solve --bundle qa\knowledge\flutter_core.atlas "navigation"
```

**Actual**: Only returns `concept:key` (1 node). The flutter_core package has NO navigation concepts.

**Expected**: Either add navigation concepts (Navigator, Routes, GoRouter) to flutter_core, or return an informative message that no navigation knowledge exists.

---

### KNOW-10 — Non-English queries return random results

**Packages**: All  
**Severity**: P1

**Exact repro**:
```
atlas solve --bundle qa\knowledge\rust_patterns.atlas "¿Cómo manejar errores?"
```

**Actual**: Returns `Serialization` as top result instead of `Error Handling`. The embedding model (unigram+bigram hash) does not handle non-English queries.

**Expected**: Either return no results (0 confidence) or add a note that only English is supported.

---

### KNOW-11 — Empty query gives raw CLI error, not friendly message

**Package**: CLI  
**Severity**: P1

**Exact repro**:
```
atlas solve --bundle qa\knowledge\atlas.atlas ""
```

**Actual**: Rust clap error: `error: the following required arguments were not provided: <QUERY>`

**Expected**: `Error: query cannot be empty. Please provide a search query.`

---

### KNOW-12 — Per-node confidence not exposed in JSON output

**Package**: CLI  
**Severity**: P1

**Exact repro**:
```
atlas solve --bundle qa\knowledge\rust_patterns.atlas "ownership" --json
```

**Actual**: The JSON output includes `confidence` at the top level but does NOT include per‑node confidence. The `NodeOutput` serialization omits the `confidence` field.

**Expected**: Add per-node confidence to the JSON output so clients can rank results intelligently.

---

### KNOW-13 — Riverpod: 7 concept nodes have null descriptions

**Package**: `riverpod.atlas`  
**Severity**: P1

**Exact repro**: `atlas dump --bundle qa\knowledge\riverpod.atlas --json | jq .nodes[].description`

**Actual**: All 7 riverpod concept nodes have `description: null`:
- State Management
- Provider
- Notifier
- AsyncNotifier
- ConsumerWidget
- WidgetRef
- ProviderContainer

**Expected**: Every concept node should have a meaningful description.

---

### KNOW-14 — Reason: TemplateReasoner displays "5 of 0" for queries with 0 exact matches

**Package**: CLI + TemplateReasoner  
**Severity**: P1

**Exact repro**:
```
atlas reason --bundle qa\knowledge\rust_patterns.atlas "what is ownership" --json
```

**Result**: `"Matched Knowledge (5 of 0)"` — confusing when 5 nodes are displayed but it says "of 0".

**Root cause**: The `total_matches` in the solve result is 0 (no exact symbol match), but 5 embedding‑fallback nodes are returned. The template displays `len(nodes) of total_matches` but should normalize the display.

---

### KNOW-15 — flutter_core: `package:flutter` node typed as Concept instead of Package

**Package**: `flutter_core.atlas`  
**Severity**: P1

**Exact repro**:
```
atlas dump --bundle qa\knowledge\flutter_core.atlas --json
```

**Actual**: Node `package:flutter` has `kind: Concept` instead of `kind: Package`.

**Expected**: `package:flutter` should be `kind: Package`.

**Suggested fix**: In `flutter_core.md`, the line `dependencies: ["package:flutter"]` should create a `package:flutter` node with kind Package, not Concept. The compiler or the source frontmatter likely mislabels this.

---

### KNOW-16 — Decide: Identical context key across different trees causes cross-tree interference

**Package**: All with decision trees  
**Severity**: P1

**Exact repro**:
```
atlas decide --bundle qa\knowledge\react_w_decisions.atlas "react" -c answer=local -c answer=false
```

**Actual**: Navigates into `react_server_vs_client` tree (via `answer=false` matching `interactivity_node`) instead of `react_state_location` (which has `answer=local`). The user intended the state‑location tree, but the server‑vs‑client tree matches first because of context key collision.

**Root cause**: `find_matching` returns all matching trees, and the walker returns the first successful walk. Since both trees use the same `answer` key, the last-provided value overwrites all others, and whichever tree happens to match first wins.

---

## P2 Issues

### KNOW-17 — Version inconsistency across packages

**Affects**: All packages  
**Severity**: P2

Some nodes are versioned, some are not:
- `atlas.atlas`: 9 versioned, 9 unversioned
- `rust_patterns.atlas`: 11 versioned, 9 unversioned
- `python_patterns.atlas`: 11 versioned, 16 unversioned

**Expected**: Concept nodes that describe stable knowledge may reasonably be unversioned, but API nodes should consistently have versions. The mix is confusing.

---

### KNOW-18 — `flutter_w_decisions` has 4 trees from 2 YAML files

**Package**: `flutter_w_decisions.atlas`  
**Severity**: P2

The compiled bundle includes:
- `flutter_core.yaml`: 2 trees (widget_choice, state_management)
- `flutter.yaml`: 2 trees (offline_sync, state_mgmt_flutter)

But `flutter.yaml` references `package:pub.dev/riverpod`, `package:pub.dev/hive`, etc. which have no corresponding nodes. The decision trees recommend packages that are not in the knowledge graph.

---

### KNOW-19 — 3-minute "no matching tree" message lacks detail

**Packages**: All with decision trees  
**Severity**: P2

**Exact repro**:
```
atlas decide --bundle qa\knowledge\py_w_decisions.atlas "pizza" --json
```

**Actual**: Prints `null` (in JSON mode) or `"No matching decision tree found for query."` (text mode). No indication of what tags exist or what the query should contain.

**Expected**: Include a hint: "Available decision trees: python_sync_vs_async (tags: python, concurrency, async), python_data_model_choice (tags: python, data, validation)".

---

### KNOW-20 — Golang package has "Systems Programming" domain node with lowercase first letter

**Package**: `go_patterns.atlas`  
**Severity**: P2

**Exact repro**: `atlas dump --bundle qa\knowledge\go_patterns.atlas --json`

```
id: "systems programming" 
name: "Systems Programming"
kind: Concept
```

Actually this is also in `rust_patterns.atlas` — the `systems programming` node is not scoped. Should be `concept:domain/systems-programming` for consistency.

---

### KNOW-21 — Some Solution/Problem descriptions contain trailing text after newline

**Package**: Various  
**Severity**: P2

The `atlas.atlas` package `node` for `package:atlas` has a description that ends with `...` (truncated in the compiled bundle). The compiled `.atlas` bundles from `.md` files strip frontmatter but keep the body text — long descriptions get truncated without warning.

---

## P3 Issues

### KNOW-22 — Special-character queries return embedding results without error

**Exact repro**:
```
atlas solve --bundle qa\knowledge\rust_patterns.atlas "!!!@@@$$$"
```

**Actual**: Returns 2 random nodes (Box::new, Pattern Matching) via embedding fallback with confidence 0.7.

**Expected**: Either return 0 results or a confidence of 0.0 with a note that the query appears nonsensical.

---

### KNOW-23 — `flutter.yaml` decision trees not compiled into flutter_core by default

**Package**: `flutter_core.atlas` (pre-compiled)  
**Severity**: P3

The pre-compiled `flutter_core.atlas` has 0 decision trees. The `flutter.yaml` and `flutter_core.yaml` decision trees must be explicitly compiled separately. This means the published `flutter_core.atlas` package (if it matches the pre-compiled version) has no decision tree support.

---

### KNOW-24 — Runtime load returns source_manifest source_id as bundle name, not meaningful name

**Exact repro**: Loading `py_w_decisions.atlas` returns bundle name `python_patterns` (from `source_manifest[0].source_id`), not the filename or a meaningful identifier. This is internal but affects debugging.

**Severity**: P3

---

## Detailed Package Scorecards

### atlas (Grade B)
- **Nodes**: 24, Edges: 24, Decision trees: 0
- **Verify**: PASS
- **Strengths**: All edges resolve internally, good coverage of CLI/API concepts
- **Weaknesses**: "template reasoner" typo miss, long description truncation
- **Edge issues**: None internal

### flutter_core (Grade C)
- **Nodes**: 18, Edges: 27, Decision trees: 0 (pre-compiled)
- **Verify**: PASS
- **Strengths**: Good API coverage, examples for common patterns
- **Weaknesses**: `package:flutter` typed as Concept; "stateful widget" (space) miss; no navigation concepts; `sdk:dart` dangling edge

### riverpod (Grade D)
- **Nodes**: 14, Edges: 24, Decision trees: 0 (pre-compiled)
- **Verify**: PASS (internal only)
- **Strengths**: Good API coverage for riverpod ref methods
- **Weaknesses**: **5 dangling cross-package edges** (P0); **7 concept nodes with null descriptions** (P1)
- **Decision trees**: None compiled in; `flutter.yaml` references riverpod but not vice versa

### rust_patterns (Grade B+)
- **Nodes**: 20, Edges: 39, Decision trees: 0 (pre-compiled)
- **Verify**: PASS
- **Strengths**: Excellent solve relevance; ownership, error handling, lifetimes all findable
- **Weaknesses**: "serde json" miss (P1); 3 dangling DependsOn edges (tokio, serde, serde_json)

### python_patterns (Grade B)
- **Nodes**: 27, Edges: 49, Decision trees: 2 (compiled fresh)
- **Verify**: PASS
- **Strengths**: Comprehensive Python coverage (typing, async, packaging, GIL, protocols)
- **Weaknesses**: 3 dangling AlternativeTo edges; dependency references to pydantic/pytest as plain strings
- **Decisions**: python_sync_vs_async ✅, python_data_model_choice ✅

### typescript_7_migration (Grade C+)
- **Nodes**: 18, Edges: 36, Decision trees: 2 (compiled fresh)
- **Verify**: PASS
- **Strengths**: Rich decision trees with detailed agent instructions
- **Weaknesses**: 10+ dangling edges using descriptive text; `TypeScript 5.x or 6.x` as DependsOn target
- **Decisions**: ts7_migration_path ✅, ts7_strict_fix_strategy ✅

### typescript_nextjs (Grade C)
- **Nodes**: 19, Edges: 38, Decision trees: 0
- **Verify**: PASS
- **Strengths**: Good node coverage (app-router, server-components, hooks, data-fetching)
- **Weaknesses**: **12 dangling edges** (P0); confidence 0.7 on all results; per-node confidence missing

### react_patterns (Grade B)
- **Nodes**: 24, Edges: 48, Decision trees: 2 (compiled fresh)
- **Verify**: PASS
- **Strengths**: Clean internal edges; good mix of hooks, suspense, concurrent features
- **Weaknesses**: DependsOn edges use `react@19` and `react-dom@19` (no matching nodes)
- **Decisions**: react_state_location ✅, react_server_vs_client ✅

### go_patterns (Grade B)
- **Nodes**: 25, Edges: 46, Decision trees: 0
- **Verify**: PASS
- **Strengths**: Good goroutine/channel/interface coverage
- **Weaknesses**: 4 dangling edges; `github.com/spf13/cobra` as DependsOn string

### docker_kubernetes (Grade B)
- **Nodes**: 23, Edges: 48, Decision trees: 0
- **Verify**: PASS
- **Strengths**: Clean internal edges for k8s concepts; pods, deployments, services
- **Weaknesses**: "docker compose" query returns `docker run` (no compose concept); 5 dangling AlternativeTo/DependsOn edges

### postgres_patterns (Grade B+)
- **Nodes**: 22, Edges: 45, Decision trees: 0
- **Verify**: PASS
- **Strengths**: Good indexing, transaction, and vacuum coverage
- **Weaknesses**: 4 dangling edges (postgresql, Solves, AlternativeTo)

### cloudflare_workers (Grade B)
- **Nodes**: 22, Edges: 46, Decision trees: 2 (compiled fresh)
- **Verify**: PASS
- **Strengths**: Good KV/D1/R2/DO coverage; both decision trees work correctly
- **Weaknesses**: 4 dangling edges (wrangler, Solves description, AlternativeTo)
- **Decisions**: cf_storage_choice ✅, cf_handler_structure ✅

---

## Recommendations (Priority Order)

### Immediate Fixes (P0)
1. **Set semantic context keys in decision trees**: Replace `answer=true/false` with node‑specific keys (`needs_state=true`, `state_source=external`, etc.)
2. **Fix dangling cross-package edges**: Every `dst` should be a valid node ID. Add missing package nodes or correct the edge targets.
3. **Fix `package:flutter` kind**: Change from `Concept` to `Package` in source markdown.

### Short-term (P1)
4. **Improve hybrid search**: Normalize spaces/underscores/hyphens in query vs symbol matching so "stateful widget" matches `stateful_widget`.
5. **Add per-node confidence to JSON output**.
6. **Add missing concept descriptions** in riverpod.
7. **Fix "5 of 0" display in TemplateReasoner**: Show "Matched Knowledge (5 nodes, 0 exact matches)" or similar.
8. **Add empty query validation** with user-friendly error message.

### Medium-term (P2)
9. **Normalize versioning**: Either version all nodes or document which nodes are versioned.
10. **Add navigation concepts to flutter_core** or create a dedicated Flutter patterns package.
11. **Improve "no matching tree" message** to list available trees and their tags.
