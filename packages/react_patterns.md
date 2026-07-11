---
kind: Package
id: package:react/patterns
name: React 19 Patterns
version: "19"
purpose: Document idiomatic React 19 patterns — hooks, Server Components, Suspense, state management, and performance for production UIs.
problem_solved: Provides a reference for React's declarative rendering model, the hooks lifecycle, concurrent features, and the server/client split, reducing re-render bugs, effect misuse, and hydration mismatches.
install: npm install react@19 react-dom@19
dependencies:
  - package:react
  - package:react-dom
concepts:
  - name: Web Platform
    id: concept:domain/web-platform
    description: The broader web platform — browsers, standards, and runtime environments that React targets.
  - name: Hooks
    id: concept:react/hooks
    description: Functions (useState, useEffect, useMemo, useCallback, useRef) that let function components hold state and run side effects following the rules of hooks.
  - name: Server Components
    id: concept:react/server-components
    description: Components rendered on the server (RSC) that ship zero client JS; can be async and read directly from data sources.
  - name: Client Components
    id: concept:react/client-components
    description: Components marked with 'use client' that hydrate in the browser and handle interactivity, events, and browser APIs.
  - name: Suspense
    id: concept:react/suspense
    description: Declarative loading boundary that shows a fallback while children resolve data or code; integrates with concurrent rendering and transitions.
  - name: Concurrent Rendering
    id: concept:react/concurrent
    description: Interruptible rendering via startTransition and useDeferredValue, keeping the UI responsive during expensive updates.
  - name: Rules of Hooks
    id: concept:react/rules-of-hooks
    description: Hooks must be called at the top level, in the same order every render, and only from React functions — never conditionally or in loops.
  - name: State Management
    id: concept:react/state
    description: Local state via useState/useReducer versus external stores (Context, Zustand, Redux) for shared, cross-tree state.
  - name: Context
    id: concept:react/context
    description: A provider-based channel to pass values deep without prop drilling; re-renders all consumers when the value changes.
  - name: Ref & useRef
    id: concept:react/useref
    description: A mutable container whose .current persists across renders without triggering re-renders; used for DOM nodes and stable values.
  - name: Actions & form state
    id: concept:react/actions
    description: React 19 useActionState and <form action> for progressive-enhancement server actions and pending UI without manual fetch wiring.
  - name: Memoization
    id: concept:react/memo
    description: React.memo, useMemo, useCallback to skip re-renders and keep stable references, used sparingly where profiling shows cost.
  - name: Reconciliation
    id: concept:react/reconciliation
    description: React's diffing algorithm that updates only changed DOM nodes; keys help it match elements across lists.
  - name: Hydration
    id: concept:react/hydration
    description: Attaching event listeners and interactivity to server-rendered HTML on the client; mismatches cause warnings and flicker.
  - name: Error Boundaries
    id: concept:react/error-boundaries
    description: Class components (componentDidCatch) that catch render errors in their subtree and show a fallback UI.
apis:
  - name: useState
    id: api:react/useState
    signature: "const [state, setState] = useState(initial)"
    returns: A tuple of the current value and a setter.
    description: Declares component-local state; calling the setter schedules a re-render with the new value (or a functional updater).
  - name: useEffect
    id: api:react/useeffect
    signature: "useEffect(() => { setup; return () => cleanup(); }, [deps])"
    returns: void
    description: Runs side effects after commit; returns a cleanup function; re-runs when deps change. Must not be async itself.
  - name: useMemo
    id: api:react/usememo
    signature: "const v = useMemo(() => compute(x), [x])"
    returns: A memoized value.
    description: Recomputes only when dependencies change; use to avoid expensive work on every render.
  - name: useCallback
    id: api:react/usecallback
    signature: "const cb = useCallback(() => fn(a), [a])"
    returns: A stable function reference.
    description: Keeps a callback identity stable across renders to prevent unnecessary child re-renders.
  - name: useTransition
    id: api:react/usetransition
    signature: "const [isPending, startTransition] = useTransition()"
    returns: A pending flag and a function to wrap non-urgent updates.
    description: Marks state updates as non-blocking so the UI stays responsive during heavy re-renders.
  - name: useDeferredValue
    id: api:react/usedeferredvalue
    signature: "const deferred = useDeferredValue(value)"
    returns: A deferred copy of the value.
    description: Lets React render with a stale value first, then update when rendering settles — useful for search inputs.
  - name: useActionState
    id: api:react/useactionstate
    signature: "const [state, formAction, isPending] = useActionState(action, initial)"
    returns: Server action state, the bound action, and a pending flag.
    description: React 19 API managing form action results and pending UI with progressive enhancement.
  - name: use
    id: api:react/use
    signature: "const data = use(promiseOrContext)"
    returns: The resolved value.
    description: React 19 primitive that reads a promise or context conditionally inside render, enabling Suspense data fetching.
  - name: createContext
    id: api:react/createcontext
    signature: "const Ctx = createContext(default)"
    returns: A context object.
    description: Creates a context whose Provider supplies values to all descendant consumers.
