import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/svelte';
import LoginForm from '../LoginForm.svelte';
import { auth } from '../../stores/auth';

describe('LoginForm', () => {
  beforeEach(() => {
    localStorage.clear();
    auth.logout();
  });

  it('renders form with title, input, and button', () => {
    render(LoginForm);

    expect(screen.getByText('SatsEscrow')).toBeTruthy();
    expect(screen.getByPlaceholderText('Your UUID')).toBeTruthy();
    expect(screen.getByText('Login')).toBeTruthy();
  });

  it('shows error for empty UUID', async () => {
    render(LoginForm);

    const button = screen.getByText('Login');
    await fireEvent.click(button);

    expect(screen.getByText('Please enter a valid UUID')).toBeTruthy();
  });

  it('shows error for invalid UUID format', async () => {
    render(LoginForm);

    const input = screen.getByPlaceholderText('Your UUID');
    await fireEvent.input(input, { target: { value: 'not-a-uuid' } });
    await fireEvent.click(screen.getByText('Login'));

    expect(
      screen.getByText('UUID must be in format xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx'),
    ).toBeTruthy();
  });

  it('calls auth.login with valid UUID', async () => {
    const loginSpy = vi.spyOn(auth, 'login');
    render(LoginForm);

    const uuid = '550e8400-e29b-41d4-a716-446655440000';
    const input = screen.getByPlaceholderText('Your UUID');
    await fireEvent.input(input, { target: { value: uuid } });
    await fireEvent.click(screen.getByText('Login'));

    expect(loginSpy).toHaveBeenCalledWith(uuid);
    loginSpy.mockRestore();
  });

  it('submits on Enter key', async () => {
    const loginSpy = vi.spyOn(auth, 'login');
    render(LoginForm);

    const uuid = '550e8400-e29b-41d4-a716-446655440000';
    const input = screen.getByPlaceholderText('Your UUID');
    await fireEvent.input(input, { target: { value: uuid } });
    await fireEvent.keyDown(input, { key: 'Enter' });

    expect(loginSpy).toHaveBeenCalledWith(uuid);
    loginSpy.mockRestore();
  });
});
