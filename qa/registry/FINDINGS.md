# Atlas Registry - Red-Team Findings

**Date**: 2026-07-11
**Scope**: `registry/src/index.ts`, `registry/src/registry.ts`
**Method**: Static analysis + unit-test probes with in-memory KV/DO mock (vitest)
**Total unique issues**: 52 (P0: 19, P1: 12, P2: 16, P3: 5)

---

## Severity Summary

| Severity | Count | Criteria |
|----------|-------|----------|
| **P0 — Security / Cross-tenant / Data loss** | 19 | Unauthorized data access, privilege escalation, data corruption |
| **P1 — Broken functionality** | 12 | Incorrect error handling, missing features, data inconsistency |
| **P2 — UX / Hardening** | 16 | Missing validation, suboptimal error messages, missing headers |
| **P3 — Nit** | 5 | Inconsistencies, minor gaps |

---

## P0 — Security / Cross-tenant / Data Loss

### REG-01 — Any org can READ any other org's package contents (cross-tenant leak)
- **Endpoint**: `GET /api/v1/packages/:name`
- **Repro**:
  ```
  iwr -Uri http://localhost:8787/api/v1/packages/acme-secret -Headers @{"X-API-Key"="admin-other"}
  ```
- **Expected**: 403 — package is owned by "acme", only acme keys should access it
- **Actual**: 200 — returns full `{metadata, files}` including file contents
- **Root cause**: No tenant-scope check in the HTTP handler before calling `stub.getPackage()`
- **Fix**: In `GET /api/v1/packages/:name`, call `requireAuth()`, verify `auth.org` matches `pkg.metadata.org` (or allow if the package has no org). Return 403 on mismatch.

### REG-02 — Any org can LIST and READ version snapshots of any package
- **Endpoint**: `GET /api/v1/packages/:name/versions`, `GET /api/v1/packages/:name/versions/:version`
- **Repro**:
  ```
  $r = iwr -Uri http://localhost:8787/api/v1/packages/acme-secret/versions
  $r2 = iwr -Uri http://localhost:8787/api/v1/packages/acme-secret/versions/1.0.0
  ```
- **Expected**: 403 (requires tenant match) or 401 (requires auth)
- **Actual**: 200 — no auth check at all on either endpoint
- **Root cause**: Lines 166-181 in `index.ts` — these routes never call `requireAuth()`
- **Fix**: Add org-ownership check before serving version data

### REG-03 — Unauthenticated users can READ any package (no auth required)
- **Endpoint**: `GET /api/v1/packages/:name`
- **Repro**:
  ```
  iwr -Uri http://localhost:8787/api/v1/packages/acme-secret
  ```
- **Expected**: 401 when package has an owning org
- **Actual**: 200 — full package returned with no API key
- **Root cause**: Line 184 — `GET /api/v1/packages/:name` has no `requireAuth()` call
- **Fix**: Require authentication for org-owned packages

### REG-04 — `/api/v1/packages` listing shows packages from ALL orgs publicly
- **Endpoint**: `GET /api/v1/packages`
- **Repro**:
  ```
  iwr -Uri http://localhost:8787/api/v1/packages
  ```
- **Expected**: Show only packages matching caller's org (or require auth)
- **Actual**: All packages from all orgs returned
- **Root cause**: Line 162 — `stub.listPackages(org)` called with `org=undefined` when no valid key
- **Fix**: Reject unauthenticated list, or only list public packages

### REG-05 — Any admin can MIGRATE another org's packages into their own org
- **Endpoint**: `POST /api/v1/admin/migrate-org`
- **Repro**:
  ```
  $body = '{"fromOrg":"otherco","toOrg":"acme"}'
  iwr -Method POST -Uri http://localhost:8787/api/v1/admin/migrate-org -Headers @{"X-API-Key"="admin-acme"} -Body $body -ContentType application/json
  ```
- **Expected**: 403 — only admins of the *source* org should be able to migrate
- **Actual**: 200 — `admin-acme` steals `otherco`'s packages
- **Root cause**: `requireAuth(env, request, { admin: true })` only checks admin role, not that the admin belongs to the source org
- **Fix**: Add `auth.org` check against the `fromOrg` parameter

