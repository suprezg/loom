/*
File Name: ProtocolSection.tsx
Purpose: Dashboard section component displaying Protocol entity cards and channel member badges.
*/

import type { ProtocolDto } from '../../types/wiki';
import { Radio, ArrowRight } from 'lucide-react';
import { MemberBadge } from '../MemberBadge';

interface ProtocolSectionProps {
  protocols: ProtocolDto[];
  onSelectEntity: (entityName: string) => void;
  onSelectMember: (entityName: string, memberName: string) => void;
}

export const ProtocolSection = ({
  protocols,
  onSelectEntity,
  onSelectMember
}: ProtocolSectionProps) => {
  return (
    <section className="space-y-4">
      <div className="flex items-center gap-3 pb-2 border-b border-[#E4D5C5] dark:border-[#3D3D3D]">
        <div className="p-2.5 rounded-xl bg-amber-500/10 text-amber-500">
          <Radio className="w-6 h-6" />
        </div>
        <div>
          <h2 className="text-2xl font-bold">Protocols</h2>
          <p className="text-xs opacity-60">Communication pipes, message channels, and event streams</p>
        </div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {protocols.map((proto) => (
          <div
            key={proto.name}
            className="p-6 rounded-2xl border transition-all hover:shadow-xl bg-[#FCF2E5] dark:bg-[#222222] border-[#E4D5C5] dark:border-[#3D3D3D] hover:border-amber-500 dark:hover:border-amber-500 flex flex-col justify-between group"
          >
            <div className="space-y-3">
              <div className="flex items-center justify-between">
                <h3
                  onClick={() => onSelectEntity(proto.name)}
                  className="text-xl font-bold cursor-pointer group-hover:text-amber-500 transition-colors"
                >
                  {proto.name}
                </h3>
                <button
                  onClick={() => onSelectEntity(proto.name)}
                  className="p-2 rounded-lg bg-black/5 dark:bg-white/5 hover:bg-amber-500/10 hover:text-amber-500 transition-colors"
                >
                  <ArrowRight className="w-4 h-4" />
                </button>
              </div>

              {proto.notes[0] && (
                <p className="text-xs opacity-75 line-clamp-3">
                  {proto.notes[0]}
                </p>
              )}

              {/* Channel Members */}
              <div className="space-y-2 pt-2">
                <div className="text-[11px] font-bold uppercase tracking-wider opacity-50">
                  Channels ({proto.channels.length})
                </div>
                <div className="flex flex-wrap gap-1.5">
                  {proto.channels.map((ch) => (
                    <button
                      key={ch.name}
                      onClick={() => onSelectMember(proto.name, ch.name)}
                      className="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-lg text-xs font-mono bg-black/5 dark:bg-white/5 hover:bg-amber-500/15 hover:text-amber-500 border border-black/10 dark:border-white/10 transition-colors"
                    >
                      <MemberBadge kind="Channel" />
                      <span className="truncate max-w-[140px]">{ch.name}</span>
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
