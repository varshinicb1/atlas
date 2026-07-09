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

describe("RegistryAgent", () => {
  let agent: RegistryAgent;
  let mockEnv: MockEnv;
  const auth = { org: "test-org", role: "admin" };
  const validPackage = {
    name: "test-pkg",
    version: "1.0.0",
    description: "Test package",
    tags: ["test"],
    files: { "test.md": "# Hello" },
  };

  beforeEach(() => {
    mockEnv = createMockEnv();
    agent = new RegistryAgent({} as any, mockEnv as any);
  });

  describe("publishPackage", () => {
    it("should publish a valid package", async () => {
      const result = await agent.publishPackage(validPackage, auth, "127.0.0.1");
      expect(result.success).toBe(true);
      expect(result.name).toBe("test-pkg");
      expect(result.replaced).toBe(false);
    });

    it("should reject empty name", async () => {
      await expect(
        agent.publishPackage({ ...validPackage, name: "" }, auth, "127.0.0.1")
      ).rejects.toThrow(ValidationError);
    });

    it("should reject invalid name characters", async () => {
      await expect(
        agent.publishPackage({ ...validPackage, name: "Invalid Name!" }, auth, "127.0.0.1")
      ).rejects.toThrow(ValidationError);
    });

    it("should reject missing version", async () => {
      await expect(
        agent.publishPackage({ ...validPackage, version: "" }, auth, "127.0.0.1")
      ).rejects.toThrow(ValidationError);
    });

    it("should reject empty files", async () => {
      await expect(
        agent.publishPackage({ ...validPackage, files: {} }, auth, "127.0.0.1")
      ).rejects.toThrow(ValidationError);
    });

    it("should replace existing package", async () => {
      await agent.publishPackage(validPackage, auth, "127.0.0.1");
      const result = await agent.publishPackage(
        { ...validPackage, version: "1.0.1" },
        auth,
        "127.0.0.1"
      );
      expect(result.replaced).toBe(true);
      expect(result.version).toBe("1.0.1");
    });

    it("should reject publish from different org", async () => {
      await agent.publishPackage(validPackage, auth, "127.0.0.1");
      await expect(
        agent.publishPackage(validPackage, { org: "other-org", role: "admin" }, "127.0.0.1")
      ).rejects.toThrow(AuthError);
    });
  });

  describe("listPackages", () => {
    it("should return empty list initially", async () => {
      const packages = await agent.listPackages();
      expect(packages).toEqual([]);
    });

    it("should list published packages", async () => {
      await agent.publishPackage(validPackage, auth, "127.0.0.1");
      await agent.publishPackage(
        { ...validPackage, name: "pkg2", description: "Second" },
        auth,
        "127.0.0.1"
      );
      const packages = await agent.listPackages();
      expect(packages).toHaveLength(2);
    });

    it("should filter by org", async () => {
      await agent.publishPackage(validPackage, auth, "127.0.0.1");
      await agent.publishPackage(
        { ...validPackage, name: "other-pkg" },
        { org: "other-org", role: "admin" },
        "127.0.0.1"
      );
      const orgPackages = await agent.listPackages("test-org");
      expect(orgPackages).toHaveLength(1);
      expect(orgPackages[0].name).toBe("test-pkg");
    });
  });

  describe("searchPackages", () => {
    it("should return empty for no query", async () => {
      const results = await agent.searchPackages("");
      expect(results).toEqual([]);
    });

    it("should find packages by name", async () => {
      await agent.publishPackage(validPackage, auth, "127.0.0.1");
      const results = await agent.searchPackages("test");
      expect(results).toHaveLength(1);
      expect(results[0].name).toBe("test-pkg");
    });

    it("should find packages by description", async () => {
      await agent.publishPackage(validPackage, auth, "127.0.0.1");
      const results = await agent.searchPackages("hello");
      expect(results).toHaveLength(0); // description doesn't contain "hello"
    });

    it("should respect limit", async () => {
      await agent.publishPackage(validPackage, auth, "127.0.0.1");
      await agent.publishPackage(
        { ...validPackage, name: "test-pkg-2" },
        auth,
        "127.0.0.1"
      );
      const results = await agent.searchPackages("test", 1);
      expect(results).toHaveLength(1);
    });
  });

  describe("getPackage", () => {
    it("should return null for missing package", async () => {
      const pkg = await agent.getPackage("nonexistent");
      expect(pkg).toBeNull();
    });

    it("should return package with metadata and files", async () => {
      await agent.publishPackage(validPackage, auth, "127.0.0.1");
      const pkg = await agent.getPackage("test-pkg");
      expect(pkg).not.toBeNull();
      expect(pkg!.metadata.name).toBe("test-pkg");
      expect(pkg!.files["test.md"]).toBe("# Hello");
    });
  });

  describe("incrementDownload", () => {
    it("should increment download count", async () => {
      await agent.publishPackage(validPackage, auth, "127.0.0.1");
      const before = await agent.getPackage("test-pkg");
      expect(before!.metadata.download_count).toBe(0);

      await agent.incrementDownload("test-pkg");
      const after = await agent.getPackage("test-pkg");
      expect(after!.metadata.download_count).toBe(1);
    });
  });

  describe("deletePackage", () => {
    it("should delete a package", async () => {
      await agent.publishPackage(validPackage, auth, "127.0.0.1");
      const result = await agent.deletePackage("test-pkg", auth);
      expect(result.success).toBe(true);

      const pkg = await agent.getPackage("test-pkg");
      expect(pkg).toBeNull();
    });

    it("should reject delete from non-admin", async () => {
      await agent.publishPackage(validPackage, auth, "127.0.0.1");
      await expect(
        agent.deletePackage("test-pkg", { org: "test-org", role: "viewer" })
      ).rejects.toThrow(AuthError);
    });

    it("should reject delete of other org's package", async () => {
      await agent.publishPackage(validPackage, auth, "127.0.0.1");
      await expect(
        agent.deletePackage("test-pkg", { org: "other-org", role: "admin" })
      ).rejects.toThrow(AuthError);
    });
  });

  describe("getOrgStats", () => {
    it("should return zero stats for new org", async () => {
      const stats = await agent.getOrgStats("empty-org");
      expect(stats.total_packages).toBe(0);
      expect(stats.total_downloads).toBe(0);
    });

    it("should return accurate stats", async () => {
      await agent.publishPackage(validPackage, auth, "127.0.0.1");
      await agent.incrementDownload("test-pkg");
      const stats = await agent.getOrgStats("test-org");
      expect(stats.total_packages).toBe(1);
      expect(stats.total_downloads).toBe(1);
    });
  });

  describe("getDetailedHealth", () => {
    it("should return health with stats", async () => {
      await agent.publishPackage(validPackage, auth, "127.0.0.1");
      const health = await agent.getDetailedHealth();
      expect(health.status).toBe("ok");
      expect(health.total_packages).toBe(1);
      expect(health.organizations).toBe(1);
    });
  });
});
