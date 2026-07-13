---
kind: Package
id: package:svelte
name: Svelte 5 + SvelteKit
version: "5.0"
purpose: Document runes, snippets, server components, and SvelteKit full-stack patterns — the complete modern Svelte development model.
problem_solved: Eliminates React's re-render model entirely by shifting from virtual DOM to a compiler-based reactivity system where UI updates are direct DOM mutations triggered by granular dependency tracking, reducing boilerplate and runtime overhead.
install: npx sv create my-app
dependencies:
  - concept:web-platform
  - concept:javascript
concepts:
  - name: Runes
    id: concept:svelte/runes
    description: Svelte 5's reactive primitives — $state, $derived, $effect, $props, $bindable — that replace the legacy let/export/ reactive model. Runes are compiler-annotated declarations that enable granular dependency tracking across component boundaries without the .$store suffix.
  - name: $state
    id: concept:svelte/state
    description: Declares reactive local state. Deeply reactive by default — mutations to nested objects and arrays are tracked automatically. Supports class instances, Maps, Sets, and getter/setter patterns via $state.frozen() for opt-out deep reactivity.
  - name: $derived
    id: concept:svelte/derived
    description: Declares a value computed from $state or other $derived declarations. Recomputes only when dependencies change. Supports $derived.by(() => ...) for multi-statement computations. Equivalent to Vue's computed or React's useMemo but with automatic dependency tracking.
  - name: $effect
    id: concept:svelte/effect
    description: Runs side effects when its tracked dependencies change. Replaces onMount, beforeUpdate, afterUpdate, and $:. Automatically tracks any $state/$derived read during execution. Supports $effect.root() for manual lifecycle, and returned cleanup functions.
  - name: $props
    id: concept:svelte/props
    description: Declares component props via destructuring. Supports default values, rest spreads, and TypeScript typing. Replaces the legacy export let and $$props patterns. Props are read-only in the child by convention (mutate via callback or $bindable).
  - name: Snippets
    id: concept:svelte/snippets
    description: "Reusable template fragments defined with {#snippet name()}...{/snippet} and invoked with {@render name()}. Replace slot syntax for content projection. Can receive parameters and be passed to other components as props — equivalent to React's render props pattern."
  - name: Event Handling
    id: concept:svelte/events
    description: "Inline event handlers with on:click={handler} syntax. Svelte 5 replaces event modifiers (on:click|preventDefault) with wrapper functions (preventDefault(handler)). Supports capturing, passive, and once options via on:click|nonpassive. Custom events use createEventDispatcher or callback props."
  - name: Transition System
    id: concept:svelte/transitions
    description: "Built-in transition directives (transition:fly, transition:fade, transition:scale) with in/out variants. Supports custom transition functions returning duration, delay, easing, and css. Svelte 5 adds {#key} blocks for animating enter/exit of changing content."
  - name: Stores
    id: concept:svelte/stores
    description: Legacy but maintained — writable, readable, derived stores with $ prefix auto-subscription. Still used for cross-component state that doesn't fit the runes model. Convert to runes via get() in $effect or pass store values as $state at the boundary.
  - name: SvelteKit Routing
    id: concept:sveltekit/routing
    description: "File-based router in src/routes/ using +page.svelte, +layout.svelte, +server.ts, +page.server.ts, +error.svelte. Supports dynamic params [slug], rest params [...path], groups (marketing), and optional params [[optional]]. Layouts nest and persist across page transitions."
  - name: SvelteKit Server Load
    id: concept:sveltekit/load
    description: Server-side data loading via exported load function in +page.server.ts or +layout.server.ts. Returns serializable data to the page component. Supports parent data access, URL dependency tracking, and streaming with defer() for progressive rendering.
  - name: SvelteKit Actions
    id: concept:sveltekit/actions
    description: Form actions in +page.server.ts — named exports (default, login, register) that handle POST requests with built-in validation, error returns, and progressive enhancement via use:enhance for client-side pending states without JavaScript.
  - name: SvelteKit Hooks
    id: concept:sveltekit/hooks
    description: Server hooks (src/hooks.server.ts) and client hooks (src/hooks.client.ts) for intercepting requests — handle, handleFetch, handleError. Used for auth guards, session injection, API proxying, and global error handling.
  - name: SvelteKit Adapters
    id: concept:sveltekit/adapters
    description: Platform-specific output via adapters (@sveltejs/adapter-vercel, adapter-netlify, adapter-node, adapter-static, adapter-cloudflare). Zero-config for most platforms; adapter-cloudflare requires cf fetch binding for D1/R2 access.
