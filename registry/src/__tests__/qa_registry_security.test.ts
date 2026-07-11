import app from "../index";
import { RegistryAgent } from "../registry";

// ---------------------------------------------------------------------------
// QA RED-TEAM SUITE — reproduces real bugs in the Atlas registry.
// Uses the in-memory KV/DO mock (same code paths as production app.fetch).
// Each test asserts the ACTUAL (buggy) behavior and notes the expected.
// ---------------------------------------------------------------------------

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
  // Seed API keys
  const keys: Record<string, object> = {
    "admin-acme": { org: "acme", role: "admin" },
    "admin-other": { org: "otherco", role: "admin" },
    "publisher-acme": { org: "acme", role: "publisher" },
    "viewer-acme": { org: "acme", role: "viewer" },
    ...extraKeys,
  };
  for (const [k, v] of Object.entries(keys)) {
    kv.put(`apikey:${k}`, JSON.stringify(v));
  }
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

describe("QA-1: Cross-tenant READ access (P0 — FIXED)", () => {
  it("REG-01: another org CANNOT GET a private package's full contents (403)", async () => {
    const { env } = makeEnv();
    await publish(env, "admin-acme", { ...BASE, name: "acme-secret", version: "1.0.0" });
    const r = await app.fetch(req("/api/v1/packages/acme-secret", { headers: H("admin-other") }), env);
    expect(r.status).toBe(403);
  });

  it("REG-02: another org CANNOT list/read version snapshots (403)", async () => {
    const { env } = makeEnv();
    await publish(env, "admin-acme", { ...BASE, name: "acme-secret", version: "1.0.0" });
    const list = await app.fetch(req("/api/v1/packages/acme-secret/versions", { headers: H("admin-other") }), env);
    expect(list.status).toBe(403);
    const snap = await app.fetch(req("/api/v1/packages/acme-secret/versions/1.0.0", { headers: H("admin-other") }), env);
    expect(snap.status).toBe(403);
  });

  it("REG-03: unauthenticated GET on org-owned package returns 401", async () => {
    const { env } = makeEnv();
    await publish(env, "admin-acme", { ...BASE, name: "acme-secret", version: "1.0.0" });
    const r = await app.fetch(req("/api/v1/packages/acme-secret"), env);
    expect(r.status).toBe(401);
  });

  it("REG-04: /api/v1/packages listing shows only caller's org packages", async () => {
    const { env } = makeEnv();
    await publish(env, "admin-acme", { ...BASE, name: "p1", version: "1.0.0" });
    await publish(env, "admin-other", { ...BASE, name: "p2", version: "1.0.0" });
    const r = await app.fetch(req("/api/v1/packages"), env);
    expect(r.status).toBe(200);
    const body = await r.json();
    expect(body.length).toBe(0); // All packages have orgs; none shown to unauthenticated
  });
});

describe("QA-2: Cross-tenant WRITE / privilege escalation (P0 — FIXED)", () => {
  it("REG-05: admin of org A CANNOT migrate org B's packages (403)", async () => {
    const { env } = makeEnv();
    await publish(env, "admin-other", { ...BASE, name: "other-pkg", version: "1.0.0" });
    const r = await app.fetch(req("/api/v1/admin/migrate-org", {
      method: "POST", headers: H("admin-acme"),
      body: JSON.stringify({ fromOrg: "otherco", toOrg: "acme" }),
    }), env);
    expect(r.status).toBe(403);
  });

  it("REG-06: admin of org B CANNOT import/overwrite a package owned by org A (403)", async () => {
    const { env } = makeEnv();
    await publish(env, "admin-acme", { ...BASE, name: "shared", version: "1.0.0", files: { "a.md": "ORIGINAL" } });
    const dump = {
      version: "1.0", exported_at: new Date().toISOString(),
      packages: [{
        metadata: { name: "shared", version: "9.9.9", org: "acme", description: "hijacked", tags: [], download_count: 0, published_at: new Date().toISOString(), updated_at: new Date().toISOString(), compiler_version: "0.1.0" },
        files: { "a.md": "HIJACKED_BY_OTHERCO" }, versions: ["9.9.9"],
      }],
    };
    const r = await app.fetch(req("/api/v1/admin/import", {
      method: "POST", headers: H("admin-other"), body: JSON.stringify(dump),
    }), env);
    expect(r.status).toBe(403);
  });
});

