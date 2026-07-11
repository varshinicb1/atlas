# Contributing to Atlas

Thank you for your interest in contributing to Atlas! This document explains how to get involved.

## Ways to Contribute

1. **Knowledge packages** — The easiest way to help. Package engineering knowledge (Flutter, Rust, TypeScript, Python, Go, etc.) into `.atlas` format.
2. **Core features** — Compiler, runtime, CLI, SDK improvements.
3. **Bug reports** — File issues with repro steps.
4. **Documentation** — Fix typos, improve guides, add examples.
5. **Registry** — Help build the enterprise package hub.

## Getting Started

### Prerequisites

- Rust 1.75+ (`rustup`)
- Node.js 18+ (for registry, studio, MCP server)
- Python 3.10+ (for SDK)
- `wrangler` (for registry deployment)

### Local Development

```bash
# Clone
git clone https://github.com/varshinicb1/atlas
cd atlas

# Build all Rust crates
cargo build --release

# Run tests
cargo test

# Lint (CI enforces zero warnings)
cargo clippy -- -D warnings
```

## Contribution Workflow

1. **Fork** the repo and create a branch: `git checkout -b feature/my-feature`
2. **Make changes** — keep commits focused.
3. **Test locally**:
   - Rust: `cargo test && cargo clippy -- -D warnings`
   - Registry: `cd registry && npm test` (vitest)
4. **Commit** using [Conventional Commits](https://www.conventionalcommits.org/):
   - `feat:` new feature
   - `fix:` bug fix
   - `docs:` documentation
   - `test:` tests
   - `chore:` maintenance
   - `perf:` performance
   - `refactor:` refactoring
5. **Push** and open a **Pull Request** against `main`.
6. **CI must pass** — build, test, clippy, compile-all, validate-all.

## Knowledge Package Guidelines

A knowledge package is a markdown file + optional decision tree YAML.

**Markdown structure:**
```markdown
---
kind: Package
id: package:my_framework
name: My Framework
version: 0.1.0
purpose: One-line description
concepts:
  - name: ConceptName
    id: concept:my_framework/concept_name
    description: What it is
apis:
  - name: api_name
    id: api:my_framework/api_name
    signature: func(arg: Type) -> Return
    description: What it does
---

# My Framework

Narrative prose that the embedding + search engine indexes.
```

**Rules:**
- Every node must have a stable, namespaced `id`.
- Cross-references use `part_of:` / `depends_on:` frontmatter fields.
- Decision trees go in `decisions/<name>.yaml`.

## Code Style

- **Rust:** `cargo fmt` before committing. No `#[allow(dead_code)]` hacks.
- **TypeScript:** Prettier defaults. No `any` unless unavoidable.
- **Python:** Black + type hints.

## Reporting Bugs

Open an issue with:
- Atlas version (`atlas --version`)
- Steps to reproduce
- Expected vs actual behavior
- Logs (`RUST_LOG=debug`)

## Security

Found a vulnerability? **Do not open a public issue.** Report it at https://github.com/varshinicb1/atlas/issues or use GitHub's private vulnerability reporting.

## Code of Conduct

This project adheres to the [Contributor Covenant](CODE_OF_CONDUCT.md). By participating, you agree to uphold it.

## Release Process

- Maintainers cut releases by tagging `vX.Y.Z`.
- CI builds binaries for Linux/macOS/Windows and publishes to GitHub Releases + crates.io.
- CHANGELOG is generated from conventional commits.

Thank you for making AI agents deterministic, verifiable, and compliant! 🚀
