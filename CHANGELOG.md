# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-07-09

### Added
- **CLI** (`atlas` binary) with commands: `compile`, `solve`, `decide`, `verify`, `install`, `reason`, `dump`, `init`, `search`, `publish`
- **Compiler** — Markdown + YAML → canonical Engineering IR (11 node kinds, 13 edge types)
- **Runtime** — memory-mapped loader, hybrid search, decision tree walker, verification engine, pluggable reasoner
- **Python SDK** — `atlas-sdk` with typed dataclasses
- **MCP server** — 5 tools + 1 resource template (stdio)
- **Studio** — Next.js 16 + React Flow graph explorer
- **VS Code extension** — in-editor knowledge graph explorer
- **GitHub Action** — `atlas-compile` + `atlas-verify` CI gates
- **Registry** — Cloudflare Worker + Durable Object + KV with:
  - API-key auth (`X-API-Key`)
  - Rate limiting (60 req/min per key)
  - Multi-tenant orgs
  - Usage tracking (per-org daily publishes/downloads)
  - Audit logging
  - OpenAPI 3.0.3 spec at `/api/v1/openapi`
  - V1 API + legacy backward compatibility
- **Website** — landing page + registry search (Cloudflare Pages)
- **Knowledge packages**: `atlas`, `flutter_core`, `riverpod`, `rust_patterns`, `typescript_7_migration`, `typescript_nextjs`
- **Compliance** — `atlas verify --policy eu-ai-act`
- **Benchmarks** — Atlas vs smolagents comparison
- **CI/CD** — build, test, clippy, compile-all, validate-all, website + registry checks
- **Release automation** — cross-platform binaries (Linux/macOS/Windows) + crates.io publish
- **Dependabot** — weekly scans for cargo, npm, github-actions
- **Docs** — README with badges, CONTRIBUTING, CODE_OF_CONDUCT, CHANGELOG

### Security
- API-key-protected publish endpoint
- Input validation on all registry inputs
- Proper HTTP status codes (400/401/403/404/429/500)
