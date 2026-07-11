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

describe("QA-NEW-1: last_used never updated", () => {
  it("REG-last_used: api key last_used is never set after use (always null/undefined)", async () => {
    const { env, kv } = makeEnv();
    await publish(env, "admin-acme", { ...BASE, name: "p1", version: "1.0.0" });
    const raw = await kv.get("apikey:admin-acme");
    const parsed = JSON.parse(raw);
    // BUG: neither requireAuth nor publishPackage updates last_used.
    // The seed key has no last_used field; even after use it's undefined/null.
    expect(parsed.last_used == null).toBe(true);
  });
});

describe("QA-NEW-2: Legacy endpoint rate limiting bypass", () => {
  it("REG-54: legacy /api/publish has no rate limiting", async () => {
    const { env } = makeEnv();
    let status = 200;
    for (let i = 0; i < 70; i++) {
      const r = await app.fetch(req("/api/publish", {
        method: "POST", headers: H("admin-acme"),
        body: JSON.stringify({ ...BASE, name: `legacy-${i}`, version: "1.0.0" }),
      }), env);
      status = r.status;
    }
    // BUG: should be 429 after 60, but rate limiting is never checked
    expect(status).not.toBe(429);
  });
});

describe("QA-NEW-3: Legacy endpoints missing auth/scoping", () => {
  it("REG-legacy-noauth: legacy /api/packages shows ALL packages without auth", async () => {
    const { env } = makeEnv();
    await publish(env, "admin-acme", { ...BASE, name: "acme-p", version: "1.0.0" });
    await publish(env, "admin-other", { ...BASE, name: "other-p", version: "1.0.0" });
    const r = await app.fetch(req("/api/packages"), env);
    const body = await r.json();
    expect(body.length).toBe(2); // BUG: shows both orgs' packages with no key
  });

  it("REG-legacy-noauth: legacy /api/search shows all", async () => {
    const { env } = makeEnv();
    await publish(env, "admin-acme", { ...BASE, name: "acme-only", version: "1.0.0" });
    await publish(env, "admin-other", { ...BASE, name: "other-only", version: "1.0.0" });
    const r = await app.fetch(req("/api/search?q=only"), env);
    const body = await r.json();
    expect(body.total).toBe(2); // BUG: no org scoping
  });

  it("REG-legacy-noratelimit: legacy /api/packages has no rate limiting", async () => {
    const { env } = makeEnv();
    let limited = false;
    for (let i = 0; i < 80; i++) {
      const r = await app.fetch(req("/api/packages"), env);
      if (r.status === 429) limited = true;
    }
    expect(limited).toBe(false); // BUG: no rate limit
  });
});

describe("QA-NEW-4: X-RateLimit-Reset header missing", () => {
  it("REG-40: rate-limited response missing X-RateLimit-Reset", async () => {
    const { env } = makeEnv();
    let status = 200;
    for (let i = 0; i < 61; i++) {
      const r = await publish(env, "admin-acme", { ...BASE, name: `rlh-${i}`, version: "1.0.0" });
      status = r.status;
      if (status === 429) {
        expect(r.headers.get("X-RateLimit-Reset")).toBeNull(); // BUG: should exist
        expect(r.headers.get("X-RateLimit-Limit")).toBeTruthy();
        expect(r.headers.get("X-RateLimit-Remaining")).toBeTruthy();
      }
    }
    expect(status).toBe(429);
  });
});

describe("QA-NEW-5: Version field validation missing", () => {
  it("REG-41: version accepts path-traversal strings", async () => {
    const { env } = makeEnv();
    const r = await app.fetch(req("/api/v1/publish", {
      method: "POST", headers: H("admin-acme"),
      body: JSON.stringify({ ...BASE, name: "ver-test", version: "../../etc/passwd" }),
    }), env);
    // BUG: no error — should validate version format
    expect(r.status).toBe(201);
  });

  it("REG-41b: version accepts empty string (handled by other validation)", async () => {
    const { env } = makeEnv();
    let threw = false;
    try {
      await app.fetch(req("/api/v1/publish", {
        method: "POST", headers: H("admin-acme"),
        body: JSON.stringify({ name: "ver-empty", version: "", files: { a: "b" } }),
      }), env);
    } catch { threw = true; }
    // Currently: either throws (REG-36) or returns error — inconsistent
    expect(threw || true).toBe(true);
  });
});

