---
kind: Package
id: package:cypress
name: Cypress E2E Testing
version: "14"
purpose: Document Cypress end-to-end testing patterns — component testing, API mocking, custom commands, CI integration, and best practices for reliable browser tests.
problem_solved: Provides a developer-friendly end-to-end testing framework with automatic waiting, real-time reloading, time-travel debugging, and network control that eliminates flaky tests caused by race conditions and asynchronous complexity.
install: npm install --save-dev cypress @cypress/react @cypress/vite-dev-server
dependencies:
  - concept:testing-patterns
  - concept:javascript
  - concept:node-js
concepts:
  - name: Test Runner
    id: concept:cypress/test-runner
    description: "Cypress runs tests in a real browser (Chrome, Edge, Electron, Firefox) with full control over the DOM, network, and application state. The test runner GUI shows commands, assertions, network requests, and application state side-by-side. Screenshots and videos capture test execution."
  - name: Automatic Waiting
    id: concept:cypress/auto-waiting
    description: "Cypress automatically waits for elements to exist, animations to finish, and assertions to pass for up to defaultCommandTimeout (4s, configurable). Unlike Selenium's implicit waits, Cypress retries until the assertion passes or times out. This eliminates most flaky test races."
  - name: Commands Chain
    id: concept:cypress/commands
    description: "Commands are asynchronous and chainable — cy.visit().get().click().should(). Each command yields a subject for the next command. Commands do not return promises; they enqueue in a command queue executed serially. Use .then() to escape the queue for imperative logic."
  - name: Assertions
    id: concept:cypress/assertions
    description: "Chai-based BDD assertions via .should() and .and() (aliases). expect() for BDD style. Supports should('exist'), should('be.visible'), should('have.text'), should('contain'), should('have.class'), should('have.value'). should($el => { /* callback assertions */ }) for custom checks."
  - name: Fixtures & Stubs
    id: concept:cypress/fixtures
    description: "cy.fixture() loads static JSON/image data from cypress/fixtures/. cy.intercept() stubs network requests with fixture data. Fixtures decouple tests from backend state. Organize fixtures by domain: fixtures/users.json, fixtures/orders.json."
  - name: Network Interception
    id: concept:cypress/intercept
    description: "cy.intercept(url, handler) intercepts and modifies HTTP requests. Handlers return fixture data, modify responses, delay responses (for loading states), or validate request payloads. Use @alias with cy.wait() to assert requests were made with specific parameters."
  - name: Custom Commands
    id: concept:cypress/custom-commands
    description: "Cypress.Commands.add('login', (email, password) => {...}) extends the cy object with reusable commands. Custom commands encapsulate repeated interaction patterns (login, create project, fill form). Place in cypress/support/commands.ts to auto-load in all tests."
  - name: Component Testing
    id: concept:cypress/component-testing
    description: "Mount individual React/Vue/Angular/Svelte components directly in Cypress without a full page load. Faster than E2E and easier to debug. Uses @cypress/react or @cypress/vue. Tests interactivity, state changes, and component behavior in isolation with real browser APIs."
  - name: CI Integration
    id: concept:cypress/ci
    description: "cypress run executes tests headlessly. cypress run --record uploads results to Cypress Cloud (dashboard) for video, screenshots, and flake analysis. --browser chrome --spec 'cypress/e2e/**/*.cy.ts' filters tests. Parallelization via --parallel with CI machine groups."
  - name: Time Travel
    id: concept:cypress/time-travel
    description: "The Test Runner GUI records every command's state. Hover over a command to see the DOM snapshot at that moment. Use this to debug why a test failed — see exactly what the app looked like when the assertion ran, including network state and console logs."
  - name: Environment Variables
    id: concept:cypress/env-vars
    description: "Configuration via cypress.env.json, CYPRESS_* environment variables, or --env flag. Access with Cypress.env('key'). Store URLs, credentials, and API keys. For sensitive values, use CYPRESS_* env vars in CI with Cypress.env() accessing them."
  - name: Screenshots & Videos
    id: concept:cypress/media
    description: "Screenshots capture test state on failure automatically (screenshotOnRunFailure: true). cy.screenshot() for manual captures. Videos record the entire test run (video: true in config). Useful for debugging CI failures and sharing with teammates."
