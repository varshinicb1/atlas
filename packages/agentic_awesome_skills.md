---
kind: Package
id: package:agentic-awesome-skills
name: "Agentic Awesome Skills"
version: "14.1.0"
purpose: Installable library of 1,943+ agentic SKILL.md playbooks for Claude Code, Gemini CLI, Cursor, Codex CLI, Copilot, OpenCode, and other AI coding assistants — with specialised plugins, bundles, workflows, and a CLI installer
problem_solved: AI coding agents are only as good as their operating instructions. Instead of writing one-off prompts for every recurring task (code review, test generation, PR writing, security audit), Agentic Awesome Skills provides a searchable, installable catalog of proven SKILL.md playbooks that give agents better context, stronger constraints, and clearer outputs — covering development, testing, security, infrastructure, product, and marketing.
install: npx agentic-awesome-skills
dependencies: []
concepts:
  - name: Skill Library
    id: concept:agentic-awesome/skills
    description: A collection of 1,943+ reusable SKILL.md playbooks organised under skills/<name>/SKILL.md, each containing structured instructions for an AI agent to perform a specific task — code review, test generation, architecture design, security audit, etc.
  - name: CLI Installer
    id: concept:agentic-awesome/installer
    description: npx agentic-awesome-skills installs skills to the correct path for each AI coding assistant. Supports --claude, --cursor, --codex, --gemini, --antigravity, --kiro, --opencode, --path custom, and --category/--risk/--tags filtered installs to reduce context overhead.
  - name: Specialized Plugins
    id: concept:agentic-awesome/plugins
    description: Focused, domain-specific distributions of the skill library — AAS Web App Builder (10 skills), Security Engineer (10), Data Analytics (10), DevOps & Cloud (10), Agent & MCP Builder (10), QA & Test Automation (10), SaaS Launch & Revenue (10), and more. Marketplace-style packaging for Claude Code and Codex.
  - name: Bundles
    id: concept:agentic-awesome/bundles
    description: Curated recommendations for roles or goals — Web Wizard, Security Engineer, OSS Maintainer, Full-Stack Developer. Bundles are metadata-level groupings for exploration after a full install, not separate installable units.
  - name: Workflows
    id: concept:agentic-awesome/workflows
    description: Ordered execution playbooks for outcome-driven results — shipping a SaaS MVP, running a security audit, doing QA/browser automation. Defined in docs/users/workflows.md and data/workflows.json for both human and machine consumption.
  - name: Stable Skills Manifest
    id: concept:agentic-awesome/manifest
    description: V1 manifest (skills_index.json) provides a canonical array-format index of every skill with id, path, category, tags, risk level, and dependencies — enabling hosts to discover and load only requested skills without scanning the filesystem.
  - name: Multi-Platform Support
    id: concept:agentic-awesome/platforms
    description: Skills are written once and installed across Claude Code, Cursor, Codex CLI, Gemini CLI, Autohand Code, Antigravity (IDE + CLI), Kiro, OpenCode, GitHub Copilot, and AdaL CLI — each with tool-specific path conventions and activation mechanisms.
  - name: Categories & Risk Levels
    id: concept:agentic-awesome/categorization
    description: Every skill is tagged with category (development, backend, frontend, security, devops, data, product, design, ai, testing, docs) and risk level (safe, none, dangerous, risky, critical), allowing filtered installs and context-appropriate activation.
  - name: Skill Sources & Attribution
    id: concept:agentic-awesome/sources
    description: Aggregates from 30+ official sources (Anthropic, OpenAI, Microsoft, Google, Vercel, Supabase, Expo, Hugging Face, Neon, Weaviate, Browserbase) and community contributors — each with documented provenance and license attribution.
  - name: Catalog Discovery
    id: concept:agentic-awesome/discovery
    description: Three discovery surfaces — CATALOG.md (full markdown registry), skills_index.json (machine-readable manifest for hosts), and a hosted GitHub Pages web app at sickn33.github.io/agentic-awesome-skills/ for searchable browsing.
