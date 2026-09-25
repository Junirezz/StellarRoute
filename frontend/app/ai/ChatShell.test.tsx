import { describe, expect, it } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { ChatShell } from './ChatShell';

describe('ChatShell', () => {
  it('shows empty copy when no messages', () => {
    render(<ChatShell />);
    expect(screen.getByText(/convert, send, receive/i)).toBeInTheDocument();
  });

  it('appends a message when composer sends text', async () => {
    const user = userEvent.setup();
    render(<ChatShell />);
    const input = screen.getByPlaceholderText('Type a command...');
    await user.type(input, 'swap 10 XLM to USDC');
    await user.keyboard('{Enter}');
    expect(screen.getByText('swap 10 XLM to USDC')).toBeInTheDocument();
  });

  it('shows intent preview card after parsing a valid intent', async () => {
    const user = userEvent.setup();
    render(<ChatShell />);
    const input = screen.getByPlaceholderText('Type a command...');
    await user.type(input, 'send 5 USDC to GABC');
    await user.keyboard('{Enter}');
    expect(screen.getByTestId('intent-preview-card')).toBeInTheDocument();
  });

  it('does not show intent preview card for clarification', async () => {
    const user = userEvent.setup();
    render(<ChatShell />);
    const input = screen.getByPlaceholderText('Type a command...');
    await user.type(input, 'do something weird');
    await user.keyboard('{Enter}');
    expect(screen.queryByTestId('intent-preview-card')).not.toBeInTheDocument();
  });

  it('shows confirm status after clicking confirm', async () => {
    const user = userEvent.setup();
    render(<ChatShell />);
    const input = screen.getByPlaceholderText('Type a command...');
    await user.type(input, 'swap 10 XLM to USDC');
    await user.keyboard('{Enter}');
    const confirmBtn = screen.getByTestId('confirm-btn');
    await user.click(confirmBtn);
    expect(screen.getByTestId('confirmed-status')).toBeInTheDocument();
  });

  it('clears the card when cancel is clicked', async () => {
    const user = userEvent.setup();
    render(<ChatShell />);
    const input = screen.getByPlaceholderText('Type a command...');
    await user.type(input, 'swap 10 XLM to USDC');
    await user.keyboard('{Enter}');
    const cancelBtn = screen.getByTestId('cancel-btn');
    await user.click(cancelBtn);
    expect(screen.queryByTestId('intent-preview-card')).not.toBeInTheDocument();
  });
});
