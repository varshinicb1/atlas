---
kind: Package
id: package:testing-library
name: Testing Library
version: "16"
purpose: Document Testing Library patterns — DOM testing queries, user-centric assertions, accessibility-first selectors, and framework-specific wrappers (@testing-library/react, vue, svelte).
problem_solved: Encourages testing software the way users interact with it — by querying accessible elements (labels, text, roles) rather than implementation details (component state, internal CSS classes, data-testid as last resort), producing tests that survive refactors and catch real regressions.
install: npm install --save-dev @testing-library/react @testing-library/jest-dom @testing-library/user-event
dependencies:
  - concept:testing-patterns
  - concept:dom-api
  - concept:accessibility
concepts:
  - name: Queries
    id: concept:testing-library/queries
    description: "Methods to find elements in the rendered DOM, prioritized by user visibility: getByRole (ARIA role), getByLabelText (form labels), getByPlaceholderText, getByText (visible text), getByDisplayValue (form values), getByTitle, getByAltText (images), and getByTestId (last resort). Each has getBy, queryBy (null if not found), findBy (async, waits), and getAllBy/queryAllBy/findAllBy variants."
  - name: Priority of Queries
    id: concept:testing-library/query-priority
    description: "Accessibility-first query strategy: 1) getByRole — accessible to screen readers, 2) getByLabelText — form fields with labels, 3) getByPlaceholderText — fallback, 4) getByText — non-interactive content, 5) getByTestId — only when no accessible alternative exists. This mirrors how users and assistive technology find elements."
  - name: User-Event
    id: concept:testing-library/user-event
    description: "A companion library that simulates user interactions more realistically than fireEvent. user.click(), user.type(), user.keyboard(), user.selectOptions(), user.hover() trigger the full sequence of events a real browser dispatches (focus, keyDown, keyPress, input, keyUp, blur). async user.setup() returns a prepared user instance."
  - name: Jest-DOM Matchers
    id: concept:testing-library/jest-dom
    description: "Custom Jest matchers extending expect: toBeVisible(), toBeInTheDocument(), toHaveTextContent(), toHaveValue(), toBeChecked(), toBeDisabled(), toHaveAttribute(), toHaveClass(), toHaveStyle(), toBeEmpty(), toHaveFocus(). Import via @testing-library/jest-dom in setup files."
  - name: Async Queries
    id: concept:testing-library/async
    description: "findBy* and findAllBy* queries return Promises that wait up to timeout (default 1000ms, configurable via waitFor options). Use when elements appear after async operations (data fetching, animations). Prefer waitFor(() => expect(...)) for custom conditions beyond element presence."
  - name: Screen
    id: concept:testing-library/screen
    description: "A global object that provides all query methods without destructuring. screen.getByText('Submit'), screen.getByRole('button'), screen.logTestingPlaygroundURL() generates a Playground URL for debugging. Using screen reduces imports and keeps tests cleaner."
  - name: Act Utility
    id: concept:testing-library/act
    description: "render() is wrapped in act() automatically by testing-library. For custom updates (state changes outside render), wrap with act(() => {...}). userEvent calls are also act-wrapped. WaitFor uses act internally to flush pending state updates before assertions."
  - name: Cleanup
    id: concept:testing-library/cleanup
    description: "React Testing Library auto-cleanups after each test (afterEach). Vue and others bundle cleanup() for manual teardown. Cleanup unmounts components and removes them from the DOM, preventing test pollution. Disable with cleanup: false in render options."
  - name: Query Selectors
    id: concept:testing-library/selectors
    description: "Queries accept options: {selector: 'css-selector'} narrows matches within results. {name: 'text'} filters by accessible name. {hidden: true} includes elements hidden from accessibility tree. {exact: false} for substring text matching. {normalizeWhitespace: false} preserves whitespace."
  - name: Debugging
    id: concept:testing-library/debugging
    description: "screen.debug() prints the DOM tree, truncated to 7000 chars. screen.debug(element, 30000) with custom length. screen.logTestingPlaygroundURL() opens the element in Testing Playground for interactive query creation. container.innerHTML for raw DOM inspection."
  - name: Framework Wrappers
    id: concept:testing-library/wrappers
    description: "@testing-library/react exports render, Screen, and queries for React. @testing-library/vue for Vue 3, @testing-library/svelte for Svelte, @testing-library/angular for Angular. Each renders components and provides the same query API, making cross-framework knowledge transferable."
