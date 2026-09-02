/*
File Name: BottomBar.tsx
Purpose: Fixed bottom bar displaying breadcrumb location and history navigation with WCAG touch target sizes.
*/

import { ArrowLeft, ArrowRight, Compass } from 'lucide-react';

interface BottomBarProps {
  canGoBack: boolean;
  canGoForward: boolean;
  onGoBack: () => void;
  onGoForward: () => void;
  currentLocationText: string;
}

export const BottomBar = ({
  canGoBack,
  canGoForward,
  onGoBack,
  onGoForward,
  currentLocationText
}: BottomBarProps) => {
  return (
    <footer className="fixed bottom-0 left-0 right-0 z-30 h-12 border-t transition-colors bg-[#FCF2E5] dark:bg-[#2C2C2C] text-[#2C2C2C] dark:text-[#FCF2E5] border-[#E4D5C5] dark:border-[#3D3D3D] shadow-inner text-xs">
      <div className="max-w-7xl mx-auto h-full px-3 sm:px-6 flex items-center justify-between gap-4">
        {/* Left: Navigation Buttons with min 44px touch targets */}
        <div className="flex items-center gap-1.5 sm:gap-2">
          <button
            onClick={onGoBack}
            disabled={!canGoBack}
            className={`min-w-[44px] min-h-[44px] sm:min-w-[36px] sm:min-h-[36px] px-2.5 py-1 rounded-xl flex items-center justify-center gap-1 border transition-all font-bold ${
              canGoBack
                ? 'bg-white/50 dark:bg-black/30 border-[#E4D5C5] dark:border-[#3D3D3D] hover:border-[#EC5B38] text-[#EC5B38]'
                : 'opacity-40 border-transparent cursor-not-allowed text-current'
            }`}
            title="Navigate Back"
            aria-label="Navigate Back"
          >
            <ArrowLeft className="w-4 h-4 flex-shrink-0" />
            <span className="hidden sm:inline">Back</span>
          </button>

          <button
            onClick={onGoForward}
            disabled={!canGoForward}
            className={`min-w-[44px] min-h-[44px] sm:min-w-[36px] sm:min-h-[36px] px-2.5 py-1 rounded-xl flex items-center justify-center gap-1 border transition-all font-bold ${
              canGoForward
                ? 'bg-white/50 dark:bg-black/30 border-[#E4D5C5] dark:border-[#3D3D3D] hover:border-[#EC5B38] text-[#EC5B38]'
                : 'opacity-40 border-transparent cursor-not-allowed text-current'
            }`}
            title="Navigate Forward"
            aria-label="Navigate Forward"
          >
            <span className="hidden sm:inline">Forward</span>
            <ArrowRight className="w-4 h-4 flex-shrink-0" />
          </button>
        </div>

        {/* Right: Breadcrumb Location Indicator */}
        <div className="flex items-center gap-2 truncate opacity-80 font-mono text-[11px] sm:text-xs">
          <Compass className="w-4 h-4 text-[#EC5B38] flex-shrink-0" />
          <span className="truncate">{currentLocationText}</span>
        </div>
      </div>
    </footer>
  );
};
