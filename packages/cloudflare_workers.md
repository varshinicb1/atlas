---
kind: Package
id: package:cloudflare/workers
name: Cloudflare Workers Patterns
version: "3.0"
purpose: Document Cloudflare Workers patterns — Workers/Pages, KV/D1/R2/Durable Objects, bindings, wrangler, and platform limits.
problem_solved: Provides a reference for building serverless, edge-deployed applications on Cloudflare's global network, reducing cold-start issues, binding misconfig, and platform-limit surprises.
install: npm install wrangler --save-dev
dependencies:
  - package:wrangler
concepts:
  - name: Edge Computing
    id: concept:domain/edge-computing
    description: The broader edge-computing model of running code close to users across a global network, which Cloudflare Workers embody.
  - name: Workers Runtime
    id: concept:cf/workers-runtime
    description: A V8-isolate-based serverless runtime deploying code to 300+ edge locations with near-zero cold starts and no per-request VMs.
  - name: fetch Event Handler
    id: concept:cf/fetch-handler
    description: The entry point exporting a default fetch(request, env, ctx) function that returns a Response for incoming HTTP requests.
  - name: Bindings
    id: concept:cf/bindings
    description: Typed, in-code references to resources (KV, D1, R2, Durable Objects, secrets) injected via wrangler.toml/env, no endpoint or auth.
  - name: KV (Key-Value)
    id: concept:cf/kv
    description: Eventually-consistent global key-value store for config, flags, and caching; great for reads, not for strongly consistent writes.
  - name: D1 (SQLite)
    id: concept:cf/d1
    description: Serverless SQLite at the edge with SQL and Durable Objects-based storage; consistent for single-region read/write patterns.
  - name: R2 (Object Storage)
    id: concept:cf/r2
    description: S3-compatible object storage with zero egress fees; ideal for files, assets, and large blobs via bindings.
  - name: Durable Objects
    id: concept:cf/durable-objects
    description: Single-instance, strongly-consistent coordination primitives with transactional storage and WebSockets for stateful edge apps.
  - name: wrangler
    id: concept:cf/wrangler
    description: The CLI for init, dev, deploy, secret management, tail, and migrations; configured by wrangler.toml.
  - name: Pages & Functions
    id: concept:cf/pages
    description: Git-based static site hosting with serverless Functions for dynamic behavior; increasingly unified with Workers.
  - name: Platform Limits
    id: concept:cf/limits
    description: CPU time (50ms default, up to 30s on paid), 128MB memory, 100ms subrequest budget, and request/response size caps.
  - name: Context (waitUntil)
    id: concept:cf/ctx
    description: The third handler arg exposing waitUntil (defer work past response) and passThroughOnException for observability.
  - name: Tail & Observability
    id: concept:cf/tail
    description: wrangler tail and Workers Observability stream real-time logs and metrics from production isolates.
  - name: Hyperdrive
    id: concept:cf/hyperdrive
    description: A connection pooler/proxy that accelerates access from Workers to external databases (Postgres) over a clustered pool.
apis:
  - name: fetch handler
    id: api:cf/fetch
    signature: "export default { async fetch(req, env, ctx) { return new Response('ok') } }"
    returns: A Response.
    description: The Worker entry point; returned Response is sent to the client. env holds bindings, ctx manages lifecycle.
  - name: env.KV.get/put
    id: api:cf/kv-op
    signature: "await env.MY_KV.get('key'); await env.MY_KV.put('key','v')"
    returns: A value or void.
    description: Reads/writes a KV namespace binding; reads are fast and globally cached, writes are eventually consistent.
  - name: env.DB.prepare
    id: api:cf/d1-op
    signature: "await env.DB.prepare('SELECT * FROM t WHERE id=?').bind(id).all()"
    returns: Query results.
    description: Runs a parameterized D1 SQL query; always bind parameters to avoid injection.
  - name: env.BUCKET.put/get
    id: api:cf/r2-op
    signature: "await env.BUCKET.put('k', data); await env.BUCKET.get('k')"
    returns: An object or null.
    description: Reads/writes objects in an R2 bucket binding; S3-compatible key semantics.
  - name: ctx.waitUntil
    id: api:cf/waituntil
    signature: "ctx.waitUntil(promise)"
    returns: void
    description: Extends the handler's lifetime to finish background work (logging, analytics) after the response is sent.
  - name: wrangler deploy
    id: api:cf/wrangler-deploy
    signature: "npx wrangler deploy"
    returns: The deployed Worker URL.
    description: Bundles and uploads the Worker to the edge; reads wrangler.toml for routes, bindings, and compatibility date.
  - name: new DurableObject
    id: api:cf/durable-object
    signature: "export class Chat extends DurableObject { async fetch(req) {...} }"
    returns: A Durable Object instance.
    description: Defines a Durable Object class; instances provide single-instance coordination with transactional storage.
