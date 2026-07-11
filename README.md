# Atlas — The Knowledge Operating System

[![CI](https://github.com/varshinicb1/atlas/actions/workflows/ci.yml/badge.svg)](https://github.com/varshinicb1/atlas/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/varshinicb1/atlas?label=release)](https://github.com/varshinicb1/atlas/releases)
[![License: Apache 2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/made%20with-Rust-orange.svg)](https://www.rust-lang.org/)
[![Crates.io](https://img.shields.io/crates/v/atlas-cli?label=crates.io)](https://crates.io/crates/atlas-cli)
[![Downloads](https://img.shields.io/github/downloads/varshinicb1/atlas/total)](https://github.com/varshinicb1/atlas/releases)
[![Dependabot](https://img.shields.io/badge/dependabot-enabled-brightgreen.svg)](.github/dependabot.yml)
[![Website](https://img.shields.io/badge/website-atlas--sh.pages.dev-blue)](https://atlas-sh.pages.dev)
[![Registry](https://img.shields.io/badge/registry-atlas--hub--registry.cbvarshini1.workers.dev-blue)](https://atlas-hub-registry.cbvarshini1.workers.dev/health)

**The Terraform for AI agent knowledge.**

Atlas separates **knowledge** from **reasoning**. It compiles engineering knowledge into a deterministic, verifiable, binary format (`.atlas`) so AI agents can make decisions **without an LLM call** — 100× cheaper, 1000× faster, provably correct.

---

## Why Atlas?

Every agent framework (LangChain, CrewAI, AutoGen, smolagents) treats knowledge as "stuff in the prompt." Atlas is different:

| | Atlas | smolagents / LangChain / CrewAI |
|---|---|---|
| Deterministic decisions | ✅ LLM-free | ❌ |
| Built-in verification | ✅ | ❌ |
| Knowledge composition | ✅ | ❌ (RAG only) |
| Offline / Air-gapped | ✅ | ❌ |
| Cryptographic audit trail | ✅ | ❌ |
| Cost per query | ~$0.0001 | $0.10–1.00 |
| EU AI Act ready | ✅ Built-in | ❌ |

See [benchmarks vs smolagents](docs/benchmarks-vs-smolagents.md).

---

## Quick Start

### Install

```bash
# From crates.io
cargo install atlas-cli

# Download prebuilt binaries
# Grab the latest `atlas` binary for your platform from:
# https://github.com/varshinicb1/atlas/releases
# (Linux, macOS, and Windows builds available)

# From source
git clone https://github.com/varshinicb1/atlas
cd atlas
cargo build --release

# Or via Python SDK
pip install atlas-sdk
```

### Compile knowledge

```bash
atlas init my-package --template flutter
atlas compile my-package.md decisions/ --out my-package.atlas
```

### Query it

```bash
atlas solve --bundle my-package.atlas "which widget?"
atlas decide --bundle my-package.atlas "widget" -c "answer=true"
atlas verify --bundle my-package.atlas --policy eu-ai-act
```

### Python SDK

```python
from atlas_sdk import Agent
agent = Agent("flutter_core")
result = agent.solve("stateful widget")
print(result)
```

---

## Components

| Component | Path | Description |
|---|---|---|
| **CLI** | `atlas-cli/` | `atlas` binary — compile, solve, decide, verify, reason, dump, init, search, publish |
| **Compiler** | `atlas-compiler/` | Markdown + YAML → canonical Engineering IR |
| **Runtime** | `atlas-runtime/` | Memory-mapped loader, decision engine, verification, reasoner |
| **IR** | `atlas-ir/` | 11 node kinds, 13 edge types, embeddings, decision trees |
| **Python SDK** | `python/` | `atlas-sdk` — typed dataclasses wrapping the CLI |
| **MCP Server** | `mcp-server/` | 5 tools + resource template (stdio) |
| **Studio** | `studio/` | Next.js 16 + React Flow graph explorer |
| **VS Code Extension** | `vscode-atlas/` | In-editor knowledge graph explorer |
| **GitHub Action** | `.github/actions/` | `atlas-compile` + `atlas-verify` CI gates |
| **Registry** | `registry/` | Cloudflare Worker + Durable Object + KV — enterprise package hub |
| **Website** | `website/` | Landing page + registry search (Cloudflare Pages) |
| **Knowledge Packages** | `packages/` | flutter_core, rust_patterns, typescript_nextjs, atlas, riverpod, typescript_7_migration |

---

## Architecture

Atlas has four layers:

1. **Compiler (Rust)** — Parses markdown + YAML sources into the canonical Engineering IR, builds embeddings and indices, writes the `.atlas` binary.
2. **Binary Format** — Zstd-compressed sections (meta, ontology, nodes, edges, decision_trees, examples, failure_modes, verification_rules, indices) with BLAKE3 checksums and optional Ed25519 signing.
3. **Runtime (Rust)** — Memory-maps `.atlas` bundles; provides `solve` (hybrid search), `decide` (decision tree walker), `verify` (structural checks), `reason` (pluggable stage).
4. **CLI + SDKs** — `atlas` binary, Python SDK, MCP server, Studio.

### Key Design Decisions

- **No LLM in the loop** for `solve`, `decide`, or `verify` — purely structural operations on canonical data.
- **Polyglot**: Rust (core), TypeScript (Studio/MCP), Python (SDK).
- **Self-describing**: Every node carries provenance (source file + line) and confidence scores.

---

## Registry (Enterprise)

The Atlas Hub Registry is a multi-tenant, enterprise-grade package registry.

**Endpoints**
- `GET /health` — health + package counts
- `GET /api/v1/packages` — list packages
- `GET /api/v1/packages/:name` — get package
- `GET /api/v1/search?q=` — search packages
- `POST /api/v1/publish` — publish (requires API key)
- `GET /api/v1/org/stats` — org usage stats
- `GET /api/v1/org/audit` — org audit log (admin)
- `GET /api/v1/openapi` — OpenAPI 3.0.3 spec

**Enterprise features**
- 🔐 API-key auth (X-API-Key header)
- ⏱️ Rate limiting (60 req/min per key)
- 🏢 Multi-tenant orgs
- 📊 Usage tracking (per-org daily publishes/downloads)
- 📝 Audit logging (every publish logged with timestamp + IP)
- ✅ Proper HTTP status codes (400/401/403/404/429/500)

Live at: `https://atlas-hub-registry.cbvarshini1.workers.dev`

```bash
# Publish with auth
atlas publish packages/flutter_core.md --api-key <KEY>

# Search public registry
atlas search flutter
```

---

## Compliance

```bash
atlas verify --bundle my-package.atlas --policy eu-ai-act
```

Built-in verification checks structural integrity (API existence, version consistency, provenance) and supports compliance policies for regulated industries.

---

## Development

```bash
# Build
cargo build --release

# Test
cargo test

# Lint
cargo clippy -- -D warnings

# Compile + verify all packages
cargo build --release
for d in packages/*.md; do
  name=$(basename "$d" .md)
  ./target/release/atlas compile "$d" --out "target/$name.atlas"
  ./target/release/atlas verify --bundle "target/$name.atlas"
done
```

### CI/CD

GitHub Actions runs on every push/PR:
- **build** — `cargo build --release`, `cargo test`, `cargo clippy -- -D warnings`
- **compile-all** — compiles every knowledge package
- **validate-all** — verifies every compiled bundle
- **website** — checks website assets
- **registry-check** — checks registry code

Releases are automated via `.github/workflows/release.yml` on tag push.

---

## Documentation

- [Benchmarks: Atlas vs smolagents](docs/benchmarks-vs-smolagents.md)
- [Website](https://atlas-sh.pages.dev)
- [Registry API](https://atlas-hub-registry.cbvarshini1.workers.dev/api/v1/openapi)

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). All contributions welcome — from knowledge packages to core features.

## License

Apache 2.0 — see [LICENSE](LICENSE).

---

<p align="center">
  <sub>Atlas — stop trusting LLMs, start trusting knowledge.</sub>
</p>
