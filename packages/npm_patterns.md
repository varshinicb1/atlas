---
kind: Package
id: package:npm
name: npm Package Manager
version: "11"
purpose: npm patterns — package management, scripts, workspaces, publishing, security, and dependency resolution.
problem_solved: Provides a reference for npm's dependency resolution, package.json conventions, script lifecycle, workspace monorepos, and registry interactions, reducing dependency conflicts, broken lockfiles, and publish/release errors.
install: npm install -g npm@latest
dependencies: []
concepts:
  - name: Packages & Modules
    id: concept:npm/packages
    description: npm packages are directories with a package.json; can be local (file/project), scoped (@scope/name), or global. A module is any file or directory loadable by require()/import.
  - name: Dependencies
    id: concept:npm/dependencies
    description: Five dependency types — dependencies, devDependencies, peerDependencies, optionalDependencies, bundledDependencies — installed with --save-prod (default), --save-dev, --save-peer, --save-optional, or via bundledDependencies array.
  - name: Semantic Versioning (SemVer)
    id: concept:npm/semver
    description: MAJOR.MINOR.PATCH with caret (^1.2.3 allows minor/patch), tilde (~1.2.3 allows only patch), exact (1.2.3), and ranges (>=1.0.0 <2.0.0). ^ is npm's default; lockfile pins exact resolved versions.
  - name: package.json
    id: concept:npm/package-json
    description: The manifest file with metadata (name, version, description), scripts, dependencies, exports, engines, and publishConfig. Fields like type (module/commonjs), main, module, types, and exports control resolution.
  - name: package-lock.json
    id: concept:npm/lockfile
    description: Auto-generated lockfile that records the exact dependency tree including integrity hashes and resolved URLs. Ensures reproducible installs across environments; must be committed to version control.
  - name: Scripts & Lifecycle
    id: concept:npm/scripts
    description: "npm run executes lifecycle scripts (pre/post hooks). Built-in: preinstall, install, postinstall, prepublish, prepare, prepack, postpack, test, start, stop, restart. Custom scripts run via npm run <name>."
  - name: Workspaces (Monorepos)
    id: concept:npm/workspaces
    description: Native monorepo support via workspaces field in root package.json. Packages in workspaces symlink to each other; npm install hoists shared dependencies to the root for deduplication.
  - name: Registry
    id: concept:npm/registry
    description: The npm public registry (registry.npmjs.org) stores packages. Custom registries (private, Verdaccio, GitHub Packages) set via registry config, .npmrc, or publishConfig. Authentication uses npm token.
  - name: Publishing
    id: concept:npm/publishing
    description: npm publish pushes a package to the registry. Prepublish runs prepare/build scripts. Version bumps (npm version patch/minor/major) tag git. Deprecation (npm deprecate) warns users; unpublish is restricted to 72h.
  - name: Security & Audit
    id: concept:npm/security
    description: npm audit scans the tree for known vulnerabilities (advisories from the npm Security Advisory database). npm audit fix auto-installs compatible patches. Sigstore signing and provenance attestation verify package integrity.
  - name: npm CLI & Config
    id: concept:npm/config
    description: "Configuration via .npmrc files (project, user, global), environment variables (npm_config_*), and CLI flags. Key settings: registry, @scope:registry, engine-strict, audit-level, fund, save-exact."
  - name: CI/CD & Reproducibility
    id: concept:npm/ci-cd
    description: npm ci performs a clean install from lockfile (faster, stricter, no package.json write). Ideal for CI pipelines. npm cache verifies against integrity hashes; --prefer-offline reduces network.
