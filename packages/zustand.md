---
kind: Package
id: package:zustand
name: Zustand
version: "5.0"
purpose: Document Zustand — a small, fast, and scalable state management library for React with a hook-based API, middleware (persist, immer, devtools), and external store subscriptions outside React.
problem_solved: Provides React state management that is simpler than Redux (no actions/reducers boilerplate), more performant than Context (no re-render cascading), and more flexible than useState (works outside React, supports middleware, minimal re-renders via selectors).
install: npm install zustand
dependencies:
  - concept:react
  - concept:state-management
concepts:
  - name: Store
    id: concept:zustand/store
    description: "A store is created with create((set, get) => ({ ... })). set() merges state updates, get() reads current state outside React. The store is a plain object — no classes, no reducers, no action types. Returns a React hook and optionally a vanilla store (createStore)."
  - name: Selectors
    id: concept:zustand/selectors
    description: useStore(state => state.count) subscribes to a specific slice of state. The component re-renders only when the selected value changes. Equality functions (shallow, deep) for complex selections. Use auto-generated selectors with createSelectors() for type-safe access.
  - name: Middleware
    id: concept:zustand/middleware
    description: persist (localStorage/AsyncStorage/IndexedDB), immer (immutable updates with mutable syntax), devtools (Redux DevTools integration), subscribeWithSelector (fine-grained subscriptions), and combine (merge multiple state slices). Middleware wraps create().
  - name: Persist
    id: concept:zustand/persist
    description: persist middleware saves state to storage (localStorage by default). Configurable storage, partialize (select which fields to persist), merge (hydration strategy), onRehydrateStorage (callback after load). Versioning and migration for schema changes.
  - name: Immer Integration
    id: concept:zustand/immer
    description: "immer middleware enables set((state) => { state.items.push(newItem); state.user.name = 'Alice' }) — mutable syntax that produces immutable state updates. Eliminates spread operator chains for deeply nested state."
  - name: External Subscriptions
    id: concept:zustand/external
    description: useStore.getState() and useStore.subscribe() work outside React — in event handlers, timeout callbacks, or non-React frameworks. subscribe returns an unsubscribe function. With subscribeWithSelector middleware, subscribe accepts a selector for targeted notifications.
  - name: Slices Pattern
    id: concept:zustand/slices
    description: "Split large stores into smaller slice functions — const useStore = create((...a) => ({ ...userSlice(...a), ...cartSlice(...a), ...uiSlice(...a) })). Each slice receives set/get and returns its state/actions. Slices compose via object spread."
apis:
  - name: create(stateFn)
    id: api:zustand/create
    signature: "const useStore = create<State>((set, get) => ({ count: 0, increment: () => set((s) => ({ count: s.count + 1 })) }))"
    returns: A React hook.
    description: "Creates a Zustand store. The function receives set (partial state merger) and get (state reader). Returns a React hook (useStore) that subscribes to the store. For vanilla JS: import { createStore } from 'zustand/vanilla'."
  - name: useStore(selector, equalityFn?)
    id: api:zustand/use-store
    signature: "const count = useStore((state) => state.count); const { increment } = useStore((s) => ({ increment: s.increment }), shallow)"
    returns: Selected state value.
    description: React hook that subscribes to state changes. Selector extracts a slice of state. Component re-renders only when the selected value changes (=== by default). shallow does shallow comparison for object selectors.
  - name: set(partial)
    id: api:zustand/set
    signature: "set((state) => ({ items: [...state.items, newItem] }))"
    returns: void (triggers subscriber updates).
    description: Merges state update. Accepts a partial state object or a function receiving current state. set is the only way to update state — state is immutable, set merges shallowly. Use immer middleware for deep mutations.
  - name: useStore.getState()
    id: api:zustand/get-state
    signature: "const { count } = useStore.getState()"
    returns: Current state snapshot.
    description: Reads state outside React. Returns the current state snapshot immediately. Does not create a subscription. Use in event handlers, timeouts, or non-React code.
  - name: useStore.subscribe(listener)
    id: api:zustand/subscribe
    signature: "const unsub = useStore.subscribe((state, prevState) => console.log('changed'))"
    returns: Unsubscribe function.
    description: "Subscribes to all state changes outside React. With subscribeWithSelector middleware: subscribe(selector, callback, { equalityFn }) for targeted subscriptions."
  - name: persist(config)
    id: api:zustand/persist-middleware
    signature: "const useStore = create(persist((set) => ({ theme: 'dark' }), { name: 'app-storage', partialize: (state) => ({ theme: state.theme }) }))"
    returns: A wrapped store creator.
    description: Persists state to storage. name is the storage key. partialize selects which fields to persist. merge controls hydration. version + migrate for schema evolution. storage replaces default localStorage with AsyncStorage, IndexedDB, etc.
failures:
  - id: failure:zustand/excessive-re-renders
    symptom: Components re-render when unrelated state changes.
    cause: Using the entire store as a dependency without a selector, or destructuring values from the store hook.
    fix: "Always use a selector: useStore(state => state.count). Use shallow equality for object selectors. Use subscribeWithSelector for external subscriptions. Avoid destructuring at the component level."
  - id: failure:zustand/persist-hydration-gap
    symptom: UI flashes with default values before persisted state loads.
    cause: Persist middleware hydrates asynchronously — the store initially has default values, then hydrates from storage, causing a double render.
    fix: Use onRehydrateStorage to show a loading state. Set hasHydrated flag in onRehydrateStorage and conditionally render content. Use skipHydration in persist config and manually call rehydrate() after the initial render.
  - id: failure:zustand/immer-direct-mutation-error
    symptom: Immer middleware throws "Cannot modify a proxy" error.
    cause: Mutating state directly outside the immer producer (set callback). Immer only allows mutations inside the set function.
    fix: "Always wrap mutations in set((state) => { state.value = newValue }). Do not mutate useStore.getState() directly — immer cannot proxy external state reads."
  - id: failure:zustand/selector-memoization
    symptom: Components re-render even with stable selectors that return the same value.
    cause: "Creating new objects/arrays inside the selector on every render (e.g., state => ({ a: state.a, b: state.b }))."
    fix: "Use shallow equality for object selectors: useStore(state => ({ a: state.a, b: state.b }), shallow). For primitive values, return the primitive directly. Use createSelectors() for auto-generated memoized selectors."
extends: []
implements: []
uses:
  - concept:react
  - concept:state-management
part_of: concept:react-ecosystem
solves:
  - problem:simple-global-state-react
  - problem:react-performance-state-management
  - problem:react-state-outside-components
alternatives:
  - package:redux
  - package:jotai
  - package:valtio
  - package:react-context
---
Zustand's core design principle is simplicity through hooks. A store is a function that returns a hook — no Provider wrapper, no context, no reducer ceremony. The store lives outside the React tree, so it can be accessed from anywhere (event handlers, setTimeout, non-React code) without reconciliation overhead.

The selector-based subscription model is what makes Zustand performant. By default, a component subscribing to the store via useStore(selector) only re-renders when the selected value changes. This means changing a deeply nested field in a large store only re-renders the component that selected that field — not every component that uses the store. Compare this to Context, where any state change causes all consumers to re-render, and Redux, where connect/mapStateToProps has similar selector semantics but with more boilerplate.

Zustand's middleware stack is pluggable and functional. persist wraps the store to sync with external storage. immer enables mutable update syntax. devtools connects to Redux DevTools for debugging. Middleware composes — you can use persist + immer + devtools together. The middleware pattern is additive: each middleware wraps the store creator and returns a modified store.
