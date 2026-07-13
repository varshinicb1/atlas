---
kind: Package
id: package:vitest
name: Vitest
version: "2.1"
purpose: Document Vitest — a Vite-native testing framework with Jest-compatible API, TypeScript support, coverage, mocking, snapshot testing, and workspace mode for monorepos.
problem_solved: Provides a testing framework that shares Vite's config, transform pipeline, and dependency resolution — eliminating the Jest/Babel/ts-jest configuration burden and enabling sub-second HMR-based test updates.
install: npm install -D vitest
dependencies:
  - concept:vite
  - concept:testing
  - concept:typescript
concepts:
  - name: Test Runner
    id: concept:vitest/test-runner
    description: describe/it/expect pattern compatible with Jest and Mocha. Workers run tests in parallel using Vite's transform pipeline. Supports .test.ts and .spec.ts files. Tests can be filtered by name (.only, .skip, .todo) and file path (via CLI args).
  - name: HMR Test Updates
    id: concept:vitest/hmr
    description: vitest --watch re-runs only the tests affected by file changes, powered by Vite's module graph. A change to a source file re-runs only the tests that import it (transitively). Sub-second test feedback loop — significantly faster than Jest's --watch.
  - name: Workspace
    id: concept:vitest/workspace
    description: vitest.workspace.ts defines multiple projects with separate configs — useful for monorepos where packages use different frameworks (React, Vue, Node). Each project has its own Vite config, plugins, and environment. Tests across projects run in parallel.
  - name: Mocking
    id: concept:vitest/mocking
    description: "vi.mock() for module-level mocking, vi.spyOn() for instance methods, vi.fn() for standalone mocks. Mock factory in vi.mock('module', () => ({})). vi.hoisted() for variables that need to be hoisted above imports. Automatically hoisted to the top of the file."
  - name: Coverage
    id: concept:vitest/coverage
    description: "Built-in coverage via c8, Istanbul, or V8. vitest --coverage generates HTML, lcov, text, and JSON reports. Threshold enforcement in vitest.config. Coverage provider: 'v8' (fast, no instrumentation), 'istanbul' (more accurate, supports TypeScript source maps)."
  - name: Snapshots
    id: concept:vitest/snapshots
    description: toMatchSnapshot() and toMatchInlineSnapshot() for UI and JSON snapshot testing. Snapshots stored in __snapshots__ directory alongside tests. vitest -u updates all snapshots. Inline snapshots update in source code on -u.
  - name: Environments
    id: concept:vitest/environments
    description: Configure per-test-file or per-project environment — 'node' (default, fast), 'jsdom' (browser-like DOM), 'happy-dom' (faster jsdom alternative), 'edge-runtime' (Cloudflare Workers-like). Custom environments via Environment interface.
  - name: Component Testing
    id: concept:vitest/component-testing
    description: Test UI components in isolation — @testing-library/react, @testing-library/vue, @testing-library/svelte, or vue-test-utils. Vite's transform pipeline handles JSX, Vue SFCs, and CSS imports automatically — no extra configuration needed like Jest required.
  - name: Browser Mode
    id: concept:vitest/browser-mode
    description: Experimental browser-based testing — runs tests in real browsers using Playwright or WebdriverIO. Provides real browser APIs (window, document, fetch) without jsdom mocks. Access to devtools for debugging. Particularly useful for testing browser-specific APIs.
  - name: In-source Testing
    id: concept:vitest/in-source
    description: "Define tests alongside source code using if (import.meta.vitest) { ... } blocks. Tests are tree-shaken from production builds by Vite. Keeps tests close to implementation without affecting bundle size."
