---
kind: Package
id: package:typescript/7_migration
name: TypeScript 7 Migration Guide
version: "7.0.0-rc"
purpose: >
  Guide TypeScript teams through the migration from TypeScript 5.x/6.x to
  TypeScript 7 (Project Corsa) — the Go-native port with ~10x faster type checking.
problem_solved: >
  TypeScript 7 (June 2026) replaces the JavaScript-based tsc compiler with a
  Go-native implementation. Breaking config changes, strict mode becoming default,
  and subtle type inference differences require a structured migration plan.
  This knowledge package prevents teams from getting stuck on obscure errors,
  regressions, and performance tuning.
install: |
  ```bash
  npx typescript@7 init
  atlas install typescript_7_migration.md
  ```
dependencies:
  - concept:typescript-legacy
concepts:
  - name: Project Corsa
    id: concept:ts7/corsa
    description: >
      Codename for TypeScript 7's Go-native compiler. A port-not-rewrite that
      preserves all TypeScript semantics and API surface while replacing the
      JavaScript CLI with a native Go binary. Claims ~10x faster type checking
      and ~5x faster builds. Ships as typescript@7.0.0-rc (June 2026).
  - name: Strict Mode Default
    id: concept:ts7/strict_default
    description: >
      TypeScript 7 enables strict mode by default in tsconfig.json. Projects
      that previously relied on strict mode being opt-in will see new errors
      at noImplicitAny, strictNullChecks, strictFunctionTypes,
      strictBindCallApply, strictPropertyInitialization, noImplicitThis,
      and alwaysStrict.
  - name: Parallel Checking
    id: concept:ts7/parallel_checking
    description: >
      New --checkers flag spawns N parallel checker goroutines, distributing
      type-checking work across CPU cores. Default auto-detects CPU count.
      Set explicitly for CI environments with constrained resources.
      Works with --watch for real-time feedback.
  - name: Parallel Building
    id: concept:ts7/parallel_build
    description: >
      New --builders flag parallelizes project reference builds using --build
      mode. Default auto-detects CPU count. Notably improves monorepo build
      times where multiple project references exist.
  - name: Module Resolution Deprecation
    id: concept:ts7/module_resolution
    description: >
      moduleResolution: node is deprecated in TS 7 and produces a warning.
      Teams must migrate to node16 (Node.js ESM/CJS-aware), nodenext
      (future-proof), or bundler (for bundler-based projects like esbuild,
      Webpack, Vite, tsup).
  - name: ES5 Target Removal
    id: concept:ts7/es5_removal
    description: >
      target: es5 is now a hard error. The minimum supported target is ES2015.
      Projects targeting older browsers must use a transpiler (Babel, esbuild,
      swc) for downlevel compilation while keeping TS target at ES2015+.
  - name: Type Inference Changes
    id: concept:ts7/inference
    description: >
      The Go-native type checker may produce slightly different inference
      results in edge cases — particularly around conditional types with
      circular references, deeply nested generics, and infer in complex
      conditional types. The TS team considers these bug fixes, but they
      may surface as new type errors in existing code.
  - name: TypeScript 5.x/6.x Legacy
    id: concept:ts5-6-legacy
    description: Staying on the older JavaScript-based TypeScript compiler, losing the 10x performance gains of the Go-native TS 7
  - name: Bun tsc Alternative
    id: concept:bun-tsc
    description: Using Bun's built-in type checker as a TypeScript 7 alternative, offering less compatibility but potentially simpler setup
  - name: Go Binary Distribution
    id: concept:ts7/binary
    description: >
      TypeScript 7 ships as a platform-specific Go binary via npm (with
      optional Go source distribution). Installation is unchanged for most
      users, but CI images using tsc may need platform compatibility checks.
  - name: TypeScript (legacy)
    id: concept:typescript-legacy
    description: TypeScript 5.x and 6.x — the JavaScript-based compiler versions that TS 7 replaces with a Go-native implementation
  - name: TypeScript Ecosystem
    id: concept:ts7/ecosystem
    description: The broader TypeScript tooling and community ecosystem that the TS 7 migration operates within.
