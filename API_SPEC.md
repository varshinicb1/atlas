# API Specification

Atlas exposes four API surfaces: **CLI** (developer), **SDK** (Python programmatic),
**MCP Server** (AI agent integration), and an internal **C ABI** (Rust core).

## 1. CLI

```bash
atlas install <package>[@<version>]   # download & verify .atlas bundle
atlas compile <source>...             # compile sources → .atlas
atlas solve <problem>                 # full runtime pipeline
atlas decide <problem>                # decision engine only (fast path)
atlas graph <package>                 # print/subgraph in terminal (planned)
atlas verify [--artifact path]        # run verifier
atlas doctor                          # check integrity of installed bundles (planned)
atlas update [package]                # fetch latest version (planned)
atlas list                            # installed bundles (planned)
```

Flags: `--model` (which LLM for reasoner), `--offline`, `--emit-ir`,
`--incremental`, `--json` (structured output).

## 2. Python SDK

```python
import atlas

# loading
bundle = atlas.load("flutter@3.24")
bundle = atlas.load("flutter", version="^3.24")

# search (structured, not semantic-only)
node = bundle.find("Riverpod")
nodes = bundle.search("state management")           # hybrid: symbol + embedding
nodes = bundle.filter(kind="Function", lifecyle="stable")

# navigation
neighbors = bundle.neighbors("Riverpod", dir="outbound", depth=2)
subgraph = bundle.subgraph("StateManagement", depth=3)

# solve a problem
answer = atlas.solve("offline synced state in Flutter",
                     model="phi-3-mini",
                     bundles=[bundle])
# answer.text, answer.nodes[], answer.confidence, answer.provenance[]

# decision engine fast path
rec = atlas.decide("local-only Flutter database")
# rec.recommendation[], rec.rationale

# compile
result = atlas.compile(["./src", "./docs"], out="my-project.atlas")

# verify
report = atlas.verify("flutter@3.24", artifact="./generated/main.dart")
# report.passed, report.checks[]
```

## 3. MCP Server

Exposes Atlas tools to any MCP-compatible agent (Claude, Continue, Cursor,
custom):

| Tool | Description |
| --- | --- |
| `atlas_search` | Search installed bundles for a symbol or concept |
| `atlas_solve` | Full pipeline: context → guided answer |
| `atlas_decide` | Decision tree walk (fast, no LLM) |
| `atlas_navigate` | Traverse to neighboring nodes |
| `atlas_verify` | Verify generated code against graph |
| `atlas_install` | Install a package bundle |

All tools return structured JSON with `provenance` on every node.

## 4. C ABI (Rust cdylib)

```c
// atlas.h
typedef uint64_t AtlasHandle;
typedef struct { const char* text; uint32_t len; } AtlasString;

AtlasHandle atlas_load(const char* path, AtlasError* err);
AtlasString atlas_solve(AtlasHandle h, const char* problem, AtlasError* err);
AtlasString atlas_navigate(AtlasHandle h, const char* node_id, uint32_t depth, AtlasError* err);
void atlas_free_string(AtlasString s);
void atlas_unload(AtlasHandle h);
```

Used by the Python SDK (via `pyo3`) and the MCP server (via local FFI).

## 5. Error model

All surfaces return errors in a consistent shape:

```json
{
  "code": "NODE_NOT_FOUND",
  "message": "Symbol 'Riverpod' not found in bundle flutter@3.24",
  "candidates": ["package:flutter/riverpod", "package:flutter/provider"]
}
```

Error codes are stable identifiers (not free text) so tools can handle them
programmatically.
