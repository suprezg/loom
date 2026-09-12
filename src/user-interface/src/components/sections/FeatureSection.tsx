/*
File Name: FeatureSection.tsx
Purpose: Dashboard section component displaying Feature entity cards and scenario member badges.
*/

import type { FeatureDto } from '../../types/wiki';
import { FileText, ArrowRight } from 'lucide-react';
import { MemberBadge } from '../MemberBadge';

interface FeatureSectionProps {
  features: FeatureDto[];
  onSelectEntity: (entityName: string) => void;
  onSelectMember: (entityName: string, memberName: string) => void;
}

export const FeatureSection = ({
  features,
  onSelectEntity,
  onSelectMember
}: FeatureSectionProps) => {
  return (
    <section className="space-y-4">
      <div className="flex items-center gap-3 pb-2 border-b border-[#E4D5C5] dark:border-[#3D3D3D]">
        <div className="p-2.5 rounded-xl bg-[#EC5B38]/10 text-[#EC5B38]">
          <FileText className="w-6 h-6" />
        </div>
        <div>
          <h2 className="text-2xl font-bold">Features</h2>
          <p className="text-xs opacity-60">High-level Gherkin behavioral specifications and rules</p>
        </div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {features.map((feature) => (
          <div
            key={feature.name}
            className="p-6 rounded-2xl border transition-all hover:shadow-xl bg-[#FCF2E5] dark:bg-[#222222] border-[#E4D5C5] dark:border-[#3D3D3D] hover:border-[#EC5B38] dark:hover:border-[#EC5B38] flex flex-col justify-between group"
          >
            <div className="space-y-3">
              <div className="flex items-center justify-between">
                <h3
                  onClick={() => onSelectEntity(feature.name)}
                  className="text-xl font-bold cursor-pointer group-hover:text-[#EC5B38] transition-colors"
                >
                  {feature.name}
                </h3>
                <button
                  onClick={() => onSelectEntity(feature.name)}
                  className="p-2 rounded-lg bg-black/5 dark:bg-white/5 hover:bg-[#EC5B38]/10 hover:text-[#EC5B38] transition-colors"
                >
                  <ArrowRight className="w-4 h-4" />
                </button>
              </div>

              {feature.notes[0] && (
                <p className="text-xs opacity-75 line-clamp-3">
                  {feature.notes[0]}
                </p>
              )}

              {/* Scenario Members */}
              <div className="space-y-2 pt-2">
                <div className="text-[11px] font-bold uppercase tracking-wider opacity-50">
                  Scenarios ({feature.rules.flatMap((r) => r.scenarios).concat(feature.scenarios).length})
                </div>
                <div className="flex flex-wrap gap-1.5">
                  {feature.rules.flatMap((r) => r.scenarios).concat(feature.scenarios).map((sc) => (
                    <button
                      key={sc.name}
                      onClick={() => onSelectMember(feature.name, sc.name)}
                      className="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-lg text-xs font-mono bg-black/5 dark:bg-white/5 hover:bg-[#EC5B38]/15 hover:text-[#EC5B38] border border-black/10 dark:border-white/10 transition-colors"
                    >
                      <MemberBadge kind={sc.isOutline ? 'Scenario Outline' : 'Scenario'} />
                      <span className="truncate max-w-[140px]">{sc.name}</span>
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
