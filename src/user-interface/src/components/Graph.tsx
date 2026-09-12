/*
File Name: Graph.tsx
Purpose: Interactive Sigma.js network graph canvas orchestrator with hover highlighting and double-click navigation.
*/

import { useEffect, useRef } from 'react';
import Sigma from 'sigma';
import type { ThreadDocumentDto } from '../types/wiki';
import { buildGraphologyInstance } from './graph/SigmaGraphEngine';

interface GraphProps {
  thread: ThreadDocumentDto;
  onSelectEntity: (entityName: string) => void;
  onSelectMember: (entityName: string, memberName: string) => void;
  isDark?: boolean;
}

export const Graph = ({
  thread,
  onSelectEntity,
  onSelectMember,
  isDark = false
}: GraphProps) => {
  const containerRef = useRef<HTMLDivElement>(null);
  const sigmaRef = useRef<Sigma | null>(null);

  const fgColor = isDark ? '#FCF2E5' : '#2C2C2C';
  const dimmedEdgeColor = isDark ? '#222222' : '#F4E8DB';
  const dimmedNodeColor = isDark ? '#3A3A3A' : '#E8DDD0';

  useEffect(() => {
    if (!containerRef.current) return;

    const graph = buildGraphologyInstance(thread, fgColor);

    let hoveredNode: string | null = null;

    const sigmaSettings: any = {
      labelFont: 'Nunito, sans-serif',
      labelWeight: 'bold',
      labelColor: { color: '#FF2A85' },
      defaultNodeColor: '#EC5B38',
      defaultEdgeColor: fgColor,
      renderEdgeLabels: false,
      nodeReducer: (node: string, data: any) => {
        if (!hoveredNode) return data;

        const isHovered = node === hoveredNode;
        const isNeighbor = graph.hasEdge(hoveredNode, node) || graph.hasEdge(node, hoveredNode);

        if (isHovered || isNeighbor) {
          return { ...data, zIndex: 10 };
        }

        return {
          ...data,
          color: dimmedNodeColor,
          label: '',
          zIndex: 0
        };
      },
      edgeReducer: (edge: string, data: any) => {
        if (!hoveredNode) return data;

        const [source, target] = graph.extremities(edge);
        const isConnected = source === hoveredNode || target === hoveredNode;

        if (isConnected) {
          return { ...data, color: fgColor, size: 3.5, zIndex: 10 };
        }

        return {
          ...data,
          color: dimmedEdgeColor,
          hidden: true,
          zIndex: 0
        };
      }
    };

    const renderer = new Sigma(graph, containerRef.current, sigmaSettings);
    sigmaRef.current = renderer;

    let draggedNode: string | null = null;
    let isDragging = false;

    renderer.on('downNode', (e) => {
      isDragging = true;
      draggedNode = e.node;
    });

    const mouseCaptor = renderer.getMouseCaptor();

    const handleMouseMove = (e: any) => {
      if (isDragging && draggedNode) {
        const pos = renderer.viewportToGraph(e);
        graph.setNodeAttribute(draggedNode, 'x', pos.x);
        graph.setNodeAttribute(draggedNode, 'y', pos.y);
        e.preventSigmaDefault();
        e.original.preventDefault();
        e.original.stopPropagation();
      }
    };

    const handleMouseUp = () => {
      isDragging = false;
      draggedNode = null;
    };

    mouseCaptor.on('mousemove', handleMouseMove);
    mouseCaptor.on('mouseup', handleMouseUp);

    renderer.on('enterNode', ({ node }) => {
      hoveredNode = node;
      renderer.refresh();
    });

    renderer.on('leaveNode', () => {
      hoveredNode = null;
      renderer.refresh();
    });

    renderer.on('doubleClickNode', ({ node }) => {
      const attrs = graph.getNodeAttributes(node);
      if (attrs.nodeType === 'entity') {
        onSelectEntity(attrs.entityName);
      } else if (attrs.nodeType === 'member') {
        onSelectMember(attrs.entityName, attrs.memberName);
      }
    });

    return () => {
      mouseCaptor.off('mousemove', handleMouseMove);
      mouseCaptor.off('mouseup', handleMouseUp);
      renderer.kill();
      sigmaRef.current = null;
    };
  }, [thread, onSelectEntity, onSelectMember, isDark, fgColor, dimmedEdgeColor, dimmedNodeColor]);

  return (
    <div className="w-full h-[calc(100vh-6rem)] relative overflow-hidden bg-transparent">
      <div ref={containerRef} className="w-full h-full" />
    </div>
  );
};
