---
kind: Package
id: package:api-design
name: API Design Patterns
version: "1.0"
purpose: Document proven API design patterns — RESTful, RPC, GraphQL, and WebSocket — with authentication, error handling, pagination, rate limiting, versioning, and documentation strategies for production APIs.
problem_solved: Provides decision-ready API patterns that handle the common concerns every API needs — consistent error responses, type-safe inputs/outputs, auth middleware, rate limiting, pagination, and documentation — so developers don't reinvent these patterns on every project.
install: atlas solve api-design.atlas "design a REST API for a todo app"
dependencies:
  - concept:http
  - concept:api-design
concepts:
  - name: RESTful Design
    id: concept:api-design/rest
    description: Resource-oriented API design — nouns for resources (/users, /posts), HTTP methods for operations (GET read, POST create, PUT/PATCH update, DELETE remove), status codes for outcomes (200 success, 201 created, 204 no content, 400 bad request, 401 unauthorized, 404 not found, 500 error). HATEOAS links for discoverability.
  - name: RPC Design
    id: concept:api-design/rpc
    description: Action-oriented API design — procedure names as endpoints (/createUser, /chargePayment). tRPC and GraphQL follow RPC patterns. Simpler than REST for complex operations. Best for internal services and BFF (Backend For Frontend) patterns.
  - name: GraphQL Design
    id: concept:api-design/graphql
    description: Query-oriented API design — single endpoint with declarative data fetching. Schema-first with type definitions. Resolvers for each field. DataLoader for N+1 prevention. Subscriptions for real-time. Best for complex data graphs and multi-client scenarios.
  - name: Error Response Format
    id: concept:api-design/errors
    description: "RFC 9457 Problem Details — standard error format: { type, title, status, detail, instance }. Consistent error structure across all endpoints. Machine-readable type for programmatic handling, human-readable detail for debugging. Stack traces in development only."
  - name: Pagination
    id: concept:api-design/pagination
    description: Cursor-based pagination (recommended for most APIs) — cursor, take, orderBy. Offset-based for simple/admin UIs — offset, limit, total. Keyset pagination for real-time feeds. Include next/prev cursors in response for infinite scroll.
  - name: Rate Limiting
    id: concept:api-design/rate-limiting
    description: "Token bucket (burst allowance + sustained rate), sliding window (smooth rate calculation), or fixed window (simple, resets at boundary). Return headers: X-RateLimit-Limit, X-RateLimit-Remaining, X-RateLimit-Reset. 429 status with Retry-After header. Implement via Arcjet, Upstash, or custom middleware."
  - name: API Versioning
    id: concept:api-design/versioning
    description: "URL prefix (/v1/, /v2/) — simplest, most visible. Accept header (Accept: application/vnd.api+json;version=2) — clean URLs, harder to debug. No versioning (evolve in place with backward compatibility) — for internal/private APIs with fast iteration."
  - name: Authentication
    id: concept:api-design/auth
    description: Bearer tokens in Authorization header — JWT (stateless, self-contained) or opaque session tokens (revocable, store in DB). OAuth 2.0 for third-party access. API keys for server-to-server. Webhook signatures for event delivery verification.
  - name: Response Envelope
    id: concept:api-design/envelope
    description: "Consistent response structure — JSON:API spec ({ data, included, meta }), standard list wrapper ({ items: [], total, page }), or minimal envelope ({ success, data, error }). Choose one pattern and apply to every endpoint. Avoid mixing envelope styles across the API."
  - name: Input Validation
    id: concept:api-design/validation
    description: Validate at the API boundary — Zod schemas for request body, query params, and path params. Return 422 Unprocessable Entity with field-level errors. Never trust client input. Validate types, ranges, formats, and business rules before processing.
  - name: API Documentation
    id: concept:api-design/documentation
    description: OpenAPI 3.1 spec generated from code (zod-to-openapi, @hono/zod-openapi, @nestjs/swagger). Interactive docs via Swagger UI or Scalar. Type-safe clients generated from spec (openapi-typescript, orval, kubb). Contract testing with Specmatic or Dredd.
  - name: WebSocket Design
    id: concept:api-design/websocket
    description: "Bidirectional real-time communication — use for collaborative editing, live chat, real-time dashboards, gaming. Message format: { type, payload, id }. Connection management via Durable Objects (Workers) or Socket.io (Node). Heartbeat/ping-pong for connection health. Reconnection with exponential backoff."