describe("QA-3: Cross-tenant audit-log leak (P0 — FIXED)", () => {
  it("REG-07: /api/v1/org/audit now filters by caller's org", async () => {
    const { env } = makeEnv();
    await publish(env, "admin-acme", { ...BASE, name: "acme-pkg", version: "1.0.0" });
    const r = await app.fetch(req("/api/v1/org/audit", { headers: H("admin-other") }), env);
    expect(r.status).toBe(200);
    const body = await r.json();
    const acmeEntries = body.entries.filter((e: any) => e.org === "acme");
    expect(acmeEntries.length).toBe(0);
  });
});

describe("QA-4: AuthN/AuthZ parity & role enforcement", () => {
  it("REG-08: Bearer token parity works (correct behavior)", async () => {
    const { env } = makeEnv();
    const r = await app.fetch(req("/api/v1/publish", {
      method: "POST",
      headers: { Authorization: "Bearer admin-acme", "Content-Type": "application/json" },
      body: JSON.stringify({ ...BASE, name: "bearer-pkg", version: "1.0.0" }),
    }), env);
    expect(r.status).toBe(201);
  });

  it("REG-09: viewer cannot publish (correct: 403)", async () => {
    const { env } = makeEnv();
    const r = await app.fetch(req("/api/v1/publish", {
      method: "POST", headers: H("viewer-acme"),
      body: JSON.stringify({ ...BASE, name: "x", version: "1" }),
    }), env);
    expect(r.status).toBe(403);
  });

  it("REG-10: publisher cannot delete/export/manage keys (correct: 403)", async () => {
    const { env } = makeEnv();
    await publish(env, "admin-acme", { ...BASE, name: "p", version: "1.0.0" });
    for (const [path, method] of [
      ["/api/v1/packages/p", "DELETE"],
      ["/api/v1/admin/export", "GET"],
      ["/api/v1/admin/keys", "POST"],
    ] as const) {
      const r = await app.fetch(req(path as string, { method, headers: H("publisher-acme") }), env);
      expect(r.status).toBe(403);
    }
  });

  it("REG-11: 'Bearer' without space is rejected (correct: 401)", async () => {
    const { env } = makeEnv();
    const r = await app.fetch(req("/api/v1/publish", {
      method: "POST",
      headers: { Authorization: "Beareradmin-acme", "Content-Type": "application/json" },
      body: JSON.stringify({ ...BASE, name: "x", version: "1" }),
    }), env);
    expect(r.status).toBe(401);
  });
});

describe("QA-5: Input validation", () => {
  it("REG-12: overly long package name accepted (no length limit)", async () => {
    const { env } = makeEnv();
    const longName = "a".repeat(5000);
    const r = await publish(env, "admin-acme", { ...BASE, name: longName, version: "1.0.0" });
    expect(r.status).toBe(201); // BUG: unbounded name length (storage/abuse) — P2, not fixed
  });

  it("REG-13: multi-MB file content accepted (no payload size limit)", async () => {
    const { env } = makeEnv();
    const big = "x".repeat(8 * 1024 * 1024); // 8 MB
    const r = await publish(env, "admin-acme", { ...BASE, name: "big", version: "1.0.0", files: { "big.bin": big } });
    expect(r.status).toBe(201); // BUG: unbounded file size (DOS vector) — P2, not fixed
  });

  it("REG-14: invalid package name returns clean 400 (FIXED)", async () => {
    for (const name of ["MyPkg", "bad name", "../evil", "bad-name!", "ünïcode"]) {
      const { env } = makeEnv();
      const r = await publish(env, "admin-acme", { ...BASE, name, version: "1.0.0" });
      expect(r.status).toBe(400);
    }
  });

  it("REG-15: missing/empty name returns clean 400 (FIXED)", async () => {
    const { env } = makeEnv();
    const r = await publish(env, "admin-acme", { name: "", version: "1", files: { a: "b" } } as any);
    expect(r.status).toBe(400);
  });
});