apis:
  - name: $state(initial)
    id: api:svelte/state
    signature: "let count = $state(initial: T): T"
    returns: A reactive variable.
    description: Declares reactive state. Deeply reactive — mutating objects/arrays triggers updates. Use $state.frozen(obj) for shallow-reactive immutable objects.
  - name: $derived(expression)
    id: api:svelte/derived
    signature: "let doubled = $derived(expression: T): T"
    returns: A computed value.
    description: Automatically tracks dependencies read in expression and recomputes when any of them change. Dependencies must be synchronous and non-conditional.
  - name: $effect(fn)
    id: api:svelte/effect
    signature: "$effect(fn: () => (void | (() => void))): void"
    returns: undefined
    description: Runs fn immediately and re-runs when tracked dependencies change. Return a cleanup function from fn to teardown subscriptions/timers.
  - name: $props()
    id: api:svelte/props
    signature: "let { prop1, prop2 = default } = $props(): ComponentProps"
    returns: A destructured object of props.
    description: Declares component props with optional defaults. Use TypeScript interface for type safety. Do not destructure outside the top-level script.
  - name: $bindable()
    id: api:svelte/bindable
    signature: "let { value = $bindable() } = $props(): T"
    returns: A bindable prop.
    description: Marks a prop as bindable — parent can use bind:value for two-way binding. Requires a default value or undefined initializer.
  - name: load(event)
    id: api:sveltekit/load
    signature: "export async function load(event: ServerLoadEvent): Promise<{ [key: string]: unknown }>"
    returns: A plain object merged with component props.
    description: Server data loading function. event contains params, url, request, cookies, fetch, locals, parent(), depends(), and platform. Runs before the page renders.
  - name: form action
    id: api:sveltekit/action
    signature: "export const actions = { default: async (event: RequestEvent) => { ... } }"
    returns: Action result object with data or errors.
    description: "Handles form POST submissions. Return { success: true } on success or { errors: {...} } with field-level validation. Accessible via use:enhance for progressive enhancement."
  - name: goto(url, options)
    id: api:sveltekit/goto
    signature: "import { goto } from '$app/navigation'; goto(url: string, opts?: { replaceState, noScroll, keepFocus, invalidateAll }): Promise<void>"
    returns: Promise resolving after navigation.
    description: Programmatic client-side navigation. Replaces window.location for SPA transitions. Keeps layout state and runs load functions for the target route.
  - name: invalidate(dep)
    id: api:sveltekit/invalidate
    signature: "import { invalidate } from '$app/navigation'; invalidate(dep: string): Promise<void>"
    returns: Promise resolving after re-run.
    description: Re-runs any load functions that depend on the given URL or depends() identifier. Used for cache invalidation after mutations.
  - name: enhance
    id: api:sveltekit/enhance
    signature: "use:enhance={callback?: (opts: { form, data, action, result, update, cancel }) => void}"
    returns: An action for use:enhance.
    description: Progressive enhancement for SvelteKit forms. Submits via fetch with no page reload. Optional callback for pending/error/result UI states.
failures:
  - id: failure:svelte/missing-reactive-declaration
    symptom: Variable updates do not trigger UI re-render.
    cause: Using let x = value instead of $state(value). Svelte 5 requires runes for reactivity — plain variables are not reactive outside .svelte files.
    fix: Replace with $state() for local state or $derived() for computed values. Ensure you are using the runes mode (Svelte 5).
  - id: failure:sveltekit/server-fetch-relative
    symptom: Server-side fetch() to internal API returns an error or empty response.
    cause: Using relative URLs in load functions. Server-side fetch must use full URLs or rely on the request's origin.
    fix: Use event.fetch() instead of global fetch — it inherits cookies and origin from the page request. For external APIs, pass the full URL.
  - id: failure:sveltekit/hydration-mismatch
    symptom: DOM mismatch warnings after page load; UI flicker.
    cause: Server and client render different HTML — typically due to browser-only data, random values, or store access in component root.
    fix: "Use onMount or $effect for client-only logic. Wrap browser-dependent content in {#browser}...{/browser} blocks. Use untrack() to avoid reactive side effects during SSR."
  - id: failure:svelte/memory-leak-in-effects
    symptom: Subscriptions stacking on every re-render; stale timers continue firing.
    cause: $effect without cleanup function for subscriptions, interval, or event listeners.
    fix: Return a cleanup function from $effect that tears down the subscription, clears the interval, or removes the event listener. Use $effect.root() for imperative lifecycle management.
  - id: failure:sveltekit/prerender-404
    symptom: Dynamic routes return 404 in static prerender mode.
    cause: Prerender crawler cannot discover all route variants unless they are linked or specified in entries.
    fix: "Export prerender: true with an entries array in +page.server.ts or configure the prerender crawl in svelte.config.js."
extends: []
implements: []
uses:
  - concept:web-platform
part_of: concept:modern-web-frameworks
solves:
  - problem:reactive-ui-development
  - problem:full-stack-typescript-web-apps
alternatives:
  - package:typescript_nextjs
  - package:vue_patterns
  - package:react_patterns
  - package:astro_js
---
Svelte 5's runes model is a fundamental shift from Svelte 4's compiler-magic reactivity. Instead of implicitly reactive let declarations, runes are explicit signals: $state, $derived, $effect, $props. This makes reactivity predictable across component boundaries, enables TypeScript type inference on reactive values, and allows reactive declarations outside .svelte files (in .js/.ts via .svelte.js extension).

The most impactful architectural decision in SvelteKit is the division between +page.svelte (UI) and +page.server.ts (data). Load functions run on the server and can access databases, cookies, and secrets directly. The page component receives data as $props() — no fetch waterfalls, no client-side data fetching boilerplate. Form actions extend this model to mutations: a <form> POST automatically calls the server action, and SvelteKit updates the page data without a full reload or manual cache invalidation.

Svelte 5's snippet-based content projection ({#snippet} / {@render}) replaces slots with a more flexible, typed, and composable pattern. Snippets can be passed as props, rendered multiple times, or conditionally displayed — unlike slots which were fixed to a single outlet per name. Combined with $state for UI state and $derived for computed values, Svelte 5 achieves React's component model expressiveness with zero runtime overhead.
