/*
File Name: SearchModal.tsx
Purpose: Global modal search dialog filtering strictly by Entity Names and Member Names.
*/

import { useEffect, useState, useMemo } from 'react';
import { Search, X, Box, FileText, Layers, Database, Radio } from 'lucide-react';
import type { WikiData, EntityKind, MemberKind } from '../types/wiki';
import { MemberBadge } from './MemberBadge';
import { buildSearchIndex } from './modals/SearchModalIndex';

interface SearchModalProps {
  isOpen: boolean;
  onClose: () => void;
  wikiData: WikiData;
  onSelectResult: (type: 'entity' | 'member', entityName: string, memberName?: string) => void;
}

export const SearchModal = ({
  isOpen,
  onClose,
  wikiData,
  onSelectResult
}: SearchModalProps) => {
  const [query, setQuery] = useState('');
  const [filterType, setFilterType] = useState<'all' | 'entity' | 'member'>('all');

  const searchIndex = useMemo(() => buildSearchIndex(wikiData), [wikiData]);

  const filteredResults = useMemo(() => {
    if (!query.trim()) {
      return searchIndex.filter((item) => filterType === 'all' || item.type === filterType);
    }

    const q = query.toLowerCase().trim();
    return searchIndex.filter((item) => {
      const matchesType = filterType === 'all' || item.type === filterType;
      const matchesName = item.name.toLowerCase().includes(q);
      const matchesTitle = item.title?.toLowerCase().includes(q);
      const matchesParent = item.parentEntityName.toLowerCase().includes(q);
      return matchesType && (matchesName || matchesTitle || matchesParent);
    });
  }, [searchIndex, query, filterType]);

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'Escape' && isOpen) {
        onClose();
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [isOpen, onClose]);

  if (!isOpen) return null;

  const getEntityIcon = (kind: EntityKind | MemberKind) => {
    switch (kind) {
      case 'Feature':
        return <FileText className="w-4 h-4 text-[#EC5B38]" />;
      case 'Component':
        return <Layers className="w-4 h-4 text-blue-500" />;
      case 'Storage':
        return <Database className="w-4 h-4 text-emerald-500" />;
      case 'Protocol':
        return <Radio className="w-4 h-4 text-amber-500" />;
      default:
        return <Box className="w-4 h-4 text-purple-500" />;
    }
  };

  return (
    <div
      className="fixed inset-0 z-50 flex items-start justify-center pt-16 px-4 bg-black/60 backdrop-blur-sm animate-fade-in"
      onClick={onClose}
    >
      <div
        className="w-full max-w-2xl rounded-2xl border shadow-2xl overflow-hidden flex flex-col max-h-[80vh] bg-[#FCF2E5] dark:bg-[#2C2C2C] text-[#2C2C2C] dark:text-[#FCF2E5] border-[#E4D5C5] dark:border-[#3D3D3D]"
        onClick={(e) => e.stopPropagation()}
      >
        {/* Search Header */}
        <div className="p-4 border-b border-[#E4D5C5] dark:border-[#3D3D3D] flex items-center gap-3">
          <Search className="w-5 h-5 text-[#EC5B38]" />
          <input
            type="text"
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            placeholder="Search Entities (Feature, Component...) or Members (Scenario, Contract...)"
            className="flex-1 bg-transparent border-none outline-none text-base font-medium placeholder:text-current/40"
            autoFocus
          />
          <button
            onClick={onClose}
            className="p-1 rounded-lg hover:bg-black/10 dark:hover:bg-white/10 transition-colors"
          >
            <X className="w-5 h-5 opacity-70" />
          </button>
        </div>

        {/* Filter Tabs */}
        <div className="px-4 py-2 border-b border-[#E4D5C5] dark:border-[#3D3D3D] flex items-center gap-2 text-xs font-semibold">
          <button
            onClick={() => setFilterType('all')}
            className={`px-3 py-1 rounded-full transition-colors ${
              filterType === 'all'
                ? 'bg-[#EC5B38] text-white'
                : 'hover:bg-black/10 dark:hover:bg-white/10 opacity-70'
            }`}
          >
            All Results
          </button>
          <button
            onClick={() => setFilterType('entity')}
            className={`px-3 py-1 rounded-full transition-colors ${
              filterType === 'entity'
                ? 'bg-[#EC5B38] text-white'
                : 'hover:bg-black/10 dark:hover:bg-white/10 opacity-70'
            }`}
          >
            Entities Only
          </button>
          <button
            onClick={() => setFilterType('member')}
            className={`px-3 py-1 rounded-full transition-colors ${
              filterType === 'member'
                ? 'bg-[#EC5B38] text-white'
                : 'hover:bg-black/10 dark:hover:bg-white/10 opacity-70'
            }`}
          >
            Members Only
          </button>
        </div>

        {/* Result Items List */}
        <div className="flex-1 overflow-y-auto p-2 space-y-1">
          {filteredResults.length === 0 ? (
            <div className="p-8 text-center text-sm opacity-60 font-mono">
              No matching Entity or Member found for "{query}".
            </div>
          ) : (
            filteredResults.map((item) => (
              <button
                key={item.id}
                onClick={() => {
                  onSelectResult(item.type, item.parentEntityName, item.type === 'member' ? item.name : undefined);
                  onClose();
                }}
                className="w-full text-left p-3 rounded-xl flex items-start gap-3 transition-colors hover:bg-[#EC5B38]/10 dark:hover:bg-[#EC5B38]/20 border border-transparent hover:border-[#EC5B38]/30 group"
              >
                <div className="p-2 rounded-lg bg-black/5 dark:bg-white/5 border border-black/10 dark:border-white/10 mt-0.5">
                  {getEntityIcon(item.kind)}
                </div>

                <div className="flex-1 min-w-0">
                  <div className="flex items-center gap-2">
                    {item.type === 'member' && <MemberBadge kind={item.kind} />}
                    <span className="font-semibold text-sm truncate group-hover:text-[#EC5B38]">
                      {item.name}
                    </span>
                    {item.type === 'member' && (
                      <span className="text-xs font-mono opacity-50 truncate">
                        in {item.parentEntityName}
                      </span>
                    )}
                  </div>

                  {item.description && (
                    <p className="text-xs opacity-70 truncate mt-1">
                      {item.description}
                    </p>
                  )}
                </div>
              </button>
            ))
          )}
        </div>
      </div>
    </div>
  );
};
