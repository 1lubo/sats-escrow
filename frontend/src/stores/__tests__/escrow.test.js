import { describe, it, expect, beforeEach, vi } from 'vitest';
import { get } from 'svelte/store';

// Mock the API client before importing the store
vi.mock('../../api/client', () => ({
  escrowAPI: {
    list: vi.fn(),
    create: vi.fn(),
    fund: vi.fn(),
    deliver: vi.fn(),
    confirm: vi.fn(),
    cancel: vi.fn(),
    dispute: vi.fn(),
  },
}));

import { escrow } from '../escrow';
import { escrowAPI } from '../../api/client';

describe('escrow store', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    // Reset store state by triggering internal reset - re-import would be complex,
    // so we work with the store as-is between tests
  });

  it('has correct initial state', () => {
    const state = get(escrow);
    expect(state.escrows).toEqual([]);
    expect(state.loading).toBe(false);
    expect(state.error).toBeNull();
    expect(state.currentEscrow).toBeNull();
  });

  it('fetch populates escrows from API', async () => {
    const mockEscrows = [
      { id: 'esc-1', state: 'created', amount_sats: 100000 },
      { id: 'esc-2', state: 'funded', amount_sats: 50000 },
    ];
    escrowAPI.list.mockResolvedValue({ data: { data: mockEscrows } });

    await escrow.fetch();

    const state = get(escrow);
    expect(state.escrows).toEqual(mockEscrows);
    expect(state.loading).toBe(false);
    expect(state.error).toBeNull();
    expect(escrowAPI.list).toHaveBeenCalledOnce();
  });

  it('fetch sets error on failure', async () => {
    escrowAPI.list.mockRejectedValue(new Error('Network error'));

    await escrow.fetch();

    const state = get(escrow);
    expect(state.error).toBe('Network error');
    expect(state.loading).toBe(false);
  });

  it('create appends new escrow', async () => {
    const newEscrow = { id: 'esc-new', state: 'created', amount_sats: 75000 };
    escrowAPI.create.mockResolvedValue({ data: { data: newEscrow } });

    const result = await escrow.create({ amount_sats: 75000 });

    const state = get(escrow);
    expect(state.escrows).toContainEqual(newEscrow);
    expect(result).toEqual(newEscrow);
    expect(state.loading).toBe(false);
  });

  it('create sets error on failure', async () => {
    escrowAPI.create.mockRejectedValue({
      response: { data: { message: 'Validation failed' } },
    });

    await expect(escrow.create({ amount_sats: 0 })).rejects.toBeTruthy();

    const state = get(escrow);
    expect(state.error).toBe('Validation failed');
    expect(state.loading).toBe(false);
  });

  it('updateStatus calls fund correctly', async () => {
    const updatedEscrow = { id: 'esc-1', state: 'funded' };
    escrowAPI.fund.mockResolvedValue({
      data: { data: { success: true, escrow: updatedEscrow } },
    });

    await escrow.updateStatus('esc-1', 'fund', { txId: 'tx-123' });
    expect(escrowAPI.fund).toHaveBeenCalledWith('esc-1', 'tx-123');
  });

  it('updateStatus calls deliver correctly', async () => {
    const updatedEscrow = { id: 'esc-1', state: 'awaiting_delivery' };
    escrowAPI.deliver.mockResolvedValue({
      data: { data: { success: true, escrow: updatedEscrow } },
    });

    await escrow.updateStatus('esc-1', 'deliver');
    expect(escrowAPI.deliver).toHaveBeenCalledWith('esc-1');
  });

  it('updateStatus calls confirm correctly', async () => {
    const updatedEscrow = { id: 'esc-1', state: 'released_to_seller' };
    escrowAPI.confirm.mockResolvedValue({
      data: { data: { success: true, escrow: updatedEscrow } },
    });

    await escrow.updateStatus('esc-1', 'confirm');
    expect(escrowAPI.confirm).toHaveBeenCalledWith('esc-1');
  });

  it('updateStatus calls cancel correctly', async () => {
    const updatedEscrow = { id: 'esc-1', state: 'cancelled' };
    escrowAPI.cancel.mockResolvedValue({
      data: { data: { success: true, escrow: updatedEscrow } },
    });

    await escrow.updateStatus('esc-1', 'cancel');
    expect(escrowAPI.cancel).toHaveBeenCalledWith('esc-1');
  });

  it('updateStatus calls dispute correctly', async () => {
    const updatedEscrow = { id: 'esc-1', state: 'disputed' };
    escrowAPI.dispute.mockResolvedValue({
      data: { data: { success: true, escrow: updatedEscrow } },
    });

    await escrow.updateStatus('esc-1', 'dispute', { description: 'Bad item' });
    expect(escrowAPI.dispute).toHaveBeenCalledWith('esc-1', 'Bad item');
  });
});
