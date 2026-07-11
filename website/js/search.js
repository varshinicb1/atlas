const REGISTRY_URL = "https://atlas-hub-registry.cbvarshini1.workers.dev";

function pkgCard(p) {
  const desc = p.description ? p.description.substring(0, 80) + (p.description.length > 80 ? "..." : "") : "";
  const stats = [
    p.nodes != null ? `${p.nodes} nodes` : null,
    p.edges != null ? `${p.edges} edges` : null,
    `v${p.version}`
  ].filter(Boolean).map(s => `<span>${s}</span>`).join("");
  return `<div class="package-card">
    <div class="package-name">${p.name}</div>
    <div class="package-desc">${desc}</div>
    <div class="package-meta">${stats}</div>
  </div>`;
}

async function searchPackages(query) {
  const q = query ? query.toLowerCase().trim() : "";
  const grid = document.getElementById("packageGrid");
  if (!grid) return;
  try {
    let data;
    if (q) {
      const res = await fetch(`${REGISTRY_URL}/api/v1/search?q=${encodeURIComponent(q)}&limit=20`);
      const json = await res.json();
      data = json.results || [];
    } else {
      const res = await fetch(`${REGISTRY_URL}/health`);
      const json = await res.json();
      data = json.packages || [];
    }
    grid.innerHTML = data.length
      ? data.map(p => pkgCard(p)).join("")
      : '<div class="package-card" style="grid-column:1/-1;text-align:center;color:var(--text-2)">No packages found</div>';
  } catch {
    grid.innerHTML = '<div class="package-card" style="grid-column:1/-1;text-align:center;color:var(--text-2)">Failed to load packages from registry</div>';
  }
}

async function loadStats() {
  try {
    const res = await fetch(`${REGISTRY_URL}/health`);
    const json = await res.json();
    const pkgs = json.packages || [];
    const totalNodes = pkgs.reduce((s, p) => s + (p.nodes || 0), 0);
    const totalEdges = pkgs.reduce((s, p) => s + (p.edges || 0), 0);
    const totalPkgs = pkgs.length;
    document.getElementById("heroStats").innerHTML = `
      <div class="stat"><span class="stat-value">${totalNodes}+</span> Knowledge Nodes</div>
      <div class="stat"><span class="stat-value">${totalEdges}+</span> Edges</div>
      <div class="stat"><span class="stat-value">${totalPkgs}</span> Packages</div>
      <div class="stat"><span class="stat-value">0</span> LLM Calls Needed</div>`;
  } catch {}
}

searchPackages("");
loadStats();