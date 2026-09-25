'use client';

import { useState } from 'react';
import { parseIntent, type Intent } from '@/lib/ai/parse';
import { IntentPreviewCard } from './IntentPreviewCard';

export function ChatShell() {
  const [messages, setMessages] = useState<string[]>([]);
  const [composerText, setComposerText] = useState('');
  const [currentIntent, setCurrentIntent] = useState<(Intent & { confirmed: boolean }) | null>(null);

  function handleSend() {
    const text = composerText.trim();
    if (!text) return;
    setMessages((prev) => [...prev, text]);
    const result = parseIntent(text);
    if ('type' in result) {
      setCurrentIntent({ ...result, confirmed: false });
    } else {
      setCurrentIntent(null);
    }
    setComposerText('');
  }

  function handleConfirm() {
    setCurrentIntent((prev) => (prev ? { ...prev, confirmed: true } : null));
  }

  function handleCancel() {
    setCurrentIntent(null);
  }

  return (
    <div className="flex flex-col h-[calc(100vh-12rem)] border border-border rounded-xl">
      <div className="flex-1 overflow-y-auto p-6 space-y-4">
        {messages.length === 0 && (
          <p className="text-muted-foreground text-sm">
            The agent can convert, send, receive, bridge, cash out, and pay subscriptions after you confirm.
          </p>
        )}
        {messages.map((msg, i) => (
          <div key={i} className="p-3 rounded-lg bg-card">
            <p className="text-sm">{msg}</p>
          </div>
        ))}
      </div>
      {currentIntent && (
        <IntentPreviewCard
          intent={currentIntent}
          onConfirm={handleConfirm}
          onCancel={handleCancel}
        />
      )}
      <div className="border-t border-border p-4">
        <div className="flex gap-2">
          <input
            type="text"
            value={composerText}
            onChange={(e) => setComposerText(e.target.value)}
            placeholder="Type a command..."
            className="flex-1 rounded-md border border-input bg-background px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-ring"
            onKeyDown={(e) => e.key === 'Enter' && handleSend()}
          />
          <button
            onClick={handleSend}
            className="rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90"
          >
            Send
          </button>
        </div>
      </div>
    </div>
  );
}
