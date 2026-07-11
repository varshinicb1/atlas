/**
 * Additional QA probes — run via: npx vitest run qa/registry/qa_additional.test.ts
 * or copy into src/__tests__/ for auto-discovery.
 *
 * Covers new issues NOT in qa_registry_security.test.ts / qa_verify2.test.ts.
 */
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
    "viewer-acme": { org: "acme", role: "viewer" },
    ...extraKeys,
  };
  for (const [k, v] of Object.entries(keys)) kv.put(`apikey:${k}`, JSON.stringify(v));
  return { env, kv, agent };
}

function req(path: string, init?: RequestInit) {
  return new Request(`https://registry.test${path}`, init);
}
const H = (key: string) => ({ "X-API-Key": key, "Content-Type": "application/json" });
const BASE = { description: "d", tags: [] as string[], files: { "a.md": "b" } };
async function publish(env: any, key: string, pkg: any) {
  return app.fetch(req("/api/v1/publish", {
    method: "POST", headers: H(key), body: JSON.stringify(pkg),
  }), env);
}

// ─── NEW ISSUES ───────────────────────────────────────────────────────────

describe("QA-NEW-1: last_used updated (FIXED)", () => {
  it("REG-last_used: api key last_used is set after use", async () => {
    const { env, kv } = makeEnv();
    await publish(env, "admin-acme", { ...BASE, name: "p1", version: "1.0.0" });
    const raw = await kv.get("apikey:admin-acme");
    const parsed = JSON.parse(raw);
    expect(parsed.last_used).not.toBeNull();
  });
});

describe("QA-NEW-2: Legacy endpoint rate limiting (FIXED)", () => {
  it("REG-54: legacy /api/publish now rate limited", async () => {
    const { env } = makeEnv();
    let status = 200;
    for (let i = 0; i < 70; i++) {
      const r = await app.fetch(req("/api/publish", {
        method: "POST", headers: H("admin-acme"),
        body: JSON.stringify({ ...BASE, name: `legacy-${i}`, version: "1.0.0" }),
      }), env);
      status = r.status;
    }
    expect(status).toBe(429);
  });
});

describe("QA-NEW-3: Legacy endpoints auth/scoping (FIXED)", () => {
  it("REG-legacy-noauth: legacy /api/packages shows only public packages", async () => {
    const { env } = makeEnv();
    await publish(env, "admin-acme", { ...BASE, name: "acme-p", version: "1.0.0" });
    await publish(env, "admin-other", { ...BASE, name: "other-p", version: "1.0.0" });
    const r = await app.fetch(req("/api/packages"), env);
    const body = await r.json();
    expect(body.length).toBe(0); // All packages have orgs; none shown without auth
  });

  it("REG-legacy-noauth: legacy /api/search shows only caller's org", async () => {
    const { env } = makeEnv();
    await publish(env, "admin-acme", { ...BASE, name: "acme-only", version: "1.0.0" });
    await publish(env, "admin-other", { ...BASE, name: "other-only", version: "1.0.0" });
    const r = await app.fetch(req("/api/search?q=only"), env);
    const body = await r.json();
    expect(body.total).toBe(0); // No auth — no org scoping, all packages have orgs
  });

  it("REG-legacy-noratelimit: legacy /api/packages now rate limited", async () => {
    const { env } = makeEnv();
    let limited = false;
    for (let i = 0; i < 80; i++) {
      const r = await app.fetch(req("/api/packages"), env);
      if (r.status === 429) limited = true;
    }
    expect(limited).toBe(true);
  });
});

describe("QA-NEW-4: X-RateLimit-Reset header (FIXED)", () => {
  it("REG-40: rate-limited response includes X-RateLimit-Reset", async () => {
    const { env } = makeEnv();
    let status = 200;
    for (let i = 0; i < 61; i++) {
      const r = await publish(env, "admin-acme", { ...BASE, name: `rlh-${i}`, version: "1.0.0" });
      status = r.status;
      if (status === 429) {
        expect(r.headers.get("X-RateLimit-Reset")).toBeTruthy();
        expect(r.headers.get("X-RateLimit-Limit")).toBeTruthy();
        expect(r.headers.get("X-RateLimit-Remaining")).toBeTruthy();
      }
    }
    expect(status).toBe(429);
  });
});

describe("QA-NEW-5: Version field validation (FIXED)", () => {
  it("REG-41: version rejects path-traversal strings (400)", async () => {
    const { env } = makeEnv();
    const r = await app.fetch(req("/api/v1/publish", {
      method: "POST", headers: H("admin-acme"),
      body: JSON.stringify({ ...BASE, name: "ver-test", version: "../../etc/passwd" }),
    }), env);
    expect(r.status).toBe(400);
  });

  it("REG-41b: empty version returns 400 (FIXED)", async () => {
    const { env } = makeEnv();
    const r = await app.fetch(req("/api/v1/publish", {
      method: "POST", headers: H("admin-acme"),
      body: JSON.stringify({ name: "ver-empty", version: "", files: { a: "b" } }),
    }), env);
    expect(r.status).toBe(400);
  });
});

describe("QA-NEW-6: Files / tags validation (FIXED)", () => {
  it("REG-47: non-string file value rejected (400)", async () => {
    const { env } = makeEnv();
    const r = await app.fetch(req("/api/v1/publish", {
      method: "POST", headers: H("admin-acme"),
      body: JSON.stringify({ name: "fs-num", version: "1.0.0", description: "d", tags: [], files: { "a": 123 } }),
    }), env);
    expect(r.status).toBe(400);
  });

  it("REG-48: non-array tags rejected (400)", async () => {
    const { env } = makeEnv();
    const r = await app.fetch(req("/api/v1/publish", {
      method: "POST", headers: H("admin-acme"),
      body: JSON.stringify({ name: "tag-test", version: "1.0.0", description: "d", tags: "not-an-array", files: { a: "b" } }),
    }), env);
    expect(r.status).toBe(400);
  });
});

