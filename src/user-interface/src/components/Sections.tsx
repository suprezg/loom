/*
File Name: Sections.tsx
Purpose: Orchestrator dashboard displaying 4 vertical entity sections (Features, Components, Storage, Protocols).
*/

import type { ThreadDocumentDto } from '../types/wiki';
import { FeatureSection } from './sections/FeatureSection';
import { ComponentSection } from './sections/ComponentSection';
import { ProtocolSection } from './sections/ProtocolSection';
import { StorageSection } from './sections/StorageSection';

interface SectionsProps {
  thread: ThreadDocumentDto;
  onSelectEntity: (entityName: string) => void;
  onSelectMember: (entityName: string, memberName: string) => void;
}

export const Sections = ({
  thread,
  onSelectEntity,
  onSelectMember
}: SectionsProps) => {
  return (
    <div className="space-y-12 pb-12 animate-fade-in">
      {/* Hero Title */}
      <div className="text-center space-y-3 py-6 border-b border-[#E4D5C5] dark:border-[#3D3D3D]">
        <h1 className="text-4xl sm:text-5xl font-extrabold tracking-tight">
          Specification Architecture
        </h1>
        <p className="text-lg opacity-70 max-w-2xl mx-auto">
          Explore behavior features, component contracts, storage schemas, and communication protocols.
        </p>
      </div>

      {/* Feature Specifications */}
      <FeatureSection
        features={thread.features}
        onSelectEntity={onSelectEntity}
        onSelectMember={onSelectMember}
      />

      {/* Component Modules */}
      <ComponentSection
        components={thread.components}
        onSelectEntity={onSelectEntity}
        onSelectMember={onSelectMember}
      />

      {/* Protocol Architectures */}
      <ProtocolSection
        protocols={thread.protocols}
        onSelectEntity={onSelectEntity}
        onSelectMember={onSelectMember}
      />

      {/* Storage Schemas */}
      <StorageSection
        storages={thread.storages}
        onSelectEntity={onSelectEntity}
        onSelectMember={onSelectMember}
      />
    </div>
  );
};