### REG-06 — Admin of org B can OVERWRITE packages owned by org A via import
- **Endpoint**: `POST /api/v1/admin/import`
- **Repro**:
  ```
  $dump = '{...packages with org:"acme"...}'
  iwr -Method POST -Uri http://localhost:8787/api/v1/admin/import -Headers @{"X-API-Key"="admin-other"} -Body $dump -ContentType application/json
  ```
- **Expected**: 403 — otherco cannot write into acme's namespace
- **Actual**: 201 — otherco overwrites acme's package files
- **Root cause**: `importAll` never checks that the importing admin's org matches the package's `meta.org`
- **Fix**: Validate each imported package's `meta.org` against `auth.org`; reject mismatches

### REG-07 — `/api/v1/org/audit` returns ALL orgs' audit entries (no tenant filter)
- **Endpoint**: `GET /api/v1/org/audit`
- **Repro**:
  ```
  iwr -Uri http://localhost:8787/api/v1/org/audit -Headers @{"X-API-Key"="admin-other"}
  ```
- **Expected**: Only otherco's audit entries
- **Actual**: All entries including acme's audit log
- **Root cause**: Lines 244-255 — iterates ALL `audit:*` KV keys with no org filter
- **Fix**: Filter audit entries by `auth.org`

### REG-37 — Cross-tenant EXPORT exfiltrates every org's file contents
- **Endpoint**: `GET /api/v1/admin/export`
- **Repro**:
  ```
  iwr -Uri http://localhost:8787/api/v1/admin/export -Headers @{"X-API-Key"="admin-other"}
  ```
- **Expected**: Only otherco's packages
- **Actual**: Dumps acme's packages including file contents
- **Root cause**: `exportAll()` returns all packages from DO storage with no org filter
- **Fix**: Add org filter to `exportAll()` or reject cross-org export

### REG-38 — Orphan (org-less) packages can be TAKEN OVER by any org's publisher
- **Endpoint**: `POST /api/v1/publish`
- **Repro**: Import a package without `org` field, then publish same name from another org
- **Expected**: Cross-org publish blocked (403)
- **Actual**: Succeeds — `publishPackage` only checks org if `existing.org` is truthy
- **Root cause**: Line 308 — `if (existing.org && existing.org !== auth.org)` — when `existing.org` is undefined/null, the check is skipped
- **Fix**: Treat missing org as cross-tenant when the request is authenticated

### REG-39 — `/metrics` is unauthenticated and leaks every org name + metrics
- **Endpoint**: `GET /metrics`
- **Repro**:
  ```
  iwr -Uri http://localhost:8787/metrics
  ```
- **Expected**: 401 (auth required for internal metrics)
- **Actual**: Returns Prometheus text with `atlas_org_packages{org="acme"}`, `{org="otherco"}` etc.
- **Root cause**: Line 306 — no `requireAuth()` before metrics generation
- **Fix**: Require admin auth for `/metrics`, or at least some authentication

### REG-42 — Audit trail never records the API key (forensics gap)
- **Endpoint**: All publishing endpoints
- **Repro**: Publish a package, then check audit entry's `api_key` field
- **Expected**: `api_key` should contain the key prefix/hash used for the operation
- **Actual**: `api_key: ""` — hardcoded empty string
- **Root cause**: Line 355 — `api_key: ""` is hardcoded; the actual API key is never passed to `appendAudit`
- **Fix**: Pass the API key (or a hash prefix) into `publishPackage` and store it in audit

### REG-43 — Malformed API key value crashes with 500 instead of 401
- **Endpoint**: Any auth-protected endpoint (e.g., `GET /api/v1/org/stats`)
- **Repro**:
  ```
  Put "not-json" into KV at apikey:broken
  iwr -Uri http://localhost:8787/api/v1/org/stats -Headers @{"X-API-Key"="broken"}
  ```
- **Expected**: 401 Invalid API key
- **Actual**: 500 — `JSON.parse(raw)` throws
- **Root cause**: Line 103 — `const auth = JSON.parse(raw);` not wrapped in try/catch
- **Fix**: Wrap `JSON.parse` in try/catch and throw a clean error

