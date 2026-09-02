/*
File Name: MermaidRenderer.tsx
Purpose: Client-side diagram renderer rendering Mermaid code blocks to SVG using mermaid.js without error leaks.
*/

import { useEffect, useRef, useState } from 'react';
import mermaid from 'mermaid';

interface MermaidRendererProps {
  chart: string;
  name: string;
  isDark?: boolean;
}

export const MermaidRenderer = ({ chart, name, isDark = false }: MermaidRendererProps) => {
  const containerRef = useRef<HTMLDivElement>(null);
  const [svgContent, setSvgContent] = useState<string>('');
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    mermaid.initialize({
      startOnLoad: false,
      theme: isDark ? 'dark' : 'default',
      securityLevel: 'loose',
      fontFamily: 'Nunito, sans-serif',
      suppressErrorRendering: true,
    });

    let isMounted = true;
    const cleanChart = chart.trim();
    const uniqueId = `mermaid-${name.replace(/[^a-zA-Z0-9_-]/g, '')}-${Math.random().toString(36).substring(2, 9)}`;

    const renderChart = async () => {
      try {
        const { svg } = await mermaid.render(uniqueId, cleanChart);
        if (isMounted) {
          setSvgContent(svg);
          setError(null);
        }
      } catch (err: any) {
        if (isMounted) {
          setError(err?.message || 'Syntax error in diagram definition');
        }
      } finally {
        // Clean up any residual error elements inserted into document.body by mermaid
        document.querySelectorAll('#d, body > svg[id^="d"]').forEach((el) => el.remove());
      }
    };

    renderChart();

    return () => {
      isMounted = false;
      document.querySelectorAll('#d, body > svg[id^="d"]').forEach((el) => el.remove());
    };
  }, [chart, name, isDark]);

  if (error) {
    return (
      <div className="p-4 rounded-xl border bg-red-500/10 border-red-500/30 text-red-500 text-xs font-mono">
        <div className="font-bold flex items-center gap-2 mb-1">
          <span>Diagram Render Warning</span>
        </div>
        <div className="opacity-90">{error}</div>
      </div>
    );
  }

  return (
    <div
      ref={containerRef}
      className="p-4 rounded-2xl border bg-black/5 dark:bg-white/5 border-black/10 dark:border-white/10 overflow-x-auto flex justify-center items-center my-3"
      dangerouslySetInnerHTML={{ __html: svgContent }}
    />
  );
};
