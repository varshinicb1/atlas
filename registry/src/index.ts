import { RegistryAgent } from "./registry";

export { RegistryAgent };

declare global {
  interface Env {
    RegistryAgent: DurableObjectNamespace<RegistryAgent>;
    PACKAGES: KVNamespace;
  }
}

function jsonResponse(data: unknown, status = 200): Response {
  return new Response(JSON.stringify(data), {
    status,
    headers: {
      "Content-Type": "application/json",
      "Access-Control-Allow-Origin": "*",
      "Access-Control-Allow-Methods": "GET, POST, PUT, DELETE, OPTIONS",
      "Access-Control-Allow-Headers": "Content-Type, Authorization, X-API-Key",
      "Access-Control-Max-Age": "86400",
    },
  });
}

function errorResponse(message: string, status: number): Response {
  return jsonResponse({ error: message, status }, status);
}

function getApiKey(request: Request): string | null {
  const auth = request.headers.get("Authorization");
  if (auth?.startsWith("Bearer ")) return auth.slice(7);
  return request.headers.get("X-API-Key");
}

function getClientIp(request: Request): string {
  return request.headers.get("CF-Connecting-IP") || request.headers.get("X-Forwarded-For") || "unknown";
}

// Attaches X-RateLimit-* headers to a response. When the key is present the
// durable-object rate-limit state is checked first; on exhaustion a 429 is
// returned instead of running `fn`.
async function withRateLimit(
  stub: RegistryAgent,
  apiKey: string | null,
  fn: () => Promise<Response>
): Promise<Response> {
  const rl = await stub.rateLimitCheck(apiKey || "__anonymous__");
  if (rl.limited) {
    const resp = errorResponse("Rate limit exceeded. Max 60 requests/minute per API key.", 429);
    resp.headers.set("X-RateLimit-Limit", String(rl.limit));
    resp.headers.set("X-RateLimit-Remaining", "0");
    resp.headers.set("X-RateLimit-Reset", String(rl.reset_at));
    return resp;
  }
  const resp = await fn();
  if (resp instanceof Response) {
    resp.headers.set("X-RateLimit-Limit", String(rl.limit));
    resp.headers.set("X-RateLimit-Remaining", String(rl.remaining));
    resp.headers.set("X-RateLimit-Reset", String(rl.reset_at));
  }
  return resp;
}

function prometheusMetrics(m: {
  total_packages: number;
  total_downloads: number;
  total_publishes: number;
  per_org: Record<string, { packages: number; downloads: number }>;
  uptime_seconds: number;
}): string {
  const lines: string[] = [];
  lines.push("# HELP atlas_total_packages Total number of published packages.");
  lines.push("# TYPE atlas_total_packages gauge");
  lines.push(`atlas_total_packages ${m.total_packages}`);
  lines.push("# HELP atlas_total_downloads Total download count across all packages.");
  lines.push("# TYPE atlas_total_downloads counter");
  lines.push(`atlas_total_downloads ${m.total_downloads}`);
  lines.push("# HELP atlas_total_publishes Total publish operations.");
  lines.push("# TYPE atlas_total_publishes counter");
  lines.push(`atlas_total_publishes ${m.total_publishes}`);
  lines.push("# HELP atlas_uptime_seconds Registry uptime in seconds.");
  lines.push("# TYPE atlas_uptime_seconds gauge");
  lines.push(`atlas_uptime_seconds ${m.uptime_seconds}`);
  for (const [org, stats] of Object.entries(m.per_org)) {
    const label = `org="${org}"`;
    lines.push(`atlas_org_packages{${label}} ${stats.packages}`);
    lines.push(`atlas_org_downloads{${label}} ${stats.downloads}`);
  }
  return lines.join("\n") + "\n";
}

