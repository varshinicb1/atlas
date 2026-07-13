---
kind: Package
id: package:nuxt
name: Nuxt 3
version: "3.14"
purpose: Document Nuxt 3 — Vue.js full-stack framework with file-based routing, auto-imports, server engine (Nitro), hybrid rendering, and module ecosystem.
problem_solved: Provides a batteries-included Vue framework that handles routing, SSR/SSG/ISR rendering, server API routes, auto-imports, environment config, and deployment — abstracting the complexity of configuring Vite, Vue Router, and Nitro separately.
install: npx nuxi@latest init my-app
dependencies:
  - concept:vue
  - concept:vite
  - concept:nodejs
concepts:
  - name: File-based Routing
    id: concept:nuxt/routing
    description: "pages/ directory — each .vue file maps to a route (pages/about.vue -> /about). Dynamic params via [id].vue, catch-all via [...slug].vue. Nested routes via parent directories. Layouts via layouts/ with default.vue as the base layout."
  - name: Auto-imports
    id: concept:nuxt/auto-imports
    description: Nuxt auto-imports Vue composables (ref, computed, onMounted), Nuxt composables (useFetch, useState, useRouter), and your own composables from composables/. No import statements needed. Components from components/ are also auto-imported.
  - name: Data Fetching
    id: concept:nuxt/data-fetching
    description: "useFetch() and useAsyncData() for server-safe data fetching. useFetch wraps $fetch with SSR awareness — deduplicates requests, awaits on the server, and hydrates to the client. Options: lazy, server, default, transform, pick. useAsyncData for custom fetcher functions."
  - name: Nitro Engine
    id: concept:nuxt/nitro
    description: Nuxt's server engine — handles server/ directory API routes, middleware, and server-only code. Supports hot module replacement, hybrid rendering (SSR + SSG + ISR per route), and deployment to Node, serverless, Workers, and edge.
  - name: Modules
    id: concept:nuxt/modules
    description: "Nuxt module ecosystem — @nuxt/content for markdown CMS, @nuxt/image for optimized images, @nuxtjs/tailwindcss for Tailwind, @pinia/nuxt for state management, @nuxtjs/i18n for internationalization. Install via npm and add to modules[] in nuxt.config."
  - name: Server Routes
    id: concept:nuxt/server-routes
    description: server/api/ directory — files export event handler functions (defineEventHandler) that become API endpoints. Auto-registered as /api/* routes. Supports params, query, body, cookies, and session. Nitro compiles to platform-native handlers.
  - name: State Management
    id: concept:nuxt/state
    description: useState() for universal reactive state (shared between server and client). Pinia for complex state (persisted, devtools, modules). useState is SSR-safe — state set on server hydrates to client without extra configuration.
  - name: Middleware
    id: concept:nuxt/middleware
    description: middleware/ directory — route guards that run before page renders. Global (middleware/*.ts runs on every route) or named (used in page definePageMeta). Redirect unauthenticated users, set route metadata, or fetch pre-route data.
  - name: Rendering Modes
    id: concept:nuxt/rendering
    description: "Configure per-route rendering in nuxt.config — ssr: true (universal), prerender (SSG), swr (ISR with TTL), hybrid (mix of modes). Route rules in routeRules: { '/blog/*': { swr: 3600 }, '/admin/*': { ssr: false } }."
  - name: Layers
    id: concept:nuxt/layers
    description: Compose Nuxt applications from layers — reusable Nuxt projects that extend a base app. Layers contribute pages, components, composables, server routes, and config. Used for theme inheritance, multi-brand apps, and shared admin panels across projects.
apis:
  - name: useFetch(url, opts)
    id: api:nuxt/useFetch
    signature: "const { data, pending, error, refresh } = await useFetch('/api/users', { query: { role: 'admin' }, server: true, lazy: false })"
    returns: "{ data, pending, error, refresh, execute }"
    description: "SSR-safe data fetching — fetches on server, serializes, and hydrates to client. Deduplicates identical requests. lazy: true delays fetch until client-side. refresh() re-executes. Interceptors via onRequest, onResponse, onRequestError."
  - name: useAsyncData(key, fn, opts)
    id: api:nuxt/useAsyncData
    signature: "const { data } = await useAsyncData('users', () => $fetch('/api/users'), { watch: [someRef] })"
    returns: "{ data, pending, error, refresh }"
    description: Generic SSR-safe async data wrapper. Use when $fetch or custom async logic is needed. The key deduplicates requests. watch array re-fetches when reactive sources change.
  - name: useState(name, init)
    id: api:nuxt/useState
    signature: "const count = useState('counter', () => 0): Ref<number>"
    returns: A global reactive ref.
    description: Universal shared state — same value on server and client. Keyed by name. Initializer runs only once. Auto-serialized between server and client. Use for user preferences, UI state, and cached data.
  - name: navigateTo(path, opts)
    id: api:nuxt/navigateTo
    signature: "await navigateTo('/dashboard', { external: false, replace: false, redirectCode: 302 })"
    returns: void (navigation side effect).
    description: "Programmatic client-side navigation. Works in both components and middleware. external: true for external URLs. replace for history stack replacement. Can be returned from middleware for redirect."
  - name: definePageMeta(meta)
    id: api:nuxt/definePageMeta
    signature: "definePageMeta({ layout: 'auth', middleware: 'auth', title: 'Dashboard', keepalive: true })"
    returns: void (compile-time meta).
    description: Defines per-page metadata — layout, middleware, title, description, keepalive, key, and custom meta. The meta is extracted at compile time by Nuxt's static analysis. Route validation via validate function.
  - name: defineEventHandler(handler)
    id: api:nuxt/defineEventHandler
    signature: "export default defineEventHandler(async (event) => { const body = await readBody(event); return { success: true } })"
    returns: An event handler.
    description: Creates a Nitro server route handler. event contains node.req/res, URL, params, query, body, cookies, and headers. Return value is automatically JSON-serialized. Supports async/await.
failures:
  - id: failure:nuxt/hydration-mismatch
    symptom: Console warnings about DOM mismatch after page load.
    cause: Server and client render different HTML — typically due to browser-only data, random values, or v-if conditions based on window.
    fix: "Use ClientOnly component for browser-only content. Wrap in process.client checks for development-only code. Use lazy: true in useFetch to delay client-specific data loading."
  - id: failure:nuxt/composable-not-found
    symptom: useSomeComposable throws "is not defined" even though the composable exists.
    cause: Composables must be in composables/ directory and export a function starting with "use". Nuxt auto-imports based on file naming conventions.
    fix: Move the composable to composables/ directory. Ensure the export is named useXxx. Restart the dev server. Check the Nuxt auto-imports tab in Nuxt DevTools.
  - id: failure:nuxt/ssr-window-error
    symptom: "Build or server error: \"window is not defined\" or \"document is not defined\"."
    cause: Third-party library or code references browser globals during server-side rendering.
    fix: "Wrap browser-only code in process.client check. Use ClientOnly component. Dynamic import the library with { ssr: false }. Set the library as client-only in nuxt.config build.transpile."
  - id: failure:nuxt/module-conflict
    symptom: Build errors after installing multiple modules that modify the same Nuxt behavior.
    cause: Modules conflict when they override the same hooks, middleware, or Vite config (e.g., two CSS modules setting global styles).
    fix: Check module documentation for compatibility. Disable conflicting features in one module. Use Nuxt layers for clear separation. Check Nuxt DevTools modules tab for active hooks.
extends: []
implements: []
uses:
  - concept:vue
  - concept:vite
part_of: concept:vue-ecosystem
solves:
  - problem:full-stack-vue-development
  - problem:vue-app-structure-and-configuration
  - problem:vue-ssr-and-seo
alternatives:
  - package:vue_patterns
  - package:typescript_nextjs
  - package:svelte
  - package:astro_js
---
Nuxt 3 is a full-stack framework that reimagines the Vue development experience. Its file-based routing, auto-imports, and Nitro server engine eliminate the boilerplate of configuring Vite, Vue Router, Pinia, and a server separately. The framework convention over configuration model means a new project is productive in minutes.

The Nitro server engine is Nuxt's architectural backbone. The server/ directory handles all backend concerns — API routes, middleware, server-only code, and database access — compiled to a platform-native handler (Node.js HTTP server, Cloudflare Worker, Netlify function, Vercel serverless, or Bun). Route rules in nuxt.config control rendering per-route: SSR for dynamic content, prerender for static pages, SWR for ISR with configurable TTL, and CSR for app-like sections.

Layers are Nuxt's solution to code reuse at the framework level. A Nuxt layer is a complete Nuxt project that can contribute pages, components, composables, server routes, and config to a parent app. This enables multi-brand architectures (one base layer per brand), enterprise component libraries (UI layer installed via npm), and theme inheritance without forking or copy-pasting.