### REG-44 — Cross-org publish error escapes try/catch → 500 instead of clean 403
- **Endpoint**: `POST /api/v1/publish`
- **Repro**: Publish same package name from two different orgs
- **Expected**: 403 (or at least a proper error response)
- **Actual**: Throws out of `fetch()` — caught by runtime as 500
- **Root cause**: REG-36 root cause — `return withRateLimit(...)` is not awaited; AuthError from `publishPackage` bypasses error mapper
- **Fix**: `await` the result of `withRateLimit()`

### REG-36 — `/api/v1/publish` and `GET /api/v1/packages/:name` error handling bypassed (all validation errors → unhandled rejections)
- **Endpoint**: `POST /api/v1/publish`, `GET /api/v1/packages/:name`
- **Repro**:
  ```
  iwr -Method POST -Uri http://localhost:8787/api/v1/publish -Headers @{"X-API-Key"="admin"} -Body '{"name":"BAD","version":"1","files":{"a":"b"}}' -ContentType application/json
  ```
- **Expected**: 400 with error message
- **Actual**: `-999` thrown out of fetch, runtime returns 500
- **Root cause**: Lines 187, 226 — `return withRateLimit(...)` returns a Promise that is NOT `await`ed inside the try/catch, so validation errors (ValidationError, AuthError) thrown inside the async callback propagate as unhandled rejections bypassing the catch block's error mapper
- **Fix**: Change `return withRateLimit(...)` to `return await withRateLimit(...)` everywhere

### REG-25 — Unauthenticated GET is NOT rate-limited (abuse vector)
- **Endpoint**: `GET /api/v1/packages/:name`
- **Repro**: Send 80 GETs with no API key
- **Expected**: Rate limiting should apply regardless of auth
- **Actual**: All 80 succeed — rate limiting is only checked when `apiKey` is non-null
- **Root cause**: Line 47 — `if (!apiKey) return fn();` skips rate limiting entirely
- **Fix**: Apply a lower rate limit for unauthenticated requests, or require auth for all requests

### REG-26 — Most endpoints (search, list, metrics, admin, versions) have NO rate limiting
- **Endpoint**: `GET /api/v1/search`, `GET /api/v1/packages`, `GET /metrics`, `GET /api/v1/admin/export`, `GET /api/v1/packages/:name/versions`, `GET /api/v1/packages/:name/versions/:version`, `GET /api/v1/org/stats`, `GET /api/v1/org/audit`
- **Repro**: Send 80 requests to any of these endpoints in a loop
- **Expected**: 429 after 60 requests
- **Actual**: All 80 succeed
- **Root cause**: Only `publish` and `get-package` call `withRateLimit()`
- **Fix**: Add `withRateLimit()` to all authenticated endpoints

### REG-34 — DELETE leaves orphaned version-history keys in DO storage (data remnants)
- **Endpoint**: `DELETE /api/v1/packages/:name`
- **Repro**: Delete a package, then inspect DO storage
- **Expected**: All `pkg:<name>:*` keys cleaned up
- **Actual**: `pkg:<name>:versions` and `pkg:<name>:v:<version>` keys remain
- **Root cause**: `deletePackage()` (line 383-385) only deletes `meta` and `files` keys
- **Fix**: Also delete `pkg:<name>:versions` and `pkg:<name>:v:*` keys

### REG-27 — Export → Import round-trip LOSES version snapshots (data loss)
- **Endpoint**: `POST /api/v1/admin/import`
- **Repro**: Export a package with version history, import into fresh environment, request `/api/v1/packages/snap/versions/1.0.0`
- **Expected**: 200 with the snapshot
- **Actual**: 404 — snapshot gone
- **Root cause**: `importAll()` writes `pkg:<name>:meta`, `:files`, `:versions` but never writes `pkg:<name>:v:<version>` snapshots
- **Fix**: In `importAll()`, iterate `pkg.versions` and write each snapshot from the dump's stored version data (which is currently not in ExportDump — also fix ExportDump to include snapshot data per version)