apis:
  - name: tsc --checkers
    id: api:ts7/checkers
    signature: tsc --checkers <N>
    returns: Exit code 0 on success, non-zero on errors
    description: >
      Spawns N parallel checker goroutines for type checking. N=0 auto-detects
      CPU count. Recommended: set to CORES-1 in CI to leave headroom for other
      processes.
  - name: tsc --builders
    id: api:ts7/builders
    signature: tsc --build --builders <N>
    returns: Exit code 0 on success, non-zero on errors
    description: >
      Parallelizes project reference builds across N goroutines. Only
      meaningful with --build mode. Combine with --checkers for maximum
      throughput.
  - name: tsc --init
    id: api:ts7/init
    signature: tsc --init
    returns: Generates tsconfig.json
    description: >
      Creates a TypeScript 7 tsconfig.json with strict: true, target: es2022,
      moduleResolution: bundler (or node16), and other modern defaults.
      Run this fresh and merge differences with your existing config.
  - name: npx typescript@7 init
    id: api:ts7/npx_init
    signature: npx typescript@7 init
    returns: Upgraded project to TS 7
    description: >
      Installs TypeScript 7 and runs the migration assistant. Checks your
      current tsconfig for deprecated options and provides fix suggestions.
      Preferred over manual upgrades.
workflows:
  - name: Migration Assessment
    id: workflow:ts7/assess
    description: Assess migration scope and prepare.
    steps:
      - order: 1
        action: Run npx typescript@7 init in the project root
      - order: 2
        action: Check output for deprecated moduleResolution node warnings
      - order: 3
        action: Check output for target es5 errors
      - order: 4
        action: Check output for new strict mode errors
      - order: 5
        action: Estimate 1 hour per 10K lines for strict mode fixes
  - name: Config Migration
    id: workflow:ts7/config
    description: Migrate tsconfig.json to TypeScript 7 compatible settings.
    steps:
      - order: 1
        action: Change target to es2022 (or es2016 minimum for older Node)
      - order: 2
        action: Change moduleResolution to node16 for Node.js or bundler for bundler projects
      - order: 3
        action: Enable strict true (or explicit sub-flags if gradual migration needed)
      - order: 4
        action: Add --checkers 4 to build scripts if CPU count permits
      - order: 5
        action: Test with npx typescript@7 --noEmit --checkers 4
  - name: Strict Mode Fixes
    id: workflow:ts7/strict_fixes
    description: Systematic approach to fixing strict mode errors.
    steps:
      - order: 1
        action: Run tsc --noEmit --strict and capture all errors to a file
      - order: 2
        action: Fix strictNullChecks errors first - add explicit null and undefined to signatures
      - order: 3
        action: Fix noImplicitAny errors - add explicit type annotations to parameters
      - order: 4
        action: Fix strictPropertyInitialization errors - add initializers or assertions
      - order: 5
        action: Fix strictFunctionTypes errors - use contravariant annotations for callbacks
      - order: 6
        action: Run test suite to validate runtime behavior has not changed
  - name: CI Pipeline Update
    id: workflow:ts7/ci
    description: Update CI configuration for TypeScript 7's performance improvements.
    steps:
      - order: 1
        action: Update CI image to ensure Node.js 18+ for TypeScript 7 npm package
      - order: 2
        action: Add --checkers 2 to CI type-check step (respect CI CPU limits)
      - order: 3
        action: If using --build with project references, add --builders 2
      - order: 4
        action: Set TSC_NON_COMPOSITE_CHECK env var if using composite projects
      - order: 5
        action: Compare CI pipeline time before vs after migration
