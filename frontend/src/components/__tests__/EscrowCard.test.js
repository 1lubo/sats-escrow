import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import EscrowCard from '../EscrowCard.svelte';

// Mock the escrow store to prevent real API calls
vi.mock('../../stores/escrow', () => ({
  escrow: {
    subscribe: vi.fn((cb) => {
      cb({ escrows: [], loading: false, error: null, currentEscrow: null });
      return () => {};
    }),
    updateStatus: vi.fn().mockResolvedValue(undefined),
  },
}));

const makeEscrow = (overrides = {}) => ({
  id: '550e8400-e29b-41d4-a716-446655440000',
  state: 'created',
  amount_sats: 100000,
  buyer: 'aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee',
  seller: '11111111-2222-3333-4444-555555555555',
  created_at: '2025-01-15T12:00:00Z',
  ...overrides,
});

describe('EscrowCard', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders escrow details', () => {
    render(EscrowCard, { props: { escrowItem: makeEscrow() } });

    expect(screen.getByText(/550e8400/)).toBeTruthy();
    expect(screen.getByText(/100000 sats/)).toBeTruthy();
    expect(screen.getByText('created')).toBeTruthy();
  });

  it('shows Fund button when created', () => {
    render(EscrowCard, { props: { escrowItem: makeEscrow({ state: 'created' }) } });

    expect(screen.getByText('Fund')).toBeTruthy();
  });

  it('shows Mark Delivered when funded', () => {
    render(EscrowCard, { props: { escrowItem: makeEscrow({ state: 'funded' }) } });

    expect(screen.getByText('Mark Delivered')).toBeTruthy();
  });

  it('shows Confirm & Release when awaiting_delivery', () => {
    render(EscrowCard, {
      props: { escrowItem: makeEscrow({ state: 'awaiting_delivery' }) },
    });

    expect(screen.getByText('Confirm & Release')).toBeTruthy();
  });

  it('shows no action buttons for terminal states', () => {
    render(EscrowCard, {
      props: { escrowItem: makeEscrow({ state: 'released_to_seller' }) },
    });

    expect(screen.queryByText('Fund')).toBeNull();
    expect(screen.queryByText('Mark Delivered')).toBeNull();
    expect(screen.queryByText('Confirm & Release')).toBeNull();
    expect(screen.queryByText('Cancel')).toBeNull();
    expect(screen.queryByText('Open Dispute')).toBeNull();
  });
});
