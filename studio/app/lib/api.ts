export interface AtlasNode {
  id: string;
  name: string;
  kind: string;
  description?: string | null;
  version?: string | null;
}

export interface AtlasEdge {
  src: string;
  dst: string;
  edge_type: string;
}

export interface AtlasMeta {
  schema_version: string;
  generator: string;
  created_at: number;
}

export interface AtlasIR {
  nodes: AtlasNode[];
  edges: AtlasEdge[];
  meta: AtlasMeta;
}

export async function loadBundle(bundlePath: string): Promise<AtlasIR> {
  const res = await fetch(`/api/load?path=${encodeURIComponent(bundlePath)}`);
  if (!res.ok) {
    const err = await res.json();
    throw new Error(err.error || "Failed to load bundle");
  }
  return res.json();
}

export const KIND_COLORS: Record<string, string> = {
  Package: "#6366f1",
  Concept: "#8b5cf6",
  API: "#06b6d4",
  Class: "#10b981",
  Function: "#f59e0b",
  Module: "#ec4899",
  Protocol: "#14b8a6",
  Example: "#84cc16",
  FailureMode: "#ef4444",
};

export function getNodeColor(kind: string): string {
  return KIND_COLORS[kind] || "#6b7280";
}