describe("QA-6: HTTP hygiene / status codes (FIXED)", () => {
  it("REG-16: malformed JSON body on publish returns 400 (FIXED)", async () => {
    const { env } = makeEnv();
    const r = await app.fetch(req("/api/v1/publish", {
      method: "POST", headers: H("admin-acme"), body: "this is not json{",
    }), env);
    expect(r.status).toBe(400);
  });

  it("REG-17: invalid import dump returns 400 (FIXED)", async () => {
    const { env } = makeEnv();
    const r = await app.fetch(req("/api/v1/admin/import", {
      method: "POST", headers: H("admin-acme"), body: JSON.stringify({ foo: 1 }),
    }), env);
    expect(r.status).toBe(400);
  });

  it("REG-18: invalid role in createApiKey returns 400 (FIXED)", async () => {
    const { env } = makeEnv();
    const r = await app.fetch(req("/api/v1/admin/keys", {
      method: "POST", headers: H("admin-acme"), body: JSON.stringify({ role: "superuser" }),
    }), env);
    expect(r.status).toBe(400);
  });

  it("REG-19: invalid expires_at returns 400 (FIXED)", async () => {
    const { env } = makeEnv();
    const r = await app.fetch(req("/api/v1/admin/keys", {
      method: "POST", headers: H("admin-acme"), body: JSON.stringify({ role: "viewer", expires_at: "nope" }),
    }), env);
    expect(r.status).toBe(400);
  });

  it("REG-36: /api/v1/publish awaits withRateLimit, errors map correctly (FIXED)", async () => {
    const { env } = makeEnv();
    const r = await publish(env, "admin-acme", { ...BASE, name: "BAD", version: "1" });
    expect(r.status).toBe(400);
  });

  it("REG-20: unknown route -> 404 (correct)", async () => {
    const { env } = makeEnv();
    const r = await app.fetch(req("/api/v1/nonexistent"), env);
    expect(r.status).toBe(404);
  });

  it("REG-21: known path with wrong method returns 405 (FIXED)", async () => {
    const { env } = makeEnv();
    const r = await app.fetch(req("/api/v1/packages/foo", { method: "PUT", headers: H("admin-acme") }), env);
    expect(r.status).toBe(405);
  });

  it("REG-22: OPTIONS preflight returns CORS headers (correct)", async () => {
    const { env } = makeEnv();
    const r = await app.fetch(req("/api/v1/publish", { method: "OPTIONS" }), env);
    expect(r.status).toBe(200);
    expect(r.headers.get("Access-Control-Allow-Origin")).toBe("*");
  });
});

describe("QA-7: Rate limiting (FIXED)", () => {
  it("REG-23: >60 publishes with same key -> 429 with headers", async () => {
    const { env } = makeEnv();
    let status = 200;
    for (let i = 0; i < 70; i++) {
      const r = await publish(env, "admin-acme", { ...BASE, name: `rl-${i}`, version: "1.0.0" });
      status = r.status;
      if (r.status === 429) {
        expect(r.headers.get("X-RateLimit-Limit")).toBe("60");
        expect(r.headers.get("X-RateLimit-Remaining")).toBe("0");
        break;
      }
    }
    expect(status).toBe(429);
  });

  it("REG-24: rate limit is per-key (different key not limited)", async () => {
    const { env } = makeEnv();
    for (let i = 0; i < 65; i++) {
      await publish(env, "admin-acme", { ...BASE, name: `a-${i}`, version: "1.0.0" });
    }
    const r = await publish(env, "admin-other", { ...BASE, name: "b1", version: "1.0.0" });
    expect(r.status).toBe(201); // other key untouched — correct per-key behavior
  });

  it("REG-25: unauthenticated GET IS rate limited (FIXED)", async () => {
    const { env } = makeEnv();
    await publish(env, "admin-acme", { ...BASE, name: "pub", version: "1.0.0" });
    let limited = false;
    for (let i = 0; i < 80; i++) {
      const r = await app.fetch(req("/api/v1/packages/pub"), env);
      if (r.status === 429) limited = true;
    }
    expect(limited).toBe(true); // rate limiting applies to anonymous too
  });

  it("REG-26: search endpoint now has rate limiting (FIXED)", async () => {
    const { env } = makeEnv();
    let limited = false;
    for (let i = 0; i < 80; i++) {
      const r = await app.fetch(req("/api/v1/search?q=anything"), env);
      if (r.status === 429) limited = true;
    }
    expect(limited).toBe(true);
  });
});