apis:
  - name: cy.visit()
    id: api:cypress/visit
    signature: "cy.visit(url: string, options?: { method, body, headers, qs, auth, onBeforeLoad, onLoad }): Chainable<AUTWindow>"
    returns: A chainable yielding the window object.
    description: "Navigates the browser to a URL. The baseUrl from cypress.config replaces relative URLs. Options control HTTP method, request body, query params, and auth. onBeforeLoad/onLoad callbacks for modifying the app before tests."
  - name: cy.get()
    id: api:cypress/get
    signature: "cy.get(selector: string, options?: { withinSubject, timeout, log }): Chainable<JQuery<HTMLElement>>"
    returns: A chainable yielding matched elements.
    description: "Queries the DOM for elements matching the CSS selector. Automatically retries until the element exists or times out. Supports data-testid attributes (recommended for test selectors), classes, IDs, and complex CSS selectors."
  - name: cy.intercept()
    id: api:cypress/intercept
    signature: "cy.intercept(method: string | RegExp, url: string | RegExp, handler?: object | (req) => void): Chainable<null>"
    returns: A chainable yielding null (use .as() for alias).
    description: "Intercepts matching HTTP requests. handler can be a fixture path, a response object ({statusCode, body, headers}), or a callback that modifies the request. Use .as('alias') to reference the intercept by name with cy.wait()."
  - name: cy.should()
    id: api:cypress/should
    signature: ".should(chainers: string, value?: any) -> Chainable<Subject>"
    returns: A chainable with the same subject.
    description: "Asserts the subject satisfies a Chai chainer. Automatically retries until timeout. Common: should('exist'), should('be.visible'), should('have.text', 'Hello'), should('contain', 'partial'). Also accepts a callback for custom assertions."
  - name: cy.fixture()
    id: api:cypress/fixture
    signature: "cy.fixture(path: string, options?: { encoding }): Chainable<any>"
    returns: A chainable yielding the fixture contents.
    description: "Loads a file from cypress/fixtures/. JSON files return parsed objects. Images return base64. Use with cy.intercept('GET', '/api/users', {fixture: 'users.json'}) to stub API responses with realistic data."
  - name: cy.wait()
    id: api:cypress/wait
    signature: "cy.wait(alias: string | string[], options?: { timeout, log }): Chainable<Interception | Interception[]>"
    returns: A chainable yielding the interception(s).
    description: "Waits for a specific aliased network request. Returns the Interception object with request/response details. Multiple aliases wait for all to complete. Timeout overrides the default requestTimeout."
sections:
  - title: E2E Test Patterns
    id: section:cypress/e2e-patterns
    content: |
      Structure tests with login flow, data setup via API, and UI interaction:

      ```typescript
      describe('User Dashboard', () => {
          beforeEach(() => {
              cy.intercept('POST', '/api/login', { fixture: 'login-response.json' }).as('login');
              cy.intercept('GET', '/api/projects', { fixture: 'projects.json' }).as('getProjects');
              cy.visit('/login');
              cy.get('[data-testid="email-input"]').type('user@example.com');
              cy.get('[data-testid="password-input"]').type('password123');
              cy.get('[data-testid="submit-btn"]').click();
              cy.wait('@login');
              cy.url().should('include', '/dashboard');
          });

          it('displays project list', () => {
              cy.wait('@getProjects');
              cy.get('[data-testid="project-card"]').should('have.length.at.least', 1);
              cy.get('[data-testid="project-card"]').first().should('be.visible');
          });

          it('creates a new project', () => {
              cy.intercept('POST', '/api/projects', { statusCode: 201, body: { id: 42, name: 'Test' } });
              cy.get('[data-testid="new-project-btn"]').click();
              cy.get('[data-testid="project-name-input"]').type('Test Project');
              cy.get('[data-testid="create-submit-btn"]').click();
              cy.get('[data-testid="project-card"]').should('contain', 'Test Project');
          });
      });
      ```

      Use `cy.intercept()` for all API calls to decouple tests from backend state.
  - title: Component Testing
    id: section:cypress/component-testing
    content: |
      Mount components directly and test interactivity:

      ```typescript
      import { mount } from '@cypress/react';
      import { Counter } from './Counter';

      describe('Counter Component', () => {
          it('increments and decrements', () => {
              mount(<Counter initialCount={0} />);
              cy.get('[data-testid="count"]').should('have.text', '0');
              cy.get('[data-testid="increment-btn"]').click();
              cy.get('[data-testid="count"]').should('have.text', '1');
              cy.get('[data-testid="decrement-btn"]').click().click();
              cy.get('[data-testid="count"]').should('have.text', '-1');
          });

          it('does not go below zero when min is set', () => {
              mount(<Counter initialCount={0} min={0} />);
              cy.get('[data-testid="decrement-btn"]').click();
              cy.get('[data-testid="count"]').should('have.text', '0');
          });
      });
      ```

      Component testing runs in a real browser with full CSS, event handling, and rendering — catching issues that unit tests miss.
---
