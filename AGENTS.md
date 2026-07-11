# Atlas Company Builder — Agent Guide

## Mission
Build Atlas into a billion-dollar company: the Knowledge Operating System that makes AI agents deterministic, verifiable, and compliant. Separate knowledge (`.atlas` binary) from reasoning (LLM) — the "Terraform for AI agent knowledge."

## Current State (July 2026)

### Working
- Rust CLI: `compile`, `solve`, `decide`, `verify`, `install`, `reason`, `dump`, `init`, `search`, `publish` — binary named `atlas`, all with `--json`. Publish reads version/tags from YAML frontmatter.
- Python SDK: `pip install atlas-sdk` — wraps CLI, returns typed dataclasses (Node dataclass has confidence field)
- MCP server: 5 tools + 1 resource template, verified via stdio
- Studio: Next.js 16 + React Flow graph explorer (basic)
- Knowledge packages (13): atlas, flutter_core, riverpod, rust_patterns, typescript_7_migration, typescript_nextjs, python_patterns, go_patterns, react_patterns, docker_kubernetes, postgres_patterns, cloudflare_workers, mycelium_sync — all pass verify with 0 dangling edges, all published to registry
- VS Code extension: `vscode-atlas/` — Explorer panel, command palette, hover tooltips
- GitHub Action: `.github/actions/atlas-compile/` + `atlas-verify/` — CI/CD for knowledge
- Website: `https://atlas-sh.pages.dev` — landing + registry search
- Registry: `https://atlas-hub-registry.cbvarshini1.workers.dev` — enterprise-grade with auth, rate limiting, org stats, audit logging, usage tracking, OpenAPI spec, SSO/OIDC (JWT verification via JWKS), GDPR endpoints (DSAR, data portability, right to erasure)
- Backup Worker: `registry-backup/` — daily R2 backups with 30-day retention, manual trigger endpoint
- Status page: `status/` — live health dashboard for all Atlas services
- Compliance: `atlas verify --policy eu-ai-act`
- Benchmarks: `docs/benchmarks-vs-smolagents.md`
- CI: Build, test, clippy, compile-all, validate-all, website check, registry check (7/7 green)
- Binary name: `atlas` (was `atlas-cli`). Searches `atlas` first, fallback to `atlas-cli`
- All URLs use free Cloudflare subdomains (no custom domain needed)

### Enterprise Features
- Registry API key auth (X-API-Key header) — protect publish operations
- SSO/OIDC — JWT verification via JWKS from any OpenID Connect provider (Google, Azure AD, Okta). Accepts JWTs as `Authorization: Bearer <jwt>`
- Rate limiting — 60 requests/minute per API key (applied to ALL endpoints including unauthenticated)
- Multi-tenant orgs — packages scoped to organizations, strict cross-tenant isolation
- Usage tracking — per-org daily counts for publishes/downloads
- Audit logging — every publish/delete action logged with timestamp, IP, and API key
- OpenAPI 3.0.3 spec at `/api/v1/openapi`
- GDPR compliance: DSAR (`GET /api/v1/gdpr/dsar`), data portability (`GET /api/v1/gdpr/export`), right to erasure (`DELETE /api/v1/gdpr/forget`)
- Automated daily backups to R2 with 30-day retention (`registry-backup/`)
- V1 API (`/api/v1/`) with legacy backward compatibility (`/api/`)
- Proper HTTP status codes: 400 (validation), 401 (auth), 403 (permissions), 404 (not found), 405 (method not allowed), 429 (rate limit), 500 (errors)
- `X-RateLimit-Reset` header on all rate-limited responses
- Package name/version/tags/files validation on publish
- Public search and listing (no auth needed for read, org-scoped for authenticated)

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
- **Pricing**: Free & open source (Apache 2.0). Enterprise features (SSO, RBAC, audit) included for adoption, not paywalled