apis:
  - name: defineConfig / test config
    id: api:vitest/config
    signature: "defineConfig({ test: { globals: true, environment: 'jsdom', setupFiles: './setup.ts', coverage: { provider: 'v8', thresholds: { lines: 80 } } } })"
    returns: Vitest configuration.
    description: "Extends Vite config with test-specific options. globals: true enables describe/it/expect without imports. environment sets jsdom/happy-dom/node. setupFiles run before each test file. coverage configures reporting and thresholds."
  - name: describe(name, fn)
    id: api:vitest/describe
    signature: "describe('UserService', () => { test('creates user', async () => { ... }) })"
    returns: void
    description: Groups related tests. Supports .only (run only this group), .skip (skip), .todo (mark pending), .each (parameterized suite). describe blocks can nest arbitrarily.
  - name: test(name, fn, timeout?)
    id: api:vitest/test
    signature: "test('adds 1 + 2 = 3', () => { expect(add(1, 2)).toBe(3) }, 5000)"
    returns: void
    description: Defines a test case. Also available as it(name, fn). Supports .only, .skip, .todo, .each, .concurrent (run in parallel), .fails (expect failure). timeout in ms (default 5000).
  - name: expect(value)
    id: api:vitest/expect
    signature: "expect(actual).toEqual(expected); expect(array).toContain(item); expect(fn).toThrow(); expect(mock).toHaveBeenCalledWith(...)"
    returns: Expectation chain object.
    description: Assertion API compatible with Jest — toBe, toEqual, toStrictEqual, toBeTruthy, toBeNull, toBeUndefined, toContain, toHaveLength, toThrow, toHaveBeenCalled, toHaveBeenCalledWith, toMatchSnapshot, toMatchInlineSnapshot, resolves, rejects.
  - name: vi.mock(path, factory?)
    id: api:vitest/mock
    signature: "vi.mock('axios', () => ({ default: { get: vi.fn().mockResolvedValue({ data: [] }) } }))"
    returns: void (hoisted to top of file).
    description: Mocks a module at the module level. The factory function runs before imports — use vi.hoisted() for variables used in the factory. Mocked modules are automatically unmocked between tests.
  - name: vi.fn(impl?)
    id: api:vitest/fn
    signature: "const mockFn = vi.fn((x) => x * 2); expect(mockFn(3)).toBe(6); expect(mockFn).toHaveBeenCalledWith(3)"
    returns: A mock function.
    description: Creates a spy/mock function. Records calls, arguments, instances, and return values. .mockReturnValue(value) for fixed returns. .mockResolvedValue(value) for async. .mockImplementation(fn) for custom behavior.
  - name: vi.spyOn(object, method)
    id: api:vitest/spyOn
    signature: "const spy = vi.spyOn(console, 'log'); console.log('test'); expect(spy).toHaveBeenCalledWith('test')"
    returns: A spy (mock function).
    description: Spies on an object's method. By default calls through to the original. Use .mockImplementation() to override. Use .mockRestore() to restore the original after the test.
failures:
  - id: failure:vitest/environment-mismatch
    symptom: Tests fail with "document is not defined" or "localStorage is not defined".
    cause: Test file uses 'node' environment but tests interact with browser APIs (DOM, localStorage, fetch).
    fix: "Add environment: 'jsdom' to vitest.config or use // @vitest-environment jsdom at the top of the test file. For simple DOM queries, happy-dom is faster. For full browser API compatibility, use browser mode."
  - id: failure:vitest/alias-not-resolved
    symptom: Module import errors — '@/' or path aliases not found in tests.
    cause: Vite resolve.alias configured in vite.config but Vitest does not inherit it automatically.
    fix: Vitest inherits Vite config by default. If aliases still fail, define them explicitly in vitest.config resolve.alias. Check that vite.config is in the same directory as vitest.config.
  - id: failure:vitest/mock-not-hoisted
    symptom: vi.mock factory cannot access variables defined in the same file.
    cause: vi.mock calls are hoisted to the top of the file, so variables defined after the mock call are not available in the factory.
    fix: "Use vi.hoisted(() => { const variable = ...; return { variable } }) to define variables at the hoisted scope. Then access from the mock factory via hoisted.variable."
  - id: failure:vitest/global-setup-timeout
    symptom: Test suite fails to start — global setup timeout (default 60s).
    cause: Global setup file (globalSetup in config) takes too long — typically database migrations, Docker containers, or server startups.
    fix: Increase globalSetup timeout in vitest.config. Optimize setup to run in parallel. Use setupFiles for simpler per-worker setup. For Docker-based integration tests, use testcontainers library.
extends: []
implements: []
uses:
  - concept:vite
  - concept:testing
part_of: concept:javascript-testing-ecosystem
solves:
  - problem:vite-native-testing
  - problem:test-configuration-overhead
  - problem:watch-mode-test-execution
alternatives:
  - package:jest
  - package:mocha
  - package:ava
  - package:node-test-runner
---
Vitest is not a test runner that happens to use Vite — it's built on Vite's core architecture. Tests are transformed through Vite's pipeline (TypeScript, JSX, Vue SFCs, CSS modules, asset imports) with the same plugins and resolve configuration as the application. This eliminates the Jest + Babel + ts-jest configuration nightmare where test transforms never quite match the application build.

The worker pool architecture uses Vite's dev server to transform test files on-the-fly. Each worker is a separate Vite environment, so tests are fully isolated. The --watch mode is where Vitest truly excels — thanks to Vite's module graph, changing a source file re-runs only the tests that depend on it, not the entire test suite. In large monorepos, this reduces test feedback loops from minutes to milliseconds.

Vitest's compatibility with Jest's API (describe/it/expect/vi.mock/vi.fn) means migrating from Jest is straightforward — most projects only need to replace jest.config with vitest.config and run a codemod. The workspace feature extends this to monorepos: define a vitest.workspace.ts with separate projects for packages using different frameworks (React components in jsdom, Node utilities in node, API tests in edge-runtime), all sharing the same Vitest core.
