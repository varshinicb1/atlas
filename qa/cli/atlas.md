---
kind: Package
id: package:atlas
name: Atlas
version: 0.1.0
purpose: A Knowledge Operating System that transforms engineering knowledge into a structured, navigable graph
problem_solved: Engineering knowledge fragmentation — knowledge scattered across docs, PRs, and chat logs
install: cargo build --release
dependencies: []
concepts:
  - name: EngineeringIR
    id: concept:atlas/engineering_ir
    description: Canonical intermediate representation with 11 node kinds and 13 typed edge types
  - name: BinaryFormat
    id: concept:atlas/binary_format
    description: Zstd-compressed sectioned binary with headers, TOC, BLAKE3 checksums, optional Ed25519 signing
  - name: Embedding
    id: concept:atlas/embedding
    description: 128-dim hash-based embeddings from unigram + bigram hashing with L2 normalization
  - name: SymbolIndex
    id: concept:atlas/symbol_index
    description: Inverted index mapping symbol names and types to node IDs for O(1) lookup
  - name: DecisionEngine
    id: concept:atlas/decision_engine
    description: LLM-free rule-based decision tree walker for structured recommendations
  - name: Reasoner
    id: concept:atlas/reasoner
    description: Pluggable reasoning stage — TemplateReasoner (built-in) and OllamaReasoner for natural-language answers
  - name: Node
    id: concept:atlas/node
    description: A typed knowledge unit — one of 11 kinds (Package, Concept, API, FailureMode, etc.)
  - name: Edge
    id: concept:atlas/edge
    description: Typed relationship between two nodes — one of 13 types (PartOf, DependsOn, Extends, Solves, etc.)
  - name: HybridSearch
    id: concept:atlas/hybrid_search
    description: Two-phase search — symbol index first, embedding cosine similarity fallback for fuzzy matches
apis:
  - name: compile
    id: api:atlas/compile
    signature: atlas compile <sources...> --out <path>
    returns: EngineeringIR binary (.atlas file)
    description: Compiles markdown and YAML sources into a canonical .atlas binary
  - name: solve
    id: api:atlas/solve
    signature: atlas solve --bundle <path> <query>
    returns: JSON array of matched nodes with confidence scores
    description: Searches the knowledge graph by symbol or embedding similarity
  - name: decide
    id: api:atlas/decide
    signature: atlas decide --bundle <path> <query> [-c <key=value>...]
    returns: JSON decision result with path, rationale, recommendations
    description: Walks a decision tree to produce an LLM-free recommendation
  - name: verify
    id: api:atlas/verify
    signature: atlas verify --bundle <path>
    returns: JSON verification report with pass/fail checks
    description: Runs structural validation rules against the bundle
  - name: dump
    id: api:atlas/dump
    signature: atlas dump --bundle <path>
    returns: JSON serialization of the full EngineeringIR
    description: Serializes the entire knowledge graph as JSON for inspection or Studio
  - name: reason
    id: api:atlas/reason
    signature: atlas reason --bundle <path> <query>
    returns: Natural-language answer string
    description: Reasons over the knowledge graph using TemplateReasoner or OllamaReasoner
  - name: install
    id: api:atlas/install
    signature: atlas install <src> --name <name>
    returns: Path to installed bundle
    description: Copies a compiled bundle to ~/.atlas/bundles/ for global access
  - name: solve (Python SDK)
    id: api:atlas/solve_python
    signature: atlas.solve(bundle, query) -> SolveResult
    returns: Typed SolveResult dataclass
    description: Python SDK wrapper around the CLI subprocess
examples:
  - id: example:atlas/compile_riverpod
    language: bash
    description: Compile the Riverpod knowledge document
  - id: example:atlas/solve_query
    language: bash
    description: Search for Riverpod state management concepts
  - id: example:atlas/decision_walk
    language: bash
    description: Walk the offline sync decision tree with context
failures:
  - id: failure:atlas/corrupt_binary
    symptom: "Not a valid .atlas file"
    cause: File missing ATLAS\0 header magic or truncated
    fix: Recompile the source documents; check file integrity
  - id: failure:atlas/empty_bundle
    symptom: "No nodes found in atlas binary (corrupt or empty)"
    cause: Binary has zero nodes — compiled from empty or invalid sources
    fix: Ensure source documents have at least one node definition
  - id: failure:atlas/hash_mismatch
    symptom: Section checksum verification failed
    cause: Binary was corrupted or tampered with after compilation
    fix: Recompile from trusted sources; verify with atlas verify
part_of: ~
solves:
  - problem:engineering/knowledge_fragmentation
alternatives: []
---

# Atlas

Atlas is a Knowledge Operating System that transforms fragmented engineering knowledge into a navigable, structured knowledge graph. It separates reasoning (LLM) from knowledge (binary .atlas files), enabling deterministic solve/decide/verify operations without hallucination risk.

## Architecture

Atlas has four layers:

1. **Compiler** (Rust) — Parses markdown + YAML sources into the canonical Engineering IR, builds embeddings and indices, and writes the .atlas binary.
2. **Binary Format** — Zstd-compressed sections (meta, ontology, nodes, edges, decision_trees, examples, failure_modes, verification_rules, indices) with BLAKE3 checksums and optional Ed25519 signing.
3. **Runtime** (Rust) — Memory-maps .atlas bundles and provides solve (hybrid search), decide (decision tree walker), verify (structural checks), and reason (pluggable stage) operations.
4. **CLI + SDKs** — tlas-cli binary, Python SDK (wraps CLI with --json), MCP server (stdio-based), and Studio (Next.js + React Flow graph explorer).

## Key Design Decisions

- **No LLM in the loop** for solve, decide, or verify — purely structural operations on canonical data.
- **Polyglot**: Rust (core), TypeScript (Studio/MCP), Python (SDK).
- **Self-describing**: Every node carries provenance (source file + line) and confidence scores.