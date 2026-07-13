"use client";

import { Suspense, useCallback, useState, useMemo, useEffect } from "react";
import { useSearchParams } from "next/navigation";
import dynamic from "next/dynamic";
import { AtlasNode, AtlasEdge, loadBundle, loadRegistryPackage, listRegistryPackages, RegistryPackageInfo, getNodeColor } from "./lib/api";

const GraphView = dynamic(() => import("./components/GraphView"), { ssr: false });

const DEFAULT_BUNDLE = "../bundle.atlas";

type Tab = "local" | "registry";

function HomeContent() {
  const [tab, setTab] = useState<Tab>("local");
  const [nodes, setNodes] = useState<AtlasNode[]>([]);
  const [edges, setEdges] = useState<AtlasEdge[]>([]);
  const [selected, setSelected] = useState<AtlasNode | null>(null);
  const [error, setError] = useState("");
  const [loading, setLoading] = useState(false);
  const [loaded, setLoaded] = useState(false);
  const [bundleName, setBundleName] = useState("");
  const [search, setSearch] = useState("");
  const searchParams = useSearchParams();
  const [bundlePath, setBundlePath] = useState(
    () => searchParams.get("bundle") || DEFAULT_BUNDLE
  );
  const [registryPackages, setRegistryPackages] = useState<RegistryPackageInfo[]>([]);
  const [registryLoading, setRegistryLoading] = useState(false);
  const [registrySearch, setRegistrySearch] = useState("");

  useEffect(() => {
    let cancelled = false;
    // eslint-disable-next-line react-hooks/set-state-in-effect
    setRegistryLoading(true);
    listRegistryPackages()
      .then((pkgs) => {
        if (!cancelled) {
          setRegistryPackages(pkgs);
          setRegistryLoading(false);
        }
      })
      .catch(() => {
        if (!cancelled) setRegistryLoading(false);
      });
    return () => { cancelled = true; };
  }, []);

  async function doLoad(path: string) {
    setError("");
    setLoading(true);
    try {
      const ir = await loadBundle(path);
      setNodes(ir.nodes);
      setEdges(ir.edges);
      setBundleName(path.split("/").pop() || path);
      setSelected(null);
      setLoaded(true);
    } catch (err: unknown) {
      setNodes([]);
      setEdges([]);
      setSelected(null);
      setLoaded(false);
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setLoading(false);
    }
  }

  async function doLoadRegistry(name: string) {
    setError("");
    setLoading(true);
    try {
      const ir = await loadRegistryPackage(name);
      setNodes(ir.nodes);
      setEdges(ir.edges);
      setBundleName(name);
      setSelected(null);
      setLoaded(true);
    } catch (err: unknown) {
      setNodes([]);
      setEdges([]);
      setSelected(null);
      setLoaded(false);
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setLoading(false);
    }
  }

  const handleLoad = useCallback(() => {
    if (bundlePath) {
      window.history.replaceState({}, "", `?bundle=${encodeURIComponent(bundlePath)}`);
      doLoad(bundlePath);
    }
  }, [bundlePath]);

  const selectedEdges = useMemo(
    () => (selected ? edges.filter((e) => e.src === selected.id || e.dst === selected.id) : []),
    [selected, edges]
  );

  const filteredNodes = useMemo(() => {
    if (!search) return nodes;
    const q = search.toLowerCase();
    return nodes.filter((n) => n.name.toLowerCase().includes(q) || n.id.toLowerCase().includes(q));
  }, [nodes, search]);

  const filteredEdges = useMemo(() => {
    if (!search) return edges;
    const ids = new Set(filteredNodes.map((n) => n.id));
    return edges.filter((e) => ids.has(e.src) && ids.has(e.dst));
  }, [edges, search, filteredNodes]);

  const filteredRegistryPackages = useMemo(() => {
    if (!registrySearch) return registryPackages;
    const q = registrySearch.toLowerCase();
    return registryPackages.filter(
      (p) =>
        p.name.toLowerCase().includes(q) ||
        (p.description || "").toLowerCase().includes(q)
    );
  }, [registryPackages, registrySearch]);

  const hasGraph = loaded && nodes.length > 0;

  return (
    <div className="studio-layout">
      <aside className="sidebar">
        <div className="sidebar-header">
          <h2 className="sidebar-title">Atlas Studio</h2>
        </div>

        <div className="sidebar-tabs">
          <button
            className={`sidebar-tab ${tab === "local" ? "active" : ""}`}
            onClick={() => setTab("local")}
          >
            Local
          </button>
          <button
            className={`sidebar-tab ${tab === "registry" ? "active" : ""}`}
            onClick={() => setTab("registry")}
          >
            Registry
          </button>
        </div>

        {tab === "local" ? (
          <div className="sidebar-section">
            <input
              type="text"
              value={bundlePath}
              onChange={(e) => setBundlePath(e.target.value)}
              onKeyDown={(e) => { if (e.key === "Enter") handleLoad(); }}
              placeholder="Path to .atlas file"
              className="sidebar-input"
            />
            <button
              onClick={handleLoad}
              disabled={loading || !bundlePath}
              className="sidebar-btn"
            >
              {loading ? "Loading\u2026" : "Load"}
            </button>
          </div>
        ) : (
          <div className="sidebar-section">
            <input
              type="text"
              value={registrySearch}
              onChange={(e) => setRegistrySearch(e.target.value)}
              placeholder="Search registry..."
              className="sidebar-input"
            />
            <div className="registry-list">
              {registryLoading ? (
                <div className="sidebar-muted">Loading packages\u2026</div>
              ) : filteredRegistryPackages.length === 0 ? (
                <div className="sidebar-muted">
                  {registrySearch ? "No packages match your search." : "No packages found."}
                </div>
              ) : (
                filteredRegistryPackages.slice(0, 100).map((pkg) => (
                  <button
                    key={pkg.name}
                    className="registry-item"
                    onClick={() => doLoadRegistry(pkg.name)}
                    disabled={loading}
                  >
                    <div className="registry-item-name">{pkg.name}</div>
                    <div className="registry-item-meta">
                      {pkg.version}
                      {pkg.description
                        ? ` \u00B7 ${pkg.description.slice(0, 60)}${pkg.description.length > 60 ? "\u2026" : ""}`
                        : ""}
                    </div>
                  </button>
                ))
              )}
            </div>
          </div>
        )}

        {error && (
          <div className="sidebar-error">
            {error.includes("ENOENT") || error.includes("not found")
              ? "Bundle not found. Try: ../bundle.atlas or pick a registry package."
              : error}
          </div>
        )}

        {loaded && (
          <div className="sidebar-search-section">
            <input
              type="text"
              value={search}
              onChange={(e) => setSearch(e.target.value)}
              placeholder="Filter nodes..."
              className="sidebar-input"
            />
          </div>
        )}

        <div className="sidebar-node-list">
          <div className="sidebar-node-count">
            {filteredNodes.length} nodes, {filteredEdges.length} edges
            {bundleName ? ` \u00B7 ${bundleName}` : ""}
          </div>
          {filteredNodes.map((n) => (
            <div
              key={n.id}
              onClick={() => setSelected(n)}
              className={`sidebar-node-item ${selected?.id === n.id ? "selected" : ""}`}
            >
              <div className="sidebar-node-kind" style={{ borderLeftColor: getNodeColor(n.kind) }}>
                <div className="sidebar-node-name">{n.name}</div>
                <div className="sidebar-node-type">{n.kind}</div>
              </div>
            </div>
          ))}
          {hasGraph && search && filteredNodes.length === 0 && (
            <div className="sidebar-muted" style={{ padding: "8px 10px" }}>
              No nodes match &ldquo;{search}&rdquo;.
            </div>
          )}
        </div>
      </aside>

      <main className="graph-area">
        {loading ? (
          <CenteredMessage>Loading bundle\u2026</CenteredMessage>
        ) : error ? (
          <CenteredMessage tone="error">
            Failed to load bundle.
            <div className="centered-sub">{error}</div>
          </CenteredMessage>
        ) : hasGraph ? (
          <GraphView
            atlasNodes={filteredNodes}
            atlasEdges={filteredEdges}
            onNodeClick={setSelected}
          />
        ) : (
          <CenteredMessage>
            No knowledge loaded.
            <div className="centered-sub">
              {tab === "local"
                ? "Enter a path to a compiled .atlas bundle and press Load."
                : "Select a package from the registry above."}
            </div>
          </CenteredMessage>
        )}
      </main>

      {selected && (
        <aside className="detail-panel">
          <div className="detail-kind-badge" style={{ background: getNodeColor(selected.kind) }}>
            {selected.kind}
          </div>
          <h3 className="detail-name">{selected.name}</h3>
          <div className="detail-id">{selected.id}</div>
          {selected.version && (
            <div className="detail-row">
              Version: <strong>{selected.version}</strong>
            </div>
          )}
          {selected.description && (
            <div className="detail-desc">
              {selected.description.slice(0, 300)}
              {selected.description.length > 300 ? "\u2026" : ""}
            </div>
          )}
          {selectedEdges.length > 0 && (
            <div className="detail-connections">
              <div className="detail-connections-title">Connections</div>
              {selectedEdges.map((e, i) => {
                const isSrc = e.src === selected.id;
                const other = nodes.find((n) => n.id === (isSrc ? e.dst : e.src));
                return (
                  <div key={i} className="detail-connection-row">
                    <span className="detail-connection-dir">
                      {isSrc ? "\u2192" : "\u2190"} {e.edge_type}
                    </span>{" "}
                    {other ? other.name : isSrc ? e.dst : e.src}
                  </div>
                );
              })}
            </div>
          )}
        </aside>
      )}
    </div>
  );
}

function CenteredMessage({ children, tone }: { children: React.ReactNode; tone?: "error" }) {
  return (
    <div className={`centered-message ${tone === "error" ? "error" : ""}`}>
      {children}
    </div>
  );
}

export default function Home() {
  return (
    <Suspense fallback={null}>
      <HomeContent />
    </Suspense>
  );
}
