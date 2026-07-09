# System Design

## 1. Thesis

Large language models should **not** memorize engineering knowledge. Model
weights are expensive to update, impossible to version per-domain, and prone to
hallucination when a fact is stale. Atlas externalizes knowledge into a
first-class, compiler-produced artifact (the `.atlas` binary) and reduces the
model's job to **reasoning, planning, traversal, synthesis, and verification**
over that artifact.

This mirrors how an operating system separates the **CPU** (general-purpose
compute) from **memory** (state). Atlas is the memory subsystem for engineering
AI.

## 2. Design principles

| # | Principle | Consequence |
| --- | --- | --- |
| P1 | **Knowledge outside weights** | Knowledge updates are data updates, not retrains. |
| P2 | **Everything is versioned** | `flutter@3.24` ≠ `flutter@5.0`; both can coexist. |
| P3 | **Every node has provenance** | No fact without a source reference + checksum. |
| P4 | **Typed edges, not free text** | Relationships are queryable, not prose. |
| P5 | **Deterministic compilation** | Same inputs → byte-identical IR (modulo timestamps). |
| P6 | **No hallucination by construction** | Generator may only emit what exists in the graph or is verified. |
| P7 | **One source, many renderers** | Studio, SDK, MCP, CLI, books all read the same binary. |
| P8 | **Small general model + big knowledge** | A 1B–3B model can solve hard tasks via navigation. |

## 3. Components and responsibilities

| Component | Lang | Role |
| --- | --- | --- |
| **Compiler** | Rust | Parse sources → Engineering IR → `.atlas` |
| **Runtime** | Rust | Load binary, navigate graph, plan, verify, answer |
| **CLI** | Rust | `install`, `compile`, `verify`, `doctor`, `graph` |
| **Package Manager** | Rust | Resolve, fetch, cache, verify `.atlas` bundles |
| **Studio** | TypeScript | Interactive zoomable knowledge map + authoring |
| **Diagram engine** | TypeScript | Graph/diagram rendering |
| **MCP server** | TypeScript | Expose Atlas to AI agents as tools |
| **SDK** | Python | `load / search / solve / compile` ergonomics |
| **Orchestrator** | Python | Bind a reasoning model to the Runtime |
| **Registry** | Rust + object store | Distribution of signed `.atlas` bundles |

## 4. Information flow

```
 Sources ──▶ Compiler ──▶ Engineering IR ──▶ .atlas ──▶ Registry
                                                    │
                                                    ▼
                                              Package Manager
                                                    │
   Problem ──▶ Orchestrator ──▶ Runtime (graph nav + verify) ──▶ Answer
                                    ▲
                                    │
                            loaded .atlas bundle
```

## 5. What Atlas is NOT

- Not a fine-tuned model.
- Not a vector-DB semantic search wrapper.
- Not a documentation CMS.
- Not a chatbot with a system prompt.

## 6. Non-goals (v1)

- No autonomous internet crawling as a primary source (sources are pinned/versioned).
- No model training or weight updates.
- No natural-language summarization of docs that can be parsed into structure.
- No proprietary model hosting.

## 7. Success criteria

1. A 1B–3B model using an Atlas bundle answers a domain problem with **0
   fabricated APIs** across a 200-question eval.
2. `atlas install flutter@3.24 && atlas solve "<problem>"` completes offline.
3. Compiling a package repo is deterministic and reproducible.
4. A private `.atlas` can be built from a private repo without any cloud call.
