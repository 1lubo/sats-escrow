import { describe, it, expect, beforeEach, vi } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import App from '../../App.svelte';
import { auth } from '../../stores/auth';

describe('App', () => {
  beforeEach(() => {
    localStorage.clear();
    window.location.hash = '';
    auth.logout();
  });

  it('shows LandingPage when not authenticated', () => {
    render(App);

    // LandingPage renders the SatsEscrow heading and tagline
    expect(screen.getByText('SatsEscrow')).toBeTruthy();
    expect(screen.getAllByText('Try Demo').length).toBeGreaterThan(0);
  });
});