// Resolves the caller's API key into an auth record, enforcing presence,
// validity and (optionally) admin role. Throws plain Errors whose messages
// are mapped to status codes by the catch block at the end of fetch().
async function requireAuth(
  env: Env,
  request: Request,
  opts: { admin?: boolean } = {}
): Promise<{ org: string; role: string }> {
  const apiKey = getApiKey(request);
  if (!apiKey) throw new Error("API key required");
  const raw = await env.PACKAGES.get(`apikey:${apiKey}`);
  if (!raw) throw new Error("Invalid API key");
  let auth: { org: string; role: string; last_used?: string | null; expires_at?: string | null };
  try {
    auth = JSON.parse(raw);
  } catch {
    throw new Error("Invalid API key");
  }
  if (auth.expires_at && new Date(auth.expires_at).getTime() < Date.now()) {
    throw new Error("API key has expired.");
  }
  if (opts.admin && auth.role !== "admin") {
    throw new Error("Admin access required");
  }
  const updated = { ...auth, last_used: new Date().toISOString() };
  await env.PACKAGES.put(`apikey:${apiKey}`, JSON.stringify(updated));
  return auth;
}

export default {
  async fetch(request: Request, env: Env): Promise<Response> {
    if (request.method === "OPTIONS") {
      return new Response(null, {
        headers: {
          "Access-Control-Allow-Origin": "*",
          "Access-Control-Allow-Methods": "GET, POST, PUT, DELETE, OPTIONS",
          "Access-Control-Allow-Headers": "Content-Type, Authorization, X-API-Key",
          "Access-Control-Max-Age": "86400",
        },
      });
    }

    const url = new URL(request.url);
    const path = url.pathname;
    const id = env.RegistryAgent.idFromName("default");
    const stub = env.RegistryAgent.get(id) as unknown as RegistryAgent;

    try {
      if (path === "/api/v1/admin/migrate-org" && request.method === "POST") {
        const auth = await requireAuth(env, request, { admin: true });
        const apiKey = getApiKey(request);
        return await withRateLimit(stub, apiKey, async () => {
          const body = await request.json() as { fromOrg: string | null; toOrg: string };
          const result = await stub.migrateOrg(body.fromOrg ?? null, body.toOrg, auth);
          return jsonResponse(result);
        });
      }

      if (path === "/health") {
        const packages = await stub.listPackages();
        return jsonResponse({
          status: "ok",
          total_packages: packages.length,
          packages: packages.map((p: any) => ({ name: p.name, version: p.version, downloads: p.download_count })),
          timestamp: new Date().toISOString(),
        });
      }

      if (path === "/api/v1/openapi") {
        return jsonResponse(OPENAPI_SPEC);
      }

      if (path === "/api/v1/packages" && request.method === "GET") {
        const apiKey = getApiKey(request);
        return await withRateLimit(stub, apiKey, async () => {
          let org: string | undefined;
          if (apiKey) {
            try {
              const raw = await env.PACKAGES.get(`apikey:${apiKey}`);
              if (raw) org = JSON.parse(raw).org;
            } catch { org = ""; }
            if (org === undefined) org = "";
          } else {
            org = "";
          }
          const packages = await stub.listPackages(org);
          return jsonResponse(packages);
        });
      }

      const versionSnapshotMatch = path.match(/^\/api\/v1\/packages\/(.+?)\/versions\/([^/]+)$/);
      if (versionSnapshotMatch && request.method === "GET") {
        const apiKey = getApiKey(request);
        return await withRateLimit(stub, apiKey, async () => {
          const name = decodeURIComponent(versionSnapshotMatch[1]);
          const version = decodeURIComponent(versionSnapshotMatch[2]);
          const pkg = await stub.getPackage(name);
          if (!pkg) return errorResponse("Package not found", 404);
          if (pkg.metadata.org) {
            let auth: { org: string };
            try { auth = await requireAuth(env, request); } catch { return errorResponse("Authentication required to access this package.", 401); }
            if (auth.org !== pkg.metadata.org) return errorResponse("Package belongs to a different organization.", 403);
          }
          const snap = await stub.getPackageVersion(name, version);
          if (!snap) return errorResponse("Version not found", 404);
          return jsonResponse(snap);
        });
      }

      const versionsListMatch = path.match(/^\/api\/v1\/packages\/(.+?)\/versions$/);
      if (versionsListMatch && request.method === "GET") {
        const apiKey = getApiKey(request);
        return await withRateLimit(stub, apiKey, async () => {
          const name = decodeURIComponent(versionsListMatch[1]);
          const pkg = await stub.getPackage(name);
          if (!pkg) return errorResponse("Package not found", 404);
          if (pkg.metadata.org) {
            let auth: { org: string };
            try { auth = await requireAuth(env, request); } catch { return errorResponse("Authentication required to access this package.", 401); }
            if (auth.org !== pkg.metadata.org) return errorResponse("Package belongs to a different organization.", 403);
          }
          const result = await stub.getPackageVersions(name);
          if (!result) return errorResponse("Package not found", 404);
          return jsonResponse(result);
        });
      }

      const pkgMatch = path.match(/^\/api\/v1\/packages\/(.+)$/);
      if (pkgMatch && request.method === "GET") {
        const apiKey = getApiKey(request);
        return await withRateLimit(stub, apiKey, async () => {
          const name = decodeURIComponent(pkgMatch[1]);
          if (!name.match(/^[a-z0-9_-]+$/)) return errorResponse("Invalid package name.", 400);
          const pkg = await stub.getPackage(name);
          if (!pkg) return errorResponse("Package not found", 404);
          if (pkg.metadata.org) {
            let auth: { org: string };
            try { auth = await requireAuth(env, request); } catch { return errorResponse("Authentication required to access this package.", 401); }
            if (auth.org !== pkg.metadata.org) return errorResponse("Package belongs to a different organization.", 403);
          }
          const count = await stub.incrementDownload(name);
          pkg.metadata.download_count = count;
          return jsonResponse(pkg);
        });
      }

      if (pkgMatch && request.method === "DELETE" && !path.includes("/versions")) {
        const auth = await requireAuth(env, request, { admin: true });
        const name = decodeURIComponent(pkgMatch[1]);
        if (!name.match(/^[a-z0-9_-]+$/)) return errorResponse("Invalid package name.", 400);
        const result = await stub.deletePackage(name, auth);
        return jsonResponse(result);
      }
      if (pkgMatch && !path.includes("/versions")) {
        return errorResponse("Method not allowed", 405);
      }

      if (path === "/api/v1/search" && request.method === "GET") {
        const query = url.searchParams.get("q") || "";
        if (!query) return jsonResponse({ results: [], total: 0 });
        const limit = Math.min(parseInt(url.searchParams.get("limit") || "20"), 100);
        const apiKey = getApiKey(request);
        return await withRateLimit(stub, apiKey, async () => {
          let org: string | undefined;
          if (apiKey) {
            try {
              const raw = await env.PACKAGES.get(`apikey:${apiKey}`);
              if (raw) org = JSON.parse(raw).org;
            } catch { org = ""; }
            if (org === undefined) org = "";
          } else {
            org = "";
          }
          const results = await stub.searchPackages(query, limit, org);
          return jsonResponse({ results, total: results.length });
        });
      }

      if (path === "/api/v1/publish" && request.method === "POST") {
        const auth = await requireAuth(env, request);
        if (auth.role === "viewer") {
          return errorResponse("Viewers cannot publish packages.", 403);
        }
        const apiKey = getApiKey(request);

        return await withRateLimit(stub, apiKey, async () => {
          let body: PublishInput;
          try {
            body = await request.json() as PublishInput;
          } catch {
            return errorResponse("Malformed JSON body.", 400);
          }
          const ip = getClientIp(request);
          const result = await stub.publishPackage(body, auth, ip, apiKey || undefined);
          return jsonResponse(result, result.replaced ? 200 : 201);
        });
      }

      if (path === "/api/v1/packages" && request.method === "POST") {
        return jsonResponse({ error: "Use POST /api/v1/publish to publish packages", docs: "https://atlas-sh.pages.dev/docs/api" }, 400);
      }

      if (path.startsWith("/api/v1/org/stats") && request.method === "GET") {
        const auth = await requireAuth(env, request);
        const apiKey = getApiKey(request);
        return await withRateLimit(stub, apiKey, async () => {
          const stats = await stub.getOrgStats(auth.org);
          return jsonResponse(stats);
        });
      }

      if (path.startsWith("/api/v1/org/audit") && request.method === "GET") {
        const auth = await requireAuth(env, request, { admin: true });
        const apiKey = getApiKey(request);
        return await withRateLimit(stub, apiKey, async () => {
          const limit = Math.min(parseInt(url.searchParams.get("limit") || "50"), 200);
          const entries: AuditEntry[] = [];
          const list = await env.PACKAGES.list({ prefix: "audit:", limit });
          for (const key of list.keys) {
            const val = await env.PACKAGES.get(key.name);
            if (val) {
              const entry = JSON.parse(val) as AuditEntry;
              if (entry.org !== auth.org) continue;
              entries.push(entry);
            }
          }
          entries.sort((a, b) => b.timestamp.localeCompare(a.timestamp));
          return jsonResponse({ entries, total: entries.length });
        });
      }

      // --- Admin: API key management -------------------------------------

      if (path === "/api/v1/admin/keys" && request.method === "POST") {
        const auth = await requireAuth(env, request, { admin: true });
        const apiKey = getApiKey(request);
        return await withRateLimit(stub, apiKey, async () => {
          const body = (await request.json().catch(() => ({}))) as {
            role?: string;
            expires_at?: string;
            key?: string;
          };
          const result = await stub.createApiKey({
            org: auth.org,
            role: (body.role as "admin" | "publisher" | "viewer") || "publisher",
            expires_at: body.expires_at ?? null,
            key: body.key,
          });
          return jsonResponse(result, 201);
        });
      }

      if (path === "/api/v1/admin/keys" && request.method === "GET") {
        const auth = await requireAuth(env, request, { admin: true });
        const apiKey = getApiKey(request);
        return await withRateLimit(stub, apiKey, async () => {
          const keys = await stub.listApiKeys(auth.org);
          return jsonResponse({ keys });
        });
      }

      const keyRevokeMatch = path.match(/^\/api\/v1\/admin\/keys\/(.+)$/);
      if (keyRevokeMatch && request.method === "DELETE") {
        const auth = await requireAuth(env, request, { admin: true });
        const apiKey = getApiKey(request);
        return await withRateLimit(stub, apiKey, async () => {
          const targetKey = decodeURIComponent(keyRevokeMatch[1]);
          const result = await stub.revokeApiKey(auth.org, targetKey);
          return jsonResponse(result);
        });
      }

      // --- Admin: backup / export / import -----------------------------

      if (path === "/api/v1/admin/export" && request.method === "GET") {
        const auth = await requireAuth(env, request, { admin: true });
        const apiKey = getApiKey(request);
        return await withRateLimit(stub, apiKey, async () => {
          const dump = await stub.exportAll(auth);
          return jsonResponse(dump);
        });
      }

      if (path === "/api/v1/admin/import" && request.method === "POST") {
        const auth = await requireAuth(env, request, { admin: true });
        const apiKey = getApiKey(request);
        return await withRateLimit(stub, apiKey, async () => {
          const body = (await request.json().catch(() => null)) as unknown;
          const result = await stub.importAll(body as any, auth);
          return jsonResponse(result, 201);
        });
      }

      // --- Metrics ------------------------------------------------------

      if (path === "/metrics") {
        const auth = await requireAuth(env, request, { admin: true });
        const apiKey = getApiKey(request);
        return await withRateLimit(stub, apiKey, async () => {
          const metrics = await stub.getMetrics();
          const accept = request.headers.get("Accept") || "";
          if (accept.includes("application/json")) {
            return jsonResponse(metrics);
          }
          return new Response(prometheusMetrics(metrics), {
            status: 200,
            headers: {
              "Content-Type": "text/plain; version=0.0.4",
              "Access-Control-Allow-Origin": "*",
            },
          });
        });
      }

      // Legacy API paths (backward compat)
      if (path === "/api/packages" && request.method === "GET") {
        const apiKey = getApiKey(request);
        return await withRateLimit(stub, apiKey, async () => {
          let org: string | undefined;
          if (apiKey) {
            try { const raw = await env.PACKAGES.get(`apikey:${apiKey}`); if (raw) org = JSON.parse(raw).org; } catch { org = ""; }
            if (org === undefined) org = "";
          } else {
            org = "";
          }
          const packages = await stub.listPackages(org);
          return jsonResponse(packages);
        });
      }

      const legacyMatch = path.match(/^\/api\/packages\/(.+)$/);
      if (legacyMatch && request.method === "GET") {
        const apiKey = getApiKey(request);
        return await withRateLimit(stub, apiKey, async () => {
          const name = decodeURIComponent(legacyMatch[1]);
          if (!name.match(/^[a-z0-9_-]+$/)) return errorResponse("Invalid package name.", 400);
          const pkg = await stub.getPackage(name);
          if (!pkg) return errorResponse("Package not found", 404);
          if (pkg.metadata.org) {
            let auth: { org: string };
            try { auth = await requireAuth(env, request); } catch { return errorResponse("Authentication required to access this package.", 401); }
            if (auth.org !== pkg.metadata.org) return errorResponse("Package belongs to a different organization.", 403);
          }
          return jsonResponse(pkg);
        });
      }

      if (path === "/api/search" && request.method === "GET") {
        const query = url.searchParams.get("q") || "";
        const limit = Math.min(parseInt(url.searchParams.get("limit") || "20"), 100);
        const apiKey = getApiKey(request);
        return await withRateLimit(stub, apiKey, async () => {
          let org: string | undefined;
          if (apiKey) {
            try { const raw = await env.PACKAGES.get(`apikey:${apiKey}`); if (raw) org = JSON.parse(raw).org; } catch { org = ""; }
            if (org === undefined) org = "";
          } else {
            org = "";
          }
          const results = await stub.searchPackages(query, limit, org);
          return jsonResponse({ results, total: results.length });
        });
      }

      if (path === "/api/publish" && request.method === "POST") {
        const apiKey = getApiKey(request);
        if (!apiKey) {
          return jsonResponse({ error: "API key required", docs: "Use --api-key <key> or POST to /api/v1/publish" }, 401);
        }
        const auth = await requireAuth(env, request);
        if (auth.role === "viewer") {
          return errorResponse("Viewers cannot publish packages.", 403);
        }
        return await withRateLimit(stub, apiKey, async () => {
          const body = await request.json() as PublishInput;
          const ip = getClientIp(request);
          const result = await stub.publishPackage(body, auth, ip, apiKey);
          return jsonResponse(result, result.replaced ? 200 : 201);
        });
      }

      if (path.startsWith("/api/")) {
        return errorResponse("Not found", 404);
      }
      return errorResponse("Not found", 404);
    } catch (err: unknown) {
      const msg = err instanceof Error ? err.message : "Internal error";
      if (msg.includes("Rate limit")) return errorResponse(msg, 429);
      if (
        msg.includes("API key required") ||
        msg.includes("Invalid API key") ||
        msg.includes("Invalid or missing") ||
        msg.includes("expired")
      ) return errorResponse(msg, 401);
      if (
        msg.includes("organization") ||
        msg.includes("admins") ||
        msg.includes("Viewers") ||
        msg.includes("Admin") ||
        msg.includes("permission")
      ) return errorResponse(msg, 403);
      if (
        msg.includes("Package name") ||
        msg.includes("required") ||
        msg.includes("Invalid role") ||
        msg.includes("Invalid export dump") ||
        msg.includes("expires_at") ||
        msg.includes("Malformed JSON") ||
        msg.includes("Invalid package name") ||
        msg.includes("Version") ||
        msg.includes("File") ||
        msg.includes("Tags")
      ) return errorResponse(msg, 400);
      if (msg.includes("not found")) return errorResponse(msg, 404);
      return errorResponse(msg, 500);
    }
  },
} satisfies ExportedHandler<Env>;