apis:
  - name: npm install
    id: api:npm/install
    signature: "npm install [<pkg>...] [--save-prod|--save-dev|--save-peer|--save-optional] [--global]"
    returns: Updated node_modules and package-lock.json.
    description: Installs a package and its dependencies. Without arguments, installs all dependencies from package.json. --global installs system-wide. npm i is the shorthand.
  - name: npm run
    id: api:npm/run
    signature: "npm run <script> [-- <args>]"
    returns: Runs the script defined in package.json scripts.
    description: Executes a named script from package.json. The -- separator passes arguments to the script itself. npm start/test/stop/restart are aliases for their respective scripts.
  - name: npm publish
    id: api:npm/publish
    signature: "npm publish [<tarball>|<folder>] [--tag <tag>] [--access public|restricted]"
    returns: Package URL on the registry.
    description: Publishes a package to the registry. --tag sets a dist-tag (default latest). --access controls scoped package visibility. Runs prepublishOnly and prepare before packing.
  - name: npm audit
    id: api:npm/audit
    signature: "npm audit [--audit-level=<severity>] [--json]"
    returns: Vulnerability report or JSON.
    description: Submits the dependency tree to the registry for known-vulnerability scanning. --audit-level filters (moderate, high, critical). npm audit fix auto-instates compatible updates.
  - name: npm ci
    id: api:npm/ci
    signature: "npm ci [--only=prod|dev] [--no-audit] [--no-fund]"
    returns: Deterministic node_modules from lockfile.
    description: Clean install from package-lock.json. Deletes node_modules first, skips package-lock.json write, fails if lockfile and package.json are out of sync. Preferred for CI pipelines.
  - name: npm ls
    id: api:npm/ls
    signature: "npm ls [<pkg>...] [--all] [--depth=<n>] [--json]"
    returns: Dependency tree printed to stdout or JSON.
    description: Lists installed packages in the current project as a tree. Missing peer dependencies or extraneous packages are flagged. --all shows every package; --depth controls nesting.
  - name: npm outdated
    id: api:npm/outdated
    signature: "npm outdated [<pkg>...] [--long] [--json]"
    returns: Table of outdated packages with current/wanted/latest versions.
    description: Checks the registry against installed versions. Wanted is the latest satisfying the SemVer range; latest is the most recent published version. Exit code 1 if any are out of date.
  - name: npm update
    id: api:npm/update
    signature: "npm update [<pkg>...] [--save] [--global]"
    returns: Updated node_modules and package-lock.json.
    description: Updates packages to the latest version within the SemVer range specified in package.json. Does not modify the range itself. --global updates globally installed packages.
  - name: npm link
    id: api:npm/link
    signature: "npm link [<pkg>]  # in package dir; npm link <pkg>  # in consumer dir"
    returns: Symlinked package for local development.
    description: Creates a global symlink from a package folder, then links it into a consumer project. Useful for testing local packages before publishing without repeated npm pack/install.
  - name: npm pack
    id: api:npm/pack
    signature: "npm pack [<pkg>...] [--dry-run]"
    returns: A .tgz tarball in the current directory.
    description: "Creates a tarball of the package as it would be published. --dry-run shows what would be included. Used for testing package contents, .npmignore/files field resolution, and local file: protocol installs."
  - name: npm config
    id: api:npm/config
    signature: "npm config set|get|delete|list [<key>] [<value>]"
    returns: Configuration values.
    description: "Manages npm configuration across global, user, and project .npmrc files. Key keys: registry, _authToken, save-exact, audit-level, engine-strict."
  - name: npm init
    id: api:npm/init
    signature: "npm init [--yes] [--scope=<scope>] [<initializer>]"
    returns: New package.json (and boilerplate if using an initializer).
    description: Scaffolds a new package.json interactively. --yes accepts all defaults. npm init <initializer> (e.g., npm init react-app) delegates to npx <initializer> for project generators.
