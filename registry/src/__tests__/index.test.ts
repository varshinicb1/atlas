import app from "../index";
import { RegistryAgent } from "../registry";

function makeMockKV() {
  const store = new Map<string, string>();
  return {
    get: async (key: string) => store.get(key) ?? null,
    put: async (key: string, value: string) => { store.set(key, String(value)); },
    delete: async (key: string) => { store.delete(key); },
    list: async (opts?: { prefix?: string; limit?: number }) => {
      const keys = Array.from(store.keys())
        .filter((k) => !opts?.prefix || k.startsWith(opts.prefix))
        .slice(0, opts?.limit ?? 100)
        .map((name) => ({ name }));
      return { keys };
    },
  };
}

interface TestEnv {
  PACKAGES: ReturnType<typeof makeMockKV>;
  RegistryAgent: { idFromName: (n: string) => string; get: () => RegistryAgent };
}

function makeEnv() {
  const kv = makeMockKV();
  const agent = new RegistryAgent({} as any, { PACKAGES: kv } as any);
  const env = {
    PACKAGES: kv,
    RegistryAgent: {
      idFromName: (_n: string) => "default",
      get: () => agent,
    },
  } as unknown as TestEnv;
  return { env, kv, agent };
}

const ADMIN_KEY = { name: "admin-key", value: JSON.stringify({ org: "acme", role: "admin" }) };
const VIEWER_KEY = { name: "viewer-key", value: JSON.stringify({ org: "acme", role: "viewer" }) };

function req(path: string, init?: RequestInit) {
  return new Request(`https://registry.test${path}`, init);
}

async function json(r: Response) {
  return { status: r.status, body: await r.json(), headers: r.headers };
}