apis:
  - name: render(ui, options)
    id: api:testing-library/render
    signature: "render(ui: React.ReactElement, options?: { wrapper, container, baseElement, hydrate, queries, legacyRoot }): RenderResult"
    returns: RenderResult with query methods and container.
    description: "Renders a React component into a detached DOM node. The wrapper option wraps the component in providers (Router, Theme, Context). The result provides getBy*, queryBy*, findBy* queries, container, baseElement, debug(), rerender(), and unmount()."
  - name: screen.getByRole(role, options)
    id: api:testing-library/getbyrole
    signature: "screen.getByRole(role: string, options?: { name, hidden, selected, checked, pressed, level, description }): HTMLElement"
    returns: A matching HTMLElement or throws.
    description: "Queries by ARIA role (button, heading, textbox, link, checkbox, radio, list, listitem, dialog). The name option matches accessible name (label text, aria-label). This is the most preferred query as it tests accessibility."
  - name: screen.getByText(text, options)
    id: api:testing-library/getbytext
    signature: "screen.getByText(text: string | RegExp, options?: { selector, exact, ignore, trim, collapseWhitespace }): HTMLElement"
    returns: A matching HTMLElement or throws.
    description: "Queries by visible text content. text can be a string (exact match default) or RegExp. Use getByText('Submit') for buttons with text, getByText(/confirm/i) for case-insensitive matching."
  - name: user.setup()
    id: api:testing-library/user-setup
    signature: "const user = userEvent.setup(options?: { delay, advanceTimers, writeToClipboard, pointerEventsCheck }): UserInstance"
    returns: A user instance with interaction methods.
    description: "Creates a configured user instance. delay (ms) adds realistic typing delays. advanceTimers integrates with jest fake timers. The returned user provides .click(), .type(), .keyboard(), .clear(), .selectOptions(), .tab(), .hover(), .unhover()."
  - name: waitFor(callback, options)
    id: api:testing-library/waitfor
    signature: "waitFor<T>(callback: () => T | Promise<T>, options?: { container, timeout, interval, onTimeout, mutationObserverOptions }): Promise<T>"
    returns: The callback's return value.
    description: "Polls the callback until it stops throwing. Default timeout 1000ms, interval 50ms. Use for custom async conditions: waitFor(() => expect(screen.getByRole('dialog')).toBeVisible())."
  - name: within(element)
    id: api:testing-library/within
    signature: "within(element: HTMLElement) -> { getByRole, getByText, getByLabelText, ... }"
    returns: A scoped queries object.
    description: "Creates query functions scoped to a given element. Useful for querying within a specific container: within(dialog).getByRole('button', {name: 'Confirm'}). All query variants (getBy, queryBy, findBy) are available."
sections:
  - title: User-Centric Testing Pattern
    id: section:testing-library/user-centric
    content: |
      Write tests based on accessible queries and realistic user interactions:

      ```typescript
      import { render, screen } from '@testing-library/react';
      import userEvent from '@testing-library/user-event';
      import '@testing-library/jest-dom';
      import { LoginForm } from './LoginForm';

      describe('LoginForm', () => {
          it('submits credentials when form is filled', async () => {
              const onSubmit = jest.fn();
              const user = userEvent.setup();

              render(<LoginForm onSubmit={onSubmit} />);

              await user.type(screen.getByLabelText(/email/i), 'user@example.com');
              await user.type(screen.getByLabelText(/password/i), 'secure123');
              await user.click(screen.getByRole('button', { name: /sign in/i }));

              expect(onSubmit).toHaveBeenCalledWith({
                  email: 'user@example.com',
                  password: 'secure123'
              });
          });

          it('shows validation errors for empty fields', async () => {
              const user = userEvent.setup();
              render(<LoginForm onSubmit={jest.fn()} />);

              await user.click(screen.getByRole('button', { name: /sign in/i }));

              expect(screen.getByText(/email is required/i)).toBeVisible();
              expect(screen.getByText(/password is required/i)).toBeVisible();
          });
      });
      ```

      Note: no `data-testid` was needed. The test finds elements by label and role — the same way users find them.
  - title: Async Loading States
    id: section:testing-library/async-loading
    content: |
      Test loading, success, and error states for async operations:

      ```typescript
      it('shows success message after form submission', async () => {
          const user = userEvent.setup();
          render(<UserProfile userId="123" />);

          // Loading state appears before data resolves
          expect(screen.getByText(/loading/i)).toBeVisible();

          // Wait for success state
          const heading = await screen.findByRole('heading', { name: /profile/i });
          expect(heading).toBeVisible();
          expect(screen.getByText(/user@example.com/i)).toBeInTheDocument();

          // Loading state is gone
          expect(screen.queryByText(/loading/i)).toBeNull();
      });

      it('shows error message on fetch failure', async () => {
          render(<UserProfile userId="invalid" />);
          const errorMessage = await screen.findByRole('alert');
          expect(errorMessage).toHaveTextContent(/failed to load/i);
      });
      ```

      Use `findBy*` (async) when elements appear after data fetching. Use `queryBy*` to assert an element is NOT in the DOM after async transitions.
---
