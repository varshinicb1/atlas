---
kind: Package
id: package:trpc
name: tRPC
version: "11"
purpose: Document tRPC patterns — end-to-end type-safe APIs without code generation, with server-side routers, React/Next.js integration, and WebSocket subscriptions.
problem_solved: Eliminates the HTTP API contract layer entirely — instead of manually defining REST endpoints or GraphQL schemas and keeping client types in sync, tRPC infers TypeScript types from server procedures and exposes them directly to the client, making the API as type-safe as calling a local function.
install: npm install @trpc/server @trpc/client
dependencies:
  - concept:typescript
  - concept:http-rpc
concepts:
  - name: Routers
    id: concept:trpc/routers
    description: "A hierarchical tree of procedures — t.router({ user: t.router({ get: t.procedure.query(...), list: t.procedure.query(...) }) }). Routers merge and nest infinitely via .merge() and sub-routers. The router type is inferred and exposed to the client."
  - name: Procedures
    id: concept:trpc/procedures
    description: The atomic unit of an API — a function exposed as .query() (read, cacheable) or .mutation() (write, side effects). .subscribe() for WebSocket-based real-time streams. Each procedure has input validation (optional), middleware pipeline, and resolver function.
  - name: Input Validation
    id: concept:trpc/input
    description: .input(zodSchema) attaches a Zod schema to a procedure. Input is validated server-side before the resolver runs. tRPC infers the TypeScript type from the Zod schema and exposes it to the client for autocomplete — no manual type sharing.
  - name: Context
    id: concept:trpc/context
    description: A per-request object created by a context factory function (async). Contains things like current user, session, database client, and request metadata. Context is available in every middleware and resolver via ctx parameter. Types flow through automatically.
  - name: Middleware
    id: concept:trpc/middleware
    description: "Reusable logic that wraps procedures — t.middleware(({ ctx, next, input }) => { ... return next({ ctx: { ... } }) }). Used for auth checks, logging, rate limiting, and request enrichment. Middleware can modify context for downstream resolvers."
  - name: Client
    id: concept:trpc/client
    description: "createTRPCClient<AppRouter>({ links: [...] }) creates a fully typed client. Every query/mutation on the client has autocomplete for inputs and typed return values. The client is just a fetch wrapper — no proxy, no codegen, no build step."
  - name: Links
    id: concept:trpc/links
    description: Composable HTTP pipeline — similar to Apollo links. httpBatchLink collects concurrent queries into a single POST (batching), httpLink sends individual requests, wsLink for WebSocket subscriptions, loggerLink for dev debugging. Custom links can intercept/modify requests.
  - name: React Query Integration
    id: concept:trpc/react-query
    description: "@trpc/react-query wraps tRPC client with React Query for caching, refetching, optimistic updates, and invalidation. Exposes hooks: trpc.user.get.useQuery(), trpc.user.create.useMutation(). Query keys are derived from the procedure path automatically."
  - name: Server-Side Rendering
    id: concept:trpc/ssr
    description: Pre-fetches queries on the server and dehydrates the cache to the client. Next.js SSR/SSG support via server helpers — trpcServer.createCaller() for direct server-side calls without HTTP, or prefetch queries during getServerSideProps/getStaticProps.
  - name: Subscriptions
    id: concept:trpc/subscriptions
    description: Real-time event streams via WebSocket or SSE. .subscribe() procedures accept an emitter and stream data to connected clients. Requires wsLink on the client and a subscription handler on the server. Use @trpc/server for EventEmitter-based pub/sub.
  - name: Authorization
    id: concept:trpc/auth
    description: Patterns for route-level and procedure-level authorization. Typically implemented as middleware that checks ctx.user.role against required permissions. Can use split routers for public vs protected routes, or guard individual procedures.
  - name: Error Handling
    id: concept:trpc/errors
    description: TRPCError with codes (UNAUTHORIZED, FORBIDDEN, NOT_FOUND, BAD_REQUEST, etc.). Client receives typed errors. Custom error formatters for logging, sanitization, and i18n. Unhandled errors become INTERNAL_SERVER_ERROR (500) by default.
  - name: File Uploads
    id: concept:trpc/uploads
    description: Multipart form data handling via tRPC's FormData support. Procedures accept File objects in input and parse them server-side. Often combined with uploadthing or app-based upload endpoints for large files.
