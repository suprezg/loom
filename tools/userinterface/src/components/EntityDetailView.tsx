/*
File Name: EntityDetailView.tsx
Purpose: Orchestrator Markdown Wiki documentation detail view composing feature, component, storage, protocol, TOC, and lightbox fragments.
*/

import { useEffect, useState, useMemo } from 'react';
import type { WikiData } from '../types/wiki';
import { TableOfContents } from './detail/TableOfContents';
import { DiagramLightboxModal } from './detail/DiagramLightboxModal';
import { FeatureDetailView, type ScenarioWithRule } from './detail/FeatureDetailView';
import { ComponentDetailView } from './detail/ComponentDetailView';
import { StorageDetailView } from './detail/StorageDetailView';
import { ProtocolDetailView } from './detail/ProtocolDetailView';
import { FileText, Layers, Database, Radio, ArrowLeft } from 'lucide-react';

interface EntityDetailViewProps {
  entityName: string;
  targetMemberName?: string;
  wikiData: WikiData;
  onNavigateToEntity: (entityName: string) => void;
  onNavigateToMember: (entityName: string, memberName: string) => void;
  onGoBack: () => void;
  isDark?: boolean;
}

export const EntityDetailView = ({
  entityName,
  targetMemberName,
  wikiData,
  onNavigateToEntity,
  onNavigateToMember,
  onGoBack,
  isDark = false
}: EntityDetailViewProps) => {
  const feature = wikiData.thread.features.find((f) => f.name === entityName);
  const component = wikiData.thread.components.find((c) => c.name === entityName);
  const storage = wikiData.thread.storages.find((s) => s.name === entityName);
  const protocol = wikiData.thread.protocols.find((p) => p.name === entityName);

  const [highlightedMember, setHighlightedMember] = useState<string | undefined>(targetMemberName);
  const [activeDiagramModal, setActiveDiagramModal] = useState<{ name: string; chart: string } | null>(null);

  // Flatten feature scenarios and attach ruleName if scenario belongs to a rule
  const allFeatureScenarios = useMemo<ScenarioWithRule[]>(() => {
    if (!feature) return [];
    const list: ScenarioWithRule[] = [];

    feature.rules.forEach((rule) => {
      rule.scenarios.forEach((sc) => {
        list.push({ ...sc, ruleName: rule.name });
      });
    });

    feature.scenarios.forEach((sc) => {
      if (!list.some((existing) => existing.name === sc.name)) {
        list.push(sc);
      }
    });

    return list;
  }, [feature]);

  useEffect(() => {
    setHighlightedMember(targetMemberName);
    if (targetMemberName) {
      const el = document.getElementById(`member-${targetMemberName}`);
      if (el) {
        setTimeout(() => {
          el.scrollIntoView({ behavior: 'smooth', block: 'start' });
        }, 100);
      }
      const timer = setTimeout(() => {
        setHighlightedMember(undefined);
      }, 3000);
      return () => clearTimeout(timer);
    }
  }, [targetMemberName, entityName]);

  // Lock body scroll and handle Escape key while Diagram Lightbox Modal is open
  useEffect(() => {
    if (activeDiagramModal) {
      document.body.style.overflow = 'hidden';
    } else {
      document.body.style.overflow = '';
    }

    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'Escape' && activeDiagramModal) {
        setActiveDiagramModal(null);
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => {
      document.body.style.overflow = '';
      window.removeEventListener('keydown', handleKeyDown);
    };
  }, [activeDiagramModal]);

  const handleDecoratorClick = (target: string) => {
    if (target.includes('::')) {
      const [eName, mName] = target.split('::');
      onNavigateToMember(eName, mName);
    } else {
      onNavigateToEntity(target);
    }
  };

  const scrollToSection = (id: string) => {
    const el = document.getElementById(id);
    if (el) {
      el.scrollIntoView({ behavior: 'smooth', block: 'start' });
    }
  };

  if (!feature && !component && !storage && !protocol) {
    return (
      <div className="py-16 text-center space-y-4 max-w-md mx-auto">
        <h2 className="text-2xl font-bold text-red-500">Entity Not Found</h2>
        <p className="opacity-70 text-sm">No entity named "{entityName}" exists in the ingested specification.</p>
        <button
          onClick={onGoBack}
          className="px-5 py-2.5 rounded-xl bg-[#EC5B38] text-[#FCF2E5] font-semibold text-sm shadow-md"
        >
          Return to Dashboard
        </button>
      </div>
    );
  }

  const modalLabelColorClass = feature
    ? 'text-[#EC5B38]'
    : component
    ? 'text-blue-500'
    : storage
    ? 'text-emerald-500'
    : protocol
    ? 'text-amber-500'
    : 'text-[#EC5B38]';

  return (
    <div className="max-w-7xl mx-auto flex gap-8 items-start relative px-2 sm:px-0">
      {/* Main Wiki Article */}
      <article className="flex-1 min-w-0 space-y-10 pb-20 animate-fade-in text-[#2C2C2C] dark:text-[#FCF2E5]">
        {/* Navigation Top Bar */}
        <nav className="flex items-center justify-between gap-4 pb-4 border-b border-[#E4D5C5] dark:border-[#3D3D3D]">
          <button
            onClick={onGoBack}
            className="inline-flex items-center gap-2 px-3 py-1.5 rounded-xl border text-xs font-bold transition-colors bg-white/50 dark:bg-black/30 border-[#E4D5C5] dark:border-[#3D3D3D] hover:border-[#EC5B38] text-[#EC5B38]"
          >
            <ArrowLeft className="w-4 h-4" />
            <span>Back</span>
          </button>

          <div className="flex items-center gap-2 text-xs font-bold uppercase tracking-wider opacity-60">
            {feature && <FileText className="w-4 h-4 text-[#EC5B38]" />}
            {component && <Layers className="w-4 h-4 text-blue-500" />}
            {storage && <Database className="w-4 h-4 text-emerald-500" />}
            {protocol && <Radio className="w-4 h-4 text-amber-500" />}
            <span>
              {feature && 'Feature Specification'}
              {component && 'Component Module'}
              {storage && 'Storage Schema'}
              {protocol && 'Protocol Architecture'}
            </span>
          </div>
        </nav>

        {/* H1 Title Header */}
        <header className="space-y-3 pb-6 border-b border-[#E4D5C5] dark:border-[#3D3D3D]">
          <h1 className="text-3xl sm:text-5xl font-extrabold tracking-tight">
            {entityName}
          </h1>
        </header>

        {/* Dynamic Detail Fragments */}
        {feature && (
          <FeatureDetailView
            feature={feature}
            allFeatureScenarios={allFeatureScenarios}
            highlightedMember={highlightedMember}
            onDecoratorClick={handleDecoratorClick}
            onOpenDiagramModal={(name, chart) => setActiveDiagramModal({ name, chart })}
            isDark={isDark}
          />
        )}

        {component && (
          <ComponentDetailView
            component={component}
            highlightedMember={highlightedMember}
            onDecoratorClick={handleDecoratorClick}
            onOpenDiagramModal={(name, chart) => setActiveDiagramModal({ name, chart })}
            isDark={isDark}
          />
        )}

        {storage && (
          <StorageDetailView
            storage={storage}
            highlightedMember={highlightedMember}
            onDecoratorClick={handleDecoratorClick}
            onOpenDiagramModal={(name, chart) => setActiveDiagramModal({ name, chart })}
            isDark={isDark}
          />
        )}

        {protocol && (
          <ProtocolDetailView
            protocol={protocol}
            highlightedMember={highlightedMember}
            onDecoratorClick={handleDecoratorClick}
            onOpenDiagramModal={(name, chart) => setActiveDiagramModal({ name, chart })}
            isDark={isDark}
          />
        )}
      </article>

      {/* Sticky Table of Contents */}
      <TableOfContents
        feature={feature}
        component={component}
        storage={storage}
        protocol={protocol}
        allFeatureScenarios={allFeatureScenarios}
        onScrollToSection={scrollToSection}
      />

      {/* Interactive Diagram Lightbox Modal */}
      <DiagramLightboxModal
        activeDiagramModal={activeDiagramModal}
        onClose={() => setActiveDiagramModal(null)}
        modalLabelColorClass={modalLabelColorClass}
        isDark={isDark}
      />
    </div>
  );
};
