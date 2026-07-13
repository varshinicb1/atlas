---
kind: Package
id: package:vue
name: Vue.js Patterns
version: "3.5"
purpose: Document Vue 3 patterns — Composition API, reactivity, SFCs, Pinia state management, routing, and SSR
problem_solved: Provides a reference for Vue 3's composition-based reactivity model, single-file components, Pinia stores, and the router/navigation lifecycle, reducing watcher leaks, reactivity loss bugs, and hydration mismatches in SSR.
install: npm create vue@latest
dependencies:
  - package:typescript
  - package:npm
concepts:
  - name: Composition API
    id: concept:vue/composition-api
    description: A set of functions (ref, reactive, computed, watch) that organize component logic by feature rather than lifecycle options, enabling better code reuse via composables.
  - name: Reactivity
    id: concept:vue/reactivity
    description: Vue's proxy-based reactivity system that tracks dependencies automatically and re-renders only the effects that depend on changed values.
  - name: Single-File Components
    id: concept:vue/sfc
    description: .vue files co-locating template, script, and style in one file with scoped CSS and compiler optimizations.
  - name: Directives
    id: concept:vue/directives
    description: Template directives (v-if, v-for, v-model, v-bind, v-on) that bind data to the DOM declaratively.
  - name: Computed Properties
    id: concept:vue/computed
    description: Derived state that caches its value until dependencies change, computed lazily and synchronously via computed().
  - name: Watchers
    id: concept:vue/watchers
    description: Functions that perform side effects when tracked reactive sources change, with deep, immediate, and flush options.
  - name: Slots
    id: concept:vue/slots
    description: Content distribution mechanism where parent components inject markup into child component placeholders, supporting named and scoped slots.
  - name: Teleport
    id: concept:vue/teleport
    description: Moving template content to a different DOM node (e.g., modals to body) while preserving Vue's component hierarchy.
  - name: Suspense
    id: concept:vue/suspense
    description: A built-in component that handles async dependencies in components, showing fallback content while async setup or nested async components resolve.
  - name: Server-Side Rendering
    id: concept:vue/ssr
    description: Rendering Vue components on the server with Vue's SSR API or Nuxt for hydration and SEO; requires careful handling of browser-only APIs.
  - name: Composables
    id: concept:vue/composables
    description: Functions that use the Composition API to encapsulate stateful logic (useMouse, useFetch) for reuse across components.
  - name: Provide / Inject
    id: concept:vue/provide-inject
    description: A dependency injection mechanism for passing values through the component tree without prop drilling, with optional reactive binding.
apis:
  - name: ref
    id: api:vue/ref
    signature: "const count = ref(0)"
    returns: A reactive reference object with a .value property.
    description: Wraps a primitive or object in a reactive reference; access the value via .value in script, auto-unwrapped in templates.
  - name: reactive
    id: api:vue/reactive
    signature: "const state = reactive({ count: 0 })"
    returns: A deeply reactive proxy of the original object.
    description: Creates a deeply reactive proxy that tracks all property access and mutations on the object.
  - name: computed
    id: api:vue/computed-api
    signature: "const double = computed(() => count.value * 2)"
    returns: A read-only reactive ref.
    description: Returns a cached derived value that re-evaluates only when its reactive dependencies change.
  - name: watch
    id: api:vue/watch
    signature: "watch(source, (newVal, oldVal) => { ... }, { deep: true, immediate: true })"
    returns: A stop handle function.
    description: Observes one or more reactive sources and runs a callback when they change; supports deep, immediate, and flush options.
  - name: defineComponent
    id: api:vue/definecomponent
    signature: "defineComponent({ setup() { ... } })"
    returns: A typed component options object.
    description: A type helper that provides proper TypeScript inference for component options, especially useful in Composition API mode.
  - name: defineProps
    id: api:vue/defineprops
    signature: "const props = defineProps<{ title: string }>()"
    returns: A typed props object.
    description: A compile-time macro in <script setup> that declares component props with full TypeScript validation.
  - name: defineEmits
    id: api:vue/defineemits
    signature: "const emit = defineEmits<{ (e: 'submit', id: number): void }>()"
    returns: An emit function with typed payloads.
    description: A compile-time macro in <script setup> that declares emitted events with typed payloads.
  - name: provide/inject
    id: api:vue/provide-inject-api
    signature: "provide('key', value); const value = inject('key', defaultValue)"
    returns: Injection key and value.
    description: provide sets a value on the component tree; inject retrieves it from the nearest ancestor that provided the key.
  - name: onMounted
    id: api:vue/onmounted
    signature: "onMounted(() => { ... })"
    returns: void
    description: Lifecycle hook called after the component is mounted to the DOM; used for DOM queries and third-party library initialization.
  - name: nextTick
    id: api:vue/nexttick
    signature: "await nextTick()"
    returns: A promise that resolves after the next DOM update.
    description: Waits for Vue to flush pending DOM updates; useful when you need the DOM to reflect state changes before acting.
  - name: RouterView
    id: api:vue/routerview
    signature: "<RouterView v-slot='{ Component }'><component :is='Component' /></RouterView>"
    returns: A routed component outlet.
    description: The Vue Router component that renders the matched route's component, supporting slots for transition and keep-alive wrappers.
  - name: Pinia store
    id: api:vue/pinia-store
    signature: "export const useStore = defineStore('id', () => { const count = ref(0); return { count } })"
    returns: A composable store function.
    description: Pinia's setup-store syntax that uses the Composition API to define state, getters, and actions in one function.
