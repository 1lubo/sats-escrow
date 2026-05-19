import { writable } from 'svelte/store';

function parseHash(hash) {
  const raw = hash.replace(/^#\/?/, '') || '';
  const segments = raw.split('/').filter(Boolean);

  if (segments.length === 0) return { path: '/', params: {} };
  if (segments[0] === 'login') return { path: '/login', params: {} };
  if (segments[0] === 'dashboard') return { path: '/dashboard', params: {} };
  if (segments[0] === 'escrow' && segments[1]) {
    return { path: '/escrow/:id', params: { id: segments[1] } };
  }

  return { path: '/' + segments.join('/'), params: {} };
}

function createRouter() {
  const initial =
    typeof window !== 'undefined'
      ? parseHash(window.location.hash)
      : { path: '/', params: {} };

  const { subscribe, set } = writable(initial);

  if (typeof window !== 'undefined') {
    window.addEventListener('hashchange', () => {
      set(parseHash(window.location.hash));
    });
  }

  return { subscribe };
}

export const router = createRouter();

export function navigate(path) {
  window.location.hash = path.startsWith('#') ? path : '#' + path;
}
