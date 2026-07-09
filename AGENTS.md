# Atlas Company Builder — Agent Guide

## Mission
Build Atlas into a billion-dollar company: the Knowledge Operating System that makes AI agents deterministic, verifiable, and compliant. Separate knowledge (`.atlas` binary) from reasoning (LLM) — the "Terraform for AI agent knowledge."

## Current State (July 2026)

### Working
- Rust CLI: `compile`, `solve`, `decide`, `verify`, `install`, `reason`, `dump`, `init`, `search`, `publish` — binary named `atlas`, all with `--json`
- Python SDK: `pip install atlas-sdk` — wraps CLI, returns typed dataclasses
- MCP server: 5 tools + 1 resource template, verified via stdio
- Studio: Next.js 16 + React Flow graph explorer (basic)
- Knowledge packages: atlas, flutter_core, riverpod, rust_patterns, typescript_7_migration, typescript_nextjs (6 packages, all published)
- VS Code extension: `vscode-atlas/` — Explorer panel, command palette, hover tooltips
- GitHub Action: `.github/actions/atlas-compile/` + `atlas-verify/` — CI/CD for knowledge
- Website: `https://atlas-sh.pages.dev` — landing + registry search
- Registry: `https://atlas-hub-registry.cbvarshini1.workers.dev` — enterprise-grade with auth, rate limiting, org stats, audit logging, usage tracking, OpenAPI spec
- Compliance: `atlas verify --policy eu-ai-act`
- Benchmarks: `docs/benchmarks-vs-smolagents.md`
- CI: Build, test, clippy, compile-all, validate-all, website check, registry check
- Binary name: `atlas` (was `atlas-cli`)
- All URLs use free Cloudflare subdomains (no custom domain needed)

### Enterprise Features
- Registry API key auth (X-API-Key header) — protect publish operations
- Rate limiting — 60 requests/minute per API key
- Multi-tenant orgs — packages scoped to organizations
- Usage tracking — per-org daily counts for publishes/downloads
- Audit logging — every publish/delete action logged with timestamp and IP
- OpenAPI 3.0.3 spec at `/api/v1/openapi`
- V1 API (`/api/v1/`) with legacy backward compatibility (`/api/`)
- Proper HTTP status codes: 400 (validation), 401 (auth), 403 (permissions), 404 (not found), 429 (rate limit), 500 (errors)
- Detailed health endpoint showing package counts, per-package stats
- Public search and listing (no auth needed for read)

### Architecture Rules
- CLI commands in `atlas-cli/src/main.rs` — add to `Commands` enum, add run function
- `atlas init` creates files in CWD, no Rust library changes needed
- VS Code extension: `vscode-atlas/` dir, TypeScript, calls `atlas` CLI
- GitHub Action: composite action, installs atlas CLI, runs commands
- Website: static HTML/CSS/JS, hosted on Cloudflare Pages at atlas-sh.pages.dev
- Registry: Cloudflare Worker + DurableObject + KV, hosted at atlas-hub-registry.cbvarshini1.workers.dev
- Registry admin API key stored in KV at `apikey:<key>` → `{"org":"...","role":"admin"}`
- Binary named `atlas` (not `atlas-cli`)

### Market Positioning
- **One-liner**: "Terraform for AI agent knowledge"
- **Target**: AI engineering teams (2M globally), enterprise platforms, regulated industries
- **Competition**: smolagents (28K★), LangChain (95K★), CrewAI (28K★), AutoGen (40K★)
- **Unfair advantage**: Only architecture with deterministic decision trees, built-in verification, cryptographic audit trails, enterprise registry
- **Viral hook**: `pip install atlas-sdk; from atlas_sdk import Agent; agent.solve("question")`
- **Pricing**: $500/user/month — multi-tenant registry, RBAC, audit, compliance
