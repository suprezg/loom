/*
File Name: StorageDetailView.tsx
Purpose: Storage Schema Markdown view rendering Database Tables, Fields, Boxed Indexes, and Relations with emerald theme styling.
*/

import type { StorageDto, DiagramDto } from '../../types/wiki';
import { Tag, Link2, Maximize2 } from 'lucide-react';
import { MermaidRenderer } from '../MermaidRenderer';

interface StorageDetailViewProps {
  storage: StorageDto;
  highlightedMember?: string;
  onDecoratorClick: (target: string) => void;
  onOpenDiagramModal: (name: string, chart: string) => void;
  isDark?: boolean;
}

export const StorageDetailView = ({
  storage,
  highlightedMember,
  onDecoratorClick,
  onOpenDiagramModal,
  isDark = false
}: StorageDetailViewProps) => {
  const renderMemberDiagrams = (decorators: { decoratorType: string; target: string }[], diagrams: DiagramDto[]) => {
    const diagramDecorators = decorators.filter((d) => d.decoratorType === 'diagram');
    if (diagramDecorators.length === 0) return null;

    return (
      <div className="space-y-3 pt-3">
        {diagramDecorators.map((dec, i) => {
          const diag = diagrams.find((d) => d.name === dec.target);
          if (!diag) return null;
          return (
            <div
              key={i}
              onClick={() => onOpenDiagramModal(diag.name, diag.content)}
              className="p-4 rounded-2xl border bg-black/5 dark:bg-white/5 border-black/10 dark:border-white/10 space-y-2 group cursor-pointer transition-all hover:border-emerald-500"
            >
              <div className="flex items-center justify-between text-xs font-mono font-bold uppercase tracking-wider text-emerald-500">
                <span>Diagram: {diag.name}</span>
                <span className="text-[10px] lowercase font-normal opacity-60 group-hover:opacity-100 flex items-center gap-1">
                  <Maximize2 className="w-3 h-3" /> Click to zoom & pan
                </span>
              </div>
              <MermaidRenderer chart={diag.content} name={diag.name} isDark={isDark} />
            </div>
          );
        })}
      </div>
    );
  };

  return (
    <div className="space-y-10">
      {/* Overview Notes */}
      {storage.notes && storage.notes.length > 0 && (
        <section id="section-notes" className="space-y-2 text-base font-normal opacity-95 leading-relaxed py-1 scroll-mt-24">
          {storage.notes.map((note, i) => (
            <p key={i}>{note}</p>
          ))}
        </section>
      )}

      <div className="inline-block px-3 py-1 rounded-lg bg-emerald-500/10 border border-emerald-500/30 font-mono text-xs text-emerald-500 font-bold">
        Database Engine: {storage.engine}
      </div>

      <section id="section-tables" className="space-y-8 scroll-mt-24">
        <h2 className="text-2xl font-bold border-b border-[#E4D5C5] dark:border-[#3D3D3D] pb-2">
          Database Tables
        </h2>

        <div className="space-y-8">
          {storage.tables.map((t) => {
            const isTarget = highlightedMember === t.name;
            return (
              <section
                key={t.name}
                id={`member-${t.name}`}
                className={`space-y-4 p-4 sm:p-5 rounded-2xl transition-all border-2 scroll-mt-24 ${
                  isTarget
                    ? 'animate-pulse-highlight border-[#EC5B38]'
                    : 'border-transparent'
                }`}
              >
                <div className="flex flex-wrap items-center justify-between gap-2 border-b border-[#E4D5C5] dark:border-[#3D3D3D] pb-2">
                  <h3 className="text-xl font-bold font-mono text-[#2C2C2C] dark:text-[#FCF2E5]">{t.name}</h3>

                  <div className="flex flex-wrap gap-1.5">
                    {t.decorators.filter((dec) => dec.decoratorType !== 'diagram').map((dec, i) => (
                      <button
                        key={i}
                        onClick={() => onDecoratorClick(dec.target)}
                        className="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-lg text-xs font-mono transition-all bg-black/5 dark:bg-white/5 hover:bg-black/10 dark:hover:bg-white/15 border border-black/10 dark:border-white/10 hover:border-black/20 dark:hover:border-white/20 text-current"
                      >
                        <Tag className="w-3 h-3 text-purple-500 flex-shrink-0" />
                        <span>@{dec.decoratorType}({dec.target})</span>
                      </button>
                    ))}
                  </div>
                </div>

                <div className="overflow-x-auto rounded-xl border border-black/10 dark:border-white/10">
                  <table className="w-full text-left font-mono text-sm">
                    <thead className="bg-black/5 dark:bg-white/5 border-b border-black/10 dark:border-white/10">
                      <tr>
                        <th className="p-3.5 font-bold">Field Name</th>
                        <th className="p-3.5 font-bold">Type & Constraints</th>
                      </tr>
                    </thead>
                    <tbody className="divide-y divide-black/10 dark:divide-white/10">
                      {t.fields.map((f) => (
                        <tr key={f.name}>
                          <td className="p-3.5 font-bold text-emerald-500">{f.name}</td>
                          <td className="p-3.5 opacity-90">{f.fieldType}</td>
                        </tr>
                      ))}
                    </tbody>
                  </table>
                </div>

                {/* Boxed Storage Indexes */}
                {t.indexes.length > 0 && (
                  <div className="p-3.5 sm:p-4 rounded-xl bg-black/5 dark:bg-white/5 border border-black/10 dark:border-white/10 text-sm font-mono space-y-2">
                    <span className="font-bold text-emerald-500 uppercase text-[11px] tracking-wider block">Indexes:</span>
                    <div className="flex flex-wrap gap-2">
                      {t.indexes.map((idx, i) => (
                        <span key={i} className="px-2.5 py-1 rounded-md bg-emerald-500/10 text-emerald-500 border border-emerald-500/20 font-bold text-xs">
                          {idx}
                        </span>
                      ))}
                    </div>
                  </div>
                )}

                {t.relations.length > 0 && (
                  <div className="p-3.5 sm:p-4 rounded-xl bg-black/5 dark:bg-white/5 border border-black/10 dark:border-white/10 text-sm font-mono space-y-2">
                    <span className="font-bold text-emerald-500 uppercase text-[11px] tracking-wider flex items-center gap-1">
                      <Link2 className="w-4 h-4" />
                      <span>Relations:</span>
                    </span>
                    <div className="space-y-1">
                      {t.relations.map((rel, i) => (
                        <div key={i} className="flex flex-wrap items-center gap-2 opacity-90">
                          <span className="font-bold text-emerald-500">{rel.leftTable}.{rel.leftColumn}</span>
                          <span className="px-2 py-0.5 rounded bg-emerald-500/10 text-emerald-500 font-bold text-xs">{rel.relationType}</span>
                          <span className="font-bold text-emerald-500">{rel.rightTable}.{rel.rightColumn}</span>
                        </div>
                      ))}
                    </div>
                  </div>
                )}

                {renderMemberDiagrams(t.decorators, storage.diagrams)}
              </section>
            );
          })}
        </div>
      </section>
    </div>
  );
};
