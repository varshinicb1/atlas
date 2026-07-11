---
kind: Package
id: package:mycelium/sync
name: Mycelium Sync Protocol
version: "0.1.0"
purpose: Document the multi-agent sync protocol — agents that remember, share, trust, and reconcile knowledge through provenance-tracked fact exchange.
problem_solved: >
  AI agents operating in isolation produce inconsistent, contradictory knowledge.
  Mycelium provides a deterministic protocol for agents to share facts with
  provenance tracking, detect contradictions via batched LLM calls, build
  pair-wise trust scores, and resolve conflicts — enabling cooperative multi-agent
  memory without centralized coordination.
install: |
  ```bash
  git clone https://github.com/harishkotra/mycelium
  cd mycelium
  pnpm install
  cp packages/core/.env.example packages/core/.env
  pnpm --filter travel-assistant-demo demo
  ```
dependencies:
  - package:cognee
concepts:
  - name: Agent
    id: concept:mycelium/agent
    description: >
      Wraps a Cognee dataset with remember() and recall() API. Each agent has
      persistent knowledge backed by a Kuzu graph DB with ONNX embeddings.
      Created with an ID, dataset name, and CogneeClient backend. remember()
      ingests facts with optional provenance; recall() queries the graph.
  - name: Sync Engine
    id: concept:mycelium/sync-engine
    description: >
      Brokers fact exchange between agents with provenance tracking and
      trust-based auto-merge. syncFromSource() pulls fact strings from a source
      agent, runs batched LLM contradiction detection against subscriber facts,
      and produces a sync run with decision (auto_merged or pending_review).
      Auto-merges at trust >= autoMergeThreshold (default 0.6).
  - name: Provenance
    id: concept:mycelium/provenance
    description: >
      Tags facts with source metadata via a __provenance__ JSON prefix embedded
      in the fact text. Survives round-trips through Cognee's graph pipeline.
      ProvenanceRegistry provides in-memory lookup by source agent, deletion by
      source, and full enumeration. Format: {sourceAgentId, factId, timestamp}.
  - name: Trust Store
    id: concept:mycelium/trust-store
    description: >
      Pair-wise trust scores between 0 and 1, starting at 0.5 for every pair.
      Asymmetric adjustment: +0.05 on accept/auto-merge, -0.2 on reject or
      contradiction found. Clamped to [0, 1]. A single contradiction flips
      auto_merged to pending_review regardless of trust level.
  - name: Contradiction Detection
    id: concept:mycelium/contradiction-detection
    description: >
      Batched LLM-based contradiction scanning. SyncEngine sends all source +
      subscriber fact strings in a single prompt, parses the JSON response
      {contradictions: [{sIndex, tIndex, reason, confidence}]}, deduplicates
      by pair index. A separate graph-level pass (detectContradictions)
      compares before/after snapshot nodes via structured LLM prompts.
  - name: Contradiction Resolver
    id: concept:mycelium/contradiction-resolver
    description: >
      Pure decision engine with no side effects or graph calls. Three strategies:
      flag_all (default — keep both, flag for human review), keep_newer (incoming
      fact wins above confidence threshold), keep_higher_trust (existing fact
      wins above confidence threshold). Below threshold, all are flagged.
  - name: Diff Engine
    id: concept:mycelium/diff-engine
    description: >
      Three-pass diff between two knowledge graph snapshots. Pass 1: structural
      set-diff on node/edge IDs with property-change detection (pure, no I/O).
      Pass 2: embedding drift via cosine distance between before/after vectors,
      flagging nodes beyond 0.15 threshold. Pass 3: graph-level LLM contradiction
      detection on changed nodes. Orchestrated by runDiff().
  - name: Memory Improve Pipeline
    id: concept:mycelium/memory-improve
    description: >
      Wraps Cognee's improve pipeline with before/after snapshot diff,
      contradiction detection, and optional auto-resolution. agent.improve()
      takes autoResolve boolean and resolutionStrategy, returns diff summary
      with nodesAdded, contradictionsDetected, and resolvedContradictions.
  - name: Multi-Agent Systems
    id: concept:domain/multi-agent-systems
    description: >
      The broader domain of cooperative multi-agent architectures where
      independent AI agents communicate, share knowledge, and coordinate
      without centralized control.
  - name: P2P Gossip Protocols
    id: concept:domain/p2p-gossip-protocols
    description: >
      Peer-to-peer epidemic/gossip protocols for information dissemination
      in distributed systems, an alternative to mycelium's trust-weighted
      provenance-tracked sync.
  - name: Centralized Knowledge Base
    id: concept:domain/centralized-knowledge-base
    description: >
      A single shared database or knowledge store that all agents read/write,
      an alternative to mycelium's decentralized multi-agent sync protocol.
