/*
File Name: App.tsx
Purpose: Main Single Page Application container managing navigation history, theme mode, global search, data loader dialog, landing page file validation, and layout views.
*/

import React, { useState, useEffect } from 'react';
import type { WikiData, ThreadDocumentDto, FabricDocumentDto } from './types/wiki';
import { TopBar, type HomeViewMode } from './components/TopBar';
import { BottomBar } from './components/BottomBar';
import { SearchModal } from './components/SearchModal';
import { DataLoaderModal } from './components/DataLoaderModal';
import { Canvas } from './components/Canvas';
import { Graph } from './components/Graph';
import { Sections } from './components/Sections';
import { EntityDetailView } from './components/EntityDetailView';
import { FolderUp, Upload, AlertTriangle, FileText } from 'lucide-react';

interface NavState {
  view: 'home' | 'entity';
  entityName?: string;
  memberName?: string;
  scrollY?: number;
}

declare global {
  interface Window {
    wikiData?: WikiData;
  }
}

export function App() {
  const [wikiData, setWikiData] = useState<WikiData | null>(() => {
    return window.wikiData || null;
  });

  const [currentSource, setCurrentSource] = useState<string>(
    window.wikiData ? 'Embedded window.wikiData' : 'Not Loaded'
  );
  const [homeViewMode, setHomeViewMode] = useState<HomeViewMode>('static');

  // Landing Page Error Status
  const [landingError, setLandingError] = useState<string | null>(null);

  // Dark/Light Theme State
  const [isDark, setIsDark] = useState<boolean>(() => {
    const saved = localStorage.getItem('loom_theme');
    return saved ? saved === 'dark' : true;
  });

  useEffect(() => {
    if (isDark) {
      document.documentElement.classList.add('dark');
      localStorage.setItem('loom_theme', 'dark');
    } else {
      document.documentElement.classList.remove('dark');
      localStorage.setItem('loom_theme', 'light');
    }
  }, [isDark]);

  // Global Ctrl+K / Cmd+K Search Shortcut Listener
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'k') {
        e.preventDefault();
        setIsSearchOpen((prev) => !prev);
      }
    };
    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, []);

  // Navigation History Stack State
  const [historyStack, setHistoryStack] = useState<NavState[]>([{ view: 'home', scrollY: 0 }]);
  const [historyIndex, setHistoryIndex] = useState<number>(0);

  // Modals State
  const [isSearchOpen, setIsSearchOpen] = useState<boolean>(false);
  const [isDataLoaderOpen, setIsDataLoaderOpen] = useState<boolean>(false);

  const currentNav = historyStack[historyIndex] || { view: 'home', scrollY: 0 };

  // Restore scroll position when historyIndex changes
  useEffect(() => {
    const targetScrollY = historyStack[historyIndex]?.scrollY || 0;
    const timer = setTimeout(() => {
      window.scrollTo({ top: targetScrollY, behavior: 'instant' as ScrollBehavior });
    }, 30);
    return () => clearTimeout(timer);
  }, [historyIndex, historyStack]);

  const navigateTo = (state: NavState) => {
    const updatedStack = historyStack.slice(0, historyIndex + 1);
    if (updatedStack[historyIndex]) {
      updatedStack[historyIndex] = {
        ...updatedStack[historyIndex],
        scrollY: window.scrollY
      };
    }
    const finalStack = [...updatedStack, { ...state, scrollY: state.scrollY ?? 0 }];
    setHistoryStack(finalStack);
    setHistoryIndex(finalStack.length - 1);
  };

  const handleGoBack = () => {
    if (historyIndex > 0) {
      const updatedStack = [...historyStack];
      updatedStack[historyIndex] = {
        ...updatedStack[historyIndex],
        scrollY: window.scrollY
      };
      setHistoryStack(updatedStack);
      setHistoryIndex(historyIndex - 1);
    }
  };

  const handleGoForward = () => {
    if (historyIndex < historyStack.length - 1) {
      const updatedStack = [...historyStack];
      updatedStack[historyIndex] = {
        ...updatedStack[historyIndex],
        scrollY: window.scrollY
      };
      setHistoryStack(updatedStack);
      setHistoryIndex(historyIndex + 1);
    }
  };

  const handleGoHome = () => {
    navigateTo({ view: 'home', scrollY: 0 });
  };

  const handleSelectResult = (_type: 'entity' | 'member', entityName: string, memberName?: string) => {
    navigateTo({
      view: 'entity',
      entityName,
      memberName,
      scrollY: 0
    });
  };

  // Landing Page Direct File Upload Handler
  const handleLandingThreadUpload = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;

    setLandingError(null);
    const reader = new FileReader();
    reader.onload = (event) => {
      try {
        const text = event.target?.result as string;
        const parsed = JSON.parse(text);

        // Validation check for Loom Thread AST
        if (
          !parsed ||
          typeof parsed !== 'object' ||
          !Array.isArray(parsed.features) ||
          !Array.isArray(parsed.components) ||
          !Array.isArray(parsed.storages) ||
          !Array.isArray(parsed.protocols)
        ) {
          throw new Error('File does not contain valid Loom Thread AST structure');
        }

        const thread: ThreadDocumentDto = parsed;
        setWikiData((prev) => ({
          thread,
          fabric: prev?.fabric
        }));
        setCurrentSource(`Uploaded ${file.name}`);
        setHomeViewMode('static');
      } catch (err: any) {
        setLandingError(`Invalid Specification AST: ${err?.message || 'Invalid JSON format'}`);
      }
    };
    reader.readAsText(file);
  };

  // Step 1: Load Thread AST via Modal
  const handleLoadThread = (thread: ThreadDocumentDto) => {
    setWikiData((prev) => ({
      thread,
      fabric: prev?.fabric
    }));
    setCurrentSource('Loaded thread_ast.json');
    setHomeViewMode('static');
  };

  // Step 2: Load Fabric AST via Modal
  const handleLoadFabric = (fabric: FabricDocumentDto) => {
    setWikiData((prev) => {
      if (!prev) return null;
      return {
        thread: prev.thread,
        fabric
      };
    });
    setCurrentSource('Loaded fabric_ast.json');
    setHomeViewMode('fabric');
  };

  const hasFabric = Boolean(wikiData?.fabric && wikiData.fabric.connections.length > 0);

  const currentLocationText = !wikiData
    ? 'Empty'
    : currentNav.view === 'home'
    ? homeViewMode === 'fabric' && hasFabric
      ? 'Home / Canvas'
      : homeViewMode === 'thread'
      ? 'Home / Graph'
      : 'Home / Sections'
    : `${currentNav.entityName}${currentNav.memberName ? ` :: ${currentNav.memberName}` : ''}`;

  const isFullCanvasView = Boolean(
    wikiData && currentNav.view === 'home' && (homeViewMode === 'fabric' || homeViewMode === 'thread')
  );

  return (
    <div className="min-h-screen transition-colors duration-200 bg-[#FCF2E5] dark:bg-[#2C2C2C] text-[#2C2C2C] dark:text-[#FCF2E5] font-['Nunito',sans-serif] antialiased selection:bg-[#EC5B38] selection:text-white">
      {/* Top Fixed Bar */}
      <TopBar
        onGoHome={handleGoHome}
        homeViewMode={homeViewMode}
        onSelectHomeViewMode={setHomeViewMode}
        hasFabric={hasFabric}
        onOpenSearch={() => setIsSearchOpen(true)}
        onOpenDataLoader={() => setIsDataLoaderOpen(true)}
        isDark={isDark}
        onToggleTheme={() => setIsDark(!isDark)}
      />

      {/* Main Viewport */}
      <main
        className={
          isFullCanvasView
            ? 'pt-16 pb-12 w-full h-[calc(100vh-3rem)] overflow-hidden'
            : 'pt-20 pb-16 px-3 sm:px-6 max-w-7xl mx-auto min-h-[calc(100vh-7rem)]'
        }
      >
        {!wikiData ? (
          /* Landing Page Screen with File Picker & AST Error Validation Box */
          <div className="flex flex-col items-center justify-center py-16 text-center space-y-6 max-w-xl mx-auto animate-fade-in">
            <div className="p-6 rounded-3xl bg-[#EC5B38]/10 text-[#EC5B38] shadow-inner">
              <FolderUp className="w-16 h-16" />
            </div>

            <div className="space-y-2">
              <h2 className="text-3xl font-extrabold tracking-tight">Upload Specification AST</h2>
              <p className="text-sm opacity-70">
                Select your local <code className="font-mono text-[#EC5B38]">thread_ast.json</code> file from your desktop to render your interactive specification wiki.
              </p>
            </div>

            {/* Error Validation Box */}
            {landingError && (
              <div className="w-full p-4 rounded-2xl border bg-red-500/10 border-red-500/30 text-red-500 flex items-center gap-3 text-xs font-bold transition-all text-left animate-fade-in">
                <AlertTriangle className="w-5 h-5 flex-shrink-0 text-red-500" />
                <span className="flex-1">{landingError}</span>
              </div>
            )}

            {/* File Selection Zone */}
            <div className="w-full space-y-3">
              <input
                type="file"
                accept=".json"
                onChange={handleLandingThreadUpload}
                className="hidden"
                id="landing-thread-file-input"
              />
              <label
                htmlFor="landing-thread-file-input"
                className="w-full p-6 rounded-3xl border-2 border-dashed flex flex-col items-center justify-center gap-3 cursor-pointer hover:border-[#EC5B38] bg-black/5 dark:bg-white/5 border-black/20 dark:border-white/20 transition-all group"
              >
                <div className="p-3 rounded-2xl bg-[#EC5B38]/10 text-[#EC5B38] group-hover:scale-110 transition-transform">
                  <Upload className="w-8 h-8" />
                </div>
                <div>
                  <span className="font-bold text-sm text-[#EC5B38] block">Click or Drag & Drop thread_ast.json</span>
                  <span className="text-xs opacity-60">Accepts valid Loom Specification AST (.json)</span>
                </div>
              </label>

              <button
                onClick={() => setIsDataLoaderOpen(true)}
                className="w-full py-3 rounded-2xl font-bold text-xs text-current border border-black/10 dark:border-white/10 hover:bg-black/5 dark:hover:bg-white/5 transition-colors flex items-center justify-center gap-2"
              >
                <FileText className="w-4 h-4 text-[#EC5B38]" />
                <span>Open Specification AST Dialog</span>
              </button>
            </div>
          </div>
        ) : currentNav.view === 'home' ? (
          homeViewMode === 'fabric' && hasFabric && wikiData.fabric ? (
            <div className="w-full h-full">
              <Canvas
                fabric={wikiData.fabric}
                onSelectEntity={(entityName) => navigateTo({ view: 'entity', entityName })}
                onSelectMember={(entityName, memberName) => navigateTo({ view: 'entity', entityName, memberName })}
                isDark={isDark}
              />
            </div>
          ) : homeViewMode === 'thread' ? (
            <div className="w-full h-full">
              <Graph
                thread={wikiData.thread}
                onSelectEntity={(entityName) => navigateTo({ view: 'entity', entityName })}
                onSelectMember={(entityName, memberName) => navigateTo({ view: 'entity', entityName, memberName })}
                isDark={isDark}
              />
            </div>
          ) : (
            <Sections
              thread={wikiData.thread}
              onSelectEntity={(entityName) => navigateTo({ view: 'entity', entityName })}
              onSelectMember={(entityName, memberName) => navigateTo({ view: 'entity', entityName, memberName })}
            />
          )
        ) : (
          currentNav.entityName && (
            <EntityDetailView
              entityName={currentNav.entityName}
              targetMemberName={currentNav.memberName}
              wikiData={wikiData}
              onNavigateToEntity={(entityName) => navigateTo({ view: 'entity', entityName })}
              onNavigateToMember={(entityName, memberName) => navigateTo({ view: 'entity', entityName, memberName })}
              onGoBack={handleGoBack}
              isDark={isDark}
            />
          )
        )}
      </main>

      {/* Bottom Fixed Bar */}
      <BottomBar
        canGoBack={historyIndex > 0}
        canGoForward={historyIndex < historyStack.length - 1}
        onGoBack={handleGoBack}
        onGoForward={handleGoForward}
        currentLocationText={currentLocationText}
      />

      {/* Global Search Modal */}
      {wikiData && (
        <SearchModal
          isOpen={isSearchOpen}
          onClose={() => setIsSearchOpen(false)}
          wikiData={wikiData}
          onSelectResult={handleSelectResult}
        />
      )}

      {/* Data AST Loader Dialog */}
      <DataLoaderModal
        isOpen={isDataLoaderOpen}
        onClose={() => setIsDataLoaderOpen(false)}
        onLoadThread={handleLoadThread}
        onLoadFabric={handleLoadFabric}
        currentSource={currentSource}
        hasThread={Boolean(wikiData?.thread)}
        hasFabric={hasFabric}
      />
    </div>
  );
}

export default App;