examples:
  - id: example:react/use-transition
    language: tsx
    description: Wrapping a slow filter update in startTransition for responsive input.
  - id: example:react/use-action-state
    language: tsx
    description: A form with server action and pending state via useActionState.
  - id: example:react/suspense-data
    language: tsx
    description: Suspense boundary around an async Server Component fetch.
  - id: example:react/memo-child
    language: tsx
    description: React.memo-wrapped child with useCallback to avoid re-render churn.
failures:
  - id: failure:react/stale-closure
    symptom: Handlers read old state/props; interval captures a fixed value forever.
    cause: A closure captured a snapshot of state at a particular render and deps omitted it.
    fix: Use functional updates (setState(p => ...)), include the value in the effect deps, or store it in a ref.
  - id: failure:react/infinite-effect
    symptom: useEffect fires in an endless loop, crashing the tab.
    cause: An object/array literal or new function in the dependency array creates a new reference every render.
    fix: Stabilize with useMemo/useCallback, depend on primitive fields, or move the value out of deps with useRef.
  - id: failure:react/hydration-mismatch
    symptom: "\"Text content did not match\" warning; UI flicker on load."
    cause: Server and client render different markup due to Date.now(), Math.random(), or window access during render.
    fix: Compute time/random values in useEffect, gate on a mounted flag, or use suppressHydrationWarning sparingly.
  - id: failure:react/missing-key
    symptom: "\"Each child in a list should have a unique key\" warning; state shifts incorrectly on reorder."
    cause: Using array index or no key when mapping lists; index keys break identity on insert/delete.
    fix: Use a stable unique id from the data as the key; never the array index for mutable lists.
  - id: failure:react/over-memoization
    symptom: App is slower or code is harder to maintain with no benefit.
    cause: Wrapping everything in useMemo/useCallback/React.memo defensively without profiling.
    fix: Profile first (React DevTools); memoize only proven hot paths; remember hooks have their own cost.
  - id: failure:react/use-client-misuse
    symptom: "\"You're importing a component that needs useEffect\" or 'use client' boundary errors."
    cause: Calling hooks inside a Server Component, or passing non-serializable values across the server/client boundary.
    fix: Mark the interactive component 'use client'; keep Server Components free of browser-only APIs and pass serializable props.
  - id: failure:react/effect-async
    symptom: "\"An effect function must not be async\" error or unhandled rejections after unmount."
    cause: Declaring useEffect's callback as async creates a promise it cannot clean up, leaking requests.
    fix: Define an inner async function and call it; clean up with an AbortController or a cancelled flag.
extends:
  - concept:react/hooks
uses:
  - concept:react/server-components
  - concept:react/client-components
  - concept:react/suspense
  - concept:react/concurrent
  - concept:react/state
part_of: concept:domain/web-platform
depends_on:
  - package:typescript/nextjs
  - package:typescript/7_migration
solves:
  - problem:declarative-ui
alternatives:
  - package:solidjs
  - package:svelte
  - package:vue
---
# React 19 Patterns

React renders UI as a function of state: given the same inputs, a component returns the same element tree. React 19 leans harder into this model with Server Components (RSC) that render on the server and ship zero JavaScript, and `use` / `useActionState` primitives that blur the server/client boundary. The core discipline remains the rules of hooks — call them unconditionally at the top level, in the same order every render — because React relies on call order to associate state with each hook.

Hooks are the language of function components. `useState` holds local state; `useEffect` runs side effects after commit and must return a cleanup function (otherwise you leak listeners, timers, and subscriptions). A classic bug is the stale closure: an effect or callback captures a snapshot of state and never sees updates, fixed with functional updates or by adding the value to deps. The `exhaustive-deps` ESLint rule is your friend. Do not make `useEffect` async directly — React cannot clean up a promise — instead define and call an inner async function guarded by a cancellation flag or `AbortController`.

Concurrency is React 19's headline. `useTransition` and `useDeferredValue` mark updates as non-urgent so typing and clicks stay responsive while expensive re-renders happen in the background. `Suspense` wraps subtrees that suspend (data or code loading) and shows a fallback, integrating cleanly with `startTransition` and streaming SSR. `useDeferredValue` is perfect for search boxes: render immediately with the previous value, then swap in the new one when rendering settles.

State management is about scope. Local UI state belongs in `useState`/`useReducer`; shared state across the tree uses Context (cheap to read, but every consumer re-renders on change, so split contexts by concern) or external stores like Zustand/Redux for high-frequency, app-wide state. Memoization (`React.memo`, `useMemo`, `useCallback`) is a scalpel, not a default — profile before applying, because hooks themselves cost something. Finally, the server/client split is now first-class: keep Server Components free of browser-only APIs and `'use client'` only where interactivity is needed, passing serializable props across the boundary.
