---
kind: Package
id: package:remix
name: Remix
version: "2.14"
purpose: Document Remix — React full-stack framework with nested routes, server-side data loading (loader), mutations (action), progressive enhancement, and Web Fetch API standards.
problem_solved: Tames the React SPA complexity by moving data loading and mutations to the server with loaders and actions, giving developers the UX of SPAs (instant navigation, optimistic UI) with the simplicity of server-rendered HTML (simpler state management, no client-side cache libraries, graceful JS-failure degradation).
install: npx create-remix@latest
dependencies:
  - concept:react
  - concept:web-standards
  - concept:http
concepts:
  - name: Loaders
    id: concept:remix/loaders
    description: "Server-only data loading — export async function loader({ params, request, context }) from a route module. Runs on the server during GET requests. Returns data that the page component receives via useLoaderData(). Loaders for parent routes run before child routes — nested data loading is automatic."
  - name: Actions
    id: concept:remix/actions
    description: "Server-only data mutation — export async function action({ params, request, context }) from a route module. Runs on the server during POST/PUT/PATCH/DELETE requests. Form submissions call the nearest action. Actions return data or redirect. After action, all loaders on the page revalidate automatically."
  - name: Nested Routes
    id: concept:remix/nested-routes
    description: Routes defined by the file system — app/routes/ with layout nesting. A parent route's layout component renders <Outlet /> for child routes. Nested routes create parallel data loading — parent and child loaders run in parallel, not waterfall.
  - name: Form Handling
    id: concept:remix/forms
    description: Native HTML <Form method="post"> calls the route's action without client-side JavaScript. With JS enabled, Remix intercepts the form submission (fetch + FormData), and the action handler receives standard FormData. Errors returned from actions are available via useActionData().
  - name: Progressive Enhancement
    id: concept:remix/progressive-enhancement
    description: All Remix features work without JavaScript. Forms submit natively, links navigate via <a>, loaders run server-side. With JavaScript, <Form> submits via fetch, <Link> prefetches data, and navigation is SPA-style. The same code works in both modes.
  - name: Revalidation
    id: concept:remix/revalidation
    description: After an action runs, Remix automatically re-runs all loaders on the current page. This means the UI is always in sync with the server — no manual cache invalidation, no stale state, no need for React Query or SWR for page-level data.
  - name: Error Boundaries
    id: concept:remix/error-boundaries
    description: "Route-level ErrorBoundary export catches rendering errors. export function ErrorBoundary({ error }) renders a fallback UI. Nested routes boundaries are inherited — an error in a child bubbles up to the nearest parent ErrorBoundary, keeping sibling routes intact."
  - name: Session Management
    id: concept:remix/session
    description: Server-side session storage via createSessionStorage() — stores session data in cookies, memory, or any database adapter (Prisma, Redis, SQLite). Remix sessions are encrypted cookies by default. Flash messages for one-time notifications.
  - name: Resource Routes
    id: concept:remix/resource-routes
    description: Routes that do not render UI — they return data (JSON, CSV, PDF, images). Create a route file that exports loader (GET) and optionally action (POST). Used for API endpoints, file downloads, webhooks, and sitemaps.
  - name: Headers & SEO
    id: concept:remix/headers
    description: "export function headers({ loaderHeaders, parentHeaders }) sets HTTP headers (Cache-Control, Content-Security-Policy). export function meta({ data, params }) returns HTML meta tags, title, and links. Parent route meta merges with child meta for SEO composition."
