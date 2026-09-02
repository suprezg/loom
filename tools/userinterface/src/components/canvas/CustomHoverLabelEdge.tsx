/*
File Name: CustomHoverLabelEdge.tsx
Purpose: Custom React Flow Edge component revealing connection labels on hover.
*/

import { useState } from 'react';
import { BaseEdge, EdgeLabelRenderer, getSmoothStepPath, type EdgeProps } from '@xyflow/react';

export const CustomHoverLabelEdge = ({
  id,
  sourceX,
  sourceY,
  targetX,
  targetY,
  sourcePosition,
  targetPosition,
  style = {},
  markerEnd,
  label
}: EdgeProps) => {
  const [isHovered, setIsHovered] = useState(false);

  const [edgePath, labelX, labelY] = getSmoothStepPath({
    sourceX,
    sourceY,
    sourcePosition,
    targetX,
    targetY,
    targetPosition
  });

  return (
    <g
      className="react-flow__edge-hover-group cursor-pointer"
      onMouseEnter={() => setIsHovered(true)}
      onMouseLeave={() => setIsHovered(false)}
      style={{ pointerEvents: 'all' }}
    >
      {/* Invisible wider target path for easy hover */}
      <path
        d={edgePath}
        fill="none"
        stroke="transparent"
        strokeWidth={36}
        style={{ pointerEvents: 'stroke' }}
      />

      {/* Visible Connection Line */}
      <BaseEdge
        id={id}
        path={edgePath}
        markerEnd={markerEnd}
        style={{
          ...style,
          strokeWidth: isHovered ? 3.5 : 2.5,
          stroke: isHovered ? '#EC5B38' : '#EC5B38',
          transition: 'stroke-width 0.15s ease'
        }}
      />

      {/* Connection Label directly on edge - hidden by default, visible on hover */}
      {label && (
        <EdgeLabelRenderer>
          <div
            style={{
              position: 'absolute',
              left: 0,
              top: 0,
              transform: `translate(-50%, -50%) translate(${labelX}px, ${labelY}px)`,
              pointerEvents: 'none',
              opacity: isHovered ? 1 : 0,
              visibility: isHovered ? 'visible' : 'hidden',
              transition: 'opacity 0.2s ease, visibility 0.2s ease'
            }}
            className="z-50 px-3 py-1.5 rounded-xl text-xs font-extrabold shadow-xl border bg-[#FCF2E5] dark:bg-[#222222] text-[#2C2C2C] dark:text-[#FCF2E5] border-[#EC5B38] whitespace-nowrap"
          >
            {label}
          </div>
        </EdgeLabelRenderer>
      )}
    </g>
  );
};