apis:
  - name: GET /api/v1/resources
    id: api:api-design/list
    signature: "GET /api/v1/resources?cursor=abc&take=20&orderBy=createdAt:desc"
    returns: '{ "items": Resource[], "nextCursor": string | null, "total": number }'
    description: List endpoint with cursor-based pagination. Filters via query params. Sort via orderBy. Include related resource IDs for client-side data composition.
  - name: POST /api/v1/resources
    id: api:api-design/create
    signature: "POST /api/v1/resources body: { name: string, ... }"
    returns: '{ "data": Resource }'
    description: Create endpoint. Accept JSON body. Return created resource with 201 status. Include Location header with resource URL. Validate input against Zod schema before processing.
  - name: GET /api/v1/resources/:id
    id: api:api-design/get
    signature: "GET /api/v1/resources/:id"
    returns: '{ "data": Resource }'
    description: Read endpoint. Return 404 if not found. Support sparse fieldsets (?fields=id,name) and includes (?include=relatedResource). Use findUniqueOrThrow for clean error handling.
  - name: PATCH /api/v1/resources/:id
    id: api:api-design/update
    signature: "PATCH /api/v1/resources/:id body: { name?: string, ... }"
    returns: '{ "data": Resource }'
    description: Partial update endpoint. Accept partial JSON body. Return updated resource. PATCH (not PUT) for partial updates. Validate only provided fields. Return 404 if resource not found.
  - name: DELETE /api/v1/resources/:id
    id: api:api-design/delete
    signature: "DELETE /api/v1/resources/:id"
    returns: '204 No Content'
    description: Delete endpoint. Return 204 No Content on success. Return 404 if not found. Consider soft-delete for data recovery. Cascade or restrict based on relation constraints.
  - name: Error Response
    id: api:api-design/error-response
    signature: "HTTP 422 { type: 'validation_error', title: 'Validation Failed', status: 422, detail: 'Name is required', errors: { name: ['Name is required'] } }"
    returns: RFC 9457 problem detail.
    description: Standard error response for all error cases. type for machine-readable category, title for human category, status for HTTP code, detail for human explanation, errors for field-level validation messages.
failures:
  - id: failure:api-design/n-plus-one
    symptom: API responses are slow because each resource triggers N additional database queries.
    cause: Serializers/resolvers that fetch related data one-by-one instead of batching.
    fix: Use DataLoader for GraphQL. Use Prisma include/select for REST. Batch relation fetches. Profile response times with middleware logging.
  - id: failure:api-design/breaking-changes
    symptom: Mobile app crashes after backend deployment without app update.
    cause: Backward-incompatible API change (renamed field, removed endpoint, changed type) deployed without versioning.
    fix: Never remove or rename fields — add new fields and deprecate old ones. Use API versioning for breaking changes. Deprecation headers (Sunset, Deprecation) warn clients. Maintain at least one version back.
  - id: failure:api-design/no-rate-limiting
    symptom: API goes down under burst traffic or is abused by a single client.
    cause: No rate limiting implemented — all clients consume unlimited resources.
    fix: Implement rate limiting at the API gateway or middleware level. Use Arcjet, Upstash Ratelimit, or Cloudflare Rate Limiting. Set sensible defaults (100 req/min for most endpoints). Return 429 with Retry-After.
  - id: failure:api-design/inconsistent-errors
    symptom: Clients need to parse different error formats for different endpoints.
    cause: Each endpoint returns errors in a custom format — some string, some object, some array.
    fix: "Adopt RFC 9457 Problem Details across ALL endpoints. Create a shared error formatter middleware. Return consistent structure: { type, title, status, detail, errors }. Document error codes in API docs."
extends: []
implements: []
uses:
  - concept:http
  - concept:api-design
part_of: concept:software-architecture
solves:
  - problem:api-design-consistency
  - problem:production-api-best-practices
  - problem:api-versioning-and-evolution
alternatives:
  - package:rest-apis-manual
  - package:graphql
  - package:trpc
---
Production API design is about consistency and predictability, not creativity. Every endpoint should follow the same patterns for input validation, error responses, pagination, authentication, and rate limiting. The envelope pattern, error format, and pagination strategy should be decided at the start and applied uniformly — mixing styles confuses clients and complicates code generation.

The most impactful decision for API quality is the validation boundary. Zod schemas at the API entry point catch malformed input before it reaches business logic, reducing error-handling code in every service function. Combined with TypeScript's inference (z.infer), the validated input is automatically typed throughout the request lifecycle — no manual type assertions or runtime checks scattered across the codebase.

For long-lived APIs, versioning strategy determines maintenance cost. URL-based versioning (/v1/, /v2/) is the clearest for clients but creates code duplication. Header-based versioning keeps URLs clean but is invisible in logs and curl commands. The pragmatic approach: evolve APIs with backward-compatible changes (add fields, don't remove), use URL versioning only for breaking changes, and communicate deprecation via Sunset headers months in advance.
