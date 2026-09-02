/*
File Name: TopBar.tsx
Purpose: Fixed top bar navigation with 44px WCAG mobile touch targets and Home View Mode Selector.
*/

import { useState, useRef, useEffect } from 'react';
import { Search, Sun, Moon, FolderUp } from 'lucide-react';
import { HomeViewMenu } from './topbar/HomeViewMenu';

export type HomeViewMode = 'static' | 'fabric' | 'thread';

interface TopBarProps {
  onGoHome: () => void;
  homeViewMode: HomeViewMode;
  onSelectHomeViewMode: (mode: HomeViewMode) => void;
  hasFabric: boolean;
  onOpenSearch: () => void;
  onOpenDataLoader: () => void;
  isDark: boolean;
  onToggleTheme: () => void;
}

export const TopBar = ({
  onGoHome,
  homeViewMode,
  onSelectHomeViewMode,
  hasFabric,
  onOpenSearch,
  onOpenDataLoader,
  isDark,
  onToggleTheme
}: TopBarProps) => {
  const [isHomeMenuOpen, setIsHomeMenuOpen] = useState(false);
  const homeMenuRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const handleClickOutside = (e: MouseEvent) => {
      if (homeMenuRef.current && !homeMenuRef.current.contains(e.target as Node)) {
        setIsHomeMenuOpen(false);
      }
    };

    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        setIsHomeMenuOpen(false);
      }
    };

    document.addEventListener('mousedown', handleClickOutside);
    document.addEventListener('keydown', handleKeyDown);

    return () => {
      document.removeEventListener('mousedown', handleClickOutside);
      document.removeEventListener('keydown', handleKeyDown);
    };
  }, []);

  return (
    <header className="fixed top-0 left-0 right-0 z-40 h-16 border-b transition-colors bg-[#FCF2E5] dark:bg-[#2C2C2C] text-[#2C2C2C] dark:text-[#FCF2E5] border-[#E4D5C5] dark:border-[#3D3D3D] shadow-sm">
      <div className="max-w-7xl mx-auto h-full px-3 sm:px-6 flex items-center justify-between gap-2 sm:gap-4">
        {/* Left: Home Button & View Mode Selector */}
        <HomeViewMenu
          homeMenuRef={homeMenuRef}
          isHomeMenuOpen={isHomeMenuOpen}
          setIsHomeMenuOpen={setIsHomeMenuOpen}
          homeViewMode={homeViewMode}
          hasFabric={hasFabric}
          onGoHome={onGoHome}
          onSelectHomeViewMode={onSelectHomeViewMode}
        />

        {/* Center: Search Trigger */}
        <div className="flex-1 max-w-xs sm:max-w-md md:max-w-xl">
          <button
            onClick={onOpenSearch}
            className="w-full min-h-[44px] sm:min-h-[40px] px-3 sm:px-4 rounded-xl border flex items-center justify-between text-xs sm:text-sm transition-all bg-white/50 dark:bg-black/30 border-[#E4D5C5] dark:border-[#3D3D3D] hover:border-[#EC5B38] dark:hover:border-[#EC5B38] text-current/70 hover:text-current group"
            aria-label="Search Entities and Members"
          >
            <div className="flex items-center gap-2 truncate">
              <Search className="w-4 h-4 text-[#EC5B38] flex-shrink-0" />
              <span className="truncate">Search Entities & Members...</span>
            </div>
            <kbd className="hidden md:inline-flex items-center gap-1 text-xs font-mono px-2 py-0.5 rounded bg-black/10 dark:bg-white/10 border border-black/10 dark:border-white/10 font-semibold opacity-80">
              Ctrl + K
            </kbd>
          </button>
        </div>

        {/* Right: AST Directory Loader & Theme Toggle */}
        <div className="flex items-center gap-1.5 sm:gap-2 flex-shrink-0">
          <button
            onClick={onOpenDataLoader}
            className="min-w-[44px] min-h-[44px] sm:min-w-[40px] sm:min-h-[40px] p-2.5 rounded-xl border flex items-center justify-center transition-all bg-white/50 dark:bg-black/30 border-[#E4D5C5] dark:border-[#3D3D3D] hover:border-[#EC5B38] dark:hover:border-[#EC5B38] text-[#EC5B38]"
            title="Load AST Specification Directory"
            aria-label="Load AST Specification Directory"
          >
            <FolderUp className="w-5 h-5" />
          </button>

          <button
            onClick={onToggleTheme}
            className="min-w-[44px] min-h-[44px] sm:min-w-[40px] sm:min-h-[40px] p-2.5 rounded-xl border flex items-center justify-center transition-all bg-white/50 dark:bg-black/30 border-[#E4D5C5] dark:border-[#3D3D3D] hover:border-[#EC5B38] dark:hover:border-[#EC5B38] text-[#EC5B38]"
            title={isDark ? 'Switch to Light Mode' : 'Switch to Dark Mode'}
            aria-label={isDark ? 'Switch to Light Mode' : 'Switch to Dark Mode'}
          >
            {isDark ? <Sun className="w-5 h-5" /> : <Moon className="w-5 h-5" />}
          </button>
        </div>
      </div>
    </header>
  );
};