apis:
  - name: npx installer
    id: api:agentic-awesome/installer
    signature: "npx agentic-awesome-skills [--claude | --cursor | --codex | --gemini | --opencode | --path <dir>] [--category dev,backend] [--risk safe,none]"
    returns: Skills installed to the target directory.
    description: The primary entrypoint — installs the full library or filtered subset to the correct path for each AI coding assistant.
  - name: skills_index.json
    id: api:agentic-awesome/manifest
    signature: "https://raw.githubusercontent.com/sickn33/agentic-awesome-skills/main/skills_index.json"
    returns: A JSON array of all 1,943+ skills with id, path, category, tags, risk, and metadata.
    description: The canonical machine-readable manifest for skill discovery, used by AI coding assistants to load only requested skills.
  - name: CATALOG.md
    id: api:agentic-awesome/catalog
    signature: "https://raw.githubusercontent.com/sickn33/agentic-awesome-skills/main/CATALOG.md"
    returns: A human-readable markdown registry of all skills organised by category.
    description: The full catalog for human browsing — read directly or served via the hosted web app.
  - name: Specialized Plugin Bundles
    id: api:agentic-awesome/plugins-api
    signature: "https://github.com/sickn33/agentic-awesome-skills/tree/main/plugins/<plugin-name>"
    returns: A focused SKILL.md pack for a specific domain (web apps, security, data, etc.).
    description: Generated plugin bundles that package the 10 most relevant skills for a domain — installable as Claude Code or Codex plugins.
  - name: Workflow Definitions
    id: api:agentic-awesome/workflows-api
    signature: "https://raw.githubusercontent.com/sickn33/agentic-awesome-skills/main/data/workflows.json"
    returns: A JSON array of ordered execution playbooks with step-by-step skill references.
    description: Machine-readable workflow definitions for automating multi-step processes (SaaS launch, security audit, QA pipeline).
examples:
  - id: example:agentic-awesome/install-basic
    language: shell
    description: "npx agentic-awesome-skills --path .agents/skills --category development,backend --risk safe,none"
  - id: example:agentic-awesome/install-claude
    language: shell
    description: "npx agentic-awesome-skills --claude"
  - id: example:agentic-awesome/use-skill
    language: text
    description: "In Claude Code: /brainstorming help me plan a feature"
  - id: example:agentic-awesome/install-cursor
    language: shell
    description: "npx agentic-awesome-skills --cursor"
failures:
  - id: failure:agentic-awesome/context-overload
    symptom: AI agent hits context limits or slows down with too many active skills.
    cause: Full library of 1,943+ skills was installed without filtering by category or risk.
    fix: Use --category and --risk flags for a reduced install; activate only bundles or specific skill IDs at runtime; use agent-overload-recovery.md guidance.
  - id: failure:agentic-awesome/manifest-stale
    symptom: New skills or updates not reflected in the local install.
    cause: The skills_index.json and skill files are release-pinned; updates only arrive on new npm releases.
    fix: Re-run npx agentic-awesome-skills to pick up the latest release; use --tag main for bleeding-edge (not recommended for production).
  - id: failure:agentic-awesome/path-mismatch
    symptom: Skills installed but agent does not detect them.
    cause: Skills were installed to a path the agent does not monitor.
    fix: Use the tool-specific flag (--claude, --cursor, --codex) instead of --path; or configure the agent to watch the custom path.
extends:
  - concept:agentic-awesome/skills
uses:
  - concept:agentic-awesome/installer
  - concept:agentic-awesome/plugins
  - concept:agentic-awesome/manifest
  - concept:agentic-awesome/categorization
part_of: concept:domain/developer-tools
depends_on:
  - package:npm/patterns
  - package:ai/patterns
solves:
  - problem:agent-instructions
  - problem:prompt-reusability
alternatives:
  - package:anthropic-skills
  - package:awesome-claude-skills
  - package:opencode-skills
---
# Agentic Awesome Skills — 1,943+ Playbooks for AI Coding Assistants

Agentic Awesome Skills is an installable GitHub library and npm installer for 1,943+ reusable SKILL.md playbooks designed for Claude Code, Cursor, Codex CLI, Gemini CLI, Antigravity, OpenCode, GitHub Copilot, and other AI coding assistants.

Instead of collecting one-off prompt snippets, this repository provides a searchable, installable catalog of **skills** (reusable SKILL.md files for specific tasks), **specialized plugins** (focused domain packs), **bundles** (role-based recommendations), **workflows** (ordered execution playbooks), and **a CLI installer** that puts skills where your tool expects them.

## Key Workflows

1. **Install**: `npx agentic-awesome-skills --path .agents/skills --category development,backend`
2. **Use**: In any AI coding assistant, invoke a skill by name: `/brainstorming help me plan a feature`
3. **Discover**: Browse CATALOG.md or the hosted web app for skill discovery
4. **Plugin**: Choose a specialized plugin for focused domain work (web apps, security, data, DevOps)
5. **Workflow**: Follow ordered playbooks for multi-step outcomes (shipping, auditing, testing)

## Decision Tree

- **Need skills for development work?** → `npx agentic-awesome-skills --category development,backend`
- **Need a focused domain pack?** → Install a specialized plugin (Web App Builder, Security Engineer, etc.)
- **Need ordered execution?** → Follow a workflow from docs/users/workflows.md
- **Need to discover what's available?** → Read CATALOG.md or visit sickn33.github.io/agentic-awesome-skills/
- **Need to reduce context overhead?** → Filter install with --category and --risk; use agent-overload-recovery.md
- **Need to integrate with a host?** → Read docs/users/discovery-manifest.md for the Stable Skills Manifest v1 contract
