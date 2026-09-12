/*
File Name: MemberBadge.tsx
Purpose: Visible badge component for rendering member kinds (Scenario, Contract, Model, Table, Channel).
*/

import type { MemberKind } from '../types/wiki';

interface MemberBadgeProps {
  kind: MemberKind | string;
  className?: string;
}

export const MemberBadge = ({ kind, className = '' }: MemberBadgeProps) => {
  const getBadgeStyle = () => {
    switch (kind) {
      case 'Scenario':
      case 'Scenario Outline':
        return 'bg-[#EC5B38]/15 text-[#EC5B38] border-[#EC5B38]/40 dark:bg-[#EC5B38]/25 dark:text-[#EC5B38] dark:border-[#EC5B38]/60';
      case 'Contract':
      case 'Model':
        // Component sub-members use unified Blue theme
        return 'bg-blue-500/15 text-blue-600 border-blue-500/40 dark:bg-blue-500/25 dark:text-blue-400 dark:border-blue-500/60';
      case 'Table':
        return 'bg-emerald-500/15 text-emerald-600 border-emerald-500/40 dark:bg-emerald-500/25 dark:text-emerald-400 dark:border-emerald-500/60';
      case 'Channel':
        return 'bg-amber-500/15 text-amber-600 border-amber-500/40 dark:bg-amber-500/25 dark:text-amber-400 dark:border-amber-500/60';
      default:
        return 'bg-teal-500/15 text-teal-600 border-teal-500/40 dark:bg-teal-500/25 dark:text-teal-400 dark:border-teal-500/60';
    }
  };

  return (
    <span className={`inline-flex items-center px-2.5 py-1 rounded-md text-xs sm:text-sm font-bold border shadow-xs transition-colors ${getBadgeStyle()} ${className}`}>
      [{kind}]
    </span>
  );
};
