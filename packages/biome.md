---
kind: Package
id: package:biome
name: Biome Toolchain
version: "1.9"
purpose: Document Biome — a unified toolchain for formatting, linting, and organizing JavaScript/TypeScript/JSX/JSON/CSS code, designed as a faster, Rust-based replacement for ESLint and Prettier with minimal configuration.
problem_solved: Replaces the slow, configuration-heavy ESLint + Prettier setup with a single Rust-based binary that formats and lints 10-100x faster, requires zero configuration by default, and provides consistent rules across JS, TS, JSX, and JSON without plugin management.
install: npm install -D @biomejs/biome
dependencies:
  - concept:linting
  - concept:code-formatting
  - concept:typescript
  - concept:javascript
concepts:
  - name: Formatter
    id: concept:biome/formatter
    description: "Zero-config code formatter that handles JavaScript, TypeScript, JSX, JSON, and CSS. Opinionated defaults: 2-space indent, double quotes, trailing commas, 120 line width. Override via biome.json formatter section. No plugins — biome formats everything natively."
  - name: Linter
    id: concept:biome/linter
    description: Built-in linter with 200+ rules organized by category (correctness, style, complexity, performance, security, suspicious). Rules are enabled/disabled in biome.json. No plugin system — rules are compiled into the binary. Supports rule categories (error/warn/info) and fixable rules with --apply.
  - name: Configuration
    id: concept:biome/config
    description: "Config via biome.json (or biome.jsonc) at the project root. Sections: $schema, files (include/ignore), formatter, linter, javascript (parser, globals), organizesImports. Configuration is checked into version control for team consistency."
  - name: Editor Integration
    id: concept:biome/editor
    description: Official Biome VS Code extension — format on save, inline lint diagnostics, code actions (organize imports, fix lint). Respects biome.json config. No ESLint extension needed. Format on save replaces Prettier extension entirely.
  - name: CLI
    id: concept:biome/cli
    description: npx biome check . — runs formatter and linter together. npx biome format --write . for formatting only. npx biome lint --apply for auto-fixable lint rules. npx biome ci for CI — fails on any issue. npx biome migrate eslint --write to convert ESLint config.
  - name: Organize Imports
    id: concept:biome/organize-imports
    description: npx biome check --organize-imports — sorts and groups imports automatically. Removes unused imports. Handles default, named, namespace, and type imports. Merges imports from the same source. Configurable import order via biome.json.
  - name: VCS Integration
    id: concept:biome/vcs
    description: Integrates with git to only process changed files. npx biome check --changed since main — runs only on files changed since the specified branch. Drastically reduces CI lint times for large repos.
  - name: Migration
    id: concept:biome/migration
    description: npx biome migrate eslint converts ESLint config (including .eslintrc and eslint.config.js) to biome.json. npx biome migrate prettier converts .prettierrc to biome.json formatter config. After migration, uninstall ESLint and Prettier.
  - name: Performance
    id: concept:biome/performance
    description: Biome is written in Rust and operates on a parallel file-processed architecture. Formatting large monorepos completes in milliseconds to seconds. Multi-threaded file processing. No Node.js runtime overhead — Biome is a standalone binary.
apis:
  - name: npx biome check [path]
    id: api:biome/check
    signature: "npx biome check src/ --apply"
    returns: Exit code 0 (all good) or 1 (issues found).
    description: Runs formatter + linter in sequence. --apply applies safe fixes and formats. --apply-unsafe applies all fixes including potentially unsafe ones. --organize-imports organizes imports during check. --changed runs on files changed since branch.
  - name: npx biome format [path]
    id: api:biome/format
    signature: "npx biome format --write src/"
    returns: Exit code 0 (formatted) or 1 (unformatted without --write).
    description: Formats code according to biome.json config. --write applies formatting inline. --stdin-file-path formats input from stdin. --indent-style, --indent-width, --line-width overrides config.
  - name: npx biome lint [path]
    id: api:biome/lint
    signature: "npx biome lint --apply src/"
    returns: Exit code 0 (no issues) or 1 (issues).
    description: Runs lint rules. --apply applies safe fixes. --apply-unsafe applies all fixes. --rules-only to check specific rules. --error-on-warnings treats warnings as errors. Supports --max-diagnostics for output control.
  - name: npx biome ci
    id: api:biome/ci
    signature: "npx biome ci src/"
    returns: Exit code 0 (pass) or 1 (fail).
    description: CI mode — runs formatter + linter + organize imports. Errors on any issue (even warnings). No --apply available — CI should only report, not modify files. Integrates with GitHub Actions, CircleCI, and other CI systems.
  - name: biome.json config
    id: api:biome/config
    signature: '{ "$schema": "https://biomejs.dev/schemas/1.9/schema.json", "formatter": { "indentStyle": "space", "indentWidth": 2, "lineWidth": 100 }, "linter": { "rules": { "recommended": true, "correctness": { "noUnusedVariables": "error" } } }, "javascript": { "formatter": { "quoteStyle": "single" } } }'
    returns: Configuration object.
    description: Project configuration. files.include/ignore controls which files to process. formatter sets indent, line width, quote style. linter.rules enables/disables rule groups. javascript.formatter for JS/TS-specific overrides. organizesImports configures import sorting.