apis:
  - name: t.procedure.query(resolver)
    id: api:trpc/query
    signature: "t.procedure.input(schema?).query(({ ctx, input }) => Promise<output>): Procedure"
    returns: A query procedure.
    description: Defines a read-only query — typically GET semantic. Results are cacheable. React Query treats these as stale-while-revalidate by default. No side effects should occur inside query resolvers.
  - name: t.procedure.mutation(resolver)
    id: api:trpc/mutation
    signature: "t.procedure.input(schema?).mutation(({ ctx, input }) => Promise<output>): Procedure"
    returns: A mutation procedure.
    description: Defines a write operation — POST/PUT/PATCH/DELETE semantic. Mutations invalidate associated queries via trpc.useUtils().invalidate().
  - name: createTRPCClient(opts)
    id: api:trpc/client
    signature: "createTRPCClient<AppRouter>(opts: { links: TRPCLink[] }): TRPCClient<AppRouter>"
    returns: A fully typed client.
    description: Creates a client linked to the server router. The generic AppRouter parameter provides full type inference — no manual client stubs. Operations available at client.procedureNamespace.query/mutation.
  - name: trpc.useQuery(input, opts)
    id: api:trpc/useQuery
    signature: "trpc.some.query.useQuery(input: Input, opts?: UseQueryOptions): UseQueryResult<Output>"
    returns: React Query result object.
    description: React hook wrapper for tRPC queries. Fully typed input/output from the server procedure. Supports all React Query options including enabled, refetchInterval, select, and initialData.
  - name: trpc.useMutation(opts)
    id: api:trpc/useMutation
    signature: "trpc.some.mutate.useMutation(opts?: UseMutationOptions): UseMutationResult<Output, Error, Input>"
    returns: React Query mutation result.
    description: React hook for mutations. Returns mutate/mutateAsync functions. onSuccess callback can invalidate queries via the utils object.
  - name: trpc.useUtils()
    id: api:trpc/useUtils
    signature: "const utils = trpc.useUtils(): TRPCReactUtils"
    returns: Utility object with invalidate, refetch, setData, getData, fetch.
    description: Returns invalidation and cache-manipulation helpers for all tRPC queries. utils.some.query.invalidate() refetches matching queries. setData for optimistic updates.
failures:
  - id: failure:trpc/cross-domain-cors
    symptom: Browser CORS errors when client and server are on different origins.
    cause: tRPC server does not set CORS headers by default — it expects a proxy (Next.js, remix, or API gateway) to handle CORS.
    fix: Run tRPC server behind a framework that handles CORS, or add CORS headers in the framework adapter. For standalone Express/Fastify adapters, configure cors middleware.
  - id: failure:trpc/batch-limit-413
    symptom: 413 Request Entity Too Large when many queries are batched.
    cause: httpBatchLink sends all concurrent queries in a single POST — large payloads exceed server body limits.
    fix: Increase body parser limit on the server (100kb default). Split large queries into separate requests. Disable batching for heavy procedures with httpLink instead of httpBatchLink.
  - id: failure:trpc/type-recursion-limit
    symptom: Type instantiation too deep when routers are deeply nested or merged.
    cause: Deeply nested routers with many merges hit TypeScript's recursion limit.
    fix: Use .flatten() on routers to reduce nesting depth. Minimize router merge chains. Use tRPC v11's flat router pattern if deeply nested.
  - id: failure:trpc/websocket-reconnect
    symptom: Subscriptions stop receiving events after network interruption.
    cause: WebSocket connection drops without automatic reconnection.
    fix: Configure wsLink with retry options. Implement reconnection logic with exponential backoff. Use EventSource/SSE as a simpler alternative for one-way streaming.
extends: []
implements: []
uses:
  - concept:typescript
  - concept:http-rpc
  - concept:react-query
part_of: concept:typescript-api-layer
solves:
  - problem:end-to-end-type-safety
  - problem:api-contract-synchronization
  - problem:full-stack-typescript-communication
alternatives:
  - package:graphql
  - package:rest-apis
  - package:grpc
  - package:openapi-codegen
---
tRPC's core innovation is removing the API contract as a separate artifact. Instead of defining routes in OpenAPI, endpoints in GraphQL, or handlers in REST, tRPC treats TypeScript as the contract language: the router's type IS the API spec. This eliminates the error-prone step of keeping client and server types in sync — when the server schema changes, TypeScript flags every client usage at compile time.

The link system is tRPC's architectural swappable layer. httpBatchLink is the default — it collects concurrent queries within a microtask and sends them as a single POST for network efficiency. httpLink sends individual requests for mutations or streaming. wsLink provides real-time subscriptions via WebSocket. Custom links can implement caching, retry, logging, or offline queueing. This composability makes tRPC adaptable to any network environment without changing application code.

For Next.js integration, the recommended pattern is to define the router in a shared package or a lib/trpc.ts file, export type AppRouter, and create the server handler in an API route (app/api/trpc/[trpc]/route.ts). The client uses the same AppRouter type. Server helpers (trpcServer.createCaller()) enable calling procedures directly in Server Components or getServerSideProps without an HTTP round trip — useful for initial page data.
