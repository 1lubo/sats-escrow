<script>
  import { escrowStore } from '../stores/escrow';

  export let onCreated = () => {};

  let formData = {
    amount: '',
    seller_id: '',
    description: '',
  };
  let loading = false;
  let error = '';

  const handleSubmit = async () => {
    if (!formData.amount || !formData.seller_id) {
      error = 'Amount and Seller ID are required';
      return;
    }

    loading = true;
    error = '';
    try {
      await escrowStore.createEscrow(formData);
      formData = { amount: '', seller_id: '', description: '' };
      onCreated();
    } catch (e) {
      error = e.response?.data?.error || 'Failed to create escrow';
    } finally {
      loading = false;
    }
  };
</script>

<div class="bg-white rounded-lg shadow-md p-6">
  <h2 class="text-xl font-semibold text-gray-900 mb-4">Create New Escrow</h2>

  <form on:submit|preventDefault={handleSubmit} class="space-y-4">
    <div>
      <label for="amount" class="block text-sm font-medium text-gray-700">Amount (satoshis)</label>
      <input
        id="amount"
        type="number"
        placeholder="e.g., 100000"
        bind:value={formData.amount}
        class="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 sm:text-sm"
      />
    </div>

    <div>
      <label for="seller_id" class="block text-sm font-medium text-gray-700">Seller ID (UUID)</label>
      <input
        id="seller_id"
        type="text"
        placeholder="e.g., 550e8400-e29b-41d4-a716-446655440000"
        bind:value={formData.seller_id}
        class="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 sm:text-sm"
      />
    </div>

    <div>
      <label for="description" class="block text-sm font-medium text-gray-700">Description (optional)</label>
      <textarea
        id="description"
        placeholder="Describe the transaction..."
        bind:value={formData.description}
        rows="3"
        class="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 sm:text-sm"
      />
    </div>

    {#if error}
      <div class="rounded-md bg-red-50 p-4">
        <p class="text-sm font-medium text-red-800">{error}</p>
      </div>
    {/if}

    <button
      type="submit"
      disabled={loading}
      class="w-full px-4 py-2 bg-green-600 text-white rounded-md hover:bg-green-700 disabled:opacity-50 focus:outline-none focus:ring-2 focus:ring-green-500 font-medium"
    >
      {loading ? 'Creating...' : 'Create Escrow'}
    </button>
  </form>
</div>