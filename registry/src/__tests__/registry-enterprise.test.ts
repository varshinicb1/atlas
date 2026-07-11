import { RegistryAgent, RateLimitError, AuthError, ValidationError } from "../registry";

interface MockEnv {
  PACKAGES: {
    get: (key: string) => Promise<string | null>;
    put: (key: string, value: string) => Promise<void>;
    delete: (key: string) => Promise<void>;
    list: (opts?: { prefix?: string; limit?: number }) => Promise<{ keys: { name: string }[] }>;
  };
}

function createMockEnv(): MockEnv {
  const store = new Map<string, string>();
  return {
    PACKAGES: {
      get: async (key: string) => store.get(key) || null,
      put: async (key: string, value: string) => { store.set(key, value); },
      delete: async (key: string) => { store.delete(key); },
      list: async (opts?: { prefix?: string; limit?: number }) => {
        const entries = Array.from(store.keys())
          .filter(k => !opts?.prefix || k.startsWith(opts.prefix))
          .slice(0, opts?.limit || 100)
          .map(name => ({ name }));
        return { keys: entries };
      },
    },
  };
}

const auth = { org: "test-org", role: "admin" };
const publisherAuth = { org: "test-org", role: "publisher" };
const viewerAuth = { org: "test-org", role: "viewer" };
const validPackage = {
  name: "test-pkg",
  version: "1.0.0",
  description: "Test package",
  tags: ["test"],
  files: { "test.md": "# Hello" },
};