examples:
  - id: example:vue/composition-counter
    language: vue
    description: A counter component using ref, computed, and event handlers in <script setup>.
  - id: example:vue/pinia-todo
    language: vue
    description: A todo list with Pinia store for state management, using defineStore with setup syntax.
  - id: example:vue/router-detail
    language: vue
    description: A route detail page using useRoute params, Suspense, and router navigation guards.
  - id: example:vue/composable-mouse
    language: vue
    description: A useMouse composable with reactive coordinates shared across multiple components.
  - id: example:vue/teleport-modal
    language: vue
    description: A modal component using Teleport to render at the document body level.
failures:
  - id: failure:vue/reactivity-loss
    symptom: "Reactive changes do not trigger re-renders; the UI stays frozen despite state updates."
    cause: Destructuring a reactive object or ref loses the reactive proxy connection; Vue cannot track property access on primitives.
    fix: Keep reactive objects intact; use toRefs() to destructure without losing reactivity, or access ref.value directly.
  - id: failure:vue/watcher-leak
    symptom: "Watchers or event listeners accumulate on unmounted components, causing memory leaks and unexpected side effects."
    cause: Watchers created outside setup() or composables without cleanup on onUnmounted continue firing after component destruction.
    fix: Use watchEffect's auto-stop (it stops on unmount) or manually call the watcher's stop handle in onUnmounted.
  - id: failure:vue/deep-reactivity-overflow
    symptom: "Computed properties or watchers fire too often or cause infinite loops with deeply nested objects."
    cause: "Using deep: true on a watch or mutating nested reactive properties inside a computed creates circular reactivity chains."
    fix: Avoid deep watchers by watching specific paths; use shallowRef or shallowReactive for deeply nested read-only data.
  - id: failure:vue/async-setup
    symptom: "Template renders before async data resolves, showing empty or undefined values."
    cause: An async setup() function or async composable that data is not awaited; Vue does not wait for async setup by default.
    fix: Use Suspense to wrap components with async setup; initialize with a loading state and populate on resolve.
  - id: failure:vue/ssr-hydration-mismatch
    symptom: "Hydration warnings or flicker on page load; server and client markup differ."
    cause: Using browser-only APIs, dates, or random values during server render; the server and client produce different HTML.
    fix: "Gate browser-specific code behind onMounted; use the ClientOnly component or dynamic imports with ssr: false."
  - id: failure:vue/key-warning
    symptom: "Vue warns about missing keys in v-for; list state shifts on reorder or deletion."
    cause: Using array index as the key in v-for; index-based keys break identity when items are inserted or removed.
    fix: Use a unique identifier from the data (like id) as the key; never rely on the loop index for mutable lists.
  - id: failure:vue/v-model-mutation
    symptom: "Parent state is not updated when a child component emits with v-model."
    cause: Mutating the prop directly instead of emitting the update:modelValue event; v-model requires explicit emit.
    fix: Emit update:modelValue with the new value; use computed with get/set for writable v-model proxies in the child.
extends:
  - concept:vue/composition-api
uses:
  - concept:vue/reactivity
  - concept:vue/sfc
  - concept:vue/directives
  - concept:vue/computed
  - concept:vue/watchers
  - concept:vue/slots
  - concept:vue/teleport
  - concept:vue/suspense
  - concept:vue/ssr
  - concept:vue/composables
  - concept:vue/provide-inject
part_of: concept:domain/web-platform
depends_on:
  - package:typescript
  - package:npm
solves:
  - problem:declarative-ui
alternatives:
  - package:react/patterns
  - package:svelte
  - package:solidjs
---
# Vue.js Patterns

Vue 3 is built on a proxy-based reactivity system that tracks dependencies automatically. When you wrap a value in `ref()` or `reactive()`, Vue intercepts reads and writes to know exactly which effects, computed properties, and watchers depend on that value. This means no manual dependency arrays — the framework figures out what to re-render for you. The Composition API organizes logic by feature rather than lifecycle hook, using `setup()` or `<script setup>` as the entry point for all component logic.

The `ref()` and `reactive()` primitives are the foundation. `ref` wraps any value (including primitives) in an object with a `.value` property; templates auto-unwrap refs so you never write `.value` in the template. `reactive` takes an object and returns a deeply reactive proxy — but you cannot reassign the whole reactive object without losing reactivity. A common mistake is destructuring a reactive object, which copies the values at that instant and loses all reactive tracking. Use `toRefs()` to destructure safely, or keep the object intact and access properties directly.

Computed properties and watchers are your derived state and side effect tools. `computed()` returns a cached ref that only re-evaluates when its dependencies change. `watch()` observes one or more reactive sources and runs a callback with the old and new values. Be careful with `deep: true` — it traverses the entire object on every change and can cause performance issues on large nested structures. Also watch for watcher leaks: watchers in composables or components that are not stopped on `onUnmounted` will keep firing after the component is destroyed. `watchEffect()` auto-stops when the component unmounts, making it safer for most side effects.

Pinia is the official state manager with full TypeScript support. A store is defined via `defineStore()` and uses the Composition API inside. It replaces Vuex with a simpler model: no mutations, just state and actions. Access a store by calling its composable (`useStore()`), and Pinia deduplicates it across components. For routing, Vue Router 4 provides `<RouterView>` as the outlet, route params via `useRoute()`, and navigation guards (`beforeEach`, `beforeResolve`) for auth and data prefetching. Combine RouterView with `<Transition>` and `<KeepAlive>` for animated route transitions with preserved component state.

Server-side rendering in Vue 3 is production-ready through Nuxt or the Vue SSR package. The key rules are: avoid browser-only globals during server render, gate client-only code in `onMounted`, use `<ClientOnly>` for components that cannot render on the server, and ensure all route data is fetched before render (use `asyncData` in Nuxt or Suspense boundaries manually). Hydration mismatches happen when server and client produce different HTML — the fix is almost always gating browser-specific behavior behind lifecycle hooks.
