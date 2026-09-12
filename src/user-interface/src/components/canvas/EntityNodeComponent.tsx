/*
File Name: EntityNodeComponent.tsx
Purpose: Custom React Flow Node component rendering Entity and Member cards with double-click navigation.
*/

import { Handle, Position } from '@xyflow/react';
import type { EntityKind } from '../../types/wiki';
import { FileText, Layers, Database, Radio } from 'lucide-react';

export interface CustomNodeData {
  label: string;
  kind: EntityKind;
  onSelectEntity: (name: string) => void;
  onSelectMember?: (entityName: string, memberName: string) => void;
  groupName?: string;
}

export const EntityNodeComponent = ({ data }: { data: CustomNodeData }) => {
  const getIcon = () => {
    switch (data.kind) {
      case 'Feature':
        return <FileText className="w-5 h-5 text-[#EC5B38]" />;
      case 'Component':
        return <Layers className="w-5 h-5 text-blue-500" />;
      case 'Storage':
        return <Database className="w-5 h-5 text-emerald-500" />;
      case 'Protocol':
        return <Radio className="w-5 h-5 text-amber-500" />;
    }
  };

  const getBorderColor = () => {
    switch (data.kind) {
      case 'Feature':
        return 'border-[#EC5B38]/60 hover:border-[#EC5B38]';
      case 'Component':
        return 'border-blue-500/60 hover:border-blue-500';
      case 'Storage':
        return 'border-emerald-500/60 hover:border-emerald-500';
      case 'Protocol':
        return 'border-amber-500/60 hover:border-amber-500';
    }
  };

  const handleDoubleClick = () => {
    if (data.label.includes('::')) {
      const [eName, mName] = data.label.split('::');
      if (data.onSelectMember) {
        data.onSelectMember(eName, mName);
      } else {
        data.onSelectEntity(eName);
      }
    } else {
      data.onSelectEntity(data.label);
    }
  };

  return (
    <div
      onDoubleClick={handleDoubleClick}
      className={`px-4 py-3 rounded-2xl border-2 shadow-xl transition-all transform hover:scale-105 cursor-grab active:cursor-grabbing bg-[#FCF2E5] dark:bg-[#222222] text-[#2C2C2C] dark:text-[#FCF2E5] ${getBorderColor()} min-w-[220px] focus-visible:ring-2 focus-visible:ring-[#EC5B38]`}
    >
      <Handle type="target" position={Position.Top} className="!bg-[#EC5B38] !w-3 !h-3" />
      <div className="flex items-center gap-3">
        <div className="p-2.5 rounded-xl bg-black/5 dark:bg-white/5 border border-black/10 dark:border-white/10">
          {getIcon()}
        </div>
        <div className="min-w-0">
          <span className="text-[10px] font-extrabold uppercase tracking-wider text-[#EC5B38]">
            {data.kind}
          </span>
          <h4 className="text-sm font-extrabold truncate">{data.label}</h4>
        </div>
      </div>
      <Handle type="source" position={Position.Bottom} className="!bg-[#EC5B38] !w-3 !h-3" />
    </div>
  );
};
