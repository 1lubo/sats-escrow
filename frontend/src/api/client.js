import axios from 'axios';

const API_BASE = import.meta.env.VITE_API_URL || 'http://localhost:8000/api/v1';

const client = axios.create({
  baseURL: API_BASE,
});

// Add auth token from localStorage
client.interceptors.request.use((config) => {
  const token = localStorage.getItem('authToken');
  if (token) {
    config.headers.Authorization = `Bearer ${token}`;
  }
  return config;
});

export const escrowAPI = {
  createEscrow: (data) => client.post('/escrows', data),
  listEscrows: () => client.get('/escrows'),
  getEscrow: (id) => client.get(`/escrows/${id}`),
  fundEscrow: (id) => client.post(`/escrows/${id}/fund`, {}),
  deliverEscrow: (id) => client.post(`/escrows/${id}/deliver`, {}),
  confirmDelivery: (id) => client.post(`/escrows/${id}/confirm`, {}),
  openDispute: (id, reason) => client.post(`/escrows/${id}/dispute`, { reason }),
  cancelEscrow: (id) => client.post(`/escrows/${id}/cancel`, {}),
};

export default client;