import { describe, it, expect, beforeEach, vi } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import App from '../../App.svelte';
import { auth } from '../../stores/auth';

describe('App', () => {
  beforeEach(() => {
    localStorage.clear();
    auth.logout();
  });

  it('shows LoginForm when not authenticated', () => {
    render(App);

    // LoginForm renders the SatsEscrow heading
    expect(screen.getByText('SatsEscrow')).toBeTruthy();
    expect(screen.getByPlaceholderText('Your UUID')).toBeTruthy();
  });
});
