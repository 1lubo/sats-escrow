import { writable } from 'svelte/store';
import { escrowAPI } from '../api/client';

function createEscrowStore() {
  const { subscribe, set, update } = writable({
    escrows: [],
    loading: false,
    error: null,
  });

  return {
    subscribe,
    fetch: async () => {
      update((state) => ({ ...state, loading: true, error: null }));
      try {
        const response = await escrowAPI.listEscrows();
        update((state) => ({
          ...state,
          escrows: response.data,
          loading: false,
        }));
      } catch (error) {
        update((state) => ({
          ...state,
          error: error.message,
          loading: false,
        }));
      }
    },
    createEscrow: async (data) => {
      try {
        const response = await escrowAPI.createEscrow(data);
        update((state) => ({
          ...state,
          escrows: [...state.escrows, response.data],
        }));
        return response.data;
      } catch (error) {
        update((state) => ({
          ...state,
          error: error.message,
        }));
        throw error;
      }
    },
  };
}

export const escrowStore = createEscrowStore();