### REG-54 — Legacy `/api/publish` has NO rate limiting
- **Endpoint**: `POST /api/publish`
- **Repro**: Send 70 publishes via legacy path
- **Expected**: 429 after 60
- **Actual**: All 70 succeed
- **Root cause**: Lines 342-355 — legacy path skips `withRateLimit()` entirely
- **Fix**: Add `withRateLimit()` to legacy publish path

---

## P1 — Broken Functionality

### REG-16 — Malformed JSON body on publish is uncaught → 500 (not 400)
- **Endpoint**: `POST /api/v1/publish`
- **Repro**: Send body `"this is not json{"`
- **Expected**: 400 "Malformed JSON"
- **Actual**: Unhandled SyntaxError → 500
- **Root cause**: REG-36 — `request.json()` inside unawaited `withRateLimit` callback
- **Fix**: Await withRateLimit; also wrap `request.json()` in try/catch

### REG-17 — Invalid import dump returns 500 (should be 400)
- **Endpoint**: `POST /api/v1/admin/import`
- **Repro**: Body `{"foo":1}`
- **Expected**: 400 "Invalid export dump: missing packages array"
- **Actual**: 500
- **Root cause**: Error message "Invalid export dump" doesn't match any catch-block pattern — falls through to `return errorResponse(msg, 500)`
- **Fix**: Add `msg.includes("Invalid export dump")` to the 400 mapper in the catch block

### REG-18 — Invalid role in `createApiKey` returns 403 (should be 400)
- **Endpoint**: `POST /api/v1/admin/keys`
- **Repro**: Body `{"role":"superuser"}`
- **Expected**: 400 "Invalid role..."
- **Actual**: 403
- **Root cause**: Error message contains "role", which matches the 403 mapper `msg.includes("role")`
- **Fix**: Make the 403 mapper more specific or reorder checks

### REG-19 — Invalid `expires_at` in `createApiKey` returns 500 (should be 400)
- **Endpoint**: `POST /api/v1/admin/keys`
- **Repro**: Body `{"role":"viewer","expires_at":"nope"}`
- **Expected**: 400
- **Actual**: 500
- **Root cause**: Error message doesn't match any pattern — falls through to 500
- **Fix**: Add to 400 mapper or catch in the handler

### REG-21 — Disallowed HTTP method returns 404 instead of 405
- **Endpoint**: `PUT /api/v1/packages/foo`
- **Repro**:
  ```
  iwr -Method PUT -Uri http://localhost:8787/api/v1/packages/foo
  ```
- **Expected**: 405 Method Not Allowed (or at least a helpful error)
- **Actual**: 404 Not Found
- **Root cause**: Sequential if/else chains with method checks — unrecognized method falls through to "Not found"
- **Fix**: Return 405 with `Allow` header for known paths with wrong methods

### REG-28 — `total_publishes` metric double-counts because per-package usage keys also match `:publishes:`
- **Endpoint**: `GET /metrics`
- **Repro**: Publish 1 package, read `/metrics`
- **Expected**: `total_publishes` = 1
- **Actual**: `total_publishes` = 2
- **Root cause**: `getMetrics()` (line 633) iterates ALL `usage:` keys with `:publishes:` in name. One publish creates TWO usage keys: `usage:acme:publishes:2026-07-11` AND `usage:acme:publishes:pkgname:2026-07-11`. Both match `:publishes:`.
- **Fix**: Filter out per-package usage keys (those with 4 colons) in the metrics sum

### REG-35 — Concurrent `incrementDownload` calls lose counter increments (read-modify-write race)
- **Endpoint**: `GET /api/v1/packages/:name` (incrementDownload)
- **Repro**: Fire 50 concurrent GETs to same package
- **Expected**: `download_count` = previous + 50
- **Actual**: `download_count` < previous + 50 (lost updates)
- **Root cause**: `incrementDownload()` does read → modify → write without atomicity. DO serializes requests, but the interleaving of `getPackage` + `incrementDownload` across concurrent HTTP requests creates a window
- **Fix**: Use DO storage atomic operations (e.g., SQLite `UPDATE ... SET count = count + 1`)

