---
kind: Package
id: package:hono
name: Hono Web Framework
version: "4.6"
purpose: Document Hono — a lightweight, ultrafast web framework for Cloudflare Workers, Deno, Bun, and Node.js with built-in routing, middleware, validation, and RPC-like type-safe client generation.
problem_solved: Provides a unified, minimal web framework that runs on every JavaScript runtime (Cloudflare Workers, Deno, Bun, Node.js) with sub-millisecond routing, first-class TypeScript support, and a middleware ecosystem that replaces Express.js for the edge computing era.
install: npm install hono
dependencies:
  - concept:web-standards
  - concept:http
  - concept:typescript
concepts:
  - name: App
    id: concept:hono/app
    description: The root application — create a Hono app with new Hono(), define routes with .get(), .post(), .put(), .delete(), .patch(), .on(method, path, handler), and chain middleware via .use(). Base path, strict mode, and error handling configured at the app level.
  - name: Router
    id: concept:hono/router
    description: Hono's trie-based smart router supporting RegExp, pattern matching, and parameter extraction. Routes are static strings, named parameters (/users/:id), wildcards (/api/*), and catch-all (/files/:path*). The router is pluggable — switch to LinearRouter, TrieRouter, or SmartRouter for different performance profiles.
  - name: Context
    id: concept:hono/context
    description: c (Context) object in every handler — provides c.req (request), c.res (response), c.body(), c.json(), c.text(), c.html(), c.redirect(), c.header(), c.status(), c.set()/c.get() for middleware data passing. The context is the single argument pattern — no (req, res, next) signature.
  - name: Middleware
    id: concept:hono/middleware
    description: "Hono's middleware chain is async-first: app.use('*', async (c, next) => { const start = Date.now(); await next(); console.log(Date.now() - start) }). Built-in middleware: cors, logger, etag, cache, compress, jwt, bearer, basicAuth, serveStatic, timing. Third-party: @hono/oauth, @hono/rate-limiter, @hono/zod-openapi."
  - name: Validation
    id: concept:hono/validation
    description: Zod-based input validation via @hono/zod-validator — c.req.valid('json') returns typed data after validation against a Zod schema. Validates query, param, header, cookie, json, and form data. Invalid requests return 400 with formatted error messages automatically.
  - name: RPC
    id: concept:hono/rpc
    description: "hc (Hono Client) generates a fully typed client from the server routes — import { hc } from 'hono/client' with type inference of paths, methods, inputs, and outputs. Enables end-to-end type safety without code generation or manual client stubs, similar to tRPC but for any route definition."
  - name: OpenAPI
    id: concept:hono/openapi
    description: "@hono/zod-openapi extends Hono with OpenAPI 3.1 specification generation from route definitions. Each route declares its request/response schemas using Zod, and Hono auto-generates /openapi.json and a Swagger UI at /docs. Eliminates API docs drift."
  - name: JSX & HTML
    id: concept:hono/jsx
    description: Built-in JSX support for server-side rendering — hono/jsx enables React-style component syntax on the server without React. Supports Suspense, streaming (renderToStream), and HTML escaping. Ideal for Cloudflare Workers where you want SSR without React overhead.
  - name: WebSocket
    id: concept:hono/websocket
    description: WebSocket support via @hono/ws or platform-native WebSocket (Workers Durable Objects, Deno, Bun). Upgrade middleware handles the HTTP->WS upgrade. Messages are JSON or raw text/binary. Shared state via Durable Objects for multi-connection broadcasting.
  - name: Adapters
    id: concept:hono/adapters
    description: Platform adapters for Cloudflare Workers (fetch handler), Deno (serve), Bun (Bun.serve), Node.js (Node.js HTTP server), AWS Lambda, Vercel, Netlify, and Fastly Compute. Routes and middleware are portable across all platforms — swap adapter, keep logic.