interface PublishInput {
  name: string;
  version: string;
  description: string;
  author?: string;
  tags: string[];
  files: Record<string, string>;
  nodes?: number;
  edges?: number;
}

interface AuditEntry {
  action: string;
  package_name: string;
  version: string;
  api_key: string;
  org: string;
  timestamp: string;
  ip: string;
}

const OPENAPI_SPEC = {
  openapi: "3.0.3",
  info: {
    title: "Atlas Registry API",
    version: "0.1.0",
    description: "Enterprise API for the Atlas Knowledge Package Registry",
    contact: { url: "https://atlas-sh.pages.dev" },
  },
  servers: [{ url: "https://atlas-hub-registry.cbvarshini1.workers.dev" }],
  paths: {
    "/health": { get: { summary: "Health check", operationId: "health" } },
    "/api/v1/packages": {
      get: { summary: "List packages", operationId: "listPackages", parameters: [] },
      post: { summary: "Publish package (use /api/v1/publish)", deprecated: true },
    },
    "/api/v1/packages/{name}": {
      get: { summary: "Get package details", operationId: "getPackage", parameters: [{ name: "name", in: "path", required: true, schema: { type: "string" } }] },
    },
    "/api/v1/search": {
      get: { summary: "Search packages", operationId: "searchPackages", parameters: [{ name: "q", in: "query", schema: { type: "string" } }, { name: "limit", in: "query", schema: { type: "integer", default: 20 } }] },
    },
    "/api/v1/publish": {
      post: { summary: "Publish a package", operationId: "publishPackage", security: [{ ApiKeyAuth: [] }], requestBody: { required: true, content: { "application/json": { schema: { type: "object", properties: { name: { type: "string" }, version: { type: "string" }, description: { type: "string" }, tags: { type: "array", items: { type: "string" } }, files: { type: "object", additionalProperties: { type: "string" } } }, required: ["name", "version", "files"] } } } } },
    },
    "/api/v1/org/stats": { get: { summary: "Organization statistics", operationId: "orgStats", security: [{ ApiKeyAuth: [] }] } },
    "/api/v1/org/audit": { get: { summary: "Organization audit log", operationId: "orgAudit", security: [{ ApiKeyAuth: [] }], parameters: [{ name: "limit", in: "query", schema: { type: "integer", default: 50 } }] } },
    "/api/v1/packages/{name}/versions": { get: { summary: "List package versions", operationId: "packageVersions", parameters: [{ name: "name", in: "path", required: true, schema: { type: "string" } }] } },
    "/api/v1/packages/{name}/versions/{version}": { get: { summary: "Get a specific package version", operationId: "packageVersion", parameters: [{ name: "name", in: "path", required: true, schema: { type: "string" } }, { name: "version", in: "path", required: true, schema: { type: "string" } }] } },
    "/api/v1/admin/keys": {
      get: { summary: "List API keys for org (admin)", operationId: "listKeys", security: [{ ApiKeyAuth: [] }] },
      post: { summary: "Create an API key (admin)", operationId: "createKey", security: [{ ApiKeyAuth: [] }], requestBody: { required: false, content: { "application/json": { schema: { type: "object", properties: { role: { type: "string", enum: ["admin", "publisher", "viewer"] }, expires_at: { type: "string", format: "date-time" }, key: { type: "string" } } } } } } },
    },
    "/api/v1/admin/keys/{key}": { delete: { summary: "Revoke an API key (admin)", operationId: "revokeKey", security: [{ ApiKeyAuth: [] }], parameters: [{ name: "key", in: "path", required: true, schema: { type: "string" } }] } },
    "/api/v1/admin/export": { get: { summary: "Export all packages (admin)", operationId: "exportAll", security: [{ ApiKeyAuth: [] }] } },
    "/api/v1/admin/import": { post: { summary: "Import packages from a dump (admin)", operationId: "importAll", security: [{ ApiKeyAuth: [] }] } },
    "/metrics": { get: { summary: "Prometheus-style metrics", operationId: "metrics" } },
  },
  components: {
    securitySchemes: {
      ApiKeyAuth: { type: "apiKey", in: "header", name: "X-API-Key" },
    },
  },
};
