/*
File Name: ProtocolDetailView.tsx
Purpose: Protocol Architecture Markdown view rendering Channels (Pattern, Transport, Payload, Sender, Receiver) and Boxed Errors with amber theme styling.
*/

import type { ProtocolDto, DiagramDto } from '../../types/wiki';
import { Tag, AlertCircle, Maximize2 } from 'lucide-react';
import { MermaidRenderer } from '../MermaidRenderer';

interface ProtocolDetailViewProps {
  protocol: ProtocolDto;
  highlightedMember?: string;
  onDecoratorClick: (target: string) => void;
  onOpenDiagramModal: (name: string, chart: string) => void;
  isDark?: boolean;
}

export const ProtocolDetailView = ({
  protocol,
  highlightedMember,
  onDecoratorClick,
  onOpenDiagramModal,
  isDark = false
}: ProtocolDetailViewProps) => {
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
              className="p-4 rounded-2xl border bg-black/5 dark:bg-white/5 border-black/10 dark:border-white/10 space-y-2 group cursor-pointer transition-all hover:border-amber-500"
            >
              <div className="flex items-center justify-between text-xs font-mono font-bold uppercase tracking-wider text-amber-500">
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
      {protocol.notes && protocol.notes.length > 0 && (
        <section id="section-notes" className="space-y-2 text-base font-normal opacity-95 leading-relaxed py-1 scroll-mt-24">
          {protocol.notes.map((note, i) => (
            <p key={i}>{note}</p>
          ))}
        </section>
      )}

      {/* H2: Channels */}
      <section id="section-channels" className="space-y-8 scroll-mt-24">
        <h2 className="text-2xl font-bold border-b border-[#E4D5C5] dark:border-[#3D3D3D] pb-2">
          Channels
        </h2>

        <div className="space-y-8">
          {protocol.channels.map((ch) => {
            const isTarget = highlightedMember === ch.name;
            return (
              <section
                key={ch.name}
                id={`member-${ch.name}`}
                className={`space-y-4 p-4 sm:p-5 rounded-2xl transition-all border-2 scroll-mt-24 ${
                  isTarget
                    ? 'animate-pulse-highlight border-[#EC5B38]'
                    : 'border-transparent'
                }`}
              >
                <div className="flex flex-wrap items-center justify-between gap-2 border-b border-[#E4D5C5] dark:border-[#3D3D3D] pb-2">
                  <h3 className="text-xl font-bold font-mono text-[#2C2C2C] dark:text-[#FCF2E5]">{ch.name}</h3>

                  <div className="flex flex-wrap gap-1.5">
                    {ch.decorators.filter((dec) => dec.decoratorType !== 'diagram').map((dec, i) => (
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

                {/* Sequential Channel Property Card Boxes */}
                {ch.pattern && (
                  <div className="p-3.5 sm:p-4 rounded-xl bg-black/5 dark:bg-white/5 border border-black/10 dark:border-white/10 text-sm font-mono">
                    <span className="font-bold text-amber-500 uppercase text-[11px] tracking-wider block mb-1">Pattern:</span>
                    <span className="opacity-90">{ch.pattern}</span>
                  </div>
                )}

                {ch.transport && (
                  <div className="p-3.5 sm:p-4 rounded-xl bg-black/5 dark:bg-white/5 border border-black/10 dark:border-white/10 text-sm font-mono">
                    <span className="font-bold text-amber-500 uppercase text-[11px] tracking-wider block mb-1">Transport:</span>
                    <span className="opacity-90">{ch.transport}</span>
                  </div>
                )}

                {ch.payload && (
                  <div className="p-3.5 sm:p-4 rounded-xl bg-black/5 dark:bg-white/5 border border-black/10 dark:border-white/10 text-sm font-mono">
                    <span className="font-bold text-amber-500 uppercase text-[11px] tracking-wider block mb-1">Payload:</span>
                    <span className="opacity-90">{ch.payload}</span>
                  </div>
                )}

                {ch.sender && (
                  <div className="p-3.5 sm:p-4 rounded-xl bg-black/5 dark:bg-white/5 border border-black/10 dark:border-white/10 text-sm font-mono">
                    <span className="font-bold text-amber-500 uppercase text-[11px] tracking-wider block mb-1">Sender:</span>
                    <span className="opacity-90">{ch.sender}</span>
                  </div>
                )}

                {ch.receiver && (
                  <div className="p-3.5 sm:p-4 rounded-xl bg-black/5 dark:bg-white/5 border border-black/10 dark:border-white/10 text-sm font-mono">
                    <span className="font-bold text-amber-500 uppercase text-[11px] tracking-wider block mb-1">Receiver:</span>
                    <span className="opacity-90">{ch.receiver}</span>
                  </div>
                )}

                {/* Boxed Protocol Errors */}
                {ch.errors.length > 0 && (
                  <div className="p-3.5 sm:p-4 rounded-xl bg-black/5 dark:bg-white/5 border border-black/10 dark:border-white/10 text-sm font-mono space-y-2">
                    <span className="font-bold text-red-500 uppercase text-[11px] tracking-wider flex items-center gap-1">
                      <AlertCircle className="w-4 h-4" />
                      <span>Errors:</span>
                    </span>
                    <div className="flex flex-wrap gap-1.5">
                      {ch.errors.map((err, i) => (
                        <span key={i} className="px-2.5 py-0.5 rounded bg-red-500/10 text-red-500 border border-red-500/20 font-bold text-xs">
                          [{err}]
                        </span>
                      ))}
                    </div>
                  </div>
                )}

                {renderMemberDiagrams(ch.decorators, protocol.diagrams)}
              </section>
            );
          })}
        </div>
      </section>
    </div>
  );
};
