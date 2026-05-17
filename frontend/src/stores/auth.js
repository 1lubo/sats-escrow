import { writable } from 'svelte/store';

function createAuthStore() {
  const storedToken = typeof window !== 'undefined' ? localStorage.getItem('auth_token') : null;
  const { subscribe, set, update } = writable({
    token: storedToken,
    isAuthenticated: !!storedToken,
  });

  return {
    subscribe,
    login: (uuid) => {
      localStorage.setItem('auth_token', uuid);
      set({ token: uuid, isAuthenticated: true });
    },
    logout: () => {
      localStorage.removeItem('auth_token');
      set({ token: null, isAuthenticated: false });
    },
  };
}

export const auth = createAuthStore();