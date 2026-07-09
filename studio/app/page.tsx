"use client";

import { useCallback, useEffect, useState } from "react";
import { useSearchParams } from "next/navigation";
import dynamic from "next/dynamic";
import { AtlasNode, AtlasEdge, loadBundle, getNodeColor } from "./lib/api";

const GraphView = dynamic(() => import("./components/GraphView"), { ssr: false });

export default function Home() {
  const [nodes, setNodes] = useState<AtlasNode[]>([]);
  const [edges, setEdges] = useState<AtlasEdge[]>([]);
  const [selected, setSelected] = useState<AtlasNode | null>(null);
  const [error, setError] = useState<string>("");
  const [bundlePath, setBundlePath] = useState("");
  const [search, setSearch] = useState("");
  const searchParams = useSearchParams();

  useEffect(() => {
    const path = searchParams.get("bundle") || "";
    if (path) {
      setBundlePath(path);
      doLoad(path);
    } else {
      setBundlePath("../bundle.atlas");
      doLoad("../bundle.atlas");
    }
  }, []);

  const doLoad = useCallback(async (path: string) => {
    setError("");
    try {
      const ir = await loadBundle(path);
      setNodes(ir.nodes);
      setEdges(ir.edges);
      setSelected(null);
    } catch (err: unknown) {
      setError(err instanceof Error ? err.message : String(err));
    }
  }, []);

  const handleLoad = useCallback(() => {
    if (bundlePath) {
      window.history.replaceState({}, "", `?bundle=${encodeURIComponent(bundlePath)}`);
      doLoad(bundlePath);
    }
  }, [bundlePath, doLoad]);

  const selectedEdges = selected
    ? edges.filter((e) => e.src === selected.id || e.dst === selected.id)
    : [];

  const filteredNodes = search
    ? nodes.filter(
        (n) =>
          n.name.toLowerCase().includes(search.toLowerCase()) ||
          n.id.toLowerCase().includes(search.toLowerCase())
      )
    : nodes;

  const filteredEdges = search
    ? edges.filter(
        (e) =>
          filteredNodes.some((n) => n.id === e.src) &&
          filteredNodes.some((n) => n.id === e.dst)
      )
    : edges;

  return (
    <div style={{ display: "flex", height: "100vh", fontFamily: "system-ui, sans-serif" }}>
      <div
        style={{
          width: 300,
          borderRight: "1px solid #e2e8f0",
          display: "flex",
          flexDirection: "column",
          background: "#f8fafc",
        }}
      >
        <div style={{ padding: 16, borderBottom: "1px solid #e2e8f0" }}>
          <h2 style={{ margin: 0, fontSize: 18, fontWeight: 600 }}>Atlas Studio</h2>
        </div>

        <div style={{ padding: 12, borderBottom: "1px solid #e2e8f0" }}>
          <input
            type="text"
            value={bundlePath}
            onChange={(e) => setBundlePath(e.target.value)}
            placeholder="Path to .atlas file"
            style={{
              width: "100%",
              padding: "6px 10px",
              fontSize: 13,
              border: "1px solid #cbd5e1",
              borderRadius: 6,
              boxSizing: "border-box",
              marginBottom: 8,
            }}
          />
          <button
            onClick={handleLoad}
            style={{
              width: "100%",
              padding: "6px 12px",
              fontSize: 13,
              background: "#6366f1",
              color: "white",
              border: "none",
              borderRadius: 6,
              cursor: "pointer",
            }}
          >
            Load
          </button>
          {error && (
            <div style={{ marginTop: 8, fontSize: 12, color: "#ef4444" }}>
              {error.includes("ENOENT") || error.includes("not found")
                ? "Bundle not found. Try: ../bundle.atlas or enter a valid path."
                : error}
            </div>
          )}
        </div>

        <div style={{ padding: 12, borderBottom: "1px solid #e2e8f0" }}>
          <input
            type="text"
            value={search}
            onChange={(e) => setSearch(e.target.value)}
            placeholder="Search nodes..."
            style={{
              width: "100%",
              padding: "6px 10px",
              fontSize: 13,
              border: "1px solid #cbd5e1",
              borderRadius: 6,
              boxSizing: "border-box",
            }}
          />
        </div>

        <div style={{ flex: 1, overflow: "auto", padding: 8 }}>
          <div style={{ fontSize: 12, color: "#64748b", padding: "4px 8px", marginBottom: 4 }}>
            {filteredNodes.length} nodes, {filteredEdges.length} edges
          </div>
          {filteredNodes.map((n) => (
            <div
              key={n.id}
              onClick={() => setSelected(n)}
              style={{
                padding: "6px 10px",
                cursor: "pointer",
                borderRadius: 6,
                fontSize: 13,
                marginBottom: 2,
                background: selected?.id === n.id ? "#eef2ff" : "transparent",
                borderLeft: `3px solid ${getNodeColor(n.kind)}`,
              }}
            >
              <div style={{ fontWeight: 500 }}>{n.name}</div>
              <div style={{ fontSize: 11, color: "#64748b" }}>{n.kind}</div>
            </div>
          ))}
        </div>
      </div>

      <div style={{ flex: 1, position: "relative" }}>
        <GraphView
          atlasNodes={filteredNodes}
          atlasEdges={filteredEdges}
          onNodeClick={setSelected}
        />
      </div>

      {selected && (
        <div
          style={{
            width: 320,
            borderLeft: "1px solid #e2e8f0",
            background: "#f8fafc",
            padding: 16,
            overflow: "auto",
          }}
        >
          <div
            style={{
              display: "inline-block",
              padding: "2px 8px",
              borderRadius: 4,
              fontSize: 11,
              fontWeight: 600,
              background: getNodeColor(selected.kind),
              color: "white",
              marginBottom: 8,
            }}
          >
            {selected.kind}
          </div>
          <h3 style={{ margin: "0 0 4px", fontSize: 16 }}>{selected.name}</h3>
          <div style={{ fontSize: 12, color: "#64748b", marginBottom: 12 }}>{selected.id}</div>
          {selected.version && (
            <div style={{ fontSize: 13, marginBottom: 8 }}>
              Version: <strong>{selected.version}</strong>
            </div>
          )}
          {selected.description && (
            <div style={{ fontSize: 13, marginBottom: 12, lineHeight: 1.5 }}>
              {selected.description.slice(0, 300)}
              {selected.description.length > 300 ? "..." : ""}
            </div>
          )}
          {selectedEdges.length > 0 && (
            <div>
              <div style={{ fontSize: 12, fontWeight: 600, color: "#64748b", marginBottom: 6, textTransform: "uppercase" }}>
                Connections
              </div>
              {selectedEdges.map((e, i) => {
                const isSrc = e.src === selected.id;
                const other = nodes.find((n) => n.id === (isSrc ? e.dst : e.src));
                return (
                  <div key={i} style={{ fontSize: 12, padding: "4px 0", borderBottom: "1px solid #e2e8f0" }}>
                    <span style={{ color: "#64748b" }}>
                      {isSrc ? "→" : "←"} {e.edge_type}
                    </span>{" "}
                    {other ? other.name : isSrc ? e.dst : e.src}
                  </div>
                );
              })}
            </div>
          )}
        </div>
      )}
    </div>
  );
}
