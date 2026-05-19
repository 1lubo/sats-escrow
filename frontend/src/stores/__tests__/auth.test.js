import { describe, it, expect, beforeEach } from 'vitest';
import { get } from 'svelte/store';
import { auth } from '../auth';

describe('auth store', () => {
  beforeEach(() => {
    localStorage.clear();
    auth.logout(); // reset state
  });

  it('starts unauthenticated when no stored token', () => {
    // After logout/clear, store should be unauthenticated
    const state = get(auth);
    expect(state.token).toBeNull();
    expect(state.isAuthenticated).toBe(false);
  });

  it('login sets token and isAuthenticated, writes localStorage', () => {
    const uuid = '550e8400-e29b-41d4-a716-446655440000';
    auth.login(uuid);

    const state = get(auth);
    expect(state.token).toBe(uuid);
    expect(state.isAuthenticated).toBe(true);
    expect(localStorage.getItem('auth_token')).toBe(uuid);
  });

  it('logout clears token and isAuthenticated, removes localStorage', () => {
    auth.login('550e8400-e29b-41d4-a716-446655440000');
    auth.logout();

    const state = get(auth);
    expect(state.token).toBeNull();
    expect(state.isAuthenticated).toBe(false);
    expect(localStorage.getItem('auth_token')).toBeNull();
  });
});
