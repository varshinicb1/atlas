// QA probe for the Atlas MCP server (real binary, real handlers).
// Run from repo root:  node qa/sdk/probe_mcp.mjs
import { registerAtlasTools } from "../../mcp-server/dist/index.js";
import * as atlas from "../../mcp-server/dist/atlas.js";
import path from "path";
import os from "os";

const REPO = path.resolve(process.cwd());
// Make the real atlas.exe discoverable via PATH (win32 candidates: atlas-cli.exe, atlas.exe)
const releaseDir = path.join(REPO, "target", "release");
process.env.PATH = releaseDir + ";" + (process.env.PATH || "");
process.env.HOME = process.env.HOME || os.homedir();

const BUNDLE = "packages/riverpod.atlas";

// Fake McpServer that captures handlers (mirrors SDK test harness but uses REAL atlas module)
class FakeServer {
  constructor() { this.tools = {}; this.resources = {}; }
  registerTool(name, schema, handler) { this.tools[name] = { schema, handler }; }
  registerResource(name, tpl, opts, handler) { this.resources[name] = { tpl, opts, handler }; }
  connect() { return Promise.resolve(); }
}

const server = new FakeServer();
registerAtlasTools(server);

let pass = 0, fail = 0;
function check(name, cond, detail = "") {
  if (cond) { pass++; console.log(`[PASS] ${name}` + (detail ? ` -- ${detail}` : "")); }
  else { fail++; console.log(`[FAIL] ${name}` + (detail ? ` -- ${detail}` : "")); }
}

async function main() {
  console.log("\n== A. atlas.ts wrapper (real binary) ==");
  try {
    const r = atlas.solve(BUNDLE, "AsyncNotifier");
    check("atlas.solve returns object", r && typeof r === "object");
    check("atlas.solve.confidence is number", typeof r.confidence === "number", `conf=${r.confidence}`);
    check("atlas.solve.nodes typed", Array.isArray(r.nodes) && r.nodes.every(n => "id" in n && "kind" in n));
  } catch (e) { check("atlas.solve", false, `${e.name}: ${e.message}`); }

  try {
    const ir = atlas.dump(BUNDLE);
    check("atlas.dump returns IR with nodes/edges", ir && Array.isArray(ir.nodes) && Array.isArray(ir.edges),
      `nodes=${ir?.nodes?.length} edges=${ir?.edges?.length}`);
    check("dump.meta.schema_version present", ir?.meta?.schema_version !== undefined, `v=${ir?.meta?.schema_version}`);
  } catch (e) { check("atlas.dump", false, `${e.name}: ${e.message}`); }

  console.log("\n== B. MCP tool handlers ==");
  // valid solve
  let res = await server.tools["atlas_solve"].handler({ bundle: BUNDLE, query: "AsyncNotifier" });
  check("atlas_solve valid -> content", res.content && res.content[0].text.includes("AsyncNotifier"),
    `isError=${res.isError}`);
  check("atlas_solve valid -> no isError", !res.isError);

  // missing bundle
  res = await server.tools["atlas_solve"].handler({ bundle: "packages/nope.atlas", query: "x" });
  check("atlas_solve missing bundle -> isError", res.isError === true, `text=${res.content[0].text.slice(0,60)}`);

  // leading-dash query (--help) -> JSON.parse throws, must be caught (isError, not crash)
  res = await server.tools["atlas_solve"].handler({ bundle: BUNDLE, query: "--help" });
  check("atlas_solve '--help' -> isError (no crash)", res.isError === true, `text=${res.content[0].text.slice(0,60)}`);

  // empty query
  res = await server.tools["atlas_solve"].handler({ bundle: BUNDLE, query: "" });
  check("atlas_solve empty query -> handled (isError or content)", !!res.content, `isError=${res.isError} text=${String(res.content?.[0]?.text).slice(0,40)}`);

  // very long query -> WinError 206 -> must be caught as isError
  res = await server.tools["atlas_solve"].handler({ bundle: BUNDLE, query: "x".repeat(200000) });
  check("atlas_solve long query -> isError (no crash)", res.isError === true, `text=${res.content[0].text.slice(0,60)}`);

  // unicode query
  res = await server.tools["atlas_solve"].handler({ bundle: BUNDLE, query: "状態管理 🚀" });
  check("atlas_solve unicode query -> handled", !!res.content, `isError=${res.isError}`);

  // decide with invalid context_json
  res = await server.tools["atlas_decide"].handler({ bundle: BUNDLE, query: "compile", context_json: "not json {{" });
  check("atlas_decide invalid context_json -> isError", res.isError === true, `text=${res.content[0].text.slice(0,50)}`);

  // decide with valid context_json
  res = await server.tools["atlas_decide"].handler({ bundle: BUNDLE, query: "compile", context_json: '{"complexity":"medium"}' });
  check("atlas_decide valid context_json -> handled", !!res.content, `isError=${res.isError}`);

  // decide with NON-OBJECT context_json (string)
  res = await server.tools["atlas_decide"].handler({ bundle: BUNDLE, query: "compile", context_json: '"hello"' });
  check("atlas_decide string context_json -> handled (validation gap)", !!res.content, `isError=${res.isError} text=${String(res.content?.[0]?.text).slice(0,50)}`);

  // verify
  res = await server.tools["atlas_verify"].handler({ bundle: BUNDLE });
  check("atlas_verify -> PASSED text", res.content[0].text.includes("PASSED"), `isError=${res.isError}`);

  // reason
  res = await server.tools["atlas_reason"].handler({ bundle: BUNDLE, query: "what is AsyncNotifier" });
  check("atlas_reason -> content", res.content && res.content[0].text.length > 0, `isError=${res.isError}`);

  // navigate: missing node
  res = await server.tools["atlas_navigate"].handler({ bundle: BUNDLE, node_id: "does_not_exist" });
  check("atlas_navigate missing node -> 'not found'", res.content[0].text.includes("not found"), `isError=${res.isError}`);

  // navigate: existing node
  res = await server.tools["atlas_navigate"].handler({ bundle: BUNDLE, node_id: "package:flutter/riverpod" });
  check("atlas_navigate existing node -> lists neighbors", res.content[0].text.includes("Connected nodes"), `isError=${res.isError}`);

  console.log("\n== C. bundle resource ==");
  const rh = server.resources["bundle"].handler;
  // simulate what MCP URI template would extract for a path with subdirs
  res = await rh(new URL("atlas://bundle/" + encodeURIComponent(BUNDLE)), { path: BUNDLE });
  check("bundle resource (full path in var) -> json", res.contents[0].text.includes("nodes"), `isError? mime=${res.contents[0].mimeType}`);
  // simulate single-segment extraction (URI template {path} stops at first '/')
  res = await rh(new URL("atlas://bundle/" + encodeURIComponent("packages")), { path: "packages" });
  check("bundle resource (only first segment) -> FAILS to find nested file", res.contents[0].mimeType === "text/plain", `text=${res.contents[0].text.slice(0,60)}`);

  console.log(`\n=== MCP probe summary: ${pass} pass / ${fail} fail ===`);
  process.exit(fail ? 1 : 0);
}
main().catch(e => { console.error("PROBE CRASH:", e); process.exit(2); });