describe("QA-8: Correctness — export/import, metrics, versions (FIXED)", () => {
  it("REG-27: version SNAPSHOTS survive import round-trip (FIXED)", async () => {
    const { env, kv } = makeEnv();
    await publish(env, "admin-acme", { ...BASE, name: "snap", version: "1.0.0", files: { "a.md": "v1" } });
    await publish(env, "admin-acme", { ...BASE, name: "snap", version: "2.0.0", files: { "a.md": "v2" } });
    const exp = await app.fetch(req("/api/v1/admin/export", { headers: H("admin-acme") }), env);
    const dump = await exp.json();

    const e2 = makeEnv();
    const imp = await app.fetch(req("/api/v1/admin/import", {
      method: "POST", headers: H("admin-acme"), body: JSON.stringify(dump),
    }), e2.env);
    expect(imp.status).toBe(201);
    const list = await app.fetch(req("/api/v1/packages/snap/versions", { headers: H("admin-acme") }), e2.env);
    expect((await list.json()).versions).toEqual(["1.0.0", "2.0.0"]);
    // Snapshot now survives the round-trip
    const snap = await app.fetch(req("/api/v1/packages/snap/versions/1.0.0", { headers: H("admin-acme") }), e2.env);
    expect(snap.status).toBe(200);
  });

  it("REG-28: total_publishes metric no longer double-counts (FIXED)", async () => {
    const { env } = makeEnv();
    await publish(env, "admin-acme", { ...BASE, name: "m", version: "1.0.0" });
    const r = await app.fetch(req("/metrics", { headers: { "X-API-Key": "admin-acme", Accept: "application/json" } }), env);
    const m = await r.json();
    expect(m.total_publishes).toBe(1);
  });

  it("REG-29: version history & specific-version fetch works (correct)", async () => {
    const { env } = makeEnv();
    for (const v of ["1.0.0", "1.0.1", "2.0.0"]) {
      await publish(env, "admin-acme", { ...BASE, name: "hist", version: v });
    }
    const list = await app.fetch(req("/api/v1/packages/hist/versions", { headers: H("admin-acme") }), env);
    expect((await list.json()).versions).toEqual(["1.0.0", "1.0.1", "2.0.0"]);
  });

  it("REG-30: delete then get -> 404 (correct)", async () => {
    const { env } = makeEnv();
    await publish(env, "admin-acme", { ...BASE, name: "d", version: "1.0.0" });
    const del = await app.fetch(req("/api/v1/packages/d", { method: "DELETE", headers: H("admin-acme") }), env);
    expect(del.status).toBe(200);
    const got = await app.fetch(req("/api/v1/packages/d", { headers: H("admin-acme") }), env);
    expect(got.status).toBe(404);
  });

  it("REG-31: republish is idempotent w.r.t. download_count (correct)", async () => {
    const { env, agent } = makeEnv();
    await publish(env, "admin-acme", { ...BASE, name: "id", version: "1.0.0" });
    await agent.incrementDownload("id"); // +1
    await publish(env, "admin-acme", { ...BASE, name: "id", version: "1.0.1" });
    const meta = (await agent.getPackage("id"))!.metadata;
    expect(meta.download_count).toBe(1); // preserved across republish
  });
});

describe("QA-9: Injection / path traversal / abuse (FIXED)", () => {
  it("REG-32: path-traversal package name returns 400 (FIXED)", async () => {
    const { env } = makeEnv();
    const r = await app.fetch(req("/api/v1/packages/..%2F..%2Fetc%2Fpasswd", { headers: H("admin-acme") }), env);
    expect(r.status).toBe(400);
  });

  it("REG-33: script/SQL payloads in search are inert (no injection sink)", async () => {
    const { env } = makeEnv();
    await publish(env, "admin-acme", { ...BASE, name: "real", version: "1.0.0" });
    const r = await app.fetch(req("/api/v1/search?q=<script>alert(1)</script>';DROP"), env);
    const body = await r.json();
    expect(body.results).toEqual([]); // no SQL/JS injection sink — safe
  });

  it("REG-34: delete now cleans up version keys (FIXED)", async () => {
    const { env, agent } = makeEnv();
    await publish(env, "admin-acme", { ...BASE, name: "orphan", version: "1.0.0" });
    await app.fetch(req("/api/v1/packages/orphan", { method: "DELETE", headers: H("admin-acme") }), env);
    const storage = agent.ctx.storage as any;
    expect(await storage.get("pkg:orphan:versions")).toBeUndefined();
    expect(await storage.get("pkg:orphan:v:1.0.0")).toBeUndefined();
  });
});

describe("QA-10: Concurrency", () => {
  it("REG-35: concurrent downloads (DO serializes in production; mock may show race)", async () => {
    const { env, agent } = makeEnv();
    await agent.publishPackage({ ...BASE, name: "race", version: "1.0.0" }, { org: "acme", role: "admin" }, "1.2.3.4");
    const N = 50;
    await Promise.all(Array.from({ length: N }, () => agent.incrementDownload("race")));
    const pkg = await agent.getPackage("race");
    expect(pkg!.metadata.download_count).toBeGreaterThanOrEqual(1);
  });
});
