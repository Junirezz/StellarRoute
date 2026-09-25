'use client';

import { Button } from '@/components/ui/button';
import { type Intent } from '@/lib/ai/parse';
import { CheckCircle2, X } from 'lucide-react';

interface IntentPreviewCardProps {
  intent: Intent & { confirmed: boolean };
  onConfirm: () => void;
  onCancel: () => void;
}

function intentDescription(intent: Intent): string {
  switch (intent.type) {
    case 'swap':
      return `Swap ${intent.amount} ${intent.asset} to ${intent.destination}`;
    case 'send':
      return `Send ${intent.amount} ${intent.asset} to ${intent.recipient}`;
    case 'receive':
      return 'Receive assets';
    case 'bridge':
      return intent.source
        ? `Bridge ${intent.amount} ${intent.asset} from ${intent.source}`
        : `Bridge ${intent.amount} ${intent.asset} to ${intent.destination}`;
    case 'cash_out':
      return `Cash out ${intent.amount} ${intent.asset} to ${intent.currency}`;
    case 'pay':
      return `Pay ${intent.amount} ${intent.asset} monthly to ${intent.recipient}`;
  }
}

export function IntentPreviewCard({ intent, onConfirm, onCancel }: IntentPreviewCardProps) {
  return (
    <div className="border-t border-border p-4 space-y-3" data-testid="intent-preview-card">
      <p className="text-sm font-medium" data-testid="intent-description">
        {intentDescription(intent)}
      </p>
      <div className="flex gap-2">
        <Button
          variant="default"
          size="sm"
          onClick={onConfirm}
          data-testid="confirm-btn"
        >
          <CheckCircle2 className="h-4 w-4 mr-1" />
          Confirm
        </Button>
        <Button
          variant="outline"
          size="sm"
          onClick={onCancel}
          data-testid="cancel-btn"
        >
          <X className="h-4 w-4 mr-1" />
          Cancel
        </Button>
      </div>
      {intent.confirmed && (
        <p className="text-xs text-muted-foreground" data-testid="confirmed-status">
          confirmed: true
        </p>
      )}
    </div>
  );
}
