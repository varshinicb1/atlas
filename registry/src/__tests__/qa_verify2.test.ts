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
        .filter((k) => !opts?.prefix || k.startsWith(opts?.prefix))
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

function makeEnv(extraKeys: Record<string, object> = {}) {
  const kv = makeMockKV();
  const agent = new RegistryAgent({} as any, { PACKAGES: kv } as any);
  const env = {
    PACKAGES: kv,
    RegistryAgent: { idFromName: (_n: string) => "default", get: () => agent },
  } as unknown as TestEnv;
  const keys: Record<string, object> = {
    "admin-acme": { org: "acme", role: "admin" },
    "admin-other": { org: "otherco", role: "admin" },
    "publisher-acme": { org: "acme", role: "publisher" },
    "publisher-other": { org: "otherco", role: "publisher" },
    ...extraKeys,
  };
  for (const [k, v] of Object.entries(keys)) kv.put(`apikey:${k}`, JSON.stringify(v));
  return { env, kv, agent };
}

function req(path: string, init?: RequestInit) {
  return new Request(`https://registry.test${path}`, init);
}
const H = (key: string) => ({ "X-API-Key": key, "Content-Type": "application/json" });
async function publish(env: any, key: string, pkg: any) {
  return app.fetch(req("/api/v1/publish", {
    method: "POST", headers: H(key), body: JSON.stringify(pkg),
  }), env);
}
const BASE = { description: "d", tags: [] as string[], files: { "a.md": "b" } };

describe("QA-verify: additional attack surface (FIXED)", () => {
  it("REG-37: cross-tenant EXPORT blocked (FIXED)", async () => {
    const { env } = makeEnv();
    await publish(env, "admin-acme", { ...BASE, name: "secret", version: "1.0.0", files: { "k.md": "TOPSECRET" } });
    const r = await app.fetch(req("/api/v1/admin/export", { headers: H("admin-other") }), env);
    const dump = await r.json();
    const found = dump.packages.find((p: any) => p.metadata.name === "secret");
    expect(found).toBeFalsy();
  });

  it("REG-38: org-less package takeover BLOCKED (FIXED)", async () => {
    const { env } = makeEnv();
    const dump = {
      version: "1.0", exported_at: new Date().toISOString(),
      packages: [{ metadata: { name: "legacy", version: "1.0.0", description: "d", tags: [], download_count: 0, published_at: new Date().toISOString(), updated_at: new Date().toISOString(), compiler_version: "0.1.0" }, files: { "a.md": "orig" }, versions: ["1.0.0"] }],
    };
    await app.fetch(req("/api/v1/admin/import", { method: "POST", headers: H("admin-acme"), body: JSON.stringify(dump) }), env);
    const r = await publish(env, "publisher-other", { ...BASE, name: "legacy", version: "2.0.0", files: { "a.md": "Hijacked by otherco" } });
    expect(r.status).toBe(403);
    const got = await app.fetch(req("/api/v1/packages/legacy", { headers: H("admin-acme") }), env);
    expect((await got.json()).metadata.org).toBe("acme");
  });

  it("REG-39: /metrics requires auth (FIXED)", async () => {
    const { env } = makeEnv();
    await publish(env, "admin-acme", { ...BASE, name: "m1", version: "1.0.0" });
    await publish(env, "admin-other", { ...BASE, name: "m2", version: "1.0.0" });
    const r = await app.fetch(req("/metrics"), env);
    expect(r.status).toBe(401);
  });

  it("REG-42: audit entries now record the API key (FIXED)", async () => {
    const { env, kv } = makeEnv();
    await publish(env, "admin-acme", { ...BASE, name: "aud", version: "1.0.0" });
    const list = await app.fetch(req("/api/v1/org/audit", { headers: H("admin-acme") }), env);
    const body = await list.json();
    expect(body.entries[0].api_key).toBe("admin-acme");
  });

  it("REG-43: malformed apikey on PROTECTED endpoint returns 401 (FIXED)", async () => {
    const { env, kv } = makeEnv();
    kv.put("apikey:broken", "not-json");
    const r = await app.fetch(req("/api/v1/org/stats", { headers: H("broken") }), env);
    expect(r.status).toBe(401);
  });

  it("REG-43b: malformed apikey on public list endpoint degrades to anonymous (200)", async () => {
    const { env, kv } = makeEnv();
    await publish(env, "admin-acme", { ...BASE, name: "pubonly", version: "1.0.0" });
    kv.put("apikey:broken", "not-json");
    const r = await app.fetch(req("/api/v1/packages", { headers: H("broken") }), env);
    expect(r.status).toBe(200);
    // Falls through to anonymous — only public packages shown (none have no org)
    const body = await r.json();
    expect(body.length).toBe(0);
  });

  it("REG-44: publish cross-tenant returns clean 403 (FIXED)", async () => {
    const { env } = makeEnv();
    await publish(env, "admin-acme", { ...BASE, name: "owned", version: "1.0.0" });
    const r = await publish(env, "publisher-other", { ...BASE, name: "owned", version: "2.0.0" });
    expect(r.status).toBe(403);
  });

  it("REG-45: cross-tenant DELETE is blocked (control)", async () => {
    const { env } = makeEnv();
    await publish(env, "admin-acme", { ...BASE, name: "del", version: "1.0.0" });
    const r = await app.fetch(req("/api/v1/packages/del", { method: "DELETE", headers: H("admin-other") }), env);
    expect(r.status).toBe(403);
  });
});
