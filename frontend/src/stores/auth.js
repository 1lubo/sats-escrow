import { writable } from 'svelte/store';

function createAuthStore() {
  const { subscribe, set } = writable({
    token: localStorage.getItem('authToken') || null,
    userId: localStorage.getItem('userId') || null,
  });

  return {
    subscribe,
    login: (token, userId) => {
      localStorage.setItem('authToken', token);
      localStorage.setItem('userId', userId);
      set({ token, userId });
    },
    logout: () => {
      localStorage.removeItem('authToken');
      localStorage.removeItem('userId');
      set({ token: null, userId: null });
    },
  };
}

export const auth = createAuthStore();