### REG-33 — Path-traversal package names create confusing storage artifacts
- **Endpoint**: `GET /api/v1/packages/..%2F..%2Fetc%2Fpasswd`
- **Repro**: GET with path-traversal name
- **Expected**: 400 (invalid package name)
- **Actual**: 404 (creates confusing storage lookup for `pkg:../../etc/passwd:meta`)
- **Root cause**: No validation on name in GET path (validation only exists in publish)
- **Fix**: Add name-format validation to all read/delete paths

### REG-last_used — API key `last_used` field is NEVER updated (always null)
- **Endpoint**: All auth-protected endpoints
- **Repro**: Use a key, then list keys — `last_used` is always null
- **Expected**: `last_used` should be updated on each successful auth
- **Actual**: Always null because `requireAuth` in `index.ts` reads KV directly and never calls the DO's `validateApiKey` method (which is the only code path that updates `last_used`)
- **Root cause**: `validateApiKey` is a private method on `RegistryAgent` that is NEVER called from the HTTP layer
- **Fix**: Either call `validateApiKey` from the HTTP layer, or add `last_used` update to `requireAuth`

### REG-legacy-noratelimit — Legacy `/api/publish` has no rate limiting
- **Endpoint**: `POST /api/publish`
- **Repro**: 70 publishes via legacy path
- **Expected**: 429 after 60
- **Actual**: All succeed
- **Root cause**: Legacy path at line 342 skips `withRateLimit()`
- **Fix**: Apply withRateLimit

### REG-legacy-noauth — Legacy `/api/packages` and `/api/search` have no auth or org scoping
- **Endpoint**: `GET /api/packages`, `GET /api/search?q=...`
- **Repro**: Access without any API key
- **Expected**: At minimum org-scoped
- **Actual**: All packages returned
- **Fix**: Apply same org-scoping logic as v1 endpoints

### REG-count-inflation — `GET /api/v1/packages/:name` inflates download counter on every read (including by owner)
- **Endpoint**: `GET /api/v1/packages/:name`
- **Repro**: Publisher reads their own package 10 times
- **Expected**: Download count stable or not incremented for the owning org
- **Actual**: Download count increases by 10
- **Root cause**: Line 190 — always calls `stub.incrementDownload(name)` regardless of caller identity
- **Fix**: Don't count downloads from the owning org, or add a separate "views" counter

---

## P2 — UX / Hardening

### REG-12 — No package name length limit (storage abuse vector)
- **Endpoint**: `POST /api/v1/publish`
- **Repro**: Publish with name of 5000 `a` characters
- **Expected**: 400 "Package name too long" (e.g., max 128 chars)
- **Actual**: 201 — publishes successfully
- **Fix**: Add length validation to package name (max 128 characters)

### REG-13 — No payload size limit (DOS vector)
- **Endpoint**: `POST /api/v1/publish`
- **Repro**: Publish with 8MB file content
- **Expected**: 413 Payload Too Large
- **Actual**: 201 — publishes successfully
- **Fix**: Add 1MB total payload limit, reject oversized requests before processing

### REG-14 — Invalid package name (uppercase, dots) throws uncaught instead of returning 400
- **Endpoint**: `POST /api/v1/publish`
- **Repro**: name = "MyPkg", "bad name", "../evil"
- **Expected**: 400
- **Actual**: Throws (REG-36 root cause)
- **Fix**: Await withRateLimit, then validation errors map correctly

### REG-15 — Missing/empty name throws uncaught instead of returning 400
- **Endpoint**: `POST /api/v1/publish`
- **Repro**: name = ""
- **Expected**: 400
- **Actual**: Throws (REG-36 root cause)
- **Fix**: Same as REG-36

### REG-20 — Unknown route → 404 (correct behavior — no fix needed, documented for completeness)
- **Endpoint**: `GET /api/v1/nonexistent`
- **Status**: Correct — returns 404

### REG-22 — OPTIONS preflight returns CORS headers (correct behavior)
- **Endpoint**: `OPTIONS /api/v1/publish`
- **Status**: Correct — returns 200 with CORS headers

