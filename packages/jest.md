---
kind: Package
id: package:jest
name: Jest Testing Framework
version: "29"
purpose: Document Jest testing patterns — test structure, mocks, snapshots, code coverage, and configuration for JavaScript/TypeScript projects.
problem_solved: Provides a batteries-included testing framework with built-in assertions, mocking, code coverage, and watch mode, eliminating the need to assemble separate tools for running, asserting, and reporting test results.
install: npm install --save-dev jest @types/jest ts-jest
dependencies:
  - concept:typescript
  - concept:node-js
  - concept:testing-patterns
concepts:
  - name: Test Structure
    id: concept:jest/test-structure
    description: "Tests use describe (grouping), it/test (individual test), and expect (assertion). beforeAll/beforeEach/afterEach/afterAll hooks run setup/teardown at different scopes. Tests fail on thrown errors or unmet expectations. describe blocks nest for logical organization."
  - name: Matchers
    id: concept:jest/matchers
    description: "expect(value) chains matchers: toBe (primitives), toEqual (deep equality), toStrictEqual (deep + type), toContain (arrays/strings), toMatch (regex), toThrow (errors). Custom matchers via expect.extend(). .not inverts any matcher. resolves/rejects handles promises."
  - name: Mock Functions
    id: concept:jest/mocks
    description: "jest.fn() creates a mock function with mockReturnValue, mockResolvedValue, mockImplementation, and mockReturnThis. Track calls via mock.calls, mock.results, mock.instances. jest.spyOn wraps existing methods. mockClear() resets state between tests."
  - name: Module Mocking
    id: concept:jest/module-mocking
    description: "jest.mock('module') auto-mocks all exports. __mocks__ directory provides manual mocks. jest.mock('module', () => ({...})) inline factory. jest.unmock for selective unmocking. Mock modules with .mockImplementation() or .mockReturnValue() for per-test setup."
  - name: Snapshots
    id: concept:jest/snapshots
    description: "expect(component).toMatchSnapshot() serializes output and saves to __snapshots__/. Snapshot files are committed to version control. Run jest --updateSnapshot to update when changes are intentional. Use inline snapshots (toMatchInlineSnapshot) for small values."
  - name: Code Coverage
    id: concept:jest/coverage
    description: "jest --coverage generates coverage reports (lcov, text, clover) via Istanbul. Configure thresholds in jest.config: coverageThreshold.global.branches/statements/lines/functions. collectCoverageFrom specifies file globs. Skip files via coveragePathIgnorePatterns."
  - name: Watch Mode
    id: concept:jest/watch-mode
    description: "jest --watch re-runs tests on file changes. Interactive menu: filter by test name (f), filename (p), failed tests (t), quit (q). jest --watchAll for CI. Only tests affected by changed files run by default when using --watch with git/hg."
  - name: Timer Mocks
    id: concept:jest/timer-mocks
    description: "jest.useFakeTimers() replaces setTimeout/setInterval/Date.now with controllable implementations. jest.advanceTimersByTime(ms) fast-forwards time. jest.runAllTimers() runs all pending timers. Use for testing debounce, polling, and scheduled tasks."
  - name: Environment Setup
    id: concept:jest/environment
    description: "Jest runs in Node.js by default. @jest-environment jsdom provides browser globals (document, window, HTMLElement) for DOM testing. @jest-environment ./custom.ts for custom runners. Configure per-file via @jest-environment docblock or per-project in config."
  - name: Parameterized Tests
    id: concept:jest/parameterized
    description: "test.each(table)`title` runs a test with each row of inputs. describe.each for parameterized groups. Template literal syntax labels arguments: test.each([a, b, expected])`$a + $b = $expected`. Also supports array of arrays format: test.each([[1, 2, 3]])."
  - name: Global Setup/Teardown
    id: concept:jest/global-setup
    description: "globalSetup and globalTeardown in jest.config run once per jest session. Use for database setup, server start, or environment preparation. These run in the global scope, not per-worker. Setup files via setupFilesAfterEnv run in each test file's environment."