examples:
  - id: example:npm/workspace-setup
    language: json
    description: Root package.json with workspaces pointing to packages/*.
  - id: example:npm/ci-pipeline
    language: yaml
    description: GitHub Actions CI step using npm ci for deterministic installs.
  - id: example:npm/publish-flow
    language: json
    description: Version bump and publish workflow with prepublishOnly script.
failures:
  - id: failure:npm/version-mismatch
    symptom: npm ci fails with "package.json and package-lock.json are out of sync."
    cause: package-lock.json was not regenerated after manual package.json edits or version bumps.
    fix: Run npm install to regenerate the lockfile, commit both files together, and run npm ci locally before pushing.
  - id: failure:npm/missing-peer
    symptom: npm ls shows "UNMET PEER DEPENDENCY" warnings or runtime "cannot find module" errors.
    cause: A peer dependency (e.g., react@^18) is not installed because the consumer omitted it or a version mismatch exists.
    fix: Install the missing peer at a compatible version; use --legacy-peer-deps for temporary bypass on npm 7+.
  - id: failure:npm/permissions-publish
    symptom: "'403 Forbidden' on npm publish for scoped packages or 'unauthorized' errors."
    cause: Missing or expired npm token, wrong registry configured, or trying to publish to a registry without access.
    fix: Verify npm token with npm token list, authenticate with npm login, and check publishConfig.registry and .npmrc.
  - id: failure:npm/dependency-conflict
    symptom: Multiple versions of the same package in node_modules (npm dedupe fails) or "ERESOLVE" errors.
    cause: Transitive dependencies request incompatible versions; npm 7+ strict peer resolution blocks install.
    fix: Use npm ls <pkg> to trace conflict sources, update dependencies to compatible ranges, or use --legacy-peer-deps as a short-term escape hatch.
  - id: failure:npm/engines-block
    symptom: "'Unsupported engine' warning or npm refuses to install a package."
    cause: The package's engines field specifies a Node.js or npm version incompatible with the current environment.
    fix: Update Node.js/npm to satisfy the range, or set engine-strict=false in .npmrc to treat as a warning.
  - id: failure:npm/broken-symlink
    symptom: Workspace packages resolve to unexpected versions or "module not found" in a monorepo.
    cause: workspace symlinks break due to node_modules hoisting collisions, git clean, or mixed package managers.
    fix: Run npm install from the workspace root, ensure all workspaces are listed in the root package.json, and avoid mixing npm with pnpm/yarn.
extends:
  - concept:npm/packages
uses:
  - concept:npm/semver
  - concept:npm/lockfile
  - concept:npm/scripts
  - concept:npm/workspaces
part_of: concept:domain/web-platform
depends_on: []
solves:
  - problem:package-management
alternatives:
  - package:yarn
  - package:pnpm
---
# npm Package Manager Patterns

npm is the default package manager for Node.js, shipping as part of every Node installation. Its core job is dependency resolution: given a `package.json` with SemVer ranges, npm computes a dependency tree, deduplicates where possible, and writes the exact resolved tree into `package-lock.json`. The lockfile is the source of truth for reproducible installs — `npm ci` uses it exclusively and fails if it's out of sync with the manifest.

Dependencies fall into five categories. `dependencies` are runtime requirements shipped to consumers; `devDependencies` are build tools and test runners; `peerDependencies` are shared runtime requirements the consumer must provide (common for plugins and React components); `optionalDependencies` are non-failing extras; and `bundledDependencies` ship inside the package tarball. npm 7+ enforces peer dependency auto-install by default, which eliminated many silent missing-peer bugs but introduced stricter conflict resolution — when you see an "ERESOLVE" error, trace the conflict with `npm ls` and upgrade one of the conflicting packages rather than reaching for `--legacy-peer-deps`.

Monorepos use npm Workspaces: a root `package.json` with a `workspaces` array pointing to sub-packages. Running `npm install` at the root hoists shared dependencies upward (deduplicating versions) and symlinks workspace packages to each other, so changes propagate instantly. Scripts also cascade — `npm run test` in the root runs each workspace's test script if no root-level script is defined. Publish individual workspaces from their own directories or use tools like `lerna` (or `npm version` + `npm publish` per workspace) for version coordination.

Security is built into the workflow. `npm audit` scans against the public advisory database — run it in CI and fail builds on `high` or `critical` findings. `npm audit fix` applies safe patches automatically, though it may bump minor versions if required. For supply-chain integrity, npm 9+ supports Sigstore signatures and provenance attestation (`npm publish --provenance`). Combined with lockfile integrity hashes (sha512 in `package-lock.json`), this means every install is verifiable from registry download through to disk.
