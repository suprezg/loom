/*
File Name: HomeViewMenu.tsx
Purpose: Dropdown menu fragment for Home View mode selection (Sections, Canvas, Graph).
*/

import React from 'react';
import { Home, ChevronDown, LayoutGrid, Network, Share2 } from 'lucide-react';
import type { HomeViewMode } from '../TopBar';

interface HomeViewMenuProps {
  homeMenuRef: React.RefObject<HTMLDivElement | null>;
  isHomeMenuOpen: boolean;
  setIsHomeMenuOpen: React.Dispatch<React.SetStateAction<boolean>>;
  homeViewMode: HomeViewMode;
  hasFabric: boolean;
  onGoHome: () => void;
  onSelectHomeViewMode: (mode: HomeViewMode) => void;
}

export const HomeViewMenu = ({
  homeMenuRef,
  isHomeMenuOpen,
  setIsHomeMenuOpen,
  homeViewMode,
  hasFabric,
  onGoHome,
  onSelectHomeViewMode
}: HomeViewMenuProps) => {
  const getHomeViewLabel = () => {
    switch (homeViewMode) {
      case 'static':
        return 'Sections';
      case 'fabric':
        return 'Canvas';
      case 'thread':
        return 'Graph';
    }
  };

  return (
    <div ref={homeMenuRef} className="relative flex items-center gap-1.5 flex-shrink-0">
      <button
        onClick={() => setIsHomeMenuOpen((prev) => !prev)}
        className="min-w-[44px] min-h-[44px] sm:min-h-[40px] px-3 py-2 rounded-xl flex items-center justify-center gap-2 border transition-all bg-white/50 dark:bg-black/30 border-[#E4D5C5] dark:border-[#3D3D3D] hover:border-[#EC5B38] text-[#EC5B38]"
        title="Home View Selector"
        aria-label="Home View Selector"
      >
        <Home className="w-5 h-5 flex-shrink-0" />
        <span className="hidden sm:inline-block text-xs font-bold text-[#2C2C2C] dark:text-[#FCF2E5]">
          {getHomeViewLabel()}
        </span>
        <ChevronDown className="w-3.5 h-3.5 opacity-70 flex-shrink-0" />
      </button>

      {/* Home View Menu Dropdown */}
      {isHomeMenuOpen && (
        <div
          className="absolute left-0 top-12 z-50 w-48 rounded-2xl border shadow-2xl p-2 bg-[#FCF2E5] dark:bg-[#2C2C2C] border-[#E4D5C5] dark:border-[#3D3D3D] space-y-1 animate-fade-in"
          onClick={() => setIsHomeMenuOpen(false)}
        >
          <div className="px-3 py-1.5 text-[10px] font-bold uppercase tracking-wider opacity-50">
            Choose Home View
          </div>

          <button
            onClick={() => {
              onGoHome();
              onSelectHomeViewMode('static');
            }}
            className={`w-full text-left p-2.5 rounded-xl flex items-center gap-2.5 text-xs font-bold transition-colors ${
              homeViewMode === 'static'
                ? 'bg-[#EC5B38] text-white'
                : 'hover:bg-black/10 dark:hover:bg-white/10'
            }`}
          >
            <LayoutGrid className="w-4 h-4" />
            <span>Sections</span>
          </button>

          <button
            disabled={!hasFabric}
            onClick={() => {
              if (hasFabric) {
                onGoHome();
                onSelectHomeViewMode('fabric');
              }
            }}
            className={`w-full text-left p-2.5 rounded-xl flex items-center justify-between text-xs font-bold transition-colors ${
              homeViewMode === 'fabric'
                ? 'bg-[#EC5B38] text-white'
                : hasFabric
                ? 'hover:bg-black/10 dark:hover:bg-white/10'
                : 'opacity-40 cursor-not-allowed'
            }`}
          >
            <div className="flex items-center gap-2.5">
              <Network className="w-4 h-4" />
              <span>Canvas</span>
            </div>
            {!hasFabric && <span className="text-[10px] font-mono opacity-60">No AST</span>}
          </button>

          <button
            onClick={() => {
              onGoHome();
              onSelectHomeViewMode('thread');
            }}
            className={`w-full text-left p-2.5 rounded-xl flex items-center gap-2.5 text-xs font-bold transition-colors ${
              homeViewMode === 'thread'
                ? 'bg-[#EC5B38] text-white'
                : 'hover:bg-black/10 dark:hover:bg-white/10'
            }`}
          >
            <Share2 className="w-4 h-4" />
            <span>Graph</span>
          </button>
        </div>
      )}
    </div>
  );
};