describe("QA-NEW-7: Multiple content types and edge-case auth", () => {
  it("REG-46: no Content-Type check — mock accepts JSON without header (real Workers rejects)", async () => {
    const { env } = makeEnv();
    // In the test mock, Request.json() parses body without Content-Type check.
    // Real Workers requires application/json. Both behaviors are a concern:
    //   mock is too permissive (masking a potential 415 in production).
    // We note this as a gap: no explicit Content-Type: application/json validation.
    const r = await app.fetch(req("/api/v1/publish", {
      method: "POST",
      headers: { "X-API-Key": "admin-acme" },
      body: JSON.stringify({ ...BASE, name: "ct-test", version: "1.0.0" }),
    }), env);
    // In the test mock this succeeds (201); in real Workers might fail (415 or 500).
    // The code should explicitly validate Content-Type.
    expect(r.status).toBe(201); // mock succeeds — real production behavior may differ
  });

  it("Authorization: Bearer with empty token", async () => {
    const { env } = makeEnv();
    const r = await app.fetch(req("/api/v1/publish", {
      method: "POST",
      headers: { Authorization: "Bearer ", "Content-Type": "application/json" },
      body: JSON.stringify({ ...BASE, name: "bt", version: "1.0.0" }),
    }), env);
    expect(r.status).toBe(401); // empty Bearer token -> no key -> 401 (correct)
  });

  it("X-API-Key header with empty value", async () => {
    const { env } = makeEnv();
    const r = await app.fetch(req("/api/v1/publish", {
      method: "POST",
      headers: { "X-API-Key": "", "Content-Type": "application/json" },
      body: JSON.stringify({ ...BASE, name: "empty-key", version: "1.0.0" }),
    }), env);
    expect(r.status).toBe(401); // correct — empty key treated as missing
  });
});

describe("QA-NEW-8: Rate limiting gaps on additional endpoints (FIXED)", () => {
  it("rate limiting on versions list", async () => {
    const { env } = makeEnv();
    await publish(env, "admin-acme", { ...BASE, name: "rlv", version: "1.0.0" });
    let limited = false;
    for (let i = 0; i < 80; i++) {
      const r = await app.fetch(req("/api/v1/packages/rlv/versions"), env);
      if (r.status === 429) limited = true;
    }
    expect(limited).toBe(true);
  });

  it("rate limiting on admin/keys", async () => {
    const { env } = makeEnv();
    let limited = false;
    for (let i = 0; i < 80; i++) {
      const r = await app.fetch(req("/api/v1/admin/keys", { headers: H("admin-acme") }), env);
      if (r.status === 429) limited = true;
    }
    expect(limited).toBe(true);
  });

  it("rate limiting on /metrics (requires auth first)", async () => {
    const { env } = makeEnv();
    let limited = false;
    for (let i = 0; i < 80; i++) {
      const r = await app.fetch(req("/metrics", { headers: { "X-API-Key": "admin-acme" } }), env);
      if (r.status === 429) limited = true;
    }
    expect(limited).toBe(true);
  });
});

describe("QA-NEW-9: Version snapshots cleaned up on delete (FIXED)", () => {
  it("REG-34b: delete removes version keys; stale snapshots gone", async () => {
    const { env, agent } = makeEnv();
    await publish(env, "admin-acme", { ...BASE, name: "vdel", version: "1.0.0", files: { "a.md": "original-v1" } });
    await publish(env, "admin-acme", { ...BASE, name: "vdel", version: "2.0.0", files: { "a.md": "v2-content" } });

    const del = await app.fetch(req("/api/v1/packages/vdel", { method: "DELETE", headers: H("admin-acme") }), env);
    expect(del.status).toBe(200);

    await publish(env, "admin-acme", { ...BASE, name: "vdel", version: "3.0.0", files: { "a.md": "v3-content" } });

    // Old snapshot deleted — should 404 (or 200 if the package no longer exists as a distinct version snapshot)
    const snap = await app.fetch(req("/api/v1/packages/vdel/versions/1.0.0", { headers: H("admin-acme") }), env);
    expect(snap.status).toBe(404);
  });
});

describe("QA-NEW-10: Metrics no longer double-counts (FIXED)", () => {
  it("REG-28b: total_publishes is correct (1, not 2)", async () => {
    const { env } = makeEnv();
    await publish(env, "admin-acme", { ...BASE, name: "m1", version: "1.0.0" });
    const r = await app.fetch(req("/metrics", { headers: { "X-API-Key": "admin-acme", Accept: "application/json" } }), env);
    const m = await r.json();
    expect(m.total_publishes).toBe(1);
  });
});

describe("QA-NEW-11: Admin endpoint response inconsistency", () => {
  it("import with empty dump succeeds with 0 imported", async () => {
    const { env } = makeEnv();
    const r = await app.fetch(req("/api/v1/admin/import", {
      method: "POST", headers: H("admin-acme"),
      body: JSON.stringify({ version: "1.0", exported_at: new Date().toISOString(), packages: [] }),
    }), env);
    expect(r.status).toBe(201);
    const body = await r.json();
    expect(body.imported).toBe(0); // edge case: valid but empty
  });
});
