/*
File Name: FeatureDetailView.tsx
Purpose: Feature Specification Markdown view rendering Notes, Background, Rules, Scenarios, and Diagrams.
*/

import type { FeatureDto, ScenarioDto, DiagramDto } from '../../types/wiki';
import { Tag, Maximize2 } from 'lucide-react';
import { MermaidRenderer } from '../MermaidRenderer';

export interface ScenarioWithRule extends ScenarioDto {
  ruleName?: string;
}

interface FeatureDetailViewProps {
  feature: FeatureDto;
  allFeatureScenarios: ScenarioWithRule[];
  highlightedMember?: string;
  onDecoratorClick: (target: string) => void;
  onOpenDiagramModal: (name: string, chart: string) => void;
  isDark?: boolean;
}

export const FeatureDetailView = ({
  feature,
  allFeatureScenarios,
  highlightedMember,
  onDecoratorClick,
  onOpenDiagramModal,
  isDark = false
}: FeatureDetailViewProps) => {
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
              className="p-4 rounded-2xl border bg-black/5 dark:bg-white/5 border-black/10 dark:border-white/10 space-y-2 group cursor-pointer transition-all hover:border-[#EC5B38]"
            >
              <div className="flex items-center justify-between text-xs font-mono font-bold uppercase tracking-wider text-[#EC5B38]">
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
      {feature.notes && feature.notes.length > 0 && (
        <section id="section-notes" className="space-y-2 text-base font-normal opacity-95 leading-relaxed py-1 scroll-mt-24">
          {feature.notes.map((note, i) => (
            <p key={i}>{note}</p>
          ))}
        </section>
      )}

      {/* H2: Background */}
      {feature.backgroundSteps.length > 0 && (
        <section id="section-background" className="space-y-3 scroll-mt-24">
          <h2 className="text-2xl font-bold border-b border-[#E4D5C5] dark:border-[#3D3D3D] pb-2">
            Background
          </h2>
          <div className="font-mono text-sm space-y-1.5 p-4 rounded-xl bg-black/5 dark:bg-white/5 border border-black/10 dark:border-white/10">
            {feature.backgroundSteps.map((step, idx) => (
              <div key={idx} className="flex items-start gap-2">
                <span className="font-bold text-[#EC5B38] w-14">{step.keyword}</span>
                <span className="opacity-90">{step.text}</span>
              </div>
            ))}
          </div>
        </section>
      )}

      {/* H2: Scenarios */}
      <section id="section-scenarios" className="space-y-8 scroll-mt-24">
        <h2 className="text-2xl font-bold border-b border-[#E4D5C5] dark:border-[#3D3D3D] pb-2">
          Scenarios
        </h2>

        <div className="space-y-8">
          {allFeatureScenarios.map((sc) => {
            const isTarget = highlightedMember === sc.name;
            return (
              <section
                key={sc.name}
                id={`member-${sc.name}`}
                className={`space-y-4 p-4 sm:p-5 rounded-2xl transition-all border-2 scroll-mt-24 ${
                  isTarget
                    ? 'animate-pulse-highlight border-[#EC5B38]'
                    : 'border-transparent'
                }`}
              >
                {/* H3: Scenario Identifier Name */}
                <div className="flex flex-wrap items-center justify-between gap-2 border-b border-[#E4D5C5] dark:border-[#3D3D3D] pb-2">
                  <h3 className="text-xl font-bold font-mono text-[#2C2C2C] dark:text-[#FCF2E5]">{sc.name}</h3>

                  <div className="flex flex-wrap gap-1.5">
                    {sc.decorators.filter((dec) => dec.decoratorType !== 'diagram').map((dec, i) => (
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

                {/* Rule */}
                {sc.ruleName && (
                  <div className="inline-block text-xs font-mono font-bold text-[#EC5B38] bg-[#EC5B38]/10 border border-[#EC5B38]/20 px-2.5 py-1 rounded-md">
                    Rule: {sc.ruleName}
                  </div>
                )}

                {sc.title && (
                  <p className="text-sm font-semibold opacity-85">
                    {sc.title}
                  </p>
                )}

                {/* Given / When / Then / But Steps Block */}
                <div className="font-mono text-sm space-y-2 p-4 rounded-xl bg-black/5 dark:bg-white/5 border border-black/10 dark:border-white/10">
                  {sc.steps.map((step, sIdx) => (
                    <div key={sIdx} className="flex items-start gap-2">
                      <span className="font-bold text-[#EC5B38] w-16 flex-shrink-0">{step.keyword}</span>
                      <span className="opacity-90">{step.text}</span>
                    </div>
                  ))}
                </div>

                {sc.examples.length > 0 && (
                  <div className="space-y-2">
                    <h5 className="text-xs font-bold uppercase tracking-wider opacity-60">Examples</h5>
                    <pre className="p-3.5 rounded-xl bg-black/5 dark:bg-white/5 border border-black/10 dark:border-white/10 font-mono text-sm overflow-x-auto">
                      {sc.examples.join('\n')}
                    </pre>
                  </div>
                )}

                {renderMemberDiagrams(sc.decorators, feature.diagrams)}
              </section>
            );
          })}
        </div>
      </section>
    </div>
  );
};
