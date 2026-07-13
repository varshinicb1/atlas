---
kind: Package
id: package:jotai
name: Jotai State Management
version: "2"
purpose: Document Jotai patterns — atomic state, derived atoms, async atoms, and integration with React for fine-grained, scalable state management.
problem_solved: Provides a primitive and flexible state management approach based on atoms (independent units of state) that eliminates boilerplate, supports automatic optimization (no unnecessary re-renders), and scales from simple local state to complex derived and async state without Redux-style ceremony.
install: npm install jotai
dependencies:
  - concept:react
  - concept:state-management
  - concept:functional-programming
concepts:
  - name: Atoms
    id: concept:jotai/atoms
    description: "The fundamental state unit — an atom() call creates a piece of state with an initial value. Atoms are independent, composable, and lazy (not created until read). No global store registration needed. const countAtom = atom(0). Atoms can hold primitives, objects, or complex data structures."
  - name: Derived Atoms
    id: concept:jotai/derived-atoms
    description: "Atoms computed from other atoms via a read function. const doubleAtom = atom((get) => get(countAtom) * 2). Automatically tracks dependencies and only re-computes when a dependency changes. Derived atoms can also have a write function, enabling bidirectional data flow (read-write atoms)."
  - name: Async Atoms
    id: concept:jotai/async-atoms
    description: "Atoms with async read functions — atom(async (get) => { const data = await fetch('/api/data'); return data.json(); }). Jotai suspends rendering until the promise resolves. Integrates with React Suspense for loading states. Use loadable() to handle pending/error states without Suspense."
  - name: useAtom Hook
    id: concept:jotai/use-atom
    description: "React hook that returns [value, setAtom] similar to useState. const [count, setCount] = useAtom(countAtom). setCount accepts a new value or updater function. The component re-renders only when the atom's value changes, providing granular reactivity."
  - name: write-only Atoms
    id: concept:jotai/write-atoms
    description: "Atoms with only a write function — useful for actions that modify other atoms. const incrementAtom = atom(null, (get, set) => set(countAtom, get(countAtom) + 1)). Use with useSetAtom() hook which returns the write function without subscribing to reads."
  - name: Provider
    id: concept:jotai/provider
    description: "JotaiProvider creates a scoped state store, enabling per-tree state isolation (useful for SSR, testing, or multi-instance scenarios). Without a Provider, atoms use a default global store. const ScopeProvider = Provider; wrap subtrees for independent state scopes."
  - name: Atom Families
    id: concept:jotai/atom-families
    description: "Creating atoms dynamically based on parameters. const todoAtom = (id: number) => atom({ id, text: '', completed: false }). Wrapping atom creation in a function that takes parameters. Each call creates an independent atom. Useful for per-entity state (to-do items, form fields)."
  - name: Utility Atoms
    id: concept:jotai/utils
    description: "atomWithStorage (persists to localStorage/sessionStorage/async storage), atomWithReducer (Redux-like reducer atom), splitAtom (splits an array atom into individual atoms), selectAtom (subset of a larger atom with referential equality), and freezeAtom (prevents mutation of atom values)."
  - name: Loadable
    id: concept:jotai/loadable
    description: "A utility that wraps async atoms to expose state without Suspense: const loadable = useAtomValue(loadable(asyncAtom)). Returns { state: 'loading' | 'hasData' | 'hasError', data, error }. Useful when Suspense boundaries are not desired or practical."
  - name: Immer Integration
    id: concept:jotai/immer
    description: "atomWithImmerValue and useImmerAtom enable Immer-based mutable syntax for complex state updates. import { atomWithImmer } from 'jotai-immer'. const immerAtom = atomWithImmer({ nested: { value: 0 } }). Update via set(immerAtom, draft => { draft.nested.value += 1 })."
