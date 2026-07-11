---
kind: Package
id: package:typescript/nextjs
name: TypeScript & Next.js Patterns
version: "15"
purpose: Document idiomatic patterns, component models, and type-safe practices for building full-stack web applications with Next.js App Router and TypeScript.
problem_solved: Standardizes how teams approach routing, rendering, data fetching, and type safety in Next.js projects, reducing boilerplate and preventing common pitfalls around server/client boundaries and React hook usage.
install: npx create-next-app@latest my-app --typescript --tailwind --eslint
dependencies:
  - concept:react
  - concept:nodejs
concepts:
  - name: App Router
    id: concept:app-router
    description: File-based routing system using folders and files (page.tsx, layout.tsx, loading.tsx, error.tsx) to define routes, shared layouts, loading states, and error boundaries declaratively.
  - name: Server Components
    id: concept:server-components
    description: React components that execute and render exclusively on the server, reducing client-side JavaScript and enabling direct database/file-system access.
  - name: Client Components
    id: concept:client-components
    description: Components marked with the 'use client' directive that render on the client, supporting interactivity, browser APIs, and React hooks.
  - name: API Routes
    id: concept:api-routes
    description: Route handlers in the app/api directory that act as backend endpoints, supporting HTTP methods, middleware, and request/response typing.
  - name: React Hooks
    id: concept:react-hooks
    description: Functions like useState, useEffect, and useRef that enable stateful logic and side effects in functional components.
  - name: Type System
    id: concept:type-system
    description: TypeScript's structural typing, generics, union/intersection types, and utility types for modeling complex data shapes and ensuring compile-time safety.
  - name: Data Fetching
    id: concept:data-fetching
    description: Patterns for server-side fetch in Server Components, client-side data fetching with React Query or SWR, and caching/revalidation strategies.
  - name: Middleware
    id: concept:middleware
    description: Edge-based request interception via middleware.ts for redirects, rewrites, authentication checks, and header manipulation before route handling.
  - name: React
    id: concept:react
    description: A declarative, component-based JavaScript library for building user interfaces maintained by Meta
  - name: Node.js
    id: concept:nodejs
    description: A JavaScript runtime built on V8 for building server-side and networking applications
  - name: Web Platform
    id: concept:web-platform
    description: The broader web platform — browsers, standards, and runtime environments that Next.js targets.
apis:
  - name: useState<T>(initial) -> [state, setState]
    id: api:useState
    signature: "useState<T>(initial: T | (() => T)): [T, Dispatch<SetStateAction<T>>]"
    returns: A stateful value and a setter function.
    description: Declares local state in functional components; the setter triggers a re-render.
  - name: useEffect(fn, deps)
    id: api:useEffect
    signature: "useEffect(fn: () => (void | (() => void)), deps?: any[]): void"
    returns: undefined
    description: Executes side effects after render; supports cleanup via returned function. Re-runs when dependencies change.
  - name: useContext(Context)
    id: api:useContext
    signature: "useContext<T>(context: Context<T>): T"
    returns: The current context value.
    description: Reads the nearest provider value for a given context; re-renders when the value changes.
  - name: useRef<T>(initial) -> RefObject
    id: api:useRef
    signature: "useRef<T>(initial: T): MutableRefObject<T>"
    returns: A mutable ref object whose .current property persists across renders.
    description: Holds mutable values or DOM references without causing re-renders on mutation.
  - name: useCallback(fn, deps)
    id: api:useCallback
    signature: "useCallback<T extends Function>(fn: T, deps: any[]): T"
    returns: A memoized version of the callback.
    description: Returns a stable function reference that only changes when dependencies change, preventing unnecessary child re-renders.
  - name: useMemo(fn, deps)
    id: api:useMemo
    signature: "useMemo<T>(fn: () => T, deps: any[]): T"
    returns: A memoized computed value.
    description: Recomputes a value only when dependencies change; useful for expensive calculations.
  - name: useReducer(reducer, initial) -> [state, dispatch]
    id: api:useReducer
    signature: "useReducer<R extends Reducer<any, any>, I>(reducer: R, initial: I): [ReducerState<R>, Dispatch<ReducerAction<R>>]"
    returns: Current state and a dispatch function.
    description: Manages complex state logic via a reducer function and action dispatches, analogous to Redux.
  - name: createContext<T>(default) -> Context
    id: api:createContext
    signature: "createContext<T>(defaultValue: T): Context<T>"
    returns: A context object with Provider and Consumer.
    description: Creates a context to share values across the component tree without explicit prop drilling.
  - name: Suspense(fallback)
    id: api:Suspense
    signature: "Suspense(fallback: ReactNode): ReactNode"
    returns: A React element wrapping children with a loading fallback.
    description: Declaratively specifies a loading UI while nested components are resolving asynchronous dependencies (data fetching, code splitting).
  - name: forwardRef<T, P>(render)
    id: api:forwardRef
    signature: "forwardRef<T, P>(render: (props: P, ref: ForwardedRef<T>) => ReactNode): ForwardRefExoticComponent<P>"
    returns: A component that forwards its ref prop to a child DOM element.
    description: Allows parent components to pass a ref through to a child's DOM node, useful for focus management and imperative animations.
