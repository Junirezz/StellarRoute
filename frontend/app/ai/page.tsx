'use client';

import { notFound } from 'next/navigation';
import { useFeatureFlag } from '@/hooks/useFeatureFlag';
import { ChatShell } from './ChatShell';

export default function AiPage() {
  const { enabled } = useFeatureFlag('ai_agent');

  if (!enabled) {
    notFound();
  }

  return (
    <main className="mx-auto w-full max-w-2xl py-8 px-4">
      <ChatShell />
    </main>
  );
}
