import { describe, expect, it, vi } from 'vitest';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { IntentPreviewCard } from './IntentPreviewCard';
import type { Intent } from '@/lib/ai/parse';

const mockIntent: Intent & { confirmed: boolean } = {
  type: 'swap',
  amount: '10',
  asset: 'XLM',
  destination: 'USDC',
  confirmed: false,
};

describe('IntentPreviewCard', () => {
  it('renders the intent description', () => {
    render(
      <IntentPreviewCard intent={mockIntent} onConfirm={() => {}} onCancel={() => {}} />
    );
    expect(screen.getByTestId('intent-description')).toHaveTextContent('Swap 10 XLM to USDC');
  });

  it('calls onConfirm when confirm is clicked', async () => {
    const user = userEvent.setup();
    const onConfirm = vi.fn();
    render(
      <IntentPreviewCard intent={mockIntent} onConfirm={onConfirm} onCancel={() => {}} />
    );
    await user.click(screen.getByTestId('confirm-btn'));
    expect(onConfirm).toHaveBeenCalled();
  });

  it('calls onCancel when cancel is clicked', async () => {
    const user = userEvent.setup();
    const onCancel = vi.fn();
    render(
      <IntentPreviewCard intent={mockIntent} onConfirm={() => {}} onCancel={onCancel} />
    );
    await user.click(screen.getByTestId('cancel-btn'));
    expect(onCancel).toHaveBeenCalled();
  });

  it('shows confirmed status when confirmed is true', () => {
    render(
      <IntentPreviewCard intent={{ ...mockIntent, confirmed: true }} onConfirm={() => {}} onCancel={() => {}} />
    );
    expect(screen.getByTestId('confirmed-status')).toBeInTheDocument();
  });
});
