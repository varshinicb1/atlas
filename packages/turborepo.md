---
kind: Package
id: package:turborepo
name: Turborepo Monorepo
version: "2"
purpose: Document Turborepo monorepo management — task orchestration, caching, remote caching, dependency graph, and workspace configuration for scalable multi-package repositories.
problem_solved: Provides a high-performance build system for monorepos that caches task outputs, parallelizes independent tasks, and orchestrates build/test/lint across packages using a dependency graph, reducing CI times from hours to minutes.
install: npx create-turbo@latest
dependencies:
  - concept:monorepos
  - concept:build-tools
  - concept:node-js
concepts:
  - name: Workspaces
    id: concept:turborepo/workspaces
    description: "Multiple packages in a single repository. Turborepo works with npm, pnpm, and yarn workspaces. The root package.json defines workspaces: ['apps/*', 'packages/*']. Each workspace has its own package.json and can depend on other workspaces via workspace protocol (workspace:*)."
  - name: Pipeline Configuration
    id: concept:turborepo/pipeline
    description: "turbo.json defines the task dependency graph: { pipeline: { build: { dependsOn: ['^build'], outputs: ['dist/**'] }, test: { dependsOn: ['build'], inputs: ['src/**'] } } }. ^build means all dependency packages' build tasks must complete first. The pipeline ensures correct ordering and maximum parallelism."
  - name: Caching
    id: concept:turborepo/caching
    description: "Turborepo caches task outputs (dist/, .next/, build/) based on file content hashes. A cache hit skips execution entirely and restores outputs instantly. Cache keys include inputs (source files), globalDependencies (env vars, lockfile), and external dependencies. Cache misses only rebuild changed packages."
  - name: Remote Caching
    id: concept:turborepo/remote-caching
    description: "Shared cache across team members and CI via Vercel Remote Caching. A build cached by one developer is instantly available to all others. npx turbo login authenticates. Remote caching eliminates redundant CI builds when only docs or configs change."
  - name: Task Graph & Parallelism
    id: concept:turborepo/task-graph
    description: "Turborepo builds a directed acyclic graph (DAG) of tasks from the pipeline config. Independent tasks run in parallel (maxParallel configures concurrency limit). Dependent tasks wait for their prerequisites. The task graph visualization (turbo run build --graph) shows the execution plan."
  - name: Filters & Scoping
    id: concept:turborepo/filters
    description: "Run tasks on a subset of packages: --filter=@myapp/web, --filter=...{packages/*} (include dependencies), --filter={packages/*}... (include dependents), --filter=./packages/*. Combine filters for precise execution scopes. turbo run test --filter='...[main]' runs tests for packages changed since main."
  - name: Inputs & Outputs
    id: concept:turborepo/inputs-outputs
    description: "inputs define which files trigger a cache miss (default: all package files). outputs specify directories/files to cache. Global inputs (lockfile, root tsconfig) affect all tasks. tasks.*.outputs can be explicit globs or empty for lint/test tasks that produce no artifacts."
  - name: Environment Variables
    id: concept:turborepo/env-vars
    description: "globalDependencies allows env vars and files as cache keys. { globalDependencies: ['.env'] } invalidates all caches when .env changes. Task-level env vars: { build: { env: ['NEXT_PUBLIC_API_URL'] } }. Two categories: dotenv (files) and env (variable names)."
  - name: Task Dependencies
    id: concept:turborepo/task-dependencies
    description: "dependsOn controls execution order: ['^build'] means this package's dependencies' build tasks run first. ['build'] means the same package's build runs first. ['//#build'] depends on a root task. ['some-task'] depends on the same package's other task. Combine dependsOn for complex orchestration."
  - name: Root Tasks
    id: concept:turborepo/root-tasks
    description: "Tasks defined at the root package using // prefix. A root task runs once for the whole repo, not per-package. Common uses: linting all markdown files, generating types from schemas, cleaning root node_modules. Defined in turbo.json pipeline with '//#task' syntax."
  - name: Pruning
    id: concept:turborepo/pruning
    description: "turbo prune @myapp/web --docker creates a minimal subset of the monorepo containing only the packages needed to build @myapp/web. Used for Docker builds to reduce build context size. Outputs are in out/ directory ready for docker build."
