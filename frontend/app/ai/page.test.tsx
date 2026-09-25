import { describe, expect, it, vi } from 'vitest';
import { render, screen } from '@testing-library/react';
import { AiPage } from './page';

vi.mock('@/hooks/useFeatureFlag', () => ({
  useFeatureFlag: vi.fn(),
}));

vi.mock('next/navigation', () => ({
  notFound: vi.fn(),
}));

describe('AiPage', () => {
  it('renders ChatShell when ai_agent is enabled', () => {
    const { useFeatureFlag } = require('@/hooks/useFeatureFlag');
    useFeatureFlag.mockReturnValue({ enabled: true, loading: false });
    render(<AiPage />);
    expect(screen.getByText(/convert, send, receive/i)).toBeInTheDocument();
  });

  it('calls notFound when ai_agent is disabled', () => {
    const { useFeatureFlag } = require('@/hooks/useFeatureFlag');
    const { notFound } = require('next/navigation');
    useFeatureFlag.mockReturnValue({ enabled: false, loading: false });
    render(<AiPage />);
    expect(notFound).toHaveBeenCalled();
  });
});
