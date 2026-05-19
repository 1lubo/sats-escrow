import axios from 'axios';
import { auth } from '../stores/auth';

const API_URL = import.meta.env.VITE_API_URL || 'http://localhost:8000';

const client = axios.create({
  baseURL: `${API_URL}/api/v1`,
  timeout: 30000,
  headers: {
    'Content-Type': 'application/json',
  },
});

// Add auth token to requests
client.interceptors.request.use((config) => {
  const token = localStorage.getItem('auth_token');
  if (token) {
    config.headers.Authorization = `Bearer ${token}`;
  }
  return config;
});

// Response interceptor — handle 401 auto-logout and network errors
client.interceptors.response.use(
  (response) => response,
  (error) => {
    if (error.response?.status === 401) {
      auth.logout();
      return Promise.reject(new Error('Session expired. Please log in again.'));
    }
    if (!error.response) {
      return Promise.reject(new Error('Network error. Please check your connection.'));
    }
    return Promise.reject(error);
  },
);

export const escrowAPI = {
  create: (data) => client.post('/escrows', data),
  list: () => client.get('/escrows'),
  getById: (id) => client.get(`/escrows/${id}`),
  fund: (id, txId) => client.post(`/escrows/${id}/fund`, { tx_id: txId }),
  deliver: (id) => client.post(`/escrows/${id}/deliver`),
  confirm: (id) => client.post(`/escrows/${id}/confirm`),
  dispute: (id, description) =>
    client.post(`/escrows/${id}/dispute`, { description: description || 'Dispute opened' }),
  cancel: (id) => client.post(`/escrows/${id}/cancel`, {}),
};

export default client;