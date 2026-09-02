/*
File Name: DataLoaderSteps.tsx
Purpose: Step 1 (Thread) and Step 2 (Fabric) file upload controls for DataLoaderModal.
*/

import React from 'react';
import { Folder, Upload, CheckCircle } from 'lucide-react';

interface DataLoaderStepsProps {
  hasThread: boolean;
  hasFabric: boolean;
  threadFile: File | null;
  fabricFile: File | null;
  onThreadFileChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
  onFabricFileChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
  onApplyThread: () => void;
  onApplyFabric: () => void;
}

export const DataLoaderSteps = ({
  hasThread,
  hasFabric,
  threadFile,
  fabricFile,
  onThreadFileChange,
  onFabricFileChange,
  onApplyThread,
  onApplyFabric
}: DataLoaderStepsProps) => {
  return (
    <div className="space-y-6">
      {/* STEP 1: Thread AST (Required) */}
      <div className="space-y-3 p-4 rounded-2xl border bg-black/5 dark:bg-white/5 border-black/10 dark:border-white/10">
        <div className="flex items-center justify-between">
          <span className="text-xs font-bold uppercase tracking-wider text-[#EC5B38]">
            Step 1: Select thread_ast.json (Required)
          </span>
          {hasThread && (
            <span className="inline-flex items-center gap-1 text-[10px] font-mono font-bold text-emerald-500 bg-emerald-500/10 px-2 py-0.5 rounded">
              <CheckCircle className="w-3 h-3" /> Active
            </span>
          )}
        </div>

        <div className="flex items-center gap-2">
          <input
            type="file"
            accept=".json"
            onChange={onThreadFileChange}
            className="hidden"
            id="modal-thread-file-input"
          />
          <label
            htmlFor="modal-thread-file-input"
            className="flex-1 p-3 rounded-xl border border-dashed flex items-center justify-center gap-2 cursor-pointer hover:border-[#EC5B38] bg-black/5 dark:bg-white/5 border-black/20 dark:border-white/20 transition-all text-xs font-mono"
          >
            <Upload className="w-4 h-4 text-[#EC5B38]" />
            <span className="truncate">
              {threadFile ? threadFile.name : 'Choose thread_ast.json...'}
            </span>
          </label>

          <button
            onClick={onApplyThread}
            className="px-4 py-3 rounded-xl font-bold text-xs text-white transition-all flex-shrink-0 bg-[#EC5B38] hover:bg-[#EC5B38]/90 shadow-md cursor-pointer"
          >
            Apply Thread
          </button>
        </div>
      </div>

      {/* STEP 2: Fabric AST (Optional) */}
      <div className="space-y-3 p-4 rounded-2xl border bg-black/5 dark:bg-white/5 border-black/10 dark:border-white/10">
        <div className="flex items-center justify-between">
          <span className="text-xs font-bold uppercase tracking-wider text-blue-500">
            Step 2: Select fabric_ast.json (Optional)
          </span>
          {hasFabric && (
            <span className="inline-flex items-center gap-1 text-[10px] font-mono font-bold text-emerald-500 bg-emerald-500/10 px-2 py-0.5 rounded">
              <CheckCircle className="w-3 h-3" /> Active
            </span>
          )}
        </div>

        <div className="flex items-center gap-2">
          <input
            type="file"
            accept=".json"
            onChange={onFabricFileChange}
            className="hidden"
            id="modal-fabric-file-input"
          />
          <label
            htmlFor="modal-fabric-file-input"
            className="flex-1 p-3 rounded-xl border border-dashed flex items-center justify-center gap-2 cursor-pointer hover:border-blue-500 bg-black/5 dark:bg-white/5 border-black/20 dark:border-white/20 transition-all text-xs font-mono"
          >
            <Folder className="w-4 h-4 text-blue-500" />
            <span className="truncate">
              {fabricFile ? fabricFile.name : 'Choose fabric_ast.json...'}
            </span>
          </label>

          <button
            onClick={onApplyFabric}
            className="px-4 py-3 rounded-xl font-bold text-xs text-white transition-all flex-shrink-0 bg-blue-500 hover:bg-blue-600 shadow-md cursor-pointer"
          >
            Apply Fabric
          </button>
        </div>
      </div>
    </div>
  );
};