describe("RegistryAgent enterprise features", () => {
  let agent: RegistryAgent;
  let mockEnv: MockEnv;

  beforeEach(() => {
    mockEnv = createMockEnv();
    agent = new RegistryAgent({} as any, mockEnv as any);
  });

  describe("API key management", () => {
    it("creates a scoped key and stores it in KV", async () => {
      const res = await agent.createApiKey({ org: "test-org", role: "publisher" });
      expect(res.key).toMatch(/^atlas_/);
      expect(res.role).toBe("publisher");
      const raw = await mockEnv.PACKAGES.get(`apikey:${res.key}`);
      expect(raw).not.toBeNull();
      const rec = JSON.parse(raw!);
      expect(rec.org).toBe("test-org");
      expect(rec.role).toBe("publisher");
      expect(rec.created_at).toBeDefined();
      expect(rec.last_used).toBeNull();
    });

    it("rejects an invalid role", async () => {
      await expect(
        agent.createApiKey({ org: "test-org", role: "superuser" as any })
      ).rejects.toThrow(ValidationError);
    });

    it("rejects an invalid expires_at", async () => {
      await expect(
        agent.createApiKey({ org: "test-org", role: "viewer", expires_at: "not-a-date" })
      ).rejects.toThrow(ValidationError);
    });

    it("lists keys redacted for the org", async () => {
      const a = await agent.createApiKey({ org: "test-org", role: "admin" });
      const b = await agent.createApiKey({ org: "test-org", role: "viewer" });
      // a key from another org should not appear
      await agent.createApiKey({ org: "other-org", role: "admin" });

      const keys = await agent.listApiKeys("test-org");
      expect(keys).toHaveLength(2);
      for (const k of keys) {
        expect(k.key_prefix).toContain("****");
        expect(k.key_prefix).not.toBe(a.key);
        expect(k.key_prefix).not.toBe(b.key);
        expect(k.org).toBe("test-org");
      }
    });

    it("revokes a key", async () => {
      const res = await agent.createApiKey({ org: "test-org", role: "publisher" });
      const out = await agent.revokeApiKey("test-org", res.key);
      expect(out.success).toBe(true);
      const raw = await mockEnv.PACKAGES.get(`apikey:${res.key}`);
      expect(raw).toBeNull();
    });

    it("rejects revoking a key from another org", async () => {
      const res = await agent.createApiKey({ org: "test-org", role: "publisher" });
      await expect(
        agent.revokeApiKey("other-org", res.key)
      ).rejects.toThrow(AuthError);
    });

    it("rejects revoking a missing key", async () => {
      await expect(
        agent.revokeApiKey("test-org", "does-not-exist")
      ).rejects.toThrow(ValidationError);
    });
  });

  describe("package version history", () => {
    it("keeps prior versions on republish", async () => {
      await agent.publishPackage(validPackage, auth, "127.0.0.1");
      await agent.publishPackage({ ...validPackage, version: "1.0.1" }, auth, "127.0.0.1");
      await agent.publishPackage({ ...validPackage, version: "2.0.0" }, auth, "127.0.0.1");

      const versions = await agent.getPackageVersions("test-pkg");
      expect(versions).not.toBeNull();
      expect(versions!.versions).toEqual(["1.0.0", "1.0.1", "2.0.0"]);
    });

    it("returns a version snapshot", async () => {
      await agent.publishPackage(validPackage, auth, "127.0.0.1");
      await agent.publishPackage({ ...validPackage, version: "2.0.0", description: "v2" }, auth, "127.0.0.1");

      const snap = await agent.getPackageVersion("test-pkg", "1.0.0");
      expect(snap).not.toBeNull();
      expect(snap!.metadata.version).toBe("1.0.0");
      expect(snap!.metadata.description).toBe("Test package");
      expect(snap!.files["test.md"]).toBe("# Hello");
    });

    it("returns null for unknown package versions", async () => {
      expect(await agent.getPackageVersions("nope")).toBeNull();
      expect(await agent.getPackageVersion("nope", "1.0.0")).toBeNull();
      expect(await agent.getPackageVersion("test-pkg", "9.9.9")).toBeNull();
    });
  });

  describe("backup / export / import", () => {
    it("exportAll returns a dump and importAll restores it", async () => {
      await agent.publishPackage(validPackage, auth, "127.0.0.1");
      await agent.publishPackage(
        { ...validPackage, name: "second", version: "3.0.0" },
        auth,
        "127.0.0.1"
      );

      const dump = await agent.exportAll();
      expect(dump.packages).toHaveLength(2);

      // wipe and re-import
      const fresh = new RegistryAgent({} as any, createMockEnv() as any);
      const result = await fresh.importAll(dump);
      expect(result.imported).toBe(2);

      const restored = await fresh.getPackage("test-pkg");
      expect(restored).not.toBeNull();
      expect(restored!.metadata.name).toBe("test-pkg");
      expect(restored!.files["test.md"]).toBe("# Hello");
    });

    it("rejects an invalid dump", async () => {
      await expect(agent.importAll({ foo: 1 } as any)).rejects.toThrow(ValidationError);
    });

    it("rejects a dump with an invalid package name", async () => {
      const bad = {
        version: "1.0",
        exported_at: new Date().toISOString(),
        packages: [{ metadata: { name: "Bad Name!" }, files: {}, versions: [] }],
      };
      await expect(agent.importAll(bad as any)).rejects.toThrow(ValidationError);
    });
  });

  describe("metrics", () => {
    it("aggregates package, download and per-org counts", async () => {
      await agent.publishPackage(validPackage, auth, "127.0.0.1");
      await agent.publishPackage(
        { ...validPackage, name: "second" },
        auth,
        "127.0.0.1"
      );
      await agent.incrementDownload("test-pkg");

      const m = await agent.getMetrics();
      expect(m.total_packages).toBe(2);
      expect(m.total_downloads).toBe(1);
      expect(m.per_org["test-org"].packages).toBe(2);
      expect(m.per_org["test-org"].downloads).toBe(1);
      expect(m.uptime_seconds).toBeGreaterThanOrEqual(0);
      expect(typeof m.generated_at).toBe("string");
    });
  });

  describe("rate limiting", () => {
    it("allows up to the limit then blocks", async () => {
      const first = await agent.rateLimitCheck("key1");
      expect(first.limited).toBe(false);
      expect(first.remaining).toBe(59);
      expect(first.limit).toBe(60);

      let lastLimited = false;
      let lastRemaining = 0;
      for (let i = 0; i < 59; i++) {
        const s = await agent.rateLimitCheck("key1");
        lastLimited = s.limited;
        lastRemaining = s.remaining;
      }
      expect(lastLimited).toBe(false);
      expect(lastRemaining).toBe(0);

      const blocked = await agent.rateLimitCheck("key1");
      expect(blocked.limited).toBe(true);
      expect(blocked.remaining).toBe(0);
    });
  });

  describe("role enforcement", () => {
    it("rejects publishing from a viewer", async () => {
      await expect(
        agent.publishPackage(validPackage, viewerAuth, "127.0.0.1")
      ).rejects.toThrow(AuthError);
    });

    it("allows publishing from a publisher", async () => {
      const res = await agent.publishPackage(validPackage, publisherAuth, "127.0.0.1");
      expect(res.success).toBe(true);
    });
  });
});