apis:
  - name: new Hono().get(path, handler)
    id: api:hono/get
    signature: "app.get('/users/:id', (c: Context) => c.json({ id: c.req.param('id') })): Hono"
    returns: Hono instance (chainable).
    description: Registers a GET route. Path can include named params (:id), wildcards (*), and regex patterns. Handler receives context and returns Response or any value (auto-wrapped via c.json/c.text).
  - name: c.req.param(name)
    id: api:hono/param
    signature: "c.req.param(name: string): string | undefined"
    returns: The path parameter value.
    description: Extracts a named path parameter. For /users/:id, c.req.param('id') returns the value. Use c.req.param() without args to get all params as an object.
  - name: c.req.query(key)
    id: api:hono/query
    signature: "c.req.query(key: string): string | undefined"
    returns: The query string value.
    description: Extracts a query parameter. For ?name=Alice, c.req.query('name') returns 'Alice'. Use c.req.queries(key) for multiple values of the same key.
  - name: c.json(data, status?)
    id: api:hono/json
    signature: "c.json(data: unknown, status?: number, headers?: Record<string, string>): Response"
    returns: A JSON Response.
    description: "Returns a JSON response with the given data and optional status code (default 200). Sets Content-Type: application/json automatically. Supports ResponseInit for custom headers."
  - name: app.use(path, middleware)
    id: api:hono/use
    signature: "app.use('/api/*', async (c, next) => { await next() }): Hono"
    returns: Hono instance (chainable).
    description: Registers middleware for matching routes. Path pattern supports wildcards. Middleware runs in registration order. Call next() to pass to the next handler/middleware.
  - name: validator(what, schema)
    id: api:hono/validator
    signature: "import { zValidator } from '@hono/zod-validator'; app.post('/users', zValidator('json', userSchema), handler): Hono"
    returns: Middleware function.
    description: Validates request data (json, query, param, header, cookie, form) against a Zod schema. On validation failure, returns 400 with ZodError details. Validated data available via c.req.valid('json').
  - name: hc(url)
    id: api:hono/hc
    signature: "import { hc } from 'hono/client'; const client = hc<AppType>('/api'); const res = await client.users.$get()"
    returns: A typed client object.
    description: Creates a type-safe client for a Hono server. AppType is inferred from the server app type. All paths, methods, inputs, and outputs are fully typed. Supports query params, JSON body, and typed responses.
  - name: app.fire()
    id: api:hono/fire
    signature: "export default app.fire()"
    returns: CF Worker fetch handler.
    description: "For Cloudflare Workers — wraps the Hono app as a fetch event handler. Equivalent to export default { fetch: app.fetch }. The .fire() method makes it explicit."
failures:
  - id: failure:hono/cors-in-worker
    symptom: Browser CORS errors when calling Hono API from a different origin.
    cause: Hono does not set CORS headers by default. Workers need explicit CORS middleware.
    fix: "Import cors from 'hono/cors' and apply app.use('*', cors({ origin: 'https://example.com', allowMethods: ['GET', 'POST'] })). For all origins during development: cors({ origin: '*' })."
  - id: failure:hono/rpc-type-inference-fails
    symptom: hc client has 'any' types instead of inferred route types.
    cause: The AppType export does not include the full app type or routes are defined in a separate module without chaining.
    fix: Ensure the app type is exported as export type AppType = typeof app. Verify all routes are registered on the exported app instance before the export. For modular routers, use the router's type.
  - id: failure:hono/middleware-order-matters
    symptom: Validation middleware runs after the handler, or error middleware does not catch errors.
    cause: Middleware registered after routes only applies to subsequent routes. Error middleware must be registered before routes to catch their errors.
    fix: Register global middleware (cors, auth, error handling) before route definitions. Register route-specific middleware (validation) in the route chain. Error handler via app.onError() at the top.
  - id: failure:hono/worker-size-limit
    symptom: Workers deploy fails — script exceeds 1MB size limit after adding many routes.
    cause: Each route handler contributes to the Worker bundle size. Large applications with 100+ routes can exceed the limit.
    fix: Use code splitting with esbuild conditionals. Lazy-load route modules using Vite/Rollup dynamic imports. Move heavy dependencies to external modules. Use Hono's SmartRouter for optimal bundle size.
extends: []
implements: []
uses:
  - concept:web-standards
  - concept:http
part_of: concept:edge-computing-frameworks
solves:
  - problem:serverless-web-framework
  - problem:cross-platform-api-server
  - problem:edge-runtime-development
alternatives:
  - package:express
  - package:fastify
  - package:itty-router
  - package:cheerio
---
Hono was built for the edge computing era — it runs on Cloudflare Workers, Deno, Bun, and Node.js with zero dependencies and a bundle size of under 20KB. Its trie-based router delivers sub-millisecond routing even with hundreds of routes, making it the fastest web framework across all JavaScript runtimes.

The framework's standout feature is its runtime-agnostic design. Routes, middleware, and validation logic are written once and deploy anywhere. The adapter pattern maps Hono's request/response abstraction to each runtime's native API — Workers' fetch event, Deno's serve, Bun's Bun.serve, Node.js HTTP server. This portability is critical for teams that deploy on Workers for edge APIs and Bun for internal services.

Hono's RPC (Remote Procedure Call) capability is its answer to tRPC: by exporting the app's type, the hc client infers all route paths, methods, request shapes, and response types. Unlike tRPC's single router concept, Hono RPC works with any route structure — RESTful, RPC-style, or mixed. Combined with @hono/zod-openapi, you get type-safe routes, auto-generated OpenAPI specs, and a typed client from the same definitions.
