"use client";

import { useCallback, useEffect, useMemo } from "react";
import {
  ReactFlow,
  MiniMap,
  Controls,
  Background,
  useNodesState,
  useEdgesState,
  Node,
  Edge,
  NodeTypes,
  MarkerType,
} from "@xyflow/react";
import "@xyflow/react/dist/style.css";
import dagre from "@dagrejs/dagre";
import { AtlasNode, AtlasEdge, getNodeColor } from "../lib/api";

function dagreLayout(nodes: Node[], edges: Edge[], direction: "LR" | "TB" = "LR") {
  const g = new dagre.graphlib.Graph();
  g.setDefaultEdgeLabel(() => ({}));
  g.setGraph({ rankdir: direction, nodesep: 60, ranksep: 100, marginx: 20, marginy: 20 });
  edges.forEach((e) => g.setEdge(e.source, e.target));
  nodes.forEach((n) => g.setNode(n.id, { width: 180, height: 60 }));
  dagre.layout(g);
  return nodes.map((n) => {
    const pos = g.node(n.id);
    return { ...n, position: { x: pos.x - 90, y: pos.y - 30 } };
  });
}

function AtlasNodeComponent({ data }: { data: { label: string; kind: string; id: string } }) {
  const color = getNodeColor(data.kind);
  return (
    <div
      className="atlas-node"
      style={{
        border: `2px solid ${color}`,
        borderRadius: 8,
        padding: "8px 14px",
        background: "white",
        fontSize: 13,
        minWidth: 150,
        maxWidth: 220,
        boxShadow: "0 1px 3px rgba(0,0,0,0.12)",
        cursor: "pointer",
      }}
    >
      <div
        className="atlas-node-kind"
        style={{
          fontSize: 10,
          color,
          fontWeight: 600,
          textTransform: "uppercase",
          letterSpacing: 1,
        }}
      >
        {data.kind}
      </div>
      <div
        className="atlas-node-label"
        style={{
          fontWeight: 500,
          marginTop: 2,
          overflow: "hidden",
          textOverflow: "ellipsis",
          whiteSpace: "nowrap",
        }}
      >
        {data.label}
      </div>
    </div>
  );
}

const nodeTypes: NodeTypes = { atlas: AtlasNodeComponent };

interface GraphViewProps {
  atlasNodes: AtlasNode[];
  atlasEdges: AtlasEdge[];
  onNodeClick?: (node: AtlasNode) => void;
}

const EDGE_LABELS: Record<string, string> = {
  DependsOn: "depends",
  Implements: "implements",
  PartOf: "part of",
  Uses: "uses",
  Extends: "extends",
  References: "refers",
  AlternativeTo: "alt",
  Solves: "solves",
  HasExample: "example",
  HasFailure: "failure",
  MigratesTo: "migrates",
  CompatibleWith: "compatible",
};

export default function GraphView({ atlasNodes, atlasEdges, onNodeClick }: GraphViewProps) {
  const flowNodes: Node[] = useMemo(
    () =>
      atlasNodes.map((n) => ({
        id: n.id,
        type: "atlas",
        data: { label: n.name, kind: n.kind, id: n.id },
        position: { x: 0, y: 0 },
      })),
    [atlasNodes]
  );

  const flowEdges: Edge[] = useMemo(
    () =>
      atlasEdges.map((e, i) => ({
        id: `e-${i}`,
        source: e.src,
        target: e.dst,
        label: EDGE_LABELS[e.edge_type] || e.edge_type,
        style: { stroke: "#94a3b8", strokeWidth: 1.5 },
        markerEnd: { type: MarkerType.ArrowClosed, color: "#94a3b8" },
        labelStyle: { fontSize: 10, fill: "#64748b" },
      })),
    [atlasEdges]
  );

  const layouted = useMemo(() => dagreLayout(flowNodes, flowEdges, "LR"), [flowNodes, flowEdges]);

  const [nodes, setNodes, onNodesChange] = useNodesState(layouted);
  const [edges, setEdges, onEdgesChange] = useEdgesState(flowEdges);

  useEffect(() => {
    setNodes(layouted);
    setEdges(flowEdges);
  }, [layouted, flowEdges, setNodes, setEdges]);

  const onNodeClickHandler = useCallback(
    (_: React.MouseEvent, node: Node) => {
      const atlasNode = atlasNodes.find((n) => n.id === node.id);
      if (atlasNode && onNodeClick) onNodeClick(atlasNode);
    },
    [atlasNodes, onNodeClick]
  );

  return (
    <div className="graph-container">
      <ReactFlow
        nodes={nodes}
        edges={edges}
        onNodesChange={onNodesChange}
        onEdgesChange={onEdgesChange}
        onNodeClick={onNodeClickHandler}
        nodeTypes={nodeTypes}
        fitView
        attributionPosition="bottom-left"
      >
        <Controls />
        <MiniMap
          nodeStrokeWidth={3}
          nodeColor={(n) => getNodeColor((n.data as { kind: string }).kind)}
        />
        <Background gap={16} />
      </ReactFlow>
    </div>
  );
}
