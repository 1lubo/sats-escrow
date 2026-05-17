import axios from 'axios';

const API_URL = import.meta.env.VITE_API_URL || 'http://localhost:8000';

const client = axios.create({
  baseURL: `${API_URL}/api/v1`,
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

export const escrowAPI = {
  create: (data) => client.post('/escrows', data),
  list: () => client.get('/escrows'),
  getById: (id) => client.get(`/escrows/${id}`),
  fund: (id) => client.post(`/escrows/${id}/fund`),
  deliver: (id) => client.post(`/escrows/${id}/deliver`),
  confirm: (id) => client.post(`/escrows/${id}/confirm`),
  dispute: (id, reason) => client.post(`/escrows/${id}/dispute`, { reason }),
  cancel: (id) => client.post(`/escrows/${id}/cancel`),
};

export default client;