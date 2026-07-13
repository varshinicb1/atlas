---
kind: Package
id: package:repomix
name: "Repomix"
version: "0.2.0"
purpose: Repository packing tool that serialises entire codebases into single AI-friendly files (XML/Markdown/JSON/Plain) with token counting, security scanning, remote repo support, code compression, and MCP server mode
problem_solved: AI coding assistants have limited context windows and no built-in way to understand a full repository. Repomix packs an entire codebase — directory structure, file contents, git history — into a single file that fits an LLM's context, with token counts, sensitivity scanning, and structured output formats optimised for AI comprehension.
install: npm install -g repomix
dependencies: []
concepts:
  - name: Repository Packing
    id: concept:repomix/packing
    description: The core process — traverses a directory tree (respecting .gitignore, .repomixignore, custom patterns), reads each file, and concatenates into a single output with clear separators (XML tags, markdown headings, plain text dividers).
  - name: Token Counting
    id: concept:repomix/tokens
    description: Counts tokens using configurable encodings (o200k_base for GPT-4o, cl100k_base for GPT-3.5/4) per file and total, surfaced in the output header and console summary. Supports --token-budget as a CI guard that fails when output exceeds N tokens.
  - name: Security Scanning
    id: concept:repomix/security
    description: Integrates Secretlint to detect API keys, passwords, tokens, and other sensitive data in the packed output before it reaches an LLM. Can be disabled with --no-security-check.
  - name: Code Compression
    id: concept:repomix/compress
    description: Uses Tree-sitter AST parsing to extract essential code structure (class signatures, function declarations, interfaces, type definitions) while stripping implementation bodies. Reduces token count by 40-60% while preserving structural information.
  - name: Remote Repository Cloning
    id: concept:repomix/remote
    description: Clones any public Git remote (GitHub URL, user/repo shorthand, branch, tag, or commit URL) without leaving the tool, then packs it. Uses --remote-trust-config to optionally trust remote config files.
  - name: Multi-Format Output
    id: concept:repomix/formats
    description: Supports XML (default — with AI-optimised XML tag structure), Markdown (human-readable), JSON (programmatic processing via jq), and Plain Text. Each format includes a file summary, directory structure, file contents, and optional custom instructions.
  - name: Stdin Pipeline
    id: concept:repomix/stdin
    description: Accepts file paths via stdin (find, git ls-files, rg, fd, fzf — any command that outputs file paths), enabling integration with existing developer workflows and toolchains.
  - name: File Selection System
    id: concept:repomix/patterns
    description: Glob-based include/ignore patterns with per-file inclusion levels (full content, compressed, only structural metadata). Supports .gitignore, .ignore, .repomixignore, and built-in default patterns (node_modules, .git, build dirs).
  - name: MCP Server Mode
    id: concept:repomix/mcp
    description: Run as a Model Context Protocol server for AI tool integration — enables LLMs to pack repositories on-demand via a standards-based protocol.
  - name: Watch Mode
    id: concept:repomix/watch
    description: Filesystem watcher that auto-repacks on file changes (300ms debounce), ideal for development loops where the packed file is consumed by an LLM in a side panel.
  - name: Split Output
    id: concept:repomix/split
    description: Splits large outputs into numbered parts (repomix-output.1.xml, repomix-output.2.xml) with configurable max size (e.g., 500kb, 2mb) to stay within per-file context limits.
  - name: Git Context
    id: concept:repomix/git
    description: Includes git commit history (--include-logs with configurable count) and diffs (--include-diffs) so AI models understand code evolution, not just the current state.
  - name: Agent Skills Generation
    id: concept:repomix/skills
    description: Generates Claude Agent Skills format output (SKILL.md) from a repository, enabling instant creation of installable agent skills from any codebase.
apis:
  - name: repomix CLI
    id: api:repomix/cli
    signature: "repomix [path] [options]"
    returns: "A packed file (default: repomix-output.xml) containing the entire repository in AI-friendly format."
    description: The primary CLI — run in any directory to pack it. Supports --include, --ignore, --style, --compress, --remote, --stdin, --watch, --mcp, and more.
  - name: repomix --remote
    id: api:repomix/remote
    signature: "repomix --remote <url-or-user/repo> [--remote-branch <branch|tag|commit>]"
    returns: A packed file of the remote repository.
    description: Clone and pack any public Git remote without manual cloning. Supports GitHub shorthand (user/repo), full URLs, branch URLs, and commit URLs.
  - name: repomix --compress
    id: api:repomix/compress
    signature: "repomix --compress"
    returns: A compressed output with 40-60% fewer tokens, preserving code structure.
    description: Uses Tree-sitter to extract only essential code elements — class/function signatures, interfaces, type definitions — stripping implementation bodies.
  - name: repomix --mcp
    id: api:repomix/mcp
    signature: "repomix --mcp"
    returns: Runs as an MCP server for AI tool integrations.
    description: Enables LLMs and AI tools to pack repositories on-demand via the Model Context Protocol.
  - name: repomix --skill-generate
    id: api:repomix/skill-gen
    signature: "repomix --skill-generate [name]"
    returns: A SKILL.md file in .claude/skills/<name>/ directory.
    description: Generates a reusable Claude Agent Skill from any repository's codebase, making it installable across AI coding assistants.
  - name: repomix.com (Web UI)
    id: api:repomix/web
    signature: "https://repomix.com — paste a GitHub URL, click Pack"
    returns: A downloadable packed file of the repository.
    description: Browser-based version of Repomix for quick repository analysis without CLI installation.