apis:
  - name: Agent.remember
    id: api:mycelium/agent-remember
    signature: "agent.remember(content: FactInput, opts?: { provenance?: Provenance }): Promise<void>"
    returns: void
    description: >
      Ingests a fact with optional provenance metadata. The fact is stored
      in the agent's Cognee dataset. Provenance is embedded as a
      __provenance__ JSON prefix in the fact text.
  - name: Agent.recall
    id: api:mycelium/agent-recall
    signature: "agent.recall(query: string): Promise<GraphSearchResult>"
    returns: Graph search results
    description: >
      Queries the agent's knowledge graph. First searches session cache,
      falls back to Cognee graph search (GRAPH_COMPLETION mode).
  - name: Agent.improve
    id: api:mycelium/agent-improve
    signature: "agent.improve(opts: { autoResolve?: boolean, resolutionStrategy?: Strategy }): Promise<ImproveResult>"
    returns: ImproveResult with diff summary and resolved contradictions
    description: >
      Runs Cognee's improve pipeline wrapped with before/after snapshot diff,
      contradiction detection, and optional auto-resolution.
  - name: SyncEngine.syncFromSource
    id: api:mycelium/sync-from-source
    signature: "engine.syncFromSource(client, subscriberDataset, subscriberAgentId, sourceDataset, sourceAgentId, sourceFacts, subscriberFacts?): Promise<SyncRun>"
    returns: SyncRun with diff, contradictions, and decision
    description: >
      Pulls facts from a source agent into a subscriber agent. Runs batched
      LLM contradiction detection and produces a sync run with auto_merged
      or pending_review decision based on trust threshold.
  - name: TrustStore.adjustTrust
    id: api:mycelium/adjust-trust
    signature: "trustStore.adjustTrust(sourceAgentId: string, targetAgentId: string, action: 'accept' | 'reject'): number"
    returns: Updated trust score
    description: >
      Adjusts pair-wise trust score: accept +0.05, reject -0.2. Clamped to
      [0, 1]. Returns the new score.
  - name: tagWithProvenance
    id: api:mycelium/tag-provenance
    signature: "tagWithProvenance(text: string, prov: { sourceAgentId: string, factId: string, timestamp: number }): string"
    returns: Tagged fact string with __provenance__ prefix
    description: >
      Embeds source metadata as a JSON prefix in the fact text. Survives
      Cognee graph pipeline round-trips.
  - name: resolveContradictions
    id: api:mycelium/resolve-contradictions
    signature: "resolveContradictions(contradictions: Contradiction[], opts: { strategy: Strategy, confidenceThreshold: number }): ResolvedContradiction[]"
    returns: List of resolved contradictions with resolution decision
    description: >
      Pure decision engine. Strategies: flag_all (default), keep_newer,
      keep_higher_trust. Below confidenceThreshold, all are flagged.
  - name: runDiff
    id: api:mycelium/run-diff
    signature: "runDiff(before: GraphSnapshot, after: GraphSnapshot, client: CogneeClient, dataset: string): Promise<MemoryDiffResult>"
    returns: MemoryDiffResult with structural diff, drift, and contradictions
    description: >
      Orchestrates three-pass diff: structural set-diff, embedding cosine
      drift detection, and graph-level LLM contradiction detection.
  - name: acceptSync
    id: api:mycelium/accept-sync
    signature: "acceptSync(runId: string, engine: SyncEngine, trustStore: TrustStore, agentId: string): Promise<void>"
    returns: void
    description: >
      Accepts a pending sync run. Applies source facts to subscriber and
      bumps trust score by +0.05.
  - name: rejectSync
    id: api:mycelium/reject-sync
    signature: "rejectSync(runId: string, engine: SyncEngine, trustStore: TrustStore, agentId: string): Promise<void>"
    returns: void
    description: >
      Rejects a pending sync run. Does not import facts and drops trust
      score by -0.2.
