import { describe, it, expect, vi, beforeEach } from "vitest";
import { execFileSync } from "child_process";
import { existsSync } from "fs";
import path from "path";
import { findBinary, resolvePath, runCLI, solve, decide, verify, dump, reason } from "../src/atlas.js";

vi.mock("child_process", () => ({
  execFileSync: vi.fn(),
}));

vi.mock("fs", () => ({
  existsSync: vi.fn(),
}));

const mockedExec = vi.mocked(execFileSync);
const mockedExists = vi.mocked(existsSync);

beforeEach(() => {
  mockedExec.mockReset();
  mockedExists.mockReset();
  // Default: pretend a binary exists so runCLI reaches execFileSync.
  mockedExists.mockImplementation((p: string) => typeof p === "string" && (p.endsWith("atlas.exe") || p.endsWith("atlas") || p.endsWith("atlas-cli")));
});

describe("runCLI", () => {
  it("returns the last JSON line of multi-line output", () => {
    mockedExec.mockReturnValue(
      ['{"a":1}', '{"query":"q","bundle":"b","confidence":0.7,"total_matches":0,"nodes":[]}'].join("\n")
    );
    const out = runCLI("solve", "--bundle", "b", "q");
    expect(JSON.parse(out).query).toBe("q");
  });

  it("passes --json flag and supplied args to the binary", () => {
    mockedExec.mockReturnValue("{}");
    runCLI("verify", "--bundle", "b.atlas");
    expect(mockedExec).toHaveBeenCalledTimes(1);
    const [bin, args] = mockedExec.mock.calls[0];
    expect(args[0]).toBe("--json");
    expect(args.slice(1)).toEqual(["verify", "--bundle", "b.atlas"]);
  });

  it("throws when the CLI errors", () => {
    mockedExec.mockImplementation(() => {
      throw new Error("binary failed");
    });
    expect(() => runCLI("solve", "b", "q")).toThrow(/binary failed/);
  });
});

describe("findBinary", () => {
  it("throws when no binary exists", () => {
    mockedExists.mockReturnValue(false);
    expect(() => findBinary()).toThrow(/not found/);
  });

  it("locates the binary when present on PATH", () => {
    // findBinary searches candidates ["atlas-cli","atlas"] (or .exe on win32)
    // across $HOME/.atlas/bin, /usr/local/bin, /usr/bin, then PATH dirs.
    // Make exactly ONE file exist (the first candidate in a controlled PATH dir)
    // and clear HOME/LOCALAPPDATA so home-based dirs can't interfere.
    const isWin = process.platform === "win32";
    const binDir = isWin ? "C:\\tools" : "/opt/atlas/bin";
    const binName = isWin ? "atlas-cli.exe" : "atlas-cli";
    const expected = path.join(binDir, binName);

    const orig = { PATH: process.env.PATH, HOME: process.env.HOME, LOCALAPPDATA: process.env.LOCALAPPDATA };
    delete process.env.HOME;
    delete process.env.LOCALAPPDATA;
    process.env.PATH = binDir;
    mockedExists.mockImplementation((p) => p === expected);
    try {
      expect(findBinary()).toBe(expected);
    } finally {
      process.env.PATH = orig.PATH;
      if (orig.HOME !== undefined) process.env.HOME = orig.HOME;
      if (orig.LOCALAPPDATA !== undefined) process.env.LOCALAPPDATA = orig.LOCALAPPDATA;
    }
  });
});

describe("resolvePath", () => {
  it("returns absolute paths unchanged", () => {
    const abs = process.platform === "win32" ? "C:\\abs\\b.atlas" : "/abs/b.atlas";
    expect(resolvePath(abs)).toBe(abs);
  });
});

describe("solve", () => {
  it("builds correct args and parses the result", () => {
    mockedExec.mockReturnValue(
      '{"query":"AsyncNotifier","bundle":"riverpod","confidence":0.7,"total_matches":1,"nodes":[{"id":"x","name":"X","kind":"Concept","description":null,"version":null}]}'
    );
    const r = solve("riverpod.atlas", "AsyncNotifier");
    expect(r.query).toBe("AsyncNotifier");
    expect(r.nodes[0].name).toBe("X");
    const [, args] = mockedExec.mock.calls[0];
    expect(args).toContain("solve");
    expect(args).toContain("--bundle");
    expect(args.some((a: string) => a.includes("riverpod.atlas"))).toBe(true);
    expect(args).toContain("AsyncNotifier");
  });
});

describe("decide", () => {
  it("appends -c context pairs when given", () => {
    mockedExec.mockReturnValue(
      '{"tree_id":"t","path":["a"],"rationale":"r","recommendations":[{"node_id":"x","confidence":0.9}],"agent_instructions":null}'
    );
    decide("b.atlas", "q", { complexity: "medium", lang: "dart" });
    const [, args] = mockedExec.mock.calls[0];
    expect(args).toContain("-c");
    expect(args).toContain("complexity=medium");
    expect(args).toContain("lang=dart");
  });

  it("omits -c when no context", () => {
    mockedExec.mockReturnValue('{"tree_id":"t","path":[],"rationale":"r","recommendations":[]}');
    decide("b.atlas", "q");
    const [, args] = mockedExec.mock.calls[0];
    expect(args).not.toContain("-c");
  });

  it("returns null when the CLI emits null", () => {
    mockedExec.mockReturnValue("null");
    expect(decide("b.atlas", "q")).toBeNull();
  });
});

describe("verify", () => {
  it("appends --artifact when provided", () => {
    mockedExec.mockReturnValue('{"passed":true,"checks":[{"name":"c","passed":true,"message":"ok","severity":"error"}]}');
    verify("b.atlas", "api:x");
    const [, args] = mockedExec.mock.calls[0];
    expect(args).toContain("--artifact");
    expect(args).toContain("api:x");
  });
});

describe("reason", () => {
  it("returns the answer string", () => {
    mockedExec.mockReturnValue('{"answer":"hello","query":"q"}');
    expect(reason("b.atlas", "q")).toBe("hello");
  });
});

describe("dump", () => {
  it("returns the parsed IR", () => {
    mockedExec.mockReturnValue('{"meta":{"schema_version":"0.1.0","generator":"g"},"nodes":[],"edges":[]}');
    const ir = dump("b.atlas");
    expect(ir.meta.schema_version).toBe("0.1.0");
  });
});