examples:
  - id: example:cf/router
    language: javascript
    description: A fetch handler routing requests to KV, D1, and R2 by path.
  - id: example:cf/durable-counter
    language: javascript
    description: A Durable Object maintaining a strongly-consistent global counter with WebSockets.
  - id: example:cf/wrangler-toml
    language: toml
    description: wrangler.toml declaring KV, D1, R2, and DO bindings.
  - id: example:cf/r2-proxy
    language: javascript
    description: Proxying and uploading assets through an R2 binding.
failures:
  - id: failure:cf/missing-binding
    symptom: "\"env.MY_KV is undefined\" / \"Cannot read properties of undefined\"."
    cause: A binding declared in code but missing from wrangler.toml or not deployed (wrangler deploy didn't run).
    fix: Add the binding to wrangler.toml (kv_namespaces, d1_databases, r2_buckets, durable_objects) and redeploy; verify with wrangler tail.
  - id: failure:cf/cpu-timeout
    symptom: "\"Worker exceeded CPU time limit\" — request aborted mid-work."
    cause: Synchronous heavy computation or too many sequential subrequests exceeding the 50ms (or plan) CPU budget.
    fix: Stream work, batch subrequests, upgrade plan for higher limits, or move heavy compute to a Durable Object alarm.
  - id: failure:cf/kv-eventual-consistency
    symptom: A just-written KV value is not readable immediately from another location.
    cause: KV is eventually consistent; writes propagate globally in seconds, not instantly.
    fix: Don't rely on read-after-write across regions for KV; use D1 or Durable Objects for strongly consistent state.
  - id: failure:cf/secret-plaintext
    symptom: Secrets appear in source control or client-side bundle; security incident.
    cause: Hardcoding API keys in code instead of using secrets/text bindings.
    fix: Use wrangler secret put (or [vars] for non-sensitive config) so values are injected at runtime, never bundled.
  - id: failure:cf/subrequest-limit
    symptom: "\"Too many subrequests\" or requests dropped after ~50 subrequests."
    cause: Fanning out more than the platform subrequest budget within one request.
    fix: Batch/merge upstream calls, cache via KV, or consolidate logic into fewer downstream requests.
  - id: failure:cf/do-alarm-missing
    symptom: Durable Object scheduled task never fires.
    cause: Alarm not set (this.ctx.storage.setAlarm) or the object was deleted/evicted before firing.
    fix: Call setAlarm in the constructor/fetch; ensure the object stays instantiated; check alarmHandlers wiring.
  - id: failure:cf/compatibility-date
    symptom: Runtime error referencing a newer API that doesn't exist at deploy.
    cause: Missing or stale compatibility_date in wrangler.toml for a recently added runtime feature.
    fix: Bump compatibility_date to a recent date (or set compatibility_flags) so the runtime enables the needed APIs.
extends:
  - concept:cf/workers-runtime
uses:
  - concept:cf/bindings
  - concept:cf/kv
  - concept:cf/d1
  - concept:cf/r2
  - concept:cf/durable-objects
  - concept:cf/wrangler
  - concept:cf/limits
part_of: concept:domain/edge-computing
depends_on:
  - package:rust/patterns
  - package:postgres/patterns
solves:
  - problem:cf/serverless-edge
alternatives:
  - package:aws/lambda-edge
  - package:deno/deploy
  - package:fastly/compute
---
# Cloudflare Workers Patterns

Cloudflare Workers run on V8 isolates, not containers or VMs, which is why they start in milliseconds and deploy to 300+ edge locations. A Worker is just a `fetch(request, env, ctx)` function returning a `Response`. There is no server to manage and no cold-start penalty like traditional serverless — but the model has hard edges you must design around: a CPU time budget (50ms on the free tier, up to 30s on paid), 128MB memory, and a subrequest cap per invocation.

The superpower is bindings. Instead of wiring endpoints, secrets, and SDKs, you declare resources in `wrangler.toml` (KV namespaces, D1 databases, R2 buckets, Durable Objects) and they appear as typed properties on `env` — no auth code, no network round-trip to authenticate. `KV` is a global key-value store perfect for config, feature flags, and cached reads, but it is eventually consistent, so a value you just wrote may not be readable from another region for a few seconds; never use KV for read-after-write correctness. `D1` is serverless SQLite at the edge for relational SQL with stronger consistency in single-region patterns. `R2` is S3-compatible object storage with zero egress fees — ideal for assets and large blobs. `Durable Objects` are the escape hatch for stateful, strongly-consistent coordination: one instance per key with transactional storage and WebSocket support, great for counters, queues, and real-time apps; their `setAlarm` schedules deferred work. `Hyperdrive` accelerates connections to external Postgres by pooling at the edge.

Operations center on `wrangler`: `wrangler dev` for local simulation, `wrangler deploy` to ship, `wrangler tail` and Workers Observability for live logs. The `ctx` argument exposes `waitUntil` to keep background work (analytics, logging) alive after the response is sent, and `passThroughOnException` to fall through on errors. The cardinal security rule: never hardcode secrets — use `wrangler secret put` so values are injected at runtime and never bundled into client code. Keep requests lean to stay under the subrequest budget by batching upstream calls and caching with KV. With the right storage primitive chosen per access pattern, Workers let you ship globally distributed, low-latency apps without operating a single server.
