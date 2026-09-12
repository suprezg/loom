/*
File Name: ComponentSection.tsx
Purpose: Dashboard section component displaying Component entity cards, model badges, and contract badges.
*/

import type { ComponentDto } from '../../types/wiki';
import { Layers, ArrowRight } from 'lucide-react';
import { MemberBadge } from '../MemberBadge';

interface ComponentSectionProps {
  components: ComponentDto[];
  onSelectEntity: (entityName: string) => void;
  onSelectMember: (entityName: string, memberName: string) => void;
}

export const ComponentSection = ({
  components,
  onSelectEntity,
  onSelectMember
}: ComponentSectionProps) => {
  return (
    <section className="space-y-4">
      <div className="flex items-center gap-3 pb-2 border-b border-[#E4D5C5] dark:border-[#3D3D3D]">
        <div className="p-2.5 rounded-xl bg-blue-500/10 text-blue-500">
          <Layers className="w-6 h-6" />
        </div>
        <div>
          <h2 className="text-2xl font-bold">Components</h2>
          <p className="text-xs opacity-60">Design-by-contract service modules, methods, and data models</p>
        </div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {components.map((comp) => (
          <div
            key={comp.name}
            className="p-6 rounded-2xl border transition-all hover:shadow-xl bg-[#FCF2E5] dark:bg-[#222222] border-[#E4D5C5] dark:border-[#3D3D3D] hover:border-blue-500 dark:hover:border-blue-500 flex flex-col justify-between group"
          >
            <div className="space-y-3">
              <div className="flex items-center justify-between">
                <h3
                  onClick={() => onSelectEntity(comp.name)}
                  className="text-xl font-bold cursor-pointer group-hover:text-blue-500 transition-colors"
                >
                  {comp.name}
                </h3>
                <button
                  onClick={() => onSelectEntity(comp.name)}
                  className="p-2 rounded-lg bg-black/5 dark:bg-white/5 hover:bg-blue-500/10 hover:text-blue-500 transition-colors"
                >
                  <ArrowRight className="w-4 h-4" />
                </button>
              </div>

              {comp.notes[0] && (
                <p className="text-xs opacity-75 line-clamp-3">
                  {comp.notes[0]}
                </p>
              )}

              {/* Contracts & Models Members */}
              <div className="space-y-2 pt-2">
                <div className="text-[11px] font-bold uppercase tracking-wider opacity-50">
                  Models & Contracts ({comp.models.length + comp.contracts.length})
                </div>
                <div className="flex flex-wrap gap-1.5">
                  {comp.models.map((m) => (
                    <button
                      key={m.name}
                      onClick={() => onSelectMember(comp.name, m.name)}
                      className="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-lg text-xs font-mono bg-black/5 dark:bg-white/5 hover:bg-blue-500/15 hover:text-blue-500 border border-black/10 dark:border-white/10 transition-colors"
                    >
                      <MemberBadge kind="Model" />
                      <span className="truncate max-w-[140px]">{m.name}</span>
                    </button>
                  ))}
                  {comp.contracts.map((c) => (
                    <button
                      key={c.name}
                      onClick={() => onSelectMember(comp.name, c.name)}
                      className="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-lg text-xs font-mono bg-black/5 dark:bg-white/5 hover:bg-blue-500/15 hover:text-blue-500 border border-black/10 dark:border-white/10 transition-colors"
                    >
                      <MemberBadge kind="Contract" />
                      <span className="truncate max-w-[140px]">{c.name}</span>
                    </button>
                  ))}
                </div>
              </div>
            </div>
          </div>
        ))}
      </div>
    </section>
  );
};