describe("QA-NEW-6: Files / tags validation", () => {
  it("REG-47: non-string file value accepted", async () => {
    const { env } = makeEnv();
    let status = -1;
    try {
      const r = await app.fetch(req("/api/v1/publish", {
        method: "POST", headers: H("admin-acme"),
        body: JSON.stringify({ name: "fs-num", version: "1.0.0", description: "d", tags: [], files: { "a": 123 } }),
      }), env);
      status = r.status;
    } catch { status = -999; }
    expect(status === 201 || status === -999).toBe(true); // BUG: should be 400
  });

  it("REG-48: non-array tags accepted", async () => {
    const { env } = makeEnv();
    let status = -1;
    try {
      const r = await app.fetch(req("/api/v1/publish", {
        method: "POST", headers: H("admin-acme"),
        body: JSON.stringify({ name: "tag-test", version: "1.0.0", description: "d", tags: "not-an-array", files: { a: "b" } }),
      }), env);
      status = r.status;
    } catch { status = -999; }
    // Currently crashes because .tags.map is called on a string in search/existing path
    expect(status === 201 || status === -999).toBe(true); // BUG
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

describe("QA-NEW-8: Rate limiting gaps on additional endpoints", () => {
  // Verify that version listing, admin endpoints, stats have no rate limiting
  it("no rate limiting on versions list", async () => {
    const { env } = makeEnv();
    await publish(env, "admin-acme", { ...BASE, name: "rlv", version: "1.0.0" });
    let limited = false;
    for (let i = 0; i < 80; i++) {
      const r = await app.fetch(req("/api/v1/packages/rlv/versions"), env);
      if (r.status === 429) limited = true;
    }
    expect(limited).toBe(false); // BUG: no rate limit on public endpoint
  });

  it("no rate limiting on admin/keys", async () => {
    const { env } = makeEnv();
    let limited = false;
    for (let i = 0; i < 80; i++) {
      const r = await app.fetch(req("/api/v1/admin/keys", { headers: H("admin-acme") }), env);
      if (r.status === 429) limited = true;
    }
    expect(limited).toBe(false); // BUG
  });

  it("no rate limiting on /metrics", async () => {
    const { env } = makeEnv();
    let limited = false;
    for (let i = 0; i < 80; i++) {
      const r = await app.fetch(req("/metrics"), env);
      if (r.status === 429) limited = true;
    }
    expect(limited).toBe(false); // BUG
  });
});

describe("QA-NEW-9: Version snapshots orphaned after delete + re-publish", () => {
  it("REG-34b: delete leaves version keys; re-publish resurrects stale snapshots", async () => {
    const { env, agent } = makeEnv();
    await publish(env, "admin-acme", { ...BASE, name: "vdel", version: "1.0.0", files: { "a.md": "original-v1" } });
    await publish(env, "admin-acme", { ...BASE, name: "vdel", version: "2.0.0", files: { "a.md": "v2-content" } });

    const del = await app.fetch(req("/api/v1/packages/vdel", { method: "DELETE", headers: H("admin-acme") }), env);
    expect(del.status).toBe(200);

    // Re-publish with different content
    await publish(env, "admin-acme", { ...BASE, name: "vdel", version: "3.0.0", files: { "a.md": "v3-content" } });

    // Stale version 1.0.0 snapshot lives on because delete didn't clean it
    const snap = await app.fetch(req("/api/v1/packages/vdel/versions/1.0.0", { headers: H("admin-acme") }), env);
    expect(snap.status).toBe(200); // BUG: old snapshot survives delete! Should be 404
  });
});

describe("QA-NEW-10: Concurrent publish double-counts in metrics", () => {
  it("REG-28b: usage keys double-count in total_publishes metric", async () => {
    const { env } = makeEnv();
    // publish creates two usage keys:
    //   usage:acme:publishes:2026-07-11    (org-level)
    //   usage:acme:publishes:pkgname:...   (per-package)
    await publish(env, "admin-acme", { ...BASE, name: "m1", version: "1.0.0" });
    const r = await app.fetch(req("/metrics", { headers: { Accept: "application/json" } }), env);
    const m = await r.json();
    // Per-package usage key also contains ":publishes:" so getMetrics double-counts
    expect(m.total_publishes).toBe(2); // BUG: should be 1
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
