import { writable } from 'svelte/store';
import { escrowAPI } from '../api/client';

function createEscrowStore() {
  const { subscribe, set, update } = writable({
    escrows: [],
    loading: false,
    error: null,
    currentEscrow: null,
  });

  return {
    subscribe,
    fetch: async () => {
      update((state) => ({ ...state, loading: true, error: null }));
      try {
        const { data } = await escrowAPI.list();
        update((state) => ({ ...state, escrows: data.data ?? data, loading: false }));
      } catch (error) {
        update((state) => ({
          ...state,
          error: error.response?.data?.message || error.message || 'Failed to fetch escrows',
          loading: false,
        }));
      }
    },
    create: async (escrowData) => {
      update((state) => ({ ...state, loading: true, error: null }));
      try {
        const { data } = await escrowAPI.create(escrowData);
        update((state) => ({
          ...state,
          escrows: [...state.escrows, data.data ?? data],
          loading: false,
        }));
        return data.data ?? data;
      } catch (error) {
        update((state) => ({
          ...state,
          error: error.response?.data?.message || 'Failed to create escrow',
          loading: false,
        }));
        throw error;
      }
    },
    updateStatus: async (id, action, extra) => {
      try {
        let response;
        if (action === 'fund') response = await escrowAPI.fund(id, extra?.txId || 'mock-tx');
        else if (action === 'deliver') response = await escrowAPI.deliver(id);
        else if (action === 'confirm') response = await escrowAPI.confirm(id);
        else if (action === 'cancel') response = await escrowAPI.cancel(id);
        else if (action === 'dispute') response = await escrowAPI.dispute(id, extra?.description);

        // Action endpoints return { data: { success, message, escrow } }
        const updatedEscrow = response.data?.data?.escrow ?? response.data?.escrow ?? response.data;

        update((state) => ({
          ...state,
          escrows: state.escrows.map((e) => (e.id === id ? updatedEscrow : e)),
        }));
      } catch (error) {
        throw error;
      }
    },
  };
}

export const escrow = createEscrowStore();