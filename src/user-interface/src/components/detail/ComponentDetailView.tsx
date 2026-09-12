/*
File Name: ComponentDetailView.tsx
Purpose: Component Module Markdown view rendering Notes, Invariants, Models, and Contracts with blue theme styling.
*/

import type { ComponentDto, DiagramDto } from '../../types/wiki';
import { Tag, AlertCircle, Maximize2 } from 'lucide-react';
import { MermaidRenderer } from '../MermaidRenderer';

interface ComponentDetailViewProps {
  component: ComponentDto;
  highlightedMember?: string;
  onDecoratorClick: (target: string) => void;
  onOpenDiagramModal: (name: string, chart: string) => void;
  isDark?: boolean;
}

export const ComponentDetailView = ({
  component,
  highlightedMember,
  onDecoratorClick,
  onOpenDiagramModal,
  isDark = false
}: ComponentDetailViewProps) => {
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
              className="p-4 rounded-2xl border bg-black/5 dark:bg-white/5 border-black/10 dark:border-white/10 space-y-2 group cursor-pointer transition-all hover:border-blue-500"
            >
              <div className="flex items-center justify-between text-xs font-mono font-bold uppercase tracking-wider text-blue-500">
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
      {component.notes && component.notes.length > 0 && (
        <section id="section-notes" className="space-y-2 text-base font-normal opacity-95 leading-relaxed py-1 scroll-mt-24">
          {component.notes.map((note, i) => (
            <p key={i}>{note}</p>
          ))}
        </section>
      )}

      {/* H2: Invariants */}
      {component.invariants.length > 0 && (
        <section id="section-invariants" className="space-y-3 scroll-mt-24">
          <h2 className="text-2xl font-bold border-b border-[#E4D5C5] dark:border-[#3D3D3D] pb-2 text-[#2C2C2C] dark:text-[#FCF2E5]">
            Invariants
          </h2>
          <ul className="list-disc list-inside space-y-1 text-sm font-mono opacity-90 p-4 rounded-xl bg-black/5 dark:bg-white/5 border border-black/10 dark:border-white/10">
            {component.invariants.map((inv, i) => (
              <li key={i}>{inv}</li>
            ))}
          </ul>
        </section>
      )}

      {/* H2: Models */}
      <section id="section-models" className="space-y-8 scroll-mt-24">
        <h2 className="text-2xl font-bold border-b border-[#E4D5C5] dark:border-[#3D3D3D] pb-2">
          Models
        </h2>

        <div className="space-y-8">
          {component.models.map((m) => {
            const isTarget = highlightedMember === m.name;
            return (
              <section
                key={m.name}
                id={`member-${m.name}`}
                className={`space-y-4 p-4 sm:p-5 rounded-2xl transition-all border-2 scroll-mt-24 ${
                  isTarget
                    ? 'animate-pulse-highlight border-[#EC5B38]'
                    : 'border-transparent'
                }`}
              >
                <div className="flex flex-wrap items-center justify-between gap-2 border-b border-[#E4D5C5] dark:border-[#3D3D3D] pb-2">
                  <h3 className="text-xl font-bold font-mono text-[#2C2C2C] dark:text-[#FCF2E5]">{m.name}</h3>

                  <div className="flex flex-wrap gap-1.5">
                    {m.decorators.filter((dec) => dec.decoratorType !== 'diagram').map((dec, i) => (
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

                <div className="inline-block text-xs font-mono font-bold text-blue-500 bg-blue-500/10 border border-blue-500/20 px-2.5 py-1 rounded-md">
                  Type: {m.modelType}
                </div>

                <div className="font-mono text-sm space-y-2 p-4 rounded-xl bg-black/5 dark:bg-white/5 border border-black/10 dark:border-white/10">
                  {m.members.map((mem, i) => (
                    <div key={i} className="flex justify-between border-b border-black/5 dark:border-white/5 pb-1 last:border-none">
                      <span className="font-bold text-blue-500">{mem[0]}</span>
                      <span className="opacity-75">{mem[1]}</span>
                    </div>
                  ))}
                </div>

                {renderMemberDiagrams(m.decorators, component.diagrams)}
              </section>
            );
          })}
        </div>
      </section>

      {/* H2: Contracts */}
      <section id="section-contracts" className="space-y-8 scroll-mt-24">
        <h2 className="text-2xl font-bold border-b border-[#E4D5C5] dark:border-[#3D3D3D] pb-2">
          Contracts
        </h2>

        <div className="space-y-8">
          {component.contracts.map((c) => {
            const isTarget = highlightedMember === c.name;
            return (
              <section
                key={c.name}
                id={`member-${c.name}`}
                className={`space-y-4 p-4 sm:p-5 rounded-2xl transition-all border-2 scroll-mt-24 ${
                  isTarget
                    ? 'animate-pulse-highlight border-[#EC5B38]'
                    : 'border-transparent'
                }`}
              >
                <div className="flex flex-wrap items-center justify-between gap-2 border-b border-[#E4D5C5] dark:border-[#3D3D3D] pb-2">
                  <h3 className="text-xl font-bold font-mono text-[#2C2C2C] dark:text-[#FCF2E5]">{c.name}</h3>

                  <div className="flex flex-wrap gap-1.5">
                    {c.decorators.filter((dec) => dec.decoratorType !== 'diagram').map((dec, i) => (
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

                {/* Signature Box */}
                <div className="p-3.5 sm:p-4 rounded-xl bg-black/5 dark:bg-white/5 border border-black/10 dark:border-white/10 font-mono text-sm space-y-1">
                  <span className="font-bold text-blue-500 uppercase text-[11px] tracking-wider block">Signature:</span>
                  <pre className="font-bold text-blue-500 overflow-x-auto whitespace-pre-wrap">{c.signature}</pre>
                </div>

                {/* Requires Box */}
                {c.requires && (
                  <div className="p-3.5 sm:p-4 rounded-xl bg-black/5 dark:bg-white/5 border border-black/10 dark:border-white/10 text-sm font-mono">
                    <span className="font-bold text-blue-500 uppercase text-[11px] tracking-wider block mb-1">Requires:</span>
                    <span className="opacity-90">{c.requires}</span>
                  </div>
                )}

                {/* Ensures Box */}
                {c.ensures && (
                  <div className="p-3.5 sm:p-4 rounded-xl bg-black/5 dark:bg-white/5 border border-black/10 dark:border-white/10 text-sm font-mono">
                    <span className="font-bold text-blue-500 uppercase text-[11px] tracking-wider block mb-1">Ensures:</span>
                    <span className="opacity-90">{c.ensures}</span>
                  </div>
                )}

                {/* Preconditions Box */}
                {c.preconditions.length > 0 && (
                  <div className="p-3.5 sm:p-4 rounded-xl bg-black/5 dark:bg-white/5 border border-black/10 dark:border-white/10 text-sm font-mono space-y-2">
                    <span className="font-bold text-blue-500 uppercase text-[11px] tracking-wider block">Preconditions:</span>
                    <ul className="space-y-1 pl-1">
                      {c.preconditions.map((p, i) => (
                        <li key={i} className="flex items-start gap-2 opacity-90">
                          <span className="text-blue-500 font-bold">•</span>
                          <span>{p}</span>
                        </li>
                      ))}
                    </ul>
                  </div>
                )}

                {/* Postconditions Box */}
                {c.postconditions.length > 0 && (
                  <div className="p-3.5 sm:p-4 rounded-xl bg-black/5 dark:bg-white/5 border border-black/10 dark:border-white/10 text-sm font-mono space-y-2">
                    <span className="font-bold text-blue-500 uppercase text-[11px] tracking-wider block">Postconditions:</span>
                    <ul className="space-y-1 pl-1">
                      {c.postconditions.map((post, i) => (
                        <li key={i} className="flex items-start gap-2 opacity-90">
                          <span className="text-blue-500 font-bold">•</span>
                          <span>{post}</span>
                        </li>
                      ))}
                    </ul>
                  </div>
                )}

                {/* Process Logic Box */}
                {c.process.length > 0 && (
                  <div className="p-3.5 sm:p-4 rounded-xl bg-black/5 dark:bg-white/5 border border-black/10 dark:border-white/10 text-sm font-mono space-y-2">
                    <span className="font-bold text-blue-500 uppercase text-[11px] tracking-wider block">Process Logic:</span>
                    <div className="space-y-1">
                      {c.process.map((pr, i) => (
                        <div key={i} className="opacity-90">{pr}</div>
                      ))}
                    </div>
                  </div>
                )}

                {/* Errors Box */}
                {c.errors.length > 0 && (
                  <div className="p-3.5 sm:p-4 rounded-xl bg-black/5 dark:bg-white/5 border border-black/10 dark:border-white/10 text-sm space-y-2">
                    <span className="font-bold text-red-500 uppercase text-[11px] tracking-wider flex items-center gap-1">
                      <AlertCircle className="w-4 h-4" />
                      <span>Errors:</span>
                    </span>
                    <div className="flex flex-wrap gap-1.5 font-mono">
                      {c.errors.map((err, i) => (
                        <span key={i} className="px-2.5 py-0.5 rounded bg-red-500/10 text-red-500 border border-red-500/20 font-bold text-xs">
                          [{err}]
                        </span>
                      ))}
                    </div>
                  </div>
                )}

                {renderMemberDiagrams(c.decorators, component.diagrams)}
              </section>
            );
          })}
        </div>
      </section>
    </div>
  );
};
