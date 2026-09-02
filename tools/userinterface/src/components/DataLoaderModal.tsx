/*
File Name: DataLoaderModal.tsx
Purpose: Step-by-step AST file loader dialog orchestrator.
*/

import React, { useState } from 'react';
import { X, AlertTriangle, CheckCircle, FolderUp } from 'lucide-react';
import type { ThreadDocumentDto, FabricDocumentDto } from '../types/wiki';
import { DataLoaderSteps } from './modals/DataLoaderSteps';

interface DataLoaderModalProps {
  isOpen: boolean;
  onClose: () => void;
  onLoadThread: (thread: ThreadDocumentDto) => void;
  onLoadFabric: (fabric: FabricDocumentDto) => void;
  currentSource: string;
  hasThread: boolean;
  hasFabric: boolean;
}

export const DataLoaderModal = ({
  isOpen,
  onClose,
  onLoadThread,
  onLoadFabric,
  currentSource,
  hasThread,
  hasFabric
}: DataLoaderModalProps) => {
  const [threadFile, setThreadFile] = useState<File | null>(null);
  const [fabricFile, setFabricFile] = useState<File | null>(null);
  const [statusMessage, setStatusMessage] = useState<{ type: 'success' | 'error'; text: string } | null>(null);

  if (!isOpen) return null;

  const handleThreadFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;
    setThreadFile(file);
    setStatusMessage(null);
  };

  const handleFabricFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;
    setFabricFile(file);
    setStatusMessage(null);
  };

  const handleApplyThread = () => {
    if (!threadFile) {
      setStatusMessage({
        type: 'error',
        text: 'Please choose or select a thread_ast.json file first.'
      });
      return;
    }

    const reader = new FileReader();
    reader.onload = (event) => {
      try {
        const text = event.target?.result as string;
        const parsed = JSON.parse(text);

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

        onLoadThread(parsed as ThreadDocumentDto);
        setStatusMessage({
          type: 'success',
          text: 'thread_ast.json applied successfully!'
        });
      } catch (err: any) {
        setStatusMessage({
          type: 'error',
          text: `Invalid Specification AST: ${err?.message || 'Invalid JSON format'}`
        });
      }
    };
    reader.readAsText(threadFile);
  };

  const handleApplyFabric = () => {
    if (!fabricFile) {
      setStatusMessage({
        type: 'error',
        text: 'Please choose or select a fabric_ast.json file first.'
      });
      return;
    }

    const reader = new FileReader();
    reader.onload = (event) => {
      try {
        const text = event.target?.result as string;
        const parsed = JSON.parse(text);

        if (!parsed || typeof parsed !== 'object' || !Array.isArray(parsed.connections)) {
          throw new Error('File does not contain valid Loom Fabric AST structure');
        }

        onLoadFabric(parsed as FabricDocumentDto);
        setStatusMessage({
          type: 'success',
          text: 'fabric_ast.json applied successfully!'
        });
      } catch (err: any) {
        setStatusMessage({
          type: 'error',
          text: `Invalid Fabric AST: ${err?.message || 'Invalid JSON format'}`
        });
      }
    };
    reader.readAsText(fabricFile);
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/70 backdrop-blur-sm animate-fade-in">
      <div
        className="w-full max-w-xl rounded-3xl border shadow-2xl p-6 sm:p-8 space-y-6 bg-[#FCF2E5] dark:bg-[#2C2C2C] text-[#2C2C2C] dark:text-[#FCF2E5] border-[#E4D5C5] dark:border-[#3D3D3D]"
        onClick={(e) => e.stopPropagation()}
      >
        {/* Modal Header */}
        <div className="flex items-center justify-between border-b border-[#E4D5C5] dark:border-[#3D3D3D] pb-4">
          <div className="flex items-center gap-3">
            <div className="p-3 rounded-2xl bg-[#EC5B38]/10 text-[#EC5B38]">
              <FolderUp className="w-6 h-6" />
            </div>
            <div>
              <h2 className="text-xl font-bold">Specification AST Loader</h2>
              <p className="text-xs opacity-70">Source: {currentSource}</p>
            </div>
          </div>
          <button
            onClick={onClose}
            className="p-2 rounded-xl hover:bg-black/10 dark:hover:bg-white/10 transition-colors min-w-[44px] min-h-[44px] flex items-center justify-center"
            title="Close Dialog"
            aria-label="Close Dialog"
          >
            <X className="w-5 h-5 opacity-70" />
          </button>
        </div>

        {/* Validation Error / Success Box */}
        {statusMessage && (
          <div
            className={`p-4 rounded-2xl border flex items-center gap-3 text-xs font-bold transition-all animate-fade-in ${
              statusMessage.type === 'error'
                ? 'bg-red-500/10 border-red-500/30 text-red-500'
                : 'bg-emerald-500/10 border-emerald-500/30 text-emerald-500'
            }`}
          >
            {statusMessage.type === 'error' ? (
              <AlertTriangle className="w-5 h-5 flex-shrink-0 text-red-500" />
            ) : (
              <CheckCircle className="w-5 h-5 flex-shrink-0 text-emerald-500" />
            )}
            <span className="flex-1">{statusMessage.text}</span>
          </div>
        )}

        {/* Sequential Step Inputs Component */}
        <DataLoaderSteps
          hasThread={hasThread}
          hasFabric={hasFabric}
          threadFile={threadFile}
          fabricFile={fabricFile}
          onThreadFileChange={handleThreadFileChange}
          onFabricFileChange={handleFabricFileChange}
          onApplyThread={handleApplyThread}
          onApplyFabric={handleApplyFabric}
        />

        {/* Dialog Actions */}
        <div className="flex items-center justify-end gap-3 pt-2 border-t border-[#E4D5C5] dark:border-[#3D3D3D]">
          <button
            onClick={onClose}
            className="px-5 py-2.5 rounded-xl border font-bold text-xs transition-colors border-black/10 dark:border-white/10 hover:bg-black/5 dark:hover:bg-white/5"
          >
            Close
          </button>
        </div>
      </div>
    </div>
  );
};
