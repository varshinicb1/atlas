# Decision Engine

The Decision Engine answers engineering problems by walking pre-compiled
decision trees — no LLM needed when the problem matches a known decision
surface.

## 1. Philosophy: Answer problems, not definitions

```
User: "I need offline synchronized state management in Flutter"
       └─▶ triggers tree: "offline_sync_flutter"
              └─ "Needs bidirectional sync?"   ── yes ──▶ "Needs conflict resolution?"
                                                    no ──▶ "Recommend Hive/Isar"
```

Atlas does not answer "What is Riverpod?" with a paragraph. It navigates from
the user's *context* to a *concrete recommendation*.

## 2. Decision tree shape

```json
{
  "id": "offline_sync_flutter",
  "trigger": { "intent": "choose_solution", "domain": "mobile", "tags": ["offline"] },
  "root": "q1",
  "nodes": [
    {
      "id": "q1",
      "question": "Need bidirectional sync with a server?",
      "type": "boolean",
      "branches": [
        { "condition": true,  "next": "q2" },
        { "condition": false, "next": "q3" }
      ]
    },
    {
      "id": "q2",
      "question": "Need conflict resolution?",
      "type": "boolean",
      "branches": [
        { "condition": true,  "next": "t1" },
        { "condition": false, "next": "t2" }
      ]
    },
    { "id": "q3", "question": "Local only?", "type": "boolean",
      "branches": [
        { "condition": true,  "next": "t3" }
      ]
    },
    {
      "id": "t1", "terminal": true,
      "recommendation": [
        { "node_id": "package:pub.dev/supabase_flutter", "confidence": 0.85 },
        { "node_id": "concept:flutter/realtime", "confidence": 0.80 }
      ],
      "rationale": "Supabase provides realtime sync with PostgreSQL-level conflict handling.",
      "agent_instructions": "Use supabase_flutter RealtimeClient for bidirectional sync. Call .subscribe() on the channel."
    },
    { "id": "t2", "terminal": true,
      "recommendation": [
        { "node_id": "package:pub.dev/riverpod", "confidence": 0.70 },
        { "node_id": "package:pub.dev/isar", "confidence": 0.75 }
      ],
      "rationale": "Riverpod for state + Isar for local persistence with optional sync."
    },
    { "id": "t3", "terminal": true,
      "recommendation": [
        { "node_id": "package:pub.dev/hive", "confidence": 0.90 }
      ],
      "rationale": "Hive is the simplest local-only solution."
    }
  ]
}
```

## 3. Decision Engine pipeline in the Runtime

1. **Intent Detection** classifies the problem. If it matches a `trigger`,
   routes to the Decision Engine instead of the full reasoning pipeline.
2. **Context injection.** The Engine fills branch conditions from the
   Runtime's working memory (project constraints, pinned versions, existing
   dependencies).
3. **Walk.** Starting at `root`, each node evaluates the user's context against
   branches. `terminal` nodes emit `recommendation[]` + `rationale`.
4. **Fallback.** If no tree matches, fall through to the full Reasoner pipeline.

## 4. Tree sources

- **Compiled from source docs** — tutorials, official guides, and engineering
  blogs are parsed into decision trees by the Compiler's Semantic Extractor.
- **Community contributed** — users submit trees via PR; they are reviewed and
  signed like packages.
- **Ecosystem analytics** — data-driven: "teams using X often also use Y"
  becomes a scored edge, not a tree node.

## 5. Verification of decisions

- Every `recommendation` must reference an existing node in the graph.
- `confidence` is derived from the tree's provenance + the user's context match.
- Recommendations are run through the Verifier before being returned to the
  user.

## 6. Decision tree catalog (v1 targets)

| Domain | Tree count |
| --- | --- |
| Flutter state management | 8 |
| Flutter local storage | 5 |
| Flutter networking | 4 |
| Supabase setup & schema | 6 |
| React state management | 8 |
| Python async patterns | 4 |
| General: "which package for X?" | 20+ |