examples:
  - id: example:repomix/basic
    language: shell
    description: "Pack current directory: repomix → repomix-output.xml"
  - id: example:repomix/remote
    language: shell
    description: "Pack a remote repo: repomix --remote yamadashy/repomix"
  - id: example:repomix/compress
    language: shell
    description: "Pack with compression: repomix --compress"
  - id: example:repomix/stdin-find
    language: shell
    description: "Pack specific files via stdin: find src -name '*.ts' | repomix --stdin"
  - id: example:repomix/prompt
    language: text
    description: "After packing, feed to an LLM: 'This file contains my codebase. Please review the structure and suggest improvements.'"
failures:
  - id: failure:repomix/secret-exposure
    symptom: API keys, tokens, or passwords appear in the packed output.
    cause: Files containing secrets were included without security scanning enabled.
    fix: Enable --security-check (default); add sensitive files to .gitignore or .repomixignore; use .env pattern exclusions.
  - id: failure:repomix/context-overflow
    symptom: The packed file exceeds the LLM's context window and gets truncated.
    cause: --compress was not used, or --token-budget was set too high for the target model.
    fix: Use --compress; set --token-budget <model-limit>; use --split-output for very large repos.
  - id: failure:repomix/binary-garbled
    symptom: Binary files (images, PDFs, archives) produce garbled text in the output.
    cause: Repomix attempts to read binary files as text, producing unreadable content.
    fix: Add binary file patterns (".png", ".jpg", ".pdf", ".zip") to .repomixignore or --ignore.
extends:
  - concept:repomix/packing
uses:
  - concept:repomix/tokens
  - concept:repomix/security
  - concept:repomix/compress
  - concept:repomix/remote
part_of: concept:domain/developer-tools
depends_on:
  - package:npm/patterns
solves:
  - problem:llm-context-limits
  - problem:codebase-analysis
alternatives:
  - package:gitingest
  - package:code2prompt
  - package:llm-code-extractor
---
# Repomix — Pack Your Codebase for AI

Repomix packs your entire repository into a single, AI-friendly file. It is designed for when you need to feed your codebase to LLMs (Claude, ChatGPT, DeepSeek, Gemini, Grok) and other AI tools.

The output file includes a file summary (metadata and AI usage instructions), directory structure tree, all file contents with clear separators, and optional custom instructions. The XML format uses AI-optimised XML tag structure for better LLM comprehension. Token counts are provided per-file and in total, helping you stay within model context limits.

## Key Workflows

1. **Local directory**: Run `repomix` in any project directory — it respects .gitignore and outputs a single XML file.
2. **Remote repository**: `repomix --remote user/repo` clones and packs in one command.
3. **Compressed output**: `repomix --compress` uses Tree-sitter to extract only structural code (signatures, types, interfaces) reducing token count by 40-60%.
4. **CI pipeline**: `repomix --token-budget 128000` fails if the packed output exceeds 128K tokens — a guard against context overflow.
5. **MCP integration**: `repomix --mcp` runs as an MCP server, letting AI tools pack repositories on-demand.
6. **Skill generation**: `repomix --skill-generate agent-name` creates an installable Claude Code skill from any repository.

## Decision Tree

- **Need to give AI context about a codebase?** → `repomix` — pack local directory
- **Need to analyse a GitHub repo without cloning?** → `repomix --remote user/repo`
- **Repo too large for context window?** → `repomix --compress --token-budget 128000`
- **Need to pipe into an AI tool?** → `repomix --stdout | llm "explain this code"`
- **Need an AI to have access on-demand?** → `repomix --mcp`
- **Need to share codebase selectively?** → `repomix --include "src/**/*.ts" --ignore "**/*.test.ts"`
- **Need to monitor and auto-repack?** → `repomix --watch`
- **Output too large for one file?** → `repomix --split-output 2mb`
