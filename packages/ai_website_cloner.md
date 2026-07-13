---
kind: Package
id: package:ai-website-cloner
name: "AI Website Cloner Template"
version: "0.3.1"
purpose: Multi-agent pipeline for reverse-engineering any website into a Next.js + shadcn/ui codebase — reconnaissance, design token extraction, component specification, parallel building, and visual QA
problem_solved: Eliminates manual porting of production websites into modern frameworks. Instead of hand-inspecting CSS, copying HTML, and reimplementing components one by one, the cloner pipeline screenshots, extracts getComputedStyle values, writes exact component specs, and dispatches parallel builder agents via git worktrees.
install: git clone https://github.com/JCodesMore/ai-website-cloner-template; npm install
dependencies: []
concepts:
  - name: Reconnaissance Pipeline
    id: concept:ai-website-cloner/recon
    description: Phase 1 — screenshots (full-page, viewport, element), design token extraction (oklch colours, typography stack, spacing scale, border radii), interaction sweep (scroll, click, hover, responsive), and asset inventory (images, videos, icons, favicons, OG images).
  - name: Foundation Generation
    id: concept:ai-website-cloner/foundation
    description: Phase 2 — updates Tailwind v4 globals.css with extracted oklch colour tokens, imports fonts (Google Fonts, self-hosted), configures font-family and font-size in tailwind.config, downloads all assets into public/, and sets up layout.tsx with SEO metadata.
  - name: Component Specification
    id: concept:ai-website-cloner/spec
    description: Phase 3 — writes detailed spec files (docs/research/components/) with exact computed CSS values, interaction states (hover, active, focus, disabled), responsive breakpoints, content text, and asset paths for every distinct UI section on the page.
  - name: Parallel Build
    id: concept:ai-website-cloner/parallel
    description: Phase 4 — dispatches builder agents in git worktrees, one per section/component. Each builder receives the full component spec inline — exact values, no guessing. Worktrees allow true parallelisation without merge conflicts.
  - name: Assembly & QA
    id: concept:ai-website-cloner/qa
    description: Phase 5 — merges worktrees into main branch, wires up the page layout, runs visual diff against original screenshots, and iterates on mismatches before final commit.
  - name: Agent Skill System
    id: concept:ai-website-cloner/skills
    description: A multi-platform skill system where /clone-website is defined once in .claude/skills/clone-website/SKILL.md and auto-synced to .cursor, .codex, .continue, .gemini, .opencode, .windsurf, .github, and .amazonq via sync-agent-rules.sh and sync-skills.mjs.
  - name: Design Token Extraction
    id: concept:ai-website-cloner/tokens
    description: Programmatic extraction of oklch colour values, font stacks, spacing scales, border radii, and shadow definitions from getComputedStyle(), enabling pixel-perfect reproduction of the original design system.
  - name: Worktree Parallelisation
    id: concept:ai-website-cloner/worktrees
    description: Uses git worktrees to build components in parallel without conflicts. Each worktree is an independent working directory on a temporary branch; after completion, worktrees are merged sequentially into main.
apis:
  - name: /clone-website skill
    id: api:ai-website-cloner/clone-skill
    signature: "/clone-website <target-url1> [<target-url2> ...]"
    returns: A fully functional Next.js codebase matching the target website.
    description: The primary entrypoint — accepts one or more target URLs and runs the full recon → foundation → spec → build → QA pipeline.
  - name: sync-agent-rules.sh
    id: api:ai-website-cloner/sync-rules
    signature: "bash scripts/sync-agent-rules.sh"
    returns: Regenerated AGENTS.md, CLAUDE.md, GEMINI.md, .windsurfrules, .clinerules for each supported platform.
    description: Copies the single source of truth (AGENTS.md) to platform-specific instruction files.
  - name: sync-skills.mjs
    id: api:ai-website-cloner/sync-skills
    signature: "node scripts/sync-skills.mjs"
    returns: Regenerated /clone-website skill files for all supported platforms.
    description: Copies the single .claude/skills/clone-website/SKILL.md to .cursor/skills, .codex/skills, .continue/skills, etc.
  - name: Reconnaissance Scripts
    id: api:ai-website-cloner/recon-scripts
    signature: "src/lib/recon/*.ts"
    returns: Screenshots, design tokens, interaction data, asset inventory.
    description: TypeScript modules for full-page screenshots, design token extraction via Puppeteer/Playwright, interaction sweeps, and responsive breakpoint capture.