apis:
  - name: turbo run
    id: api:turborepo/turbo-run
    signature: "turbo run <task> [--filter=...] [--parallel] [--continue] [--dry] [--graph] [--cache-dir]"
    returns: Exit code 0 on success.
    description: "Executes a task across all or filtered packages. --dry shows what would run. --graph outputs the execution DAG (Mermaid or DOT). --continue carries on after task failures. --parallel disables concurrency limits. --cache-dir sets custom cache location."
  - name: turbo.json pipeline
    id: api:turborepo/pipeline-config
    signature: "{ extends: string[], pipeline: { [taskName]: { dependsOn, env, inputs, outputs, persistent, cache, interactive, dotenv, priority } } }"
    returns: Turborepo configuration.
    description: "The pipeline config in turbo.json. dependsOn is an array of task dependencies. outputs is an array of globs for cached artifacts. persistent marks long-running tasks (dev servers). interactive forwards stdin to the process. priority: 1 runs before priority: 2."
  - name: --filter syntax
    id: api:turborepo/filter-syntax
    signature: "--filter=pattern where pattern is: package-name, {glob}..., ...{glob}, [since-ref], [ref1...ref2]"
    returns: Scoped package set.
    description: "Filter syntax: @scope/name for exact match, ./packages/* for glob, ...{packages/*} includes dependents, {packages/*}... includes dependencies, [main...HEAD] changed packages since main, [since=main] for since-ref syntax."
  - name: turbo prune
    id: api:turborepo/turbo-prune
    signature: "turbo prune [--scope=...] [--docker] [--out-dir=out] [--no-gitignore]"
    returns: Minimal monorepo subset in out/ directory.
    description: "Prunes the monorepo to only the packages required by the scope. --docker adds a Dockerfile. The output directory contains a package.json, lockfile, and only relevant packages. Used to reduce Docker build context from gigabytes to kilobytes."
sections:
  - title: turbo.json Configuration
    id: section:turborepo/config
    content: |
      Pipeline configuration for a Next.js app with shared packages:

      ```json
      {
          "$schema": "https://turbo.build/schema.json",
          "globalDependencies": [".env", "tsconfig.json"],
          "globalEnv": ["NODE_ENV", "VERCEL_ENV"],
          "pipeline": {
              "build": {
                  "dependsOn": ["^build"],
                  "inputs": ["src/**", "tsconfig.json", "$TURBO_DEFAULT$"],
                  "outputs": [".next/**", "dist/**", "!.next/cache/**"],
                  "env": ["NEXT_PUBLIC_API_URL"]
              },
              "dev": {
                  "dependsOn": ["^build"],
                  "persistent": true,
                  "cache": false
              },
              "test": {
                  "dependsOn": ["build"],
                  "inputs": ["src/**/*.test.ts", "src/**/*.spec.ts"],
                  "outputs": []
              },
              "lint": {
                  "dependsOn": ["^build"],
                  "outputs": []
              },
              "typecheck": {
                  "dependsOn": ["^build"],
                  "outputs": []
              },
              "//#format": {
                  "inputs": ["*.md", "*.json"],
                  "outputs": []
              }
          }
      }
      ```
  - title: CI Optimization
    id: section:turborepo/ci
    content: |
      GitHub Actions with Turborepo remote caching:

      ```yaml
      name: CI
      on: [push, pull_request]
      jobs:
          build:
              runs-on: ubuntu-latest
              steps:
                  - uses: actions/checkout@v4
                    with: { fetch-depth: 2 }
                  - uses: actions/setup-node@v4
                    with: { node-version: 22, cache: 'npm' }
                  - run: npm ci
                  - uses: actions/cache@v4
                    with:
                        path: node_modules/.cache/turbo
                        key: turbo-${{ runner.os }}-${{ hashFiles('package-lock.json') }}
                  - run: npx turbo run build test lint --continue
                    env:
                        TURBO_TOKEN: ${{ secrets.TURBO_TOKEN }}
                        TURBO_TEAM: ${{ vars.TURBO_TEAM }}
                        TURBO_REMOTE_ONLY: true
      ```

      With remote caching enabled, CI skips building/testing packages whose inputs haven't changed, completing in seconds for most PRs.
---
