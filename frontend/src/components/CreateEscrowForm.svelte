<script>
  import { createEventDispatcher, onMount } from 'svelte';
  import { escrow } from '../stores/escrow';
  import { auth } from '../stores/auth';

  const dispatch = createEventDispatcher();

  const UUID_REGEX = /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i;

  let formData = {
    seller_id: '',
    amount: '',
    description: '',
  };
  let error = '';
  let loading = false;
  let sellerInput;

  onMount(() => {
    if (sellerInput) sellerInput.focus();
  });

  const handleKeydown = (e) => {
    if (e.key === 'Escape') {
      dispatch('close');
    }
  };

  const handleSubmit = async (e) => {
    e.preventDefault();
    loading = true;
    error = '';

    if (!formData.seller_id.trim() || !formData.amount || !formData.description.trim()) {
      error = 'All fields are required';
      loading = false;
      return;
    }

    if (!UUID_REGEX.test(formData.seller_id.trim())) {
      error = 'Seller UUID must be in format xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx';
      loading = false;
      return;
    }

    const parsedAmount = parseInt(formData.amount);
    if (!Number.isInteger(parsedAmount) || parsedAmount <= 0) {
      error = 'Amount must be a positive integer';
      loading = false;
      return;
    }

    try {
      await escrow.create({
        role: 'buyer',
        counterparty_id: formData.seller_id.trim(),
        amount_sats: parsedAmount,
        description: formData.description.trim(),
      });
      formData = { seller_id: '', amount: '', description: '' };
      dispatch('created');
    } catch (err) {
      error = err.response?.data?.message || 'Failed to create escrow';
    } finally {
      loading = false;
    }
  };
</script>

<svelte:window on:keydown={handleKeydown} />

<form on:submit={handleSubmit} class="bg-white/5 border border-white/5 rounded-xl p-6 mb-8 backdrop-blur-sm">
  <h3 class="text-2xl font-bold text-white mb-4">Create New Escrow</h3>

  {#if error}
    <div class="bg-red-500/10 border border-red-500/20 text-red-400 px-4 py-3 rounded-lg mb-4" aria-live="assertive">
      {error}
    </div>
  {/if}

  <div class="space-y-4">
    <div>
      <label for="seller_id" class="block text-sm font-semibold text-gray-300 mb-2"
        >Seller UUID</label
      >
      <input
        type="text"
        id="seller_id"
        bind:this={sellerInput}
        bind:value={formData.seller_id}
        placeholder="Enter seller's UUID"
        class="w-full px-4 py-2 bg-white/5 border border-white/10 rounded-lg text-white placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-orange-500 focus:border-transparent"
        disabled={loading}
        required
        aria-required="true"
        aria-invalid={!!error && !UUID_REGEX.test(formData.seller_id.trim())}
      />
    </div>

    <div>
      <label for="amount" class="block text-sm font-semibold text-gray-300 mb-2"
        >Amount (sats)</label
      >
      <input
        type="number"
        id="amount"
        bind:value={formData.amount}
        placeholder="Amount in satoshis"
        min="1"
        class="w-full px-4 py-2 bg-white/5 border border-white/10 rounded-lg text-white placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-orange-500 focus:border-transparent"
        disabled={loading}
        required
        aria-required="true"
        aria-invalid={!!error && (!formData.amount || parseInt(formData.amount) <= 0)}
      />
    </div>

    <div>
      <label for="description" class="block text-sm font-semibold text-gray-300 mb-2"
        >Description</label
      >
      <textarea
        id="description"
        bind:value={formData.description}
        placeholder="Describe the goods or services"
        rows="4"
        class="w-full px-4 py-2 bg-white/5 border border-white/10 rounded-lg text-white placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-orange-500 focus:border-transparent"
        disabled={loading}
        required
        aria-required="true"
      />
    </div>
  </div>

  <div class="flex gap-4 mt-6">
    <button
      type="submit"
      disabled={loading}
      class="flex-1 bg-orange-500 hover:bg-orange-600 text-white py-2 rounded-lg transition font-semibold disabled:opacity-50 shadow-lg shadow-orange-500/20"
    >
      {loading ? 'Creating...' : 'Create Escrow'}
    </button>
  </div>
</form>