apis:
  - name: atom(initialValue)
    id: api:jotai/atom
    signature: "atom<Value>(initialValue: Value): WritableAtom<Value, [Value], void>"
    returns: A writable atom.
    description: "Creates a writable atom with an initial value. The atom can be both read and written. For derived atoms: atom(read: (get) => Value, write?: (get, set, update) => void). The read function tracks dependencies via get()."
  - name: useAtom(atom)
    id: api:jotai/use-atom
    signature: "useAtom<Value>(atom: Atom<Value>): [Value, SetAtom<[Value], void>]"
    returns: A tuple of the current value and a setter function.
    description: "React hook that returns the atom's value and setter. The component subscribes to the atom. Only re-renders when the specific atom value changes. The setter accepts a new value or an updater function: set(prev => prev + 1)."
  - name: useAtomValue(atom)
    id: api:jotai/use-atom-value
    signature: "useAtomValue<Value>(atom: Atom<Value>): Value"
    returns: The atom's current value.
    description: "Read-only hook that returns the atom's value. Does not subscribe to writes. Useful for components that only display data. Combined with useSetAtom for the write side, separating read and write concerns."
  - name: useSetAtom(atom)
    id: api:jotai/use-set-atom
    signature: "useSetAtom<Value>(atom: WritableAtom<Value, any[], void>): SetAtom<[...Args], void>"
    returns: A setter function for the atom.
    description: "Returns the atom's write function without subscribing to its value. The component never re-renders when the atom changes. Optimal for performance-critical components that only trigger updates (buttons, form submits, etc.)."
  - name: atomWithStorage(key, initialValue)
    id: api:jotai/atom-with-storage
    signature: "atomWithStorage<Value>(key: string, initialValue: Value, storage?: SyncStorage | AsyncStorage): WritableAtom<Value, [Value], void>"
    returns: A writable atom backed by persistent storage.
    description: "Creates an atom synchronized with localStorage (default). The storage parameter supports custom implementations for AsyncStorage, sessionStorage, or IndexedDB. Values are JSON-serialized. Subscribe to storage events for cross-tab sync."
  - name: loadable(atom)
    id: api:jotai/loadable
    signature: "loadable<Value>(atom: Atom<Value>): Atom<Loadable<Value>>"
    returns: An atom wrapping the loadable state.
    description: "Wraps an async atom to return { state, data, error } instead of suspending. state is 'loading', 'hasData', or 'hasError'. Use loadable with useAtomValue for graceful async handling without React Suspense boundaries."
sections:
  - title: Basic State Pattern
    id: section:jotai/basic
    content: |
      Atoms for local and shared state with derived values:

      ```typescript
      import { atom, useAtom } from 'jotai';

      const countAtom = atom(0);
      const doubleAtom = atom((get) => get(countAtom) * 2);
      const incrementAtom = atom(null, (get, set) => set(countAtom, get(countAtom) + 1));

      function Counter() {
          const [count, setCount] = useAtom(countAtom);
          const [double] = useAtom(doubleAtom);
          const [, increment] = useAtom(incrementAtom);

          return (
              <div>
                  <p>Count: {count}</p>
                  <p>Double: {double}</p>
                  <button onClick={() => setCount(c => c + 1)}>+</button>
                  <button onClick={increment}>Increment via atom</button>
              </div>
          );
      }
      ```

      No Provider needed — atoms work with a default global store. Only the Counter component re-renders when countAtom changes.
  - title: Async Data with Atoms
    id: section:jotai/async
    content: |
      Fetch and cache API data using async atoms:

      ```typescript
      import { atom, useAtom } from 'jotai';
      import { loadable } from 'jotai/utils';
      import { Suspense } from 'react';

      const userIdAtom = atom(1);
      const userAtom = atom(async (get) => {
          const id = get(userIdAtom);
          const res = await fetch(`https://api.example.com/users/${id}`);
          return res.json();
      });

      // Using Suspense
      function UserProfile() {
          const [user] = useAtom(userAtom);
          return <div>{user.name} — {user.email}</div>;
      }

      function App() {
          return (
              <Suspense fallback={<div>Loading...</div>}>
                  <UserProfile />
              </Suspense>
          );
      }

      // Using loadable (no Suspense)
      function SafeProfile() {
          const [loadableUser] = useAtom(loadable(userAtom));
          if (loadableUser.state === 'loading') return <div>Loading...</div>;
          if (loadableUser.state === 'hasError') return <div>Error: {loadableUser.error.message}</div>;
          return <div>{loadableUser.data.name}</div>;
      }
      ```

      Async atoms cache the result — re-renders only happen when userIdAtom changes, not on every component mount.
---