failures:
  - id: failure:ts7/es5_target
    symptom: target es5 is no longer supported. Minimum target is es2015.
    cause: tsconfig.json has target es5 which is removed in TS 7.
    fix: Change target to es2022 (or at minimum es2016). Use Babel or esbuild for legacy browser support.
  - id: failure:ts7/module_node_deprecated
    symptom: moduleResolution node is deprecated. Use node16 or bundler instead.
    cause: tsconfig.json uses deprecated moduleResolution node.
    fix: Switch to moduleResolution node16 and update import/export syntax to match Node.js ESM conventions.
  - id: failure:ts7/implicit_any_new
    symptom: Parameter x implicitly has an any type.
    cause: Strict mode is now enabled by default. Previously noImplicitAny was off.
    fix: Add explicit type annotations or disable individually with noImplicitAny false.
  - id: failure:ts7/null_check_errors
    symptom: Object is possibly null or undefined.
    cause: strictNullChecks now enabled by default.
    fix: Add null checks, use optional chaining (?.), or add null/undefined to the type.
  - id: failure:ts7/circular_inference
    symptom: Type instantiation is excessively deep and possibly infinite.
    cause: TS 7's Go-native checker detects circular type references the old checker missed.
    fix: Simplify the conditional type, add explicit type annotations to break the cycle.
  - id: failure:ts7/ci_slow
    symptom: CI pipeline is slower with TS 7 than before.
    cause: Default --checkers 0 auto-detects all CPU cores, which may saturate CI workers.
    fix: Explicitly set --checkers 2 (or CORES-1) in CI scripts.
  - id: failure:ts7/incremental_mismatch
    symptom: Cannot write file because it would overwrite input file.
    cause: TS 7 has stricter outDir handling for files matching rootDir.
    fix: Ensure rootDir and outDir are distinct. Move source files to src/.
  - id: failure:ts7/platform_mismatch
    symptom: Cannot find module typescript or tsc is not recognized.
    cause: TypeScript 7 ships as a Go binary with platform-specific builds.
    fix: Check TS 7 platform support matrix. Use Node.js 18+ on x86_64 Linux or macOS.
extends:
  - concept:typescript-legacy
implements: []
uses:
  - concept:ts7/corsa
  - concept:ts7/strict_default
  - concept:ts7/parallel_checking
  - concept:ts7/module_resolution
  - concept:ts7/es5_removal
part_of: concept:ts7/ecosystem
solves:
  - problem:ts7-migration
alternatives:
  - concept:ts5-6-legacy
  - concept:bun-tsc
---

# TypeScript 7 Migration Guide

TypeScript 7 (Project Corsa) is a ground-up port of the TypeScript compiler from JavaScript to Go. It is not a rewrite — it preserves every TypeScript language feature, every compiler flag (with additions), and every API contract. The benefit is a ~10x speedup on type checking and ~5x faster builds.

## What Changed

### Performance
- `--checkers N`: Parallel type checking across N goroutines (default: auto CPU count)
- `--builders N`: Parallel project reference builds (default: auto CPU count)
- Combined effect: monorepos with project references see the biggest gains (often 8-15x on `--build --force`)

### Configuration Breaking Changes
| Setting | TS 5/6 | TS 7 | Action |
|---------|--------|------|--------|
| `strict` | optional (false) | **default (true)** | Fix new strict errors or set `"strict": false` |
| `target` | es5 allowed | **minimum es2015** | Upgrade to es2022 |
| `moduleResolution` | `"node"` allowed | **deprecated** | Use `"node16"` or `"bundler"` |
| `--checkers` | not available | **new flag** | Add to scripts for perf |
| `--builders` | not available | **new flag** | Add for monorepos |

## Migration Plan

### 1. Install and Assess
```bash
npm install typescript@7 --save-dev
npx typescript@7 init  # migration assistant
```

### 2. Fix Config
```json
{
  "compilerOptions": {
    "target": "es2022",
    "moduleResolution": "node16",
    "strict": true,
    "checkers": 0,
    "builders": 0
  }
}
```

### 3. Fix Strict Errors
Run `tsc --noEmit` and work through errors systematically (see Strict Mode Fixes workflow).

### 4. Optimize for CI
```bash
# GitHub Actions
- run: npx tsc --noEmit --checkers 2

# Larger machines with project references
- run: npx tsc --build --builders 4 --checkers 4
```

## Performance Expectations

| Project Size | TS 5.x | TS 7 (1 core) | TS 7 (4 checkers) |
|-------------|-------|---------------|-------------------|
| 10K lines   | 3s    | 0.8s          | 0.3s              |
| 100K lines  | 30s   | 6s            | 2s                |
| 500K lines  | 3min  | 35s           | 12s               |
| Monorepo (1M lines) | 10min | 2min    | 40s               |