failures:
  - id: failure:missing-deps
    symptom: Infinite re-render loops or stale closures in useEffect/useCallback/useMemo
    cause: Missing or incorrect entries in the dependency array, causing the hook to re-run unexpectedly or capture outdated values.
    fix: Use exhaustive-deps ESLint rule, verify all referenced variables are in the array, or restructure logic to avoid stale captures.
  - id: failure:stale-closures
    symptom: Event handlers or callbacks reading old props/state values
    cause: Closure captures a snapshot of state at render time and does not update when state changes.
    fix: Use functional state updates (setState(prev => ...)), add the stale variable to deps, or use refs for mutable values.
  - id: failure:hydration-mismatch
    symptom: Console warnings about client/server HTML mismatch; UI flicker on page load
    cause: Server-rendered HTML differs from the first client render due to browser-only APIs, random values, or conditional rendering based on window.
    fix: Use useEffect for browser-only code, suppressHydrationWarning on affected elements, or ensure deterministic server output.
  - id: failure:memory-leaks
    symptom: Growing memory usage; stale subscriptions firing after unmount
    cause: Event listeners, observers, or timers registered in useEffect without a cleanup function.
    fix: Always return a cleanup function from useEffect that removes listeners, clears intervals, or aborts pending requests.
  - id: failure:over-fetching-client
    symptom: Client-side waterfall requests for data already available on the server
    cause: Fetching data in Client Components (useEffect + fetch) when the same data could be fetched and passed down from a Server Component.
    fix: Move data fetching to Server Components or use route handlers with React Query for client-side caching.
extends:
  - concept:react
implements: []
uses:
  - concept:app-router
  - concept:server-components
  - concept:client-components
  - concept:react-hooks
  - concept:type-system
part_of: concept:web-platform
solves:
  - problem:full-stack-react-typescript
alternatives:
  - package:remix
  - package:sveltekit
  - package:nuxt
  - package:astro
---

Next.js App Router introduces a file-system convention where `page.tsx`, `layout.tsx`, `loading.tsx`, and `error.tsx` files define route segments with built-in support for nested layouts, streaming via Suspense, and granular error boundaries. This colocation pattern eliminates manual route configuration and enforces a predictable project structure. Server Components are the default — they execute on the server, can directly query databases or read the filesystem, and only send their rendered output (not JavaScript bundles) to the client. Client Components, marked with `"use client"`, are used sparingly for interactive UI that needs browser APIs, event handlers, or React hooks.

The decision boundary between Server and Client Components is the most important architectural choice in a Next.js application. Data fetching, authentication, and content rendering should default to Server Components, while interactive elements (forms, modals, real-time updates) become Client Components. TypeScript's structural type system complements this model by allowing shared type definitions across the server-client boundary — `zod` or `typebox` schemas can validate API inputs at runtime while generating inferred types for compile-time safety. Generics, union types, and template literal types reduce duplication in route handler responses and component props.

Data fetching patterns in the App Router favor `async` Server Components with native `fetch` (which includes automatic deduplication and caching) complemented by React Query or SWR on the client when optimistic updates, polling, or cache invalidation are needed. Route handlers in `app/api/` replace the old Pages Router API routes with a simpler `export async function GET/POST/PUT/DELETE` signature, and middleware at the edge handles authentication redirects, header rewrites, and bot detection before requests reach the application layer. This combination of server-first rendering, file-based routing, and type-safe interfaces makes Next.js a cohesive framework for full-stack TypeScript development.
