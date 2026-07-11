import { McpServer, ResourceTemplate } from "@modelcontextprotocol/sdk/server/mcp.js";
import { StdioServerTransport } from "@modelcontextprotocol/sdk/server/stdio.js";
import { fileURLToPath } from "url";
import { resolve } from "path";
import * as z from "zod/v4";
import * as atlas from "./atlas.js";
export const server = new McpServer({ name: "atlas-mcp", version: "0.1.0" }, {
    capabilities: { tools: {}, resources: {} },
    instructions: "Atlas Knowledge Operating System MCP server. Provides engineering knowledge graph access via .atlas bundles. Use atlas_solve to search, atlas_decide for decision trees, atlas_navigate for graph traversal, atlas_verify for validation, and atlas_reason for natural-language answers.",
});
function countBy(arr, key) {
    const counts = {};
    for (const item of arr) {
        counts[String(item[key])] = (counts[String(item[key])] || 0) + 1;
    }
    return counts;
}
// ── Tools ──────────────────────────────────────────────────────
export function registerAtlasTools(s) {
    s.registerTool("atlas_solve", {
        description: "Search the knowledge graph for nodes matching a query",
        inputSchema: {
            bundle: z.string().describe("Path to .atlas file"),
            query: z.string().describe("Search query"),
        },
    }, async ({ bundle, query }) => {
        try {
            const result = atlas.solve(bundle, query);
            const lines = result.nodes.map((n) => `[${n.kind}] ${n.name} (${n.id})${n.version ? ` v${n.version}` : ""}${n.description ? `\n  ${n.description.slice(0, 200)}` : ""}`);
            return {
                content: [{ type: "text", text: [`Query: ${result.query}`, `Confidence: ${result.confidence}`, `Found ${result.nodes.length} nodes:`, "", ...lines].join("\n") }],
            };
        }
        catch (e) {
            return { content: [{ type: "text", text: `Error: ${e instanceof Error ? e.message : String(e)}` }], isError: true };
        }
    });
    s.registerTool("atlas_decide", {
        description: "Walk a decision tree to get a recommendation (LLM-free)",
        inputSchema: {
            bundle: z.string().describe("Path to .atlas file"),
            query: z.string().describe("Decision query"),
            context_json: z.string().optional().describe("JSON object of context key-value pairs (e.g. {\"complexity\":\"medium\"})"),
        },
    }, async ({ bundle, query, context_json }) => {
        try {
            const ctx = context_json ? JSON.parse(context_json) : undefined;
            const result = atlas.decide(bundle, query, ctx);
            if (!result) {
                return { content: [{ type: "text", text: "No matching decision tree found." }] };
            }
            const lines = [
                `Decision tree: ${result.tree_id}`,
                `Path: ${result.path.join(" → ")}`,
                `Rationale: ${result.rationale}`,
                "",
                "Recommendations:",
                ...result.recommendations.map((r) => `  ${r.node_id} (confidence: ${r.confidence})`),
            ];
            if (result.agent_instructions)
                lines.push("", `Agent instructions: ${result.agent_instructions}`);
            return { content: [{ type: "text", text: lines.join("\n") }] };
        }
        catch (e) {
            return { content: [{ type: "text", text: `Error: ${e instanceof Error ? e.message : String(e)}` }], isError: true };
        }
    });
    s.registerTool("atlas_verify", {
        description: "Run verification checks against a knowledge graph bundle",
        inputSchema: {
            bundle: z.string().describe("Path to .atlas file"),
            artifact: z.string().optional().describe("Optional artifact name to verify"),
        },
    }, async ({ bundle, artifact }) => {
        try {
            const result = atlas.verify(bundle, artifact);
            const lines = [
                `Verification: ${result.passed ? "PASSED" : "FAILED"}`,
                "",
                ...result.checks.map((c) => `${c.passed ? "✓" : "✗"} ${c.name}: ${c.message}`),
            ];
            return { content: [{ type: "text", text: lines.join("\n") }] };
        }
        catch (e) {
            return { content: [{ type: "text", text: `Error: ${e instanceof Error ? e.message : String(e)}` }], isError: true };
        }
    });
    s.registerTool("atlas_reason", {
        description: "Reason over the knowledge graph and return a natural-language answer",
        inputSchema: {
            bundle: z.string().describe("Path to .atlas file"),
            query: z.string().describe("Query about the knowledge domain"),
        },
    }, async ({ bundle, query }) => {
        try {
            const answer = atlas.reason(bundle, query);
            return { content: [{ type: "text", text: answer }] };
        }
        catch (e) {
            return { content: [{ type: "text", text: `Error: ${e instanceof Error ? e.message : String(e)}` }], isError: true };
        }
    });
    s.registerTool("atlas_navigate", {
        description: "Get neighbors and subgraph context around a specific node",
        inputSchema: {
            bundle: z.string().describe("Path to .atlas file"),
            node_id: z.string().describe("Node ID to navigate from"),
        },
    }, async ({ bundle, node_id }) => {
        try {
            const ir = atlas.dump(bundle);
            const node = ir.nodes.find((n) => n.id === node_id);
            if (!node) {
                return { content: [{ type: "text", text: `Node "${node_id}" not found.` }] };
            }
            const outgoing = ir.edges.filter((e) => e.src === node_id);
            const incoming = ir.edges.filter((e) => e.dst === node_id);
            const neighborIds = new Set();
            outgoing.forEach((e) => neighborIds.add(e.dst));
            incoming.forEach((e) => neighborIds.add(e.src));
            const neighbors = ir.nodes.filter((n) => neighborIds.has(n.id));
            const lines = [
                `Node: [${node.kind}] ${node.name} (${node.id})`,
                node.description ? `  ${node.description.slice(0, 200)}` : "",
                node.version ? `  version: ${node.version}` : "",
                "",
                `Outgoing edges (${outgoing.length}):`,
                ...outgoing.map((e) => {
                    const t = ir.nodes.find((n) => n.id === e.dst);
                    return `  → ${e.edge_type} → ${t ? `[${t.kind}] ${t.name}` : e.dst}`;
                }),
                "",
                `Incoming edges (${incoming.length}):`,
                ...incoming.map((e) => {
                    const s = ir.nodes.find((n) => n.id === e.src);
                    return `  ← ${e.edge_type} ← ${s ? `[${s.kind}] ${s.name}` : e.src}`;
                }),
                "",
                `Connected nodes (${neighbors.length}):`,
                ...neighbors.map((n) => `  [${n.kind}] ${n.name} (${n.id})`),
            ];
            return { content: [{ type: "text", text: lines.join("\n") }] };
        }
        catch (e) {
            return { content: [{ type: "text", text: `Error: ${e instanceof Error ? e.message : String(e)}` }], isError: true };
        }
    });
    // ── Resource: bundle metadata ──────────────────────────────────
    s.registerResource("bundle", new ResourceTemplate("atlas://bundle/{path}", { list: undefined }), {
        description: "Metadata about an Atlas knowledge bundle",
        mimeType: "application/json",
    }, async (uri, variables) => {
        try {
            const raw = typeof variables.path === "string" ? variables.path : variables.path[0];
            const path = decodeURIComponent(raw);
            const ir = atlas.dump(path);
            const summary = {
                nodes: ir.nodes.length,
                edges: ir.edges.length,
                decision_trees: ir.decision_trees?.length || 0,
                schema_version: ir.meta.schema_version,
                generator: ir.meta.generator,
                kinds: countBy(ir.nodes, "kind"),
            };
            return { contents: [{ uri: uri.href, mimeType: "application/json", text: JSON.stringify(summary, null, 2) }] };
        }
        catch (e) {
            return { contents: [{ uri: uri.href, mimeType: "text/plain", text: `Error: ${e instanceof Error ? e.message : String(e)}` }] };
        }
    });
}
// ── Connect & start ────────────────────────────────────────────
async function main() {
    registerAtlasTools(server);
    const transport = new StdioServerTransport();
    await server.connect(transport);
    console.error("Atlas MCP server running on stdio");
}
// Only start the server when executed directly (not when imported by tests).
const invokedDirectly = process.argv[1] && fileURLToPath(import.meta.url) === resolve(process.argv[1]);
if (invokedDirectly) {
    main().catch((e) => {
        console.error("Fatal error:", e);
        process.exit(1);
    });
}