failures:
  - id: failure:biome/eslint-migration-incomplete
    symptom: Some important ESLint rules are not present in Biome and violations go undetected.
    cause: Biome does not implement all ESLint rules (about 200 of 300+ ESLint rules are covered, plus Biome-specific rules). Plugin-based rules (import, react, typescript-eslint) are not migrated.
    fix: Check the Biome rule comparison page for coverage. Use the LINT_THAT_CANNOT_BE_HANDLED for rules Biome does not support yet — often, they are for very specific project conventions. Consider if the missing rules are still needed (many ESLint rules are now Vite/TypeScript compiler errors).
  - id: failure:biome/formatting-conflicts
    symptom: Code alternates between two formatting styles on every format pass.
    cause: Multiple formatters running (Biome + Prettier, or Biome + Editor formatter) with conflicting configs.
    fix: "Use \"editor.formatOnSave\": true and \"editor.defaultFormatter\": \"biomejs.biome\" in VS Code. Disable Prettier extension after migration. Ensure biome.json is at the project root. Remove .prettierrc and eslint config files."
  - id: failure:biome/ignore-not-working
    symptom: Biome processes files that should be ignored (generated code, vendored dependencies).
    cause: The ignore pattern in biome.json matches differently than .gitignore or ESLint ignorePatterns.
    fix: "Use biome.json files.ignore with glob patterns. Use ! for negation. Check if patterns match by running biome check with --verbose. Common patterns: \"**/generated/**\", \"**/vendor/**\", \"**/*.min.*\"."
  - id: failure:biome/vue-sfc-unsupported
    symptom: Biome errors on .vue files or skips them entirely.
    cause: Biome does not support Vue Single File Components (template + script + style in one file). Same for Svelte files.
    fix: Use files.ignore to exclude .vue and .svelte files from Biome. Use the framework-specific linter (eslint-plugin-vue, eslint-plugin-svelte) for those files. Biome works on the <script> contents extracted by the framework toolchain if passed via stdin.
extends: []
implements: []
uses:
  - concept:linting
  - concept:code-formatting
  - concept:typescript
part_of: concept:developer-tooling
solves:
  - problem:code-formatting-speed
  - problem:lint-configuration-overhead
  - problem:eslint-prettier-config-complexity
alternatives:
  - package:eslint
  - package:prettier
  - package:dprint
  - package:oxlint
---
Biome is a radical simplification of the JavaScript tooling landscape. Instead of ESLint + Prettier + plugins + TypeScript-ESLint + config files for each, Biome is a single 20MB binary that does everything. Written in Rust with parallel file processing, it's 10-100x faster than equivalent ESLint + Prettier pipelines — a 10,000-file monorepo that takes minutes with ESLint/Prettier finishes in under a second with Biome.

The architectural decision to compile lint rules into the binary (no plugin system) is intentional. It eliminates the "ESLint plugin breaking" problem where updating a plugin or the plugin's peer dependency breaks CI. Biome's rule set is curated, documented, and tested together. New rules are added via Biome releases, not npm installs. The trade-off (inability to write custom rules) is offset by Biome's open-source contribution process and the fact that most custom ESLint rules were project-specific hacks that should have been PRs to the main tool.

Migration from ESLint + Prettier is handled by biome migrate commands. The migrate eslint command converts .eslintrc files to biome.json, mapping supported ESLint rules to their Biome equivalents and flagging unsupported ones. The migrate prettier command converts .prettierrc to Biome formatter config. After migration, Biome replaces both tools completely — no parallel running, no format-on-save conflicts, no npm update cycles for two tools.
