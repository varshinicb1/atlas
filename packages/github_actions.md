---
kind: Package
id: package:github-actions
name: GitHub Actions CI/CD
version: "4"
purpose: Document GitHub Actions patterns — workflow files, actions, jobs, matrix builds, caching, deployment, and reusable workflows for automated CI/CD pipelines.
problem_solved: Provides an integrated CI/CD platform within GitHub that builds, tests, and deploys code on push, PR, or schedule using reusable YAML workflows, eliminating the need for external CI tools and manual deployment processes.
install: Create .github/workflows/*.yml in your repository.
dependencies:
  - concept:ci-cd
  - concept:yaml
  - concept:git
concepts:
  - name: Workflows
    id: concept:github-actions/workflows
    description: "YAML files in .github/workflows/ defining automated processes. Each workflow has a name, trigger (on: push, pull_request, schedule, workflow_dispatch), and one or more jobs. Workflows run on GitHub-hosted or self-hosted runners. A repository can have multiple workflows."
  - name: Jobs
    id: concept:github-actions/jobs
    description: "Units of work within a workflow. Jobs run in parallel by default. Jobs can have dependencies (needs) for sequential execution. Each job runs on a fresh runner instance. Job properties: runs-on, strategy, steps, timeout-minutes, concurrency, environment."
  - name: Steps
    id: concept:github-actions/steps
    description: "Individual commands or actions within a job. Steps run sequentially and share the runner's filesystem. Each step can use: uses (reusable action), run (shell command), id (step reference), if (conditional), continue-on-error, and working-directory."
  - name: Actions
    id: concept:github-actions/actions
    description: "Reusable units of functionality. GitHub Marketplace hosts thousands of actions: actions/checkout (git checkout), actions/setup-node (Node.js setup), actions/cache (dependency caching), actions/upload-artifact, actions/download-artifact. Create custom actions via Docker, JavaScript, or composite run steps."
  - name: Matrix Builds
    id: concept:github-actions/matrix
    description: "Running a job against multiple configurations in parallel. strategy.matrix defines os, node-version, and other variables. Jobs are created for each combination. matrix.os: [ubuntu-latest, windows-latest], matrix.node: [18, 20, 22]. Include/exclude specific combinations."
  - name: Triggers
    id: concept:github-actions/triggers
    description: "Events that start workflows: push, pull_request, schedule (cron), workflow_dispatch (manual), release, issues, discussion, registry_package, page_build, and repository_dispatch. Filters via branches, tags, paths, and types. schedule uses POSIX cron syntax (UTC)."
  - name: Contexts & Expressions
    id: concept:github-actions/contexts
    description: "${{ }} expressions access runtime information: github (event, ref, actor, workspace), env (environment vars), job, steps, runner (os, arch, name), secrets, vars. Functions: contains(), startsWith(), endsWith(), format(), join(), fromJSON(), toJSON(), hashFiles(), always(), cancelled(), failure()."
  - name: Caching
    id: concept:github-actions/caching
    description: "actions/cache restores and saves dependencies by key. Cache key includes OS and lockfile hash. Restore keys provide fallbacks (partial matches). Cache size limit is 10GB per repository. Uses for npm/node_modules, pip, Maven, Gradle, Docker layers, and build outputs."
  - name: Environments
    id: concept:github-actions/environments
    description: "Deployment targets with protection rules (required reviewers, wait timer). environment: production in a job links it to an environment. Environment secrets are scoped to specific environments. Deployment logs are visible in the Environments tab. Supports branch-only deployments."
  - name: Reusable Workflows
    id: concept:github-actions/reusable-workflows
    description: "Workflow files called from other workflows. Defined with on: workflow_call. Called with uses: org/repo/.github/workflows/ci.yml@v1. Supports input and secret parameters. Reusable workflows reduce duplication across repositories. Available in the same or different organizations."
  - name: Service Containers
    id: concept:github-actions/service-containers
    description: "Additional Docker containers for dependencies (PostgreSQL, Redis, MySQL). Defined in the job's services: section. Services are accessible via hostname matching the container name and exposed ports. Ephemeral — created per-job and destroyed after completion."
apis:
  - name: "on: [push, pull_request]"
    id: api:github-actions/triggers
    signature: "on:\n  push:\n    branches: [main]\n    paths: ['src/**', 'tests/**']\n  pull_request:\n    types: [opened, synchronize, reopened]"
    returns: Event trigger configuration.
    description: "Defines which events trigger the workflow. branches filters by branch name. paths filters by changed files. types specifies activity types. paths-ignore and branches-ignore for inverse filters. Activity types vary by event (opened, closed, labeled for issues/PRs)."
  - name: steps.uses
    id: api:github-actions/uses
    signature: "- uses: actions/checkout@v4\n  with:\n    fetch-depth: 0\n    ref: ${{ github.head_ref }}"
    returns: Action execution.
    description: "Uses a GitHub Action. The @version tag pins to a specific version. with provides input parameters defined by the action. Action sources: GitHub Marketplace, public repo, Docker image, or local path (./.github/actions/my-action)."
  - name: strategy.matrix
    id: api:github-actions/matrix
    signature: "strategy:\n  matrix:\n    os: [ubuntu-latest, windows-latest]\n    node: [18, 20]\n  fail-fast: false\n  max-parallel: 4"
    returns: Multi-configuration job execution.
    description: "Runs the job with every combination of the matrix variables. fail-fast: true cancels all in-progress jobs when one fails. max-parallel limits concurrent jobs. include adds extra configs, exclude removes specific combinations."
  - name: needs
    id: api:github-actions/needs
    signature: "jobs:\n  test:\n    needs: [lint, typecheck]\n    if: always()\n    steps: [...]"
    returns: Job dependency declaration.
    description: "Makes a job dependent on the completion of listed jobs. By default, needs jobs must succeed. Use if: always() to run regardless of dependent job status (useful for cleanup steps). if: failure() to run only when dependencies fail."
  - name: artifacts
    id: api:github-actions/artifacts
    signature: "- uses: actions/upload-artifact@v4\n  with:\n    name: build-output\n    path: dist/\n    retention-days: 7"
    returns: Artifact stored in GitHub.
    description: "Uploads files from the runner to GitHub Actions. Artifacts persist for retention-days (default 90). Download via actions/download-artifact in downstream jobs. Use for build outputs, test reports, screenshots, and coverage reports."
sections:
  - title: CI Pipeline
    id: section:github-actions/ci
    content: |
      Full CI workflow with caching and matrix testing:

      ```yaml
      name: CI
      on:
          push: { branches: [main] }
          pull_request: { branches: [main] }

      jobs:
          lint-and-typecheck:
              runs-on: ubuntu-latest
              steps:
                  - uses: actions/checkout@v4
                  - uses: actions/setup-node@v4
                    with: { node-version: 22, cache: 'npm' }
                  - run: npm ci
                  - run: npx tsc --noEmit
                  - run: npx eslint .

          test:
              needs: lint-and-typecheck
              runs-on: ubuntu-latest
              strategy:
                  matrix:
                      node-version: [18, 20, 22]
                      os: [ubuntu-latest, windows-latest]
                  fail-fast: false
              steps:
                  - uses: actions/checkout@v4
                  - uses: actions/setup-node@v4
                    with: { node-version: ${{ matrix.node-version }}, cache: 'npm' }
                  - run: npm ci
                  - run: npm test -- --ci --coverage --maxWorkers=2
                  - uses: actions/upload-artifact@v4
                    if: always()
                    with:
                        name: coverage-${{ matrix.os }}-${{ matrix.node-version }}
                        path: coverage/

          deploy:
              needs: test
              if: github.ref == 'refs/heads/main'
              runs-on: ubuntu-latest
              environment: production
              steps:
                  - run: echo "Deploying..."
      ```
  - title: Reusable Workflow
    id: section:github-actions/reusable
    content: |
      A reusable build-and-test workflow called from multiple repositories:

      ```yaml
      # .github/workflows/reusable-ci.yml
      on:
          workflow_call:
              inputs:
                  node-version: { required: true, type: string, default: '22' }
                  test-command: { required: false, type: string, default: 'npm test' }
              secrets:
                  NPM_TOKEN: { required: true }

      jobs:
          build:
              runs-on: ubuntu-latest
              steps:
                  - uses: actions/checkout@v4
                  - uses: actions/setup-node@v4
                    with: { node-version: ${{ inputs.node-version }}, cache: 'npm' }
                  - run: echo //registry.npmjs.org/:_authToken=${{ secrets.NPM_TOKEN }} > .npmrc
                  - run: npm ci
                  - run: npm run build
                  - run: ${{ inputs.test-command }}

      # Caller workflow
      name: CI
      on: [push, pull_request]
      jobs:
          ci:
              uses: my-org/shared-workflows/.github/workflows/reusable-ci.yml@v1
              with:
                  node-version: '22'
                  test-command: 'npm run test:ci'
              secrets:
                  NPM_TOKEN: ${{ secrets.NPM_TOKEN }}
      ```

      Reusable workflows must be referenced by a specific tag or commit SHA — branch references are not allowed for production workflows.
---