describe("Registry HTTP API", () => {
  let env: TestEnv;
  let kv: ReturnType<typeof makeMockKV>;

  beforeEach(async () => {
    const e = makeEnv();
    env = e.env;
    kv = e.kv;
    await kv.put(`apikey:${ADMIN_KEY.name}`, ADMIN_KEY.value);
    await kv.put(`apikey:${VIEWER_KEY.name}`, VIEWER_KEY.value);
  });

  it("health returns ok", async () => {
    const r = await app.fetch(req("/health"), env as any);
    const data = await r.json();
    expect(r.status).toBe(200);
    expect(data.status).toBe("ok");
  });

  it("publishes and fetches a package with rate-limit headers", async () => {
    const publish = await app.fetch(
      req("/api/v1/publish", {
        method: "POST",
        headers: { "X-API-Key": ADMIN_KEY.name, "Content-Type": "application/json" },
        body: JSON.stringify({
          name: "atlas-core",
          version: "1.0.0",
          description: "core",
          tags: ["x"],
          files: { "a.md": "hi" },
        }),
      }),
      env as any
    );
    expect(publish.status).toBe(201);
    expect(publish.headers.get("X-RateLimit-Limit")).toBe("60");
    expect(publish.headers.get("X-RateLimit-Remaining")).not.toBeNull();

    const get = await app.fetch(
      req("/api/v1/packages/atlas-core", { headers: { "X-API-Key": ADMIN_KEY.name } }),
      env as any
    );
    expect(get.status).toBe(200);
    const body = await get.json();
    expect(body.metadata.name).toBe("atlas-core");
    expect(body.metadata.download_count).toBe(1); // incremented on GET
  });

  it("rejects publish without a key (401)", async () => {
    const r = await app.fetch(
      req("/api/v1/publish", { method: "POST", body: JSON.stringify({ name: "x", version: "1", files: {} }) }),
      env as any
    );
    expect(r.status).toBe(401);
  });

  it("rejects publish from a viewer (403)", async () => {
    const r = await app.fetch(
      req("/api/v1/publish", {
        method: "POST",
        headers: { "X-API-Key": VIEWER_KEY.name, "Content-Type": "application/json" },
        body: JSON.stringify({ name: "x", version: "1", files: { "a": "b" } }),
      }),
      env as any
    );
    expect(r.status).toBe(403);
  });

  it("returns 404 for unknown package", async () => {
    const r = await app.fetch(req("/api/v1/packages/nope"), env as any);
    expect(r.status).toBe(404);
  });

  it("serves package version history", async () => {
    for (const v of ["1.0.0", "1.0.1", "2.0.0"]) {
      await app.fetch(
        req("/api/v1/publish", {
          method: "POST",
          headers: { "X-API-Key": ADMIN_KEY.name, "Content-Type": "application/json" },
          body: JSON.stringify({ name: "hist", version: v, description: "d", tags: [], files: { "a": "b" } }),
        }),
        env as any
      );
    }
    const list = await app.fetch(req("/api/v1/packages/hist/versions", { headers: { "X-API-Key": ADMIN_KEY.name } }), env as any);
    expect(list.status).toBe(200);
    const lb = await list.json();
    expect(lb.versions).toEqual(["1.0.0", "1.0.1", "2.0.0"]);

    const snap = await app.fetch(req("/api/v1/packages/hist/versions/1.0.0", { headers: { "X-API-Key": ADMIN_KEY.name } }), env as any);
    expect(snap.status).toBe(200);
    const sb = await snap.json();
    expect(sb.metadata.version).toBe("1.0.0");

    const missing = await app.fetch(req("/api/v1/packages/hist/versions/9.9.9", { headers: { "X-API-Key": ADMIN_KEY.name } }), env as any);
    expect(missing.status).toBe(404);
  });

  describe("admin API key management", () => {
    it("creates, lists (redacted) and revokes keys", async () => {
      const create = await app.fetch(
        req("/api/v1/admin/keys", {
          method: "POST",
          headers: { "X-API-Key": ADMIN_KEY.name, "Content-Type": "application/json" },
          body: JSON.stringify({ role: "publisher" }),
        }),
        env as any
      );
      expect(create.status).toBe(201);
      const cbody = await create.json();
      expect(cbody.key).toMatch(/^atlas_/);

      const list = await app.fetch(req("/api/v1/admin/keys", { headers: { "X-API-Key": ADMIN_KEY.name } }), env as any);
      expect(list.status).toBe(200);
      const lbody = await list.json();
      expect(lbody.keys.length).toBeGreaterThanOrEqual(1);
      expect(lbody.keys[0].key_prefix).toContain("****");

      const revoke = await app.fetch(
        req(`/api/v1/admin/keys/${cbody.key}`, { method: "DELETE", headers: { "X-API-Key": ADMIN_KEY.name } }),
        env as any
      );
      expect(revoke.status).toBe(200);
    });

    it("forbids key creation for non-admins (403)", async () => {
      const r = await app.fetch(
        req("/api/v1/admin/keys", {
          method: "POST",
          headers: { "X-API-Key": VIEWER_KEY.name, "Content-Type": "application/json" },
          body: JSON.stringify({ role: "publisher" }),
        }),
        env as any
      );
      expect(r.status).toBe(403);
    });
  });

  describe("backup / export / import", () => {
    it("exports and re-imports packages", async () => {
      await app.fetch(
        req("/api/v1/publish", {
          method: "POST",
          headers: { "X-API-Key": ADMIN_KEY.name, "Content-Type": "application/json" },
          body: JSON.stringify({ name: "bk", version: "1.0.0", description: "d", tags: [], files: { "a": "b" } }),
        }),
        env as any
      );
      const exp = await app.fetch(req("/api/v1/admin/export", { headers: { "X-API-Key": ADMIN_KEY.name } }), env as any);
      expect(exp.status).toBe(200);
      const dump = await exp.json();
      expect(dump.packages.length).toBe(1);

      // import into a fresh env
      const e2 = makeEnv();
      await e2.kv.put(`apikey:${ADMIN_KEY.name}`, ADMIN_KEY.value);
      const imp = await app.fetch(
        req("/api/v1/admin/import", {
          method: "POST",
          headers: { "X-API-Key": ADMIN_KEY.name, "Content-Type": "application/json" },
          body: JSON.stringify(dump),
        }),
        e2.env as any
      );
      expect(imp.status).toBe(201);
      const ib = await imp.json();
      expect(ib.imported).toBe(1);
      const restored = await app.fetch(req("/api/v1/packages/bk", { headers: { "X-API-Key": ADMIN_KEY.name } }), e2.env as any);
      expect(restored.status).toBe(200);
    });
  });

  describe("metrics", () => {
    it("returns Prometheus text by default", async () => {
      await app.fetch(
        req("/api/v1/publish", {
          method: "POST",
          headers: { "X-API-Key": ADMIN_KEY.name, "Content-Type": "application/json" },
          body: JSON.stringify({ name: "m1", version: "1.0.0", description: "d", tags: [], files: { "a": "b" } }),
        }),
        env as any
      );
      const r = await app.fetch(req("/metrics", { headers: { "X-API-Key": ADMIN_KEY.name } }), env as any);
      expect(r.status).toBe(200);
      expect(r.headers.get("Content-Type")).toContain("text/plain");
      const text = await r.text();
      expect(text).toContain("atlas_total_packages 1");
      expect(text).toContain('atlas_org_packages{org="acme"} 1');
    });

    it("returns JSON when requested", async () => {
      const r = await app.fetch(req("/metrics", { headers: { "X-API-Key": ADMIN_KEY.name, Accept: "application/json" } }), env as any);
      const data = await r.json();
      expect(data.total_packages).toBe(0);
    });
  });

  describe("rate limiting", () => {
    it("blocks after 60 publishes with the same key (429 + headers)", async () => {
      let lastStatus = 200;
      for (let i = 0; i < 61; i++) {
        const r = await app.fetch(
          req("/api/v1/publish", {
            method: "POST",
            headers: { "X-API-Key": ADMIN_KEY.name, "Content-Type": "application/json" },
            body: JSON.stringify({ name: `rl-${i}`, version: "1.0.0", description: "d", tags: [], files: { "a": "b" } }),
          }),
          env as any
        );
        lastStatus = r.status;
        if (r.status === 429) {
          expect(r.headers.get("X-RateLimit-Remaining")).toBe("0");
          expect(r.headers.get("X-RateLimit-Limit")).toBe("60");
          break;
        }
      }
      expect(lastStatus).toBe(429);
    });
  });

  describe("expired keys", () => {
    it("rejects an expired key with 401", async () => {
      await kv.put(
        "apikey:expired-key",
        JSON.stringify({ org: "acme", role: "admin", expires_at: "2000-01-01T00:00:00Z" })
      );
      const r = await app.fetch(
        req("/api/v1/publish", {
          method: "POST",
          headers: { "X-API-Key": "expired-key", "Content-Type": "application/json" },
          body: JSON.stringify({ name: "x", version: "1", description: "d", tags: [], files: { "a": "b" } }),
        }),
        env as any
      );
      expect(r.status).toBe(401);
    });
  });

  it("serves the openapi spec", async () => {
    const r = await app.fetch(req("/api/v1/openapi"), env as any);
    const spec = await r.json();
    expect(spec.openapi).toBe("3.0.3");
    expect(spec.paths["/metrics"]).toBeDefined();
    expect(spec.paths["/api/v1/admin/keys"]).toBeDefined();
  });
});