apis:
  - name: loader(args)
    id: api:remix/loader
    signature: "export async function loader({ params, request, context }: LoaderFunctionArgs): Promise<json>"
    returns: Response or serializable data (auto-wrapped in json()).
    description: Server data loading function. params are route params, request is standard Request. Use request.url for query params. Return json(data) or a Response (redirect, plain text). Use defer() for streaming non-critical data.
  - name: useLoaderData()
    id: api:remix/useLoaderData
    signature: "const data = useLoaderData<typeof loader>()"
    returns: Typed loader return value.
    description: Returns the data returned by the route's loader. TypeScript type is inferred from the loader's return type. Must be called in the same route module as the loader. Returns undefined if no parent loader boundary exists.
  - name: Form / useSubmit
    id: api:remix/form
    signature: "<Form method='post' action='/todos/create'><input name='title' /><button type='submit'>Create</button></Form>"
    returns: Enhanced HTML form element.
    description: "Remix <Form> is an HTML form that optionally enhances with JavaScript. useSubmit(formRef, options) for imperative form submission. useNavigation() gives pending form state (state: 'submitting' | 'loading')."
  - name: redirect(path, init?)
    id: api:remix/redirect
    signature: "return redirect('/dashboard', { headers: { 'Set-Cookie': sessionCookie } })"
    returns: A 302 Response.
    description: Redirects to another route. Used in loaders (after auth check) and actions (after successful mutation). Supports custom status codes (301, 307, 308) and response init for cookies and headers.
  - name: json(data, init?)
    id: api:remix/json
    signature: "return json({ users }, { headers: { 'Cache-Control': 'max-age=60' } })"
    returns: A JSON Response with helpers.
    description: Returns serialized JSON with optional response headers and status. Remix's json() wraps Response.json() with enhanced type inference for useLoaderData. Use for loader return values.
  - name: defer(data, init?)
    id: api:remix/defer
    signature: "return defer({ user: await getUser(), posts: getPosts() })  // posts is a promise"
    returns: A deferred Response.
    description: Streams data to the client — critical data is awaited, non-critical data is streamed as a promise. Use <Suspense> with <Await> on the client to render streaming content. Reduces time-to-first-byte for pages with expensive secondary data.
  - name: useFetcher()
    id: api:remix/useFetcher
    signature: "const fetcher = useFetcher(); fetcher.submit(formRef); fetcher.load('/api/data')"
    returns: A fetcher instance with state, data, and submit/load functions.
    description: Imperative data loading and mutation outside of navigation — for typeahead autocomplete, inline updates, and actions triggered by user interaction without URL change. Does not cause navigation. Multiple fetchers are isolated.
failures:
  - id: failure:remix/loader-throws-not-redirect
    symptom: Unauthenticated users see an empty page or error boundary instead of being redirected to login.
    cause: Using throw new Response() or throwing inside loader instead of returning redirect().
    fix: Return redirect('/login') from loaders when the user is not authenticated. Do not throw — Remix's redirect() returns a Response that the router handles as navigation.
  - id: failure:remix/action-revalidation-order
    symptom: Parent loader data is stale after child route action runs.
    cause: Actions on child routes only revalidate their own loaders and ancestor loaders — sibling route loaders do not re-run.
    fix: Move shared data loading to a parent layout so it revalidates when any child action runs. Use useFetcher for cross-route invalidation. Return updated data from the action.
  - id: failure:remix/js-disabled-broken
    symptom: App breaks when JavaScript is disabled — forms don't submit or links navigate away.
    cause: Using onClick handlers or client-only navigation libraries instead of Remix's <Form> and <Link>.
    fix: Always use Remix's <Form> for mutations and <Link> for navigation — these work with and without JavaScript. Avoid client-only event handlers for navigation. Test with JS disabled.
  - id: failure:remix/server-code-in-client
    symptom: Bundled code contains database connections or secret tokens.
    cause: Importing server-only code (db.client, secrets) in a component that also runs on the client.
    best practice: Keep server-only code inside loaders and actions. Use .server.ts file extension for server-only modules — Remix strips them from client bundles. Never import server files from component files.
extends: []
implements: []
uses:
  - concept:react
  - concept:web-standards
part_of: concept:react-framework-ecosystem
solves:
  - problem:full-stack-react-state-management
  - problem:form-handling-and-validation
  - problem:progressive-enhancement
alternatives:
  - package:typescript_nextjs
  - package:svelte
  - package:nuxt
  - package:astro_js
---
Remix's architectural insight is that the Web's native model (HTML forms + server responses) already handles most state management concerns. Instead of adding client-side state libraries and cache layers, Remix embraces the server: loaders fetch data on the server, forms submit to actions on the server, and the UI is always in sync because revalidation happens automatically after every action.

The nested routing system is Remix's most distinctive feature. A URL like /dashboard/settings uses two route files: app/routes/dashboard.tsx (layout with <Outlet />) and app/routes/dashboard.settings.tsx (content). Both loaders run in parallel, not waterfall — Remix discovers all matching route loaders before fetching. When an action runs in the settings route, only the settings and dashboard loaders revalidate, leaving sibling routes (e.g., /dashboard/profile) untouched.

Progressive enhancement is not an afterthought — it's the core design. A <Form method="post"> in Remix works without JavaScript by sending a native HTML form submission. With JavaScript, Remix intercepts it, sends a fetch request, updates the page in-place, and manages focus and scroll. The same component works in both modes, so the app degrades gracefully when JS fails to load or is disabled.