apis:
  - name: describe()
    id: api:jest/describe
    signature: "describe(name: string, fn: () => void): void"
    returns: void
    description: "Creates a test grouping block. Nests for logical organization. describe blocks can have their own beforeEach/afterEach. Only runs if at least one test inside it matches the test name pattern."
  - name: it() / test()
    id: api:jest/it
    signature: "it(name: string, fn: (done: DoneCallback) => void | Promise<void>, timeout?: number): void"
    returns: void
    description: "Defines an individual test. The function can take a done callback (for async, legacy) or return a promise. The optional timeout (default 5000ms) fails the test if exceeded. Promises that reject fail the test."
  - name: expect(value)
    id: api:jest/expect
    signature: "expect<T>(actual: T) -> JestMatchers<T>"
    returns: A matchers object for assertions.
    description: "Returns an object with assertion matchers. Chain .not for negation. .resolves/.rejects for promises. To assert a value, call expect(actual).toBe(expected). Jest counts one assertion per expect with a matcher."
  - name: jest.fn()
    id: api:jest/mock-fn
    signature: "jest.fn(implementation?: (...args) => any): jest.Mock"
    returns: A mock function with call tracking.
    description: "Creates a mock function. Track calls with .mock.calls ([[arg1, arg2], ...]), .mock.results ([{type, value}]), .mock.instances. Set behavior with .mockReturnValue(), .mockResolvedValue(), .mockImplementation(), .mockReturnThis()."
  - name: jest.mock()
    id: api:jest/mock-module
    signature: "jest.mock(moduleName: string, factory?: () => object, options?: {virtual?: boolean}): void"
    returns: void
    description: "Automatically mocks a module's exports. The factory function returns an object replacing the module's exports. With no factory, all exports become jest.fn() returning undefined. hoisted to top of file by jest."
  - name: jest.spyOn()
    id: api:jest/spy-on
    signature: "jest.spyOn(object: object, methodName: string, accessType?: 'get' | 'set'): jest.Mock"
    returns: A spy (mock function).
    description: "Wraps an existing method on an object. The original is preserved until mockRestore() is called. Commonly used with jest.mockImplementation() to replace the method temporarily. Restore in afterEach to prevent leakage."
  - name: jest.clearAllMocks()
    id: api:jest/clear-all-mocks
    signature: "jest.clearAllMocks(): void"
    returns: void
    description: "Resets mock.calls, mock.instances, mock.results for all mocks. Does not reset implementations. Use in beforeEach to isolate tests. jest.resetAllMocks() also clears implementations. jest.restoreAllMocks() restores spied-on originals."
sections:
  - title: Unit Testing Patterns
    id: section:jest/unit-testing
    content: |
      Structure tests with setup and teardown for clean isolation:

      ```typescript
      import { getUsers, createUser } from './users';
      import { db } from './database';

      jest.mock('./database');

      beforeEach(() => {
          jest.clearAllMocks();
      });

      describe('getUsers', () => {
          it('returns users when database has records', async () => {
              const mockUsers = [{ id: 1, name: 'Alice' }, { id: 2, name: 'Bob' }];
              (db.query as jest.Mock).mockResolvedValue(mockUsers);

              const result = await getUsers();
              expect(result).toEqual(mockUsers);
              expect(db.query).toHaveBeenCalledWith('SELECT * FROM users');
          });

          it('returns empty array when no users exist', async () => {
              (db.query as jest.Mock).mockResolvedValue([]);
              const result = await getUsers();
              expect(result).toHaveLength(0);
          });

          it('throws when database fails', async () => {
              (db.query as jest.Mock).mockRejectedValue(new Error('DB down'));
              await expect(getUsers()).rejects.toThrow('DB down');
          });
      });
      ```
  - title: Timer and Async Testing
    id: section:jest/timer-testing
    content: |
      Testing debounced functions and timed behavior with fake timers:

      ```typescript
      jest.useFakeTimers();

      describe('debounce', () => {
          it('calls the function after the specified delay', () => {
              const fn = jest.fn();
              const debounced = debounce(fn, 1000);

              debounced();
              expect(fn).not.toHaveBeenCalled();

              jest.advanceTimersByTime(500);
              debounced(); // reset the timer
              jest.advanceTimersByTime(999);
              expect(fn).not.toHaveBeenCalled();

              jest.advanceTimersByTime(1);
              expect(fn).toHaveBeenCalledTimes(1);
          });
      });

      // Restore real timers after tests
      afterEach(() => {
          jest.useRealTimers();
      });
      ```

      For snapshot testing, ensure serialized output is deterministic (no dates/random values without mocking).
---