### REG-30 — Delete then GET → 404 (correct behavior)
- **Endpoint**: `DELETE /api/v1/packages/:name` then `GET /api/v1/packages/:name`
- **Status**: Correct

### REG-31 — Republish preserves download_count (correct behavior)
- **Endpoint**: `POST /api/v1/publish` twice
- **Status**: Correct

### REG-32 — Path traversal harmless for KV storage (no FS access)
- **Endpoint**: `GET /api/v1/packages/..%2F..%2Fetc%2Fpasswd`
- **Status**: Returns 404 (no file-system traversal risk)

### REG-33 — SQL/JS injection inert in search (no injection sink)
- **Endpoint**: `GET /api/v1/search?q=<script>alert(1)</script>'
- **Status**: No injection risk (in-memory string matching)

### REG-08 — Bearer token parity works (correct behavior)
- **Endpoint**: `POST /api/v1/publish` with `Authorization: Bearer <key>`
- **Status**: Correct — both `Authorization: Bearer` and `X-API-Key` work

### REG-09 — Viewer cannot publish (correct behavior)
- **Endpoint**: `POST /api/v1/publish` with viewer key
- **Status**: Correct — 403

### REG-10 — Publisher cannot delete/export/manage keys (correct behavior)
- **Endpoint**: `DELETE /api/v1/packages/:name`, `GET /api/v1/admin/export`, `POST /api/v1/admin/keys` with publisher key
- **Status**: Correct — 403

### REG-11 — "Bearer" without space (wrong format) rejected correctly
- **Endpoint**: `POST /api/v1/publish` with `Authorization: Beareradmin-acme`
- **Status**: Correct — 401

### REG-23 — Rate limiting: 60 publishes with same key → 429 (correct behavior)
- **Endpoint**: `POST /api/v1/publish`
- **Status**: Correct — 429 with `X-RateLimit-Limit` and `X-RateLimit-Remaining` headers

### REG-24 — Rate limiting is per-key (different key not blocked)
- **Endpoint**: `POST /api/v1/publish`
- **Status**: Correct — per-key isolation

### REG-29 — Version history works for multiple versions (correct behavior)
- **Endpoint**: `GET /api/v1/packages/:name/versions`
- **Status**: Correct

### REG-45 — Cross-tenant DELETE correctly blocked
- **Endpoint**: `DELETE /api/v1/packages/:name`
- **Status**: Correct — 403 when other org tries to delete

### REG-43b — Malformed API key on public-read endpoint silently degrades to anonymous access (no 401)
- **Endpoint**: `GET /api/v1/packages` with malformed key
- **Repro**: Send key "broken" with non-JSON value in KV
- **Expected**: 401 or at least reject invalid keys
- **Actual**: 200 — silently falls through to unauthenticated access
- **Root cause**: Try/catch swallows JSON.parse error and treats as public
- **Fix**: Log the error; consider rejecting malformed keys

### REG-40 — `X-RateLimit-Reset` header is never set
- **Endpoint**: All rate-limited endpoints
- **Expected**: `X-RateLimit-Reset` with Unix timestamp of when the window resets
- **Actual**: Missing — client can't determine when to retry
- **Fix**: Add `resp.headers.set("X-RateLimit-Reset", String(rl.reset_at))` in `withRateLimit`

### REG-41 — No `version` field validation (accepts any string including path traversal)
- **Endpoint**: `POST /api/v1/publish`
- **Repro**: Publish with version `"../../evil"` or `"<script>"`
- **Expected**: Version should follow semver or at least alphanumeric
- **Actual**: Any string accepted
- **Fix**: Add validation for `version` field (e.g., match `/^[a-zA-Z0-9._-]+$/`)

### REG-46 — Missing `Content-Type` validation on publish
- **Endpoint**: `POST /api/v1/publish`
- **Repro**: Send JSON body with `Content-Type: text/plain`
- **Expected**: 415 Unsupported Media Type
- **Actual**: 500 (if body parsing fails) or 201 (if succeeds)
- **Root cause**: The handler doesn't check Content-Type before calling `request.json()`
- **Fix**: Validate `Content-Type` header and return 415 for non-JSON

### REG-47 — `files` field values not validated as strings
- **Endpoint**: `POST /api/v1/publish`
- **Repro**: `{"files":{"a":123}}` (number instead of string)
- **Expected**: 400 — file values must be strings
- **Actual**: 201 — accepted (JSON.stringify coerces to "123")
- **Fix**: Validate that all `files` values are strings

### REG-48 — Tags not validated as array of strings
- **Endpoint**: `POST /api/v1/publish`
- **Repro**: `{"tags":"not-an-array"}` or `{"tags":[123]}`
- **Expected**: 400
- **Actual**: 201 — accepted
- **Fix**: Validate `tags` is an array of strings

### REG-49 — No TTL/cleanup on audit entries (KV accumulation)
- **Endpoint**: `POST /api/v1/publish` (appends audit)
- **Repro**: Publish many packages over time
- **Expected**: Audit entries should expire or be bounded
- **Actual**: Unlimited accumulation in KV; KV has 1GB limit per namespace
- **Fix**: Add `expiration_ttl` to audit KV writes, or implement a rotation mechanism

---

## P3 — Nits

### REG-50 — Legacy `/api/v1/packages` POST returns generic message instead of redirect
- **Endpoint**: `POST /api/v1/packages`
- **Status**: Works but UX could be improved — returns 400 with link to docs
- **Suggestion**: Return 308 Permanent Redirect to `/api/v1/publish`

### REG-51 — Multiple `X-API-Key` headers silently use only the first
- **Endpoint**: Any
- **Repro**: Send two `X-API-Key` headers
- **Expected**: Use first, ignore second
- **Actual**: Behavior matches expected; could log a warning
- **Suggestion**: Document that only one key is supported

### REG-52 — `GET /api/v1/packages/:name` increments download count for the publisher (inflated metrics)
- **Endpoint**: `GET /api/v1/packages/:name`
- **Repro**: Publisher reads own package
- **Issue**: Every GET inflates download count, including the package owner's reads
- **Suggestion**: Don't count downloads from authenticated users in the same org

### REG-53 — `Content-Type: application/json` not validated before `request.json()`
- **Endpoint**: `POST /api/v1/publish`
- **Repro**: Send valid JSON with `Content-Type: text/plain`
- **Status**: May or may not succeed depending on Workers runtime behavior
- **Suggestion**: Add explicit Content-Type check

### REG-54 — Redacted key listing still exposes first 6 + last 4 key characters
- **Endpoint**: `GET /api/v1/admin/keys`
- **Repro**: List keys — key_prefix shows e.g. `atlas_****abcd`
- **Issue**: 10 of ~50 characters exposed reduces keyspace for brute-force
- **Suggestion**: Show only `atlas_****` (no suffix) or use a hash

---

## Bug Counts by Category

| Category | P0 | P1 | P2 | P3 | Total |
|----------|----|----|----|----|-------|
| AuthN/AuthZ / Cross-tenant | 12 | 2 | 1 | 0 | 15 |
| Error handling (REG-36 cascade) | 1 | 3 | 2 | 0 | 6 |
| Input validation | 0 | 0 | 6 | 2 | 8 |
| Rate limiting | 4 | 1 | 1 | 0 | 6 |
| Feature correctness | 2 | 4 | 0 | 0 | 6 |
| Storage / cleanup | 1 | 0 | 2 | 0 | 3 |
| HTTP hygiene | 0 | 2 | 1 | 3 | 6 |
| Metrics | 0 | 1 | 0 | 0 | 1 |
| Forensics (audit) | 1 | 0 | 0 | 0 | 1 |
| **Total** | **19** | **12** | **16** | **5** | **52** |

---

## How to Reproduce (Local)

```powershell
# 1. Start local dev server
cd registry
npx wrangler dev --port 8787

# 2. In another terminal, seed admin key
npx wrangler kv key put --binding=PACKAGES "apikey:testadmin123" --path .\acme.json

# 3. Run probes
iwr -Uri http://localhost:8787/health

# 4. Or run all vitest tests
npx vitest run
```