examples:
  - id: example:mycelium/sync-travel
    language: typescript
    description: "Two agents (Planner, Assistant) sync flight delay facts; LLM detects 5PM vs 8PM contradiction"
  - id: example:mycelium/sync-supply-chain
    language: typescript
    description: "Warehouse Alpha and Beta sync inventory counts; LLM catches 500 vs 200 units contradiction"
  - id: example:mycelium/sync-healthcare
    language: typescript
    description: "Clinic and Pharmacy agents sync medication dosage; 5mg vs 10mg contradiction detected"
  - id: example:mycelium/improve-pipeline
    language: typescript
    description: "Agent runs improve with before/after snapshot diff and auto-resolution using keep_newer strategy"
failures:
  - id: failure:mycelium/cognee-pipeline-stuck
    symptom: Cognee pipeline never completes; graph extraction hangs
    cause: Pipeline run status stuck from a previous failed run
    fix: Call client.resetDatasetPipeline(datasetName) to clear stuck status
  - id: failure:mycelium/llm-provider-mismatch
    symptom: Contradiction detection silently returns no results even with contradictory facts
    cause: Cognee internally rewrites LLM_PROVIDER env var and only accepts "openai" or "mock"
    fix: CogneeClient handles this transparently — set LLM_PROVIDER input and let CogneeClient rewrite env vars
  - id: failure:mycelium/trust-below-threshold
    symptom: Sync run always returns pending_review even with no contradictions
    cause: Trust score below autoMergeThreshold (default 0.6) after repeated rejections
    fix: Manually accept syncs to rebuild trust, or lower autoMergeThreshold in SyncEngine options
uses:
  - concept:mycelium/agent
  - concept:mycelium/sync-engine
  - concept:mycelium/provenance
  - concept:mycelium/trust-store
  - concept:mycelium/contradiction-detection
  - concept:mycelium/contradiction-resolver
  - concept:mycelium/diff-engine
  - concept:mycelium/memory-improve
part_of: concept:domain/multi-agent-systems
solves:
  - problem:multi-agent-fact-sharing
  - problem:agent-contradiction-detection
  - problem:inter-agent-trust
alternatives:
  - concept:domain/p2p-gossip-protocols
  - concept:domain/centralized-knowledge-base
---

# Mycelium Sync Protocol

Mycelium is a TypeScript framework for building AI agents with persistent shared memory backed by a knowledge graph (Cognee + Kuzu). Each agent remembers facts, syncs with peer agents via a provenance-tracked protocol, detects and resolves contradictions through batched LLM calls, and builds pair-wise trust over time.

## Architecture

```
Agent Alpha ──┐                    ┌── Agent Beta
              │   Sync Engine      │
              ├── provenance ──────┤
              ├── trust store ─────┤
              ├── contradiction ───┤
              └── structural-diff ─┘
                      │
              Cognee / Kuzu Graph DB
```

### Agent Lifecycle

Every agent follows the same lifecycle: **reset → seed + cognify → sync (batched LLM contradiction detection → trust penalty) → improve → narrative summary**.

The sync protocol is the core innovation: `SyncEngine.syncFromSource()` pulls fact strings from a source agent, runs a single batched LLM prompt over all source + subscriber facts to find contradictions, and produces a `SyncRun` with either `auto_merged` (trust >= threshold, no contradictions) or `pending_review` (contradictions found). Trust adjusts asymmetrically: accept/auto-merge gives +0.05, rejection or contradiction gives -0.2, clamped to [0, 1].

Provenance is embedded as a `__provenance__:{sourceAgentId,factId,timestamp}` JSON prefix in the fact text, surviving Cognee round-trips. This enables traceability back to the source agent after facts are merged into another agent's dataset.

## Key Design Principles

- **No hidden side-effects** — pure functions (diffSnapshots, resolveContradictions, cosineDistance) are separated from I/O-bound code
- **Batched LLM detection** — contradiction scans send all facts in a single prompt, keeping costs low (~$0.01 per demo run on gpt-4o-mini)
- **Provider abstraction** — single `LLM_PROVIDER` env var changes the entire LLM stack (OpenAI, Groq, Ollama, LM Studio, custom)
- **Cognee as implementation detail** — the Agent class wraps Cognee internally; consumers don't import Cognee types directly
