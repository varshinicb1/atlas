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
    const stub = env.RegistryAgent.get(id);

    try {
      if (path === "/api/v1/admin/migrate-org" && request.method === "POST") {
        const apiKey = getApiKey(request);
        if (!apiKey) return errorResponse("API key required", 401);
        const raw = await env.PACKAGES.get(`apikey:${apiKey}`);
        if (!raw) return errorResponse("Invalid API key", 401);
        const auth = JSON.parse(raw);
        if (auth.role !== "admin") return errorResponse("Admin only", 403);
        const body = await request.json() as { fromOrg: string | null; toOrg: string };
        const result = await stub.migrateOrg(body.fromOrg ?? null, body.toOrg);
        return jsonResponse(result);
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
        let org: string | undefined;
        if (apiKey) {
          try {
            const raw = await env.PACKAGES.get(`apikey:${apiKey}`);
            if (raw) org = JSON.parse(raw).org;
          } catch { /* public access */ }
        }
        const packages = await stub.listPackages(org);
        return jsonResponse(packages);
      }

      const pkgMatch = path.match(/^\/api\/v1\/packages\/(.+)$/);
      if (pkgMatch && request.method === "GET") {
        const name = decodeURIComponent(pkgMatch[1]);
        const pkg = await stub.getPackage(name);
        if (!pkg) return errorResponse("Package not found", 404);
        await stub.incrementDownload(name);
        return jsonResponse(pkg);
      }

      if (path === "/api/v1/search" && request.method === "GET") {
        const query = url.searchParams.get("q") || "";
        if (!query) return jsonResponse({ results: [], total: 0 });
        const limit = Math.min(parseInt(url.searchParams.get("limit") || "20"), 100);
        const apiKey = getApiKey(request);
        let org: string | undefined;
        if (apiKey) {
          try {
            const raw = await env.PACKAGES.get(`apikey:${apiKey}`);
            if (raw) org = JSON.parse(raw).org;
          } catch { /* public */ }
        }
        const results = await stub.searchPackages(query, limit, org);
        return jsonResponse({ results, total: results.length });
      }

      if (path === "/api/v1/publish" && request.method === "POST") {
        const apiKey = getApiKey(request);
        if (!apiKey) return errorResponse("API key required for publish", 401);
        let auth: { org: string; role: string };
        try {
          const raw = await env.PACKAGES.get(`apikey:${apiKey}`);
          if (!raw) return errorResponse("Invalid API key", 401);
          auth = JSON.parse(raw);
        } catch {
          return errorResponse("Invalid API key", 401);
        }

        const body = await request.json() as PublishInput;
        const ip = getClientIp(request);
        const result = await stub.publishPackage(body, auth, ip);
        return jsonResponse(result, result.replaced ? 200 : 201);
      }

      if (path === "/api/v1/packages" && request.method === "POST") {
        return jsonResponse({ error: "Use POST /api/v1/publish to publish packages", docs: "https://atlas-sh.pages.dev/docs/api" }, 400);
      }

      if (path.startsWith("/api/v1/org/stats") && request.method === "GET") {
        const apiKey = getApiKey(request);
        if (!apiKey) return errorResponse("API key required", 401);
        const raw = await env.PACKAGES.get(`apikey:${apiKey}`);
        if (!raw) return errorResponse("Invalid API key", 401);
        const auth = JSON.parse(raw);
        const stats = await stub.getOrgStats(auth.org);
        return jsonResponse(stats);
      }

      if (path.startsWith("/api/v1/org/audit") && request.method === "GET") {
        const apiKey = getApiKey(request);
        if (!apiKey) return errorResponse("API key required", 401);
        const raw = await env.PACKAGES.get(`apikey:${apiKey}`);
        if (!raw) return errorResponse("Invalid API key", 401);
        const auth = JSON.parse(raw);
        if (auth.role !== "admin") return errorResponse("Admin access required", 403);
        const limit = Math.min(parseInt(url.searchParams.get("limit") || "50"), 200);
        const entries: AuditEntry[] = [];
        const list = await env.PACKAGES.list({ prefix: "audit:", limit });
        for (const key of list.keys) {
          const val = await env.PACKAGES.get(key.name);
          if (val) entries.push(JSON.parse(val));
        }
        entries.sort((a, b) => b.timestamp.localeCompare(a.timestamp));
        return jsonResponse({ entries, total: entries.length });
      }

      // Legacy API paths (backward compat)
      if (path === "/api/packages" && request.method === "GET") {
        const packages = await stub.listPackages();
        return jsonResponse(packages);
      }

      const legacyMatch = path.match(/^\/api\/packages\/(.+)$/);
      if (legacyMatch && request.method === "GET") {
        const name = decodeURIComponent(legacyMatch[1]);
        const pkg = await stub.getPackage(name);
        if (!pkg) return errorResponse("Package not found", 404);
        return jsonResponse(pkg);
      }

      if (path === "/api/search" && request.method === "GET") {
        const query = url.searchParams.get("q") || "";
        const limit = Math.min(parseInt(url.searchParams.get("limit") || "20"), 100);
        const results = await stub.searchPackages(query, limit);
        return jsonResponse({ results, total: results.length });
      }

      if (path === "/api/publish" && request.method === "POST") {
        const apiKey = getApiKey(request);
        if (!apiKey) {
          return jsonResponse({ error: "API key required", docs: "Use --api-key <key> or POST to /api/v1/publish" }, 401);
        }
        const raw = await env.PACKAGES.get(`apikey:${apiKey}`);
        if (!raw) return errorResponse("Invalid API key", 401);
        const auth = JSON.parse(raw);
        const body = await request.json() as PublishInput;
        const ip = getClientIp(request);
        const result = await stub.publishPackage(body, auth, ip);
        return jsonResponse(result, result.replaced ? 200 : 201);
      }

      return errorResponse("Not found", 404);
    } catch (err: unknown) {
      const msg = err instanceof Error ? err.message : "Internal error";
      if (msg.includes("Rate limit")) return errorResponse(msg, 429);
      if (msg.includes("Invalid or missing") || msg.includes("API key") || msg.includes("organization") || msg.includes("admins")) return errorResponse(msg, 403);
      if (msg.includes("Package name") || msg.includes("required") || msg.includes("not found")) return errorResponse(msg, 400);
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
  },
  components: {
    securitySchemes: {
      ApiKeyAuth: { type: "apiKey", in: "header", name: "X-API-Key" },
    },
  },
};
