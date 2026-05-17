<script>
  import { createEventDispatcher } from 'svelte';
  import { escrow } from '../stores/escrow';
  import { auth } from '../stores/auth';

  const dispatch = createEventDispatcher();

  let formData = {
    seller_id: '',
    amount: '',
    description: '',
  };
  let error = '';
  let loading = false;

  const handleSubmit = async (e) => {
    e.preventDefault();
    loading = true;
    error = '';

    if (!formData.seller_id.trim() || !formData.amount || !formData.description.trim()) {
      error = 'All fields are required';
      loading = false;
      return;
    }

    try {
      await escrow.create({
        seller_id: formData.seller_id,
        amount: parseInt(formData.amount),
        description: formData.description,
      });
      dispatch('created');
      formData = { seller_id: '', amount: '', description: '' };
    } catch (err) {
      error = err.response?.data?.message || 'Failed to create escrow';
    } finally {
      loading = false;
    }
  };
</script>

<form on:submit={handleSubmit} class="bg-white rounded-lg shadow-lg p-6 mb-8">
  <h3 class="text-2xl font-bold text-gray-800 mb-4">Create New Escrow</h3>

  {#if error}
    <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
      {error}
    </div>
  {/if}

  <div class="space-y-4">
    <div>
      <label for="seller_id" class="block text-sm font-semibold text-gray-700 mb-2"
        >Seller UUID</label
      >
      <input
        type="text"
        id="seller_id"
        bind:value={formData.seller_id}
        placeholder="Enter seller's UUID"
        class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
        disabled={loading}
        required
      />
    </div>

    <div>
      <label for="amount" class="block text-sm font-semibold text-gray-700 mb-2"
        >Amount (sats)</label
      >
      <input
        type="number"
        id="amount"
        bind:value={formData.amount}
        placeholder="Amount in satoshis"
        min="1"
        class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
        disabled={loading}
        required
      />
    </div>

    <div>
      <label for="description" class="block text-sm font-semibold text-gray-700 mb-2"
        >Description</label
      >
      <textarea
        id="description"
        bind:value={formData.description}
        placeholder="Describe the goods or services"
        rows="4"
        class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
        disabled={loading}
        required
      />
    </div>
  </div>

  <div class="flex gap-4 mt-6">
    <button
      type="submit"
      disabled={loading}
      class="flex-1 bg-green-600 text-white py-2 rounded-lg hover:bg-green-700 transition font-semibold disabled:opacity-50"
    >
      {loading ? 'Creating...' : 'Create Escrow'}
    </button>
  </div>
</form>