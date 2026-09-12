/*
File Name: StorageSection.tsx
Purpose: Dashboard section component displaying Storage entity cards, database engines, and table member badges.
*/

import type { StorageDto } from '../../types/wiki';
import { Database, ArrowRight } from 'lucide-react';
import { MemberBadge } from '../MemberBadge';

interface StorageSectionProps {
  storages: StorageDto[];
  onSelectEntity: (entityName: string) => void;
  onSelectMember: (entityName: string, memberName: string) => void;
}

export const StorageSection = ({
  storages,
  onSelectEntity,
  onSelectMember
}: StorageSectionProps) => {
  return (
    <section className="space-y-4">
      <div className="flex items-center gap-3 pb-2 border-b border-[#E4D5C5] dark:border-[#3D3D3D]">
        <div className="p-2.5 rounded-xl bg-emerald-500/10 text-emerald-500">
          <Database className="w-6 h-6" />
        </div>
        <div>
          <h2 className="text-2xl font-bold">Storage</h2>
          <p className="text-xs opacity-60">Database engines, relational tables, fields, and indexes</p>
        </div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {storages.map((st) => (
          <div
            key={st.name}
            className="p-6 rounded-2xl border transition-all hover:shadow-xl bg-[#FCF2E5] dark:bg-[#222222] border-[#E4D5C5] dark:border-[#3D3D3D] hover:border-emerald-500 dark:hover:border-emerald-500 flex flex-col justify-between group"
          >
            <div className="space-y-3">
              <div className="flex items-center justify-between">
                <h3
                  onClick={() => onSelectEntity(st.name)}
                  className="text-xl font-bold cursor-pointer group-hover:text-emerald-500 transition-colors"
                >
                  {st.name}
                </h3>
                <button
                  onClick={() => onSelectEntity(st.name)}
                  className="p-2 rounded-lg bg-black/5 dark:bg-white/5 hover:bg-emerald-500/10 hover:text-emerald-500 transition-colors"
                >
                  <ArrowRight className="w-4 h-4" />
                </button>
              </div>

              <div className="text-xs font-mono px-2.5 py-1 rounded-md bg-black/5 dark:bg-white/5 text-emerald-600 dark:text-emerald-400 w-fit">
                Engine: {st.engine}
              </div>

              {st.notes[0] && (
                <p className="text-xs opacity-75 line-clamp-3">
                  {st.notes[0]}
                </p>
              )}

              {/* Table Members */}
              <div className="space-y-2 pt-2">
                <div className="text-[11px] font-bold uppercase tracking-wider opacity-50">
                  Tables ({st.tables.length})
                </div>
                <div className="flex flex-wrap gap-1.5">
                  {st.tables.map((t) => (
                    <button
                      key={t.name}
                      onClick={() => onSelectMember(st.name, t.name)}
                      className="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-lg text-xs font-mono bg-black/5 dark:bg-white/5 hover:bg-emerald-500/15 hover:text-emerald-500 border border-black/10 dark:border-white/10 transition-colors"
                    >
                      <MemberBadge kind="Table" />
                      <span className="truncate max-w-[140px]">{t.name}</span>
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
