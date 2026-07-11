import { describe, it, expect, vi, beforeEach } from "vitest";

const { registry, resources } = vi.hoisted(() => ({
  registry: {} as Record<string, { schema: any; handler: (args: any) => Promise<any> }>,
  resources: {} as Record<string, boolean>,
}));

vi.mock("@modelcontextprotocol/sdk/server/mcp.js", () => {
  class McpServer {
    constructor(_info: any, _opts: any) {}
    registerTool(name: string, schema: any, handler: any) {
      registry[name] = { schema, handler };
    }
    registerResource(name: string, _tpl: any, _opts: any, _handler: any) {
      resources[name] = true;
    }
    connect() {
      return Promise.resolve();
    }
  }
  class ResourceTemplate {
    constructor(public uri: string, public opts: any) {}
  }
  return { McpServer, ResourceTemplate };
});

vi.mock("@modelcontextprotocol/sdk/server/stdio.js", () => ({
  StdioServerTransport: class {
    async connect() {
      return Promise.resolve();
    }
  },
}));

vi.mock("../src/atlas.js", () => ({
  solve: vi.fn(),
  decide: vi.fn(),
  verify: vi.fn(),
  reason: vi.fn(),
  dump: vi.fn(),
}));

import * as atlas from "../src/atlas.js";
import { server, registerAtlasTools } from "../src/index.js";

const mockedAtlas = vi.mocked(atlas);

beforeEach(() => {
  for (const k of Object.keys(registry)) delete registry[k];
  for (const k of Object.keys(resources)) delete resources[k];
  vi.clearAllMocks();
  registerAtlasTools(server);
});

describe("tool registration", () => {
  it("registers all five tools", () => {
    expect(Object.keys(registry).sort()).toEqual(
      ["atlas_decide", "atlas_navigate", "atlas_reason", "atlas_solve", "atlas_verify"].sort()
    );
  });

  it("registers the bundle resource", () => {
    expect(resources["bundle"]).toBe(true);
  });

  it("declares required input schema fields", () => {
    expect(registry["atlas_solve"].schema.inputSchema.bundle).toBeDefined();
    expect(registry["atlas_solve"].schema.inputSchema.query).toBeDefined();
    expect(registry["atlas_decide"].schema.inputSchema.context_json).toBeDefined();
    expect(registry["atlas_verify"].schema.inputSchema.artifact).toBeDefined();
    expect(registry["atlas_navigate"].schema.inputSchema.node_id).toBeDefined();
  });
});

describe("atlas_solve handler", () => {
  it("formats solve results into text", async () => {
    mockedAtlas.solve.mockReturnValue({
      query: "AsyncNotifier",
      bundle: "riverpod",
      confidence: 0.9,
      total_matches: 1,
      nodes: [{ id: "x", name: "X", kind: "Concept", description: null, version: null }],
    });
    const res = await registry["atlas_solve"].handler({ bundle: "b.atlas", query: "AsyncNotifier" });
    expect(res.content[0].text).toContain("AsyncNotifier");
    expect(res.content[0].text).toContain("Confidence: 0.9");
    expect(res.content[0].text).toContain("X");
    expect(mockedAtlas.solve).toHaveBeenCalledWith("b.atlas", "AsyncNotifier");
  });

  it("returns an error result when the CLI throws", async () => {
    mockedAtlas.solve.mockImplementation(() => {
      throw new Error("boom");
    });
    const res = await registry["atlas_solve"].handler({ bundle: "b.atlas", query: "q" });
    expect(res.isError).toBe(true);
    expect(res.content[0].text).toContain("boom");
  });
});

describe("atlas_decide handler", () => {
  it("parses context_json and forwards it", async () => {
    mockedAtlas.decide.mockReturnValue({
      tree_id: "t",
      path: ["a", "b"],
      rationale: "use provider",
      recommendations: [{ node_id: "api:x", confidence: 0.9 }],
      agent_instructions: null,
    });
    const res = await registry["atlas_decide"].handler({
      bundle: "b.atlas",
      query: "q",
      context_json: '{"complexity":"medium"}',
    });
    expect(mockedAtlas.decide).toHaveBeenCalledWith("b.atlas", "q", { complexity: "medium" });
    expect(res.content[0].text).toContain("Decision tree: t");
    expect(res.content[0].text).toContain("api:x");
  });

  it("reports when no decision tree matches", async () => {
    mockedAtlas.decide.mockReturnValue(null);
    const res = await registry["atlas_decide"].handler({ bundle: "b.atlas", query: "q" });
    expect(res.content[0].text).toContain("No matching decision tree");
  });
});

describe("atlas_verify handler", () => {
  it("formats verification checks", async () => {
    mockedAtlas.verify.mockReturnValue({
      passed: true,
      checks: [{ name: "API existence", passed: true, message: "ok", severity: "error" }],
    });
    const res = await registry["atlas_verify"].handler({ bundle: "b.atlas" });
    expect(res.content[0].text).toContain("PASSED");
    expect(res.content[0].text).toContain("API existence");
  });
});

describe("atlas_reason handler", () => {
  it("returns the answer", async () => {
    mockedAtlas.reason.mockReturnValue("the answer");
    const res = await registry["atlas_reason"].handler({ bundle: "b.atlas", query: "q" });
    expect(res.content[0].text).toBe("the answer");
  });
});

describe("atlas_navigate handler", () => {
  it("reports a missing node", async () => {
    mockedAtlas.dump.mockReturnValue({ nodes: [], edges: [] });
    const res = await registry["atlas_navigate"].handler({ bundle: "b.atlas", node_id: "nope" });
    expect(res.content[0].text).toContain("not found");
  });

  it("lists neighbors for a found node", async () => {
    mockedAtlas.dump.mockReturnValue({
      nodes: [
        { id: "a", name: "A", kind: "Concept", description: null, version: null },
        { id: "b", name: "B", kind: "API", description: null, version: null },
      ],
      edges: [{ src: "a", dst: "b", edge_type: "uses" }],
    });
    const res = await registry["atlas_navigate"].handler({ bundle: "b.atlas", node_id: "a" });
    expect(res.content[0].text).toContain("A");
    expect(res.content[0].text).toContain("Connected nodes (1)");
  });
});
