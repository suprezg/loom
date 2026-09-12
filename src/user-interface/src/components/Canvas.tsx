/*
File Name: Canvas.tsx
Purpose: Interactive React Flow node-graph canvas rendering fabric macro-architecture blueprints with entity vs member double-click navigation.
*/

import { useMemo } from 'react';
import { ReactFlow, Controls, Background, MarkerType, type Node, type Edge } from '@xyflow/react';
import '@xyflow/react/dist/style.css';
import dagre from 'dagre';
import type { FabricDocumentDto, EntityKind } from '../types/wiki';
import { EntityNodeComponent } from './canvas/EntityNodeComponent';
import { CustomHoverLabelEdge } from './canvas/CustomHoverLabelEdge';

interface CanvasProps {
  fabric: FabricDocumentDto;
  onSelectEntity: (entityName: string) => void;
  onSelectMember?: (entityName: string, memberName: string) => void;
  isDark?: boolean;
}

const nodeTypes = {
  entityNode: EntityNodeComponent
};

const edgeTypes = {
  hoverLabelEdge: CustomHoverLabelEdge
};

export const Canvas = ({ fabric, onSelectEntity, onSelectMember, isDark = false }: CanvasProps) => {
  const { initialNodes, initialEdges } = useMemo(() => {
    const dagreGraph = new dagre.graphlib.Graph();
    dagreGraph.setDefaultEdgeLabel(() => ({}));
    dagreGraph.setGraph({ rankdir: 'TB', ranksep: 180, nodesep: 120 });

    const rawNodes: Node[] = [];
    const rawEdges: Edge[] = [];
    const entitySet = new Set<string>();

    fabric.groups.forEach((group) => {
      group.members.forEach((member) => {
        if (!entitySet.has(member.path)) {
          entitySet.add(member.path);
          dagreGraph.setNode(member.path, { width: 240, height: 100 });
          rawNodes.push({
            id: member.path,
            type: 'entityNode',
            position: { x: 0, y: 0 },
            data: {
              label: member.path,
              kind: member.kind as EntityKind,
              onSelectEntity,
              onSelectMember,
              groupName: group.name
            }
          });
        }
      });
    });

    fabric.connections.forEach((conn, index) => {
      const sourceId = conn.source.path;
      const targetId = conn.target.path;

      if (!entitySet.has(sourceId)) {
        entitySet.add(sourceId);
        dagreGraph.setNode(sourceId, { width: 240, height: 100 });
        rawNodes.push({
          id: sourceId,
          type: 'entityNode',
          position: { x: 0, y: 0 },
          data: {
            label: sourceId,
            kind: conn.source.kind as EntityKind,
            onSelectEntity,
            onSelectMember
          }
        });
      }

      if (!entitySet.has(targetId)) {
        entitySet.add(targetId);
        dagreGraph.setNode(targetId, { width: 240, height: 100 });
        rawNodes.push({
          id: targetId,
          type: 'entityNode',
          position: { x: 0, y: 0 },
          data: {
            label: targetId,
            kind: conn.target.kind as EntityKind,
            onSelectEntity,
            onSelectMember
          }
        });
      }

      dagreGraph.setEdge(sourceId, targetId);
      rawEdges.push({
        id: `edge-${index}`,
        source: sourceId,
        target: targetId,
        label: conn.label,
        type: 'hoverLabelEdge',
        animated: true,
        markerEnd: {
          type: MarkerType.ArrowClosed,
          color: '#EC5B38'
        }
      });
    });

    dagre.layout(dagreGraph);

    const layoutedNodes = rawNodes.map((node) => {
      const nodeWithPosition = dagreGraph.node(node.id);
      return {
        ...node,
        position: {
          x: nodeWithPosition.x - 120,
          y: nodeWithPosition.y - 50
        }
      };
    });

    return { initialNodes: layoutedNodes, initialEdges: rawEdges };
  }, [fabric, onSelectEntity, onSelectMember]);

  return (
    <div className="w-full h-[calc(100vh-6rem)] overflow-hidden bg-transparent">
      <ReactFlow
        defaultNodes={initialNodes}
        defaultEdges={initialEdges}
        nodeTypes={nodeTypes}
        edgeTypes={edgeTypes}
        fitView
        nodesDraggable={true}
        elementsSelectable={true}
        colorMode={isDark ? 'dark' : 'light'}
        onNodeDoubleClick={(_event, node) => {
          if (node.id.includes('::')) {
            const [eName, mName] = node.id.split('::');
            if (onSelectMember) {
              onSelectMember(eName, mName);
            } else {
              onSelectEntity(eName);
            }
          } else {
            onSelectEntity(node.id);
          }
        }}
      >
        <Background color={isDark ? '#444444' : '#D5C4B3'} gap={28} size={2} />
        <Controls className="!bg-[#FCF2E5] dark:!bg-[#222222] !border-[#E4D5C5] dark:!border-[#3D3D3D] !text-current !rounded-xl" />
      </ReactFlow>
    </div>
  );
};
