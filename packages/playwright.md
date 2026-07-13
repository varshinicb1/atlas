---
kind: Package
id: package:playwright
name: Playwright
version: "1.49"
purpose: Document Playwright — cross-browser end-to-end test automation with auto-waiting, browser contexts, network interception, codegen, and trace viewer for reliable, fast browser testing.
problem_solved: Eliminates the flakiness and complexity of traditional browser automation (Selenium) by providing auto-waiting (actions wait for element readiness automatically), browser-level API (network, CDP, storage), isolated contexts (no shared state), and cross-browser support (Chromium, Firefox, WebKit) with a single API.
install: npm install @playwright/test
dependencies:
  - concept:browser-automation
  - concept:web-standards
  - concept:testing
concepts:
  - name: Auto-Waiting
    id: concept:playwright/auto-wait
    description: All action methods (click, fill, check, etc.) automatically wait for the element to be visible, enabled, and stable before performing the action. No explicit sleep or waitFor calls needed — Playwright retries until actionability checks pass or timeout expires.
  - name: Locators
    id: concept:playwright/locators
    description: The core element selection strategy — page.getByRole(), getByText(), getByTestId(), getByLabel(), getByPlaceholder(), getByTitle(), locator(css). Locators are auto-waiting and retry-able. Use getUserByRole as the primary strategy for accessibility-first selectors.
  - name: Browser Contexts
    id: concept:playwright/contexts
    description: Isolated browser sessions — each context has its own cookies, localStorage, and session storage. Contexts are lightweight (one browser process, multiple contexts). Use for testing multiple users, guest vs authenticated, or parallel scenarios without cleanup.
  - name: Fixtures
    id: concept:playwright/fixtures
    description: "Test fixtures in Playwright Test — page is the default fixture. Custom fixtures for auth state, database setup, API mocking, or per-test configuration. Fixtures can extend (authenticatedPage extends page) and compose (test.use({ storageState: 'auth.json' }))."
  - name: Component Testing
    id: concept:playwright/component-testing
    description: Mount React, Vue, Svelte, or Solid components in isolation for component-level testing within Playwright. Uses vite for dev server, supports props, slots, events, and live HMR. Interact with components using the same locator API as E2E tests.
  - name: Codegen
    id: concept:playwright/codegen
    description: npx playwright codegen — opens a browser with a recorder. Every click, type, and navigation generates Playwright code in the target language (TypeScript, Python, Java, C#). Click the locator button to generate resilient locators. Copy ready-to-use test code.
  - name: Trace Viewer
    id: concept:playwright/trace-viewer
    description: Full test recording — DOM snapshots, network logs, console output, and timing for every action. Open traces with npx playwright show-trace trace.zip. Essential for debugging flaky tests in CI where you cannot watch the browser.
  - name: Network Interception
    id: concept:playwright/network
    description: page.route('**/api/**', handler) intercepts and modifies network requests. Mock API responses, block unwanted requests (analytics, ads), modify request/response headers, and test error states (500, timeout, network off). Fulfill, abort, or continue requests programmatically.
  - name: Visual Comparison
    id: concept:playwright/visual
    description: expect(page).toHaveScreenshot() captures element or full-page screenshots and compares against a baseline. Pixel-diff comparison with configurable threshold. Update baselines with --update-snapshots. Integrates with CI for visual regression detection.
  - name: Mobile Emulation
    id: concept:playwright/mobile
    description: "Device emulation via test.use({ ...devices['iPhone 15 Pro'] }) — sets viewport, user agent, touch events, and device scale factor. Test mobile-specific behaviors (touch gestures, media queries) without physical devices."
  - name: Parallel Execution
    id: concept:playwright/parallel
    description: "Workers run tests in parallel (default: CPU core count). Each worker gets its own browser context. Tests within a file run sequentially. shard for CI across multiple machines: --shard=1/4, --shard=2/4, etc. Fully isolated — no shared state between workers."
  - name: API Testing
    id: concept:playwright/api-testing
    description: Playwright includes a full API testing client (request context) — APIRequestContext for direct API calls without a browser. Test API-layer logic, then reuse auth state (cookies/tokens) for subsequent browser tests.
apis:
  - name: page.goto(url)
    id: api:playwright/goto
    signature: "await page.goto(url: string, opts?: { waitUntil?, timeout?, referer? }): Response | null"
    returns: Response object or null if navigation fails.
    description: "Navigates to a URL. waitUntil: 'load' (default), 'domcontentloaded', 'networkidle', 'commit'. Auto-waits for the navigation to complete. Throws timeoutError on timeout (default 30s)."
  - name: page.locator(selector)
    id: api:playwright/locator
    signature: "page.getByRole('button', { name: 'Submit' }): Locator"
    returns: A Locator instance.
    description: "Creates a locator for element selection. Preferred: getByRole, getByTestId, getByText, getByLabel. Fallback: locator(cssSelector). Locators are lazy — they don't query the DOM until an action is performed."
  - name: locator.click(opts)
    id: api:playwright/click
    signature: "await page.getByText('Submit').click(opts?: { button?, clickCount?, delay?, force?, position?, modifiers?, noWaitAfter?, timeout?, trial? }): void"
    returns: void
    description: Clicks an element. Auto-waits for visibility, enabled, and stable state. Supports right-click, double-click, shift+click, and position-relative clicks for testing specific button areas.
  - name: locator.fill(value)
    id: api:playwright/fill
    signature: "await page.getByLabel('Email').fill('user@example.com'): void"
    returns: void
    description: Clears the input field and types the value. Fires input and change events. Prefer fill over type() for speed and reliability — type() presses keys one by one (useful for testing keyboard events).
  - name: expect(locator).toBeVisible()
    id: api:playwright/expect-visible
    signature: "await expect(page.getByText('Success')).toBeVisible(opts?: { timeout? }): Promise<void>"
    returns: Promise that resolves or throws.
    description: Asserts that an element is visible in the viewport. Part of Playwright's assertion library — includes toBeHidden, toBeEnabled, toBeDisabled, toHaveText, toHaveValue, toHaveCount, toBeChecked, toHaveAttribute, toHaveScreenshot.
  - name: page.route(pattern, handler)
    id: api:playwright/route
    signature: "await page.route('**/api/users', async route => { await route.fulfill({ json: { users: [] } }) }): void"
    returns: void
    description: "Intercepts network requests matching the URL pattern. handler receives the Route object: route.fulfill (mock response), route.abort (block), route.continue (pass through, optionally modified). Use page.unroute() to remove handlers."
  - name: page.evaluate(fn)
    id: api:playwright/evaluate
    signature: "const title = await page.evaluate(() => document.title): string"
    returns: The return value of the evaluated function.
    description: Executes JavaScript in the browser page context. Returns serializable values. For returning complex objects (DOM elements), use evaluateHandle. For passing arguments, use page.evaluate((arg) => ..., argValue).
  - name: context.storageState(path)
    id: api:playwright/storage-state
    signature: "await context.storageState({ path: 'auth.json' }): { cookies, origins }"
    returns: Storage state object (or writes to file).
    description: "Captures cookies and localStorage from the current context. Saved state can be loaded in another test via test.use({ storageState: 'auth.json' }) — enables authentication once, reuse across tests."
failures:
  - id: failure:playwright/flaky-timeout
    symptom: Tests randomly fail with timeout errors in CI but pass locally.
    cause: Auto-waiting mechanism cannot find or interact with the element within the default timeout (30s) due to slow network, animations, or loading spinners.
    fix: "Increase timeout for flaky actions (click({ timeout: 10000 })). Add toBeVisible assertions before actions. Disable CSS animations/transitions in test config. Use waitForSelector for elements with slow loading."
  - id: failure:playwright/locator-stale
    symptom: Element is found but action fails with "element is not attached to the DOM".
    cause: The element the locator refers to was replaced (SPA re-render, list re-order) after the locator was resolved but before the action.
    fix: Playwright handles most stale element cases automatically. If persistent, re-query the locator before the action. Avoid storing locator results — always use the locator chain directly in actions.
  - id: failure:playwright/test-data-pollution
    symptom: Tests pass individually but fail when run together.
    cause: Tests share state through the database, localStorage, or cookies because contexts are not fully isolated.
    fix: Create a fresh browser context per test (default in Playwright Test). Use test fixtures to set up unique data per test. Clean up database state in afterEach or use transactions that roll back.
  - id: failure:playwright/iframe-content
    symptom: Playwright cannot find elements inside an iframe.
    cause: Locators operate on the top-level page by default — iframe content is a separate document.
    fix: Use page.frameLocator('iframe-selector').getByRole(...) to target elements inside iframes. Frame locators chain the same way as page locators but operate within the iframe context.
  - id: failure:playwright/mobile-viewport-issues
    symptom: Tests fail on mobile viewport due to elements being off-screen or overlapped.
    cause: Viewport is set but interactive elements are hidden behind mobile navigation, cookie banners, or keyboard overlay.
    fix: "Set viewport explicitly with test.use({ viewport: { width: 375, height: 812 } }). Dismiss cookie banners and mobile overlays before assertions. Use scrollIntoViewIfNeeded for off-screen elements."
extends: []
implements: []
uses:
  - concept:browser-automation
  - concept:web-standards
part_of: concept:testing-ecosystem
solves:
  - problem:end-to-end-testing
  - problem:browser-automation-flakiness
  - problem:cross-browser-testing
alternatives:
  - package:cypress
  - package:puppeteer
  - package:selenium
  - package:vitest-browser
---
Playwright's architecture differs fundamentally from Cypress and Selenium. It uses the browser's native CDP (Chrome DevTools Protocol) for Chromium and equivalent protocols for Firefox/WebKit — not a JavaScript-injected driver loop. This means Playwright can control multiple browser contexts, tabs, and windows simultaneously, intercept network at the protocol level, and access browser internals (WebSocket frames, CDP events, performance metrics).

Auto-waiting is Playwright's most impactful feature for test reliability. Every action (click, fill, check) has built-in actionability checks: the element must be visible, enabled, and stable (not animating) before the action executes. This eliminates the most common source of test flakiness — timing-dependent code that works locally but fails in CI.

The locator strategy is accessibility-first. Playwright recommends getByRole() as the primary selector — it matches the ARIA role and accessible name, producing tests that mirror how users and assistive technology interact with the page. getByTestId() is the escape hatch for elements without semantic roles. Avoid CSS selectors and XPath for tests — they couple tests to implementation details and break on style changes.
