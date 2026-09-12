/*
File Name: DiagramLightboxModal.tsx
Purpose: Fullscreen interactive Mermaid diagram lightbox modal with zoom, drag-pan, and Escape key scroll-lock.
*/

import React, { useState, useRef, useEffect } from 'react';
import { MermaidRenderer } from '../MermaidRenderer';
import { Tag, ZoomIn, ZoomOut, RotateCcw, X } from 'lucide-react';

interface DiagramLightboxModalProps {
  activeDiagramModal: { name: string; chart: string } | null;
  onClose: () => void;
  modalLabelColorClass: string;
  isDark?: boolean;
}

export const DiagramLightboxModal = ({
  activeDiagramModal,
  onClose,
  modalLabelColorClass,
  isDark = false
}: DiagramLightboxModalProps) => {
  const [zoomLevel, setZoomLevel] = useState<number>(1);
  const [pan, setPan] = useState<{ x: number; y: number }>({ x: 0, y: 0 });
  const [isDragging, setIsDragging] = useState<boolean>(false);
  const dragStartRef = useRef<{ x: number; y: number }>({ x: 0, y: 0 });

  useEffect(() => {
    if (activeDiagramModal) {
      setZoomLevel(1);
      setPan({ x: 0, y: 0 });
    }
  }, [activeDiagramModal]);

  if (!activeDiagramModal) return null;

  const handleMouseDown = (e: React.MouseEvent) => {
    setIsDragging(true);
    dragStartRef.current = { x: e.clientX - pan.x, y: e.clientY - pan.y };
  };

  const handleMouseMove = (e: React.MouseEvent) => {
    if (!isDragging) return;
    setPan({
      x: e.clientX - dragStartRef.current.x,
      y: e.clientY - dragStartRef.current.y
    });
  };

  const handleMouseUp = () => {
    setIsDragging(false);
  };

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center p-4 sm:p-8 bg-black/80 backdrop-blur-md animate-fade-in"
      onClick={onClose}
      onWheel={(e) => {
        e.preventDefault();
        e.stopPropagation();
      }}
    >
      <div
        className="w-full max-w-5xl h-[85vh] rounded-3xl border shadow-2xl overflow-hidden flex flex-col bg-[#FCF2E5] dark:bg-[#2C2C2C] text-[#2C2C2C] dark:text-[#FCF2E5] border-[#E4D5C5] dark:border-[#3D3D3D]"
        onClick={(e) => e.stopPropagation()}
      >
        {/* Modal Controls Header */}
        <div className="p-4 border-b border-[#E4D5C5] dark:border-[#3D3D3D] flex items-center justify-between gap-4 bg-black/5 dark:bg-white/5">
          <div className={`font-mono text-sm font-bold flex items-center gap-2 ${modalLabelColorClass}`}>
            <Tag className="w-4 h-4 text-purple-500" />
            <span>Diagram: {activeDiagramModal.name}</span>
          </div>

          <div className="flex items-center gap-2 font-mono text-xs">
            <button
              onClick={() => setZoomLevel((z) => Math.min(z + 0.2, 5))}
              className="p-2 rounded-xl bg-black/10 dark:bg-white/10 hover:bg-[#EC5B38] hover:text-white transition-colors"
              title="Zoom In"
            >
              <ZoomIn className="w-4 h-4" />
            </button>
            <button
              onClick={() => setZoomLevel((z) => Math.max(z - 0.2, 0.4))}
              className="p-2 rounded-xl bg-black/10 dark:bg-white/10 hover:bg-[#EC5B38] hover:text-white transition-colors"
              title="Zoom Out"
            >
              <ZoomOut className="w-4 h-4" />
            </button>
            <button
              onClick={() => {
                setZoomLevel(1);
                setPan({ x: 0, y: 0 });
              }}
              className="p-2 rounded-xl bg-black/10 dark:bg-white/10 hover:bg-[#EC5B38] hover:text-white transition-colors flex items-center gap-1"
              title="Reset View"
            >
              <RotateCcw className="w-3.5 h-3.5" />
              <span>{Math.round(zoomLevel * 100)}%</span>
            </button>
            <button
              onClick={onClose}
              className="p-2 rounded-xl bg-red-500/10 text-red-500 hover:bg-red-500 hover:text-white transition-colors ml-2"
              title="Close Modal"
            >
              <X className="w-4 h-4" />
            </button>
          </div>
        </div>

        {/* Modal Zoom & Pan Canvas */}
        <div
          className="flex-1 overflow-hidden relative cursor-grab active:cursor-grabbing flex items-center justify-center p-6 select-none"
          onMouseDown={handleMouseDown}
          onMouseMove={handleMouseMove}
          onMouseUp={handleMouseUp}
          onMouseLeave={handleMouseUp}
          onWheel={(e) => {
            e.preventDefault();
            e.stopPropagation();
            setZoomLevel((z) => Math.min(Math.max(z - e.deltaY * 0.001, 0.4), 5));
          }}
        >
          <div
            style={{
              transform: `translate(${pan.x}px, ${pan.y}px) scale(${zoomLevel})`,
              transformOrigin: 'center center',
              transition: isDragging ? 'none' : 'transform 0.1s ease-out'
            }}
            className="w-full h-full flex items-center justify-center pointer-events-none"
          >
            <div className="pointer-events-auto max-w-none">
              <MermaidRenderer chart={activeDiagramModal.chart} name={activeDiagramModal.name} isDark={isDark} />
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};
