const REGISTRY_URL = "https://atlas-hub-registry.cbvarshini1.workers.dev";
const ATLAS_APP_URL = "https://atlas-sh.pages.dev";

async function searchPackages(query) {
  const q = query.toLowerCase().trim();
  const grid = document.getElementById("packageGrid");
  if (!grid) return;

  try {
    let data;
    if (q) {
      const res = await fetch(`${REGISTRY_URL}/api/search?q=${encodeURIComponent(q)}&limit=20`);
      const json = await res.json();
      data = json.results || [];
    } else {
      const res = await fetch(`${REGISTRY_URL}/api/packages`);
      data = await res.json();
    }
    grid.innerHTML = data.length
      ? data.map(p => pkgCard(p)).join("")
      : '<div class="package-card" style="grid-column:1/-1;text-align:center;color:var(--text-2)">No packages found</div>';
  } catch {
    grid.innerHTML = '<div class="package-card" style="grid-column:1/-1;text-align:center;color:var(--text-2)">Failed to load packages</div>';
  }
}

function pkgCard(p) {
  const stats = [];
  if (p.nodes != null) stats.push(`${p.nodes} nodes`);
  if (p.edges != null) stats.push(`${p.edges} edges`);
  stats.push(`v${p.version}`);
  const meta = stats.map(s => `<span>${s}</span>`).join("");
  return `<div class="package-card">
    <div class="package-name">${p.name}</div>
    <div class="package-desc">${p.description || ""}</div>
    <div class="package-meta">${meta}</div>
  </div>`;
}

searchPackages("");