examples:
  - id: example:ai-website-cloner/saas-migration
    language: shell
    description: "Clone a SaaS landing page: /clone-website https://example-saas.com → outputs /clone-website skeleton with all components, design tokens, and assets."
  - id: example:ai-website-cloner/multi-url
    language: shell
    description: "Clone multiple pages: /clone-website https://site.com/page1 https://site.com/page2"
failures:
  - id: failure:ai-website-cloner/stale-tokens
    symptom: Extracted colours or fonts do not match the live website.
    cause: The recon phase may capture stale CSS if the page uses dynamic theming or loads fonts asynchronously.
    fix: Re-run with --no-cache to force fresh extraction; ensure all fonts are loaded before screenshot capture.
  - id: failure:ai-website-cloner/worktree-conflict
    symptom: Git merge conflicts during worktree assembly phase.
    cause: Two builder agents modified the same component file or shared utility.
    fix: Ensure spec boundaries are strict — each worktree should touch a unique set of files under src/components/<section>/.
  - id: failure:ai-website-cloner/agent-timeout
    symptom: A builder agent stalls or produces incomplete components.
    cause: The component spec omitted interaction states, responsive variants, or edge cases.
    fix: Enrich the spec template to include all states, breakpoints, and error boundaries before dispatching.
extends:
  - concept:nextjs/app-router
uses:
  - concept:ai-website-cloner/recon
  - concept:ai-website-cloner/spec
  - concept:ai-website-cloner/worktrees
part_of: concept:domain/web-platform
depends_on:
  - package:nextjs/patterns
  - package:tailwind/patterns
  - package:shadcn-ui
solves:
  - problem:website-migration
  - problem:lost-source-code
alternatives:
  - package:teleport-hq
  - package:syte-ai
  - package:wp-json-export
---
# AI Website Cloner Template

A reusable template for reverse-engineering any website into a clean, modern Next.js codebase using AI coding agents. Point it at a URL, run `/clone-website`, and the AI agent inspects the site, extracts design tokens and assets, writes component specs, and dispatches parallel builders to reconstruct every section.

The pipeline runs five phases in sequence: **Reconnaissance** (screenshots, design tokens, interaction sweep), **Foundation** (globals.css, fonts, assets), **Component Specs** (detailed docs/research/components/ with exact computed CSS values), **Parallel Build** (git worktrees, one per section), and **Assembly & QA** (merge, visual diff, iterate).

## Decision Tree

- **Need to port a live website to Next.js?** → `/clone-website <url>` — runs full pipeline
- **Need to rebuild from lost source code?** → `/clone-website <live-url>` — recovers design system from production
- **Need to learn from a production site?** → `/clone-website <url>` — generates annotated component specs
- **Need to add a new platform support?** → Edit `AGENTS.md` or `.claude/skills/clone-website/SKILL.md`, then run `bash scripts/sync-agent-rules.sh` or `node scripts/sync-skills.mjs`
- **Need a quick foundation only (no rebuild)?** → Set `CLONE_SCOPE=foundation` — extracts tokens, downloads assets, writes globals.css but skips component generation
- **Need to rebuild a multi-page site?** → `/clone-website <url1> <url2> <url3>` — shared design system across pages
- **Need visual confirmation before committing?** → QA phase runs pixel-diff against original screenshots in `docs/design-references/`
