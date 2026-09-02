/*
File Name: TableOfContents.tsx
Purpose: Sticky collapsible Table of Contents sidebar module for entity detail views.
*/

import { useState } from 'react';
import type { FeatureDto, ComponentDto, StorageDto, ProtocolDto, ScenarioDto } from '../../types/wiki';
import { List } from 'lucide-react';

interface ScenarioWithRule extends ScenarioDto {
  ruleName?: string;
}

interface TableOfContentsProps {
  feature?: FeatureDto;
  component?: ComponentDto;
  storage?: StorageDto;
  protocol?: ProtocolDto;
  allFeatureScenarios: ScenarioWithRule[];
  onScrollToSection: (id: string) => void;
}

export const TableOfContents = ({
  feature,
  component,
  storage,
  protocol,
  allFeatureScenarios,
  onScrollToSection
}: TableOfContentsProps) => {
  const [isTocOpen, setIsTocOpen] = useState<boolean>(true);

  return (
    <aside
      className={`hidden xl:block transition-all duration-300 flex-shrink-0 sticky top-20 ${
        isTocOpen ? 'w-64' : 'w-10'
      }`}
    >
      <div
        className={`rounded-2xl border bg-[#FCF2E5] dark:bg-[#2C2C2C] border-[#E4D5C5] dark:border-[#3D3D3D] shadow-sm flex flex-col justify-center items-center ${
          isTocOpen ? 'p-3 items-stretch' : 'p-1'
        }`}
      >
        {!isTocOpen ? (
          <button
            onClick={() => setIsTocOpen(true)}
            className="w-8 h-8 rounded-xl flex items-center justify-center bg-black/5 dark:bg-white/5 hover:bg-[#EC5B38]/10 text-[#EC5B38] transition-colors"
            title="Open Table of Contents"
          >
            <List className="w-4 h-4" />
          </button>
        ) : (
          <div className="space-y-3">
            <div className="flex items-center justify-between border-b border-black/10 dark:border-white/10 pb-2">
              <div className="flex items-center gap-2 text-xs font-bold uppercase tracking-wider text-[#EC5B38]">
                <List className="w-4 h-4" />
                <span>TOC</span>
              </div>
              <button
                onClick={() => setIsTocOpen(false)}
                className="w-6 h-6 rounded-lg flex items-center justify-center bg-black/5 dark:bg-white/5 hover:bg-[#EC5B38]/10 text-[#EC5B38] font-mono text-sm font-bold transition-colors"
                title="Close Table of Contents"
              >
                &gt;
              </button>
            </div>

            <div className="space-y-3 text-xs font-mono max-h-[calc(100vh-12rem)] overflow-y-auto pr-1">
              {feature && (
                <>
                  {feature.notes && feature.notes.length > 0 && (
                    <button
                      onClick={() => onScrollToSection('section-notes')}
                      className="block w-full text-left font-bold truncate hover:text-[#EC5B38] transition-colors"
                    >
                      Overview Notes
                    </button>
                  )}
                  {feature.backgroundSteps.length > 0 && (
                    <button
                      onClick={() => onScrollToSection('section-background')}
                      className="block w-full text-left font-bold truncate hover:text-[#EC5B38] transition-colors"
                    >
                      Background
                    </button>
                  )}
                  <div className="space-y-1">
                    <button
                      onClick={() => onScrollToSection('section-scenarios')}
                      className="block w-full text-left font-bold truncate text-[#EC5B38]"
                    >
                      Scenarios
                    </button>
                    <div className="pl-3 space-y-1 border-l border-black/10 dark:border-white/10">
                      {allFeatureScenarios.map((sc) => (
                        <button
                          key={sc.name}
                          onClick={() => onScrollToSection(`member-${sc.name}`)}
                          className="block w-full text-left truncate opacity-75 hover:opacity-100 hover:text-[#EC5B38] transition-colors"
                        >
                          {sc.name}
                        </button>
                      ))}
                    </div>
                  </div>
                </>
              )}

              {component && (
                <>
                  {component.notes && component.notes.length > 0 && (
                    <button
                      onClick={() => onScrollToSection('section-notes')}
                      className="block w-full text-left font-bold truncate hover:text-blue-500 transition-colors"
                    >
                      Engine Notes
                    </button>
                  )}
                  {component.invariants.length > 0 && (
                    <button
                      onClick={() => onScrollToSection('section-invariants')}
                      className="block w-full text-left font-bold truncate hover:text-blue-500 transition-colors"
                    >
                      Invariants
                    </button>
                  )}
                  {component.models.length > 0 && (
                    <div className="space-y-1">
                      <button
                        onClick={() => onScrollToSection('section-models')}
                        className="block w-full text-left font-bold truncate text-blue-500"
                      >
                        Models
                      </button>
                      <div className="pl-3 space-y-1 border-l border-black/10 dark:border-white/10">
                        {component.models.map((m) => (
                          <button
                            key={m.name}
                            onClick={() => onScrollToSection(`member-${m.name}`)}
                            className="block w-full text-left truncate opacity-75 hover:opacity-100 hover:text-blue-500 transition-colors"
                          >
                            {m.name}
                          </button>
                        ))}
                      </div>
                    </div>
                  )}
                  {component.contracts.length > 0 && (
                    <div className="space-y-1">
                      <button
                        onClick={() => onScrollToSection('section-contracts')}
                        className="block w-full text-left font-bold truncate text-blue-[#3B82F6]"
                      >
                        Contracts
                      </button>
                      <div className="pl-3 space-y-1 border-l border-black/10 dark:border-white/10">
                        {component.contracts.map((c) => (
                          <button
                            key={c.name}
                            onClick={() => onScrollToSection(`member-${c.name}`)}
                            className="block w-full text-left truncate opacity-75 hover:opacity-100 hover:text-blue-500 transition-colors"
                          >
                            {c.name}
                          </button>
                        ))}
                      </div>
                    </div>
                  )}
                </>
              )}

              {storage && (
                <div className="space-y-1">
                  {storage.notes && storage.notes.length > 0 && (
                    <button
                      onClick={() => onScrollToSection('section-notes')}
                      className="block w-full text-left font-bold truncate hover:text-emerald-500 transition-colors"
                    >
                      Schema Notes
                    </button>
                  )}
                  <button
                    onClick={() => onScrollToSection('section-tables')}
                    className="block w-full text-left font-bold truncate text-emerald-500"
                  >
                    Database Tables
                  </button>
                  <div className="pl-3 space-y-1 border-l border-black/10 dark:border-white/10">
                    {storage.tables.map((t) => (
                      <button
                        key={t.name}
                        onClick={() => onScrollToSection(`member-${t.name}`)}
                        className="block w-full text-left truncate opacity-75 hover:opacity-100 hover:text-emerald-500 transition-colors"
                      >
                        {t.name}
                      </button>
                    ))}
                  </div>
                </div>
              )}

              {protocol && (
                <div className="space-y-1">
                  {protocol.notes && protocol.notes.length > 0 && (
                    <button
                      onClick={() => onScrollToSection('section-notes')}
                      className="block w-full text-left font-bold truncate hover:text-amber-500 transition-colors"
                    >
                      Protocol Notes
                    </button>
                  )}
                  <button
                    onClick={() => onScrollToSection('section-channels')}
                    className="block w-full text-left font-bold truncate text-amber-500"
                  >
                    Channels
                  </button>
                  <div className="pl-3 space-y-1 border-l border-black/10 dark:border-white/10">
                    {protocol.channels.map((ch) => (
                      <button
                        key={ch.name}
                        onClick={() => onScrollToSection(`member-${ch.name}`)}
                        className="block w-full text-left truncate opacity-75 hover:opacity-100 hover:text-amber-500 transition-colors"
                      >
                        {ch.name}
                      </button>
                    ))}
                  </div>
                </div>
              )}
            </div>
          </div>
        )}
      </div>
    </aside>
  );
};
