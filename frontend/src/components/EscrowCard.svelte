<script>
  import { escrowAPI } from '../api/client';
  import { escrowStore } from '../stores/escrow';

  export let escrow;

  let loading = false;
  let error = '';

  const getStatusColor = (status) => {
    const colors = {
      Created: 'bg-gray-100 text-gray-800',
      Funded: 'bg-blue-100 text-blue-800',
      AwaitingDelivery: 'bg-yellow-100 text-yellow-800',
      Released: 'bg-green-100 text-green-800',
      Disputed: 'bg-red-100 text-red-800',
      Cancelled: 'bg-gray-400 text-gray-800',
    };
    return colors[status] || 'bg-gray-100 text-gray-800';
  };

  const getNextAction = () => {
    switch (escrow.status) {
      case 'Created':
        return 'Fund';
      case 'Funded':
        return 'Mark Delivered';
      case 'AwaitingDelivery':
        return 'Confirm Delivery';
      default:
        return null;
    }
  };

  const handleAction = async () => {
    loading = true;
    error = '';
    try {
      let response;
      switch (escrow.status) {
        case 'Created':
          response = await escrowAPI.fundEscrow(escrow.id);
          break;
        case 'Funded':
          response = await escrowAPI.deliverEscrow(escrow.id);
          break;
        case 'AwaitingDelivery':
          response = await escrowAPI.confirmDelivery(escrow.id);
          break;
      }
      escrow = response.data;
      escrowStore.fetch();
    } catch (e) {
      error = e.response?.data?.error || 'Action failed';
    } finally {
      loading = false;
    }
  };

  const handleDispute = async () => {
    const reason = prompt('Enter dispute reason:');
    if (!reason) return;
    loading = true;
    error = '';
    try {
      const response = await escrowAPI.openDispute(escrow.id, reason);
      escrow = response.data;
      escrowStore.fetch();
    } catch (e) {
      error = e.response?.data?.error || 'Dispute failed';
    } finally {
      loading = false;
    }
  };
</script>

<div class="bg-white rounded-lg shadow-md p-6">
  <div class="flex justify-between items-start mb-4">
    <div>
      <h3 class="text-lg font-semibold text-gray-900">Escrow #{escrow.id.substring(0, 8)}</h3>
      <p class="text-sm text-gray-500">{escrow.buyer_id === escrow.seller_id ? 'Self' : 'P2P'} Transaction</p>
    </div>
    <span class={`px-3 py-1 rounded-full text-xs font-medium ${getStatusColor(escrow.status)}`}>
      {escrow.status}
    </span>
  </div>

  <div class="space-y-2 mb-4 text-sm">
    <p class="text-gray-700"><strong>Amount:</strong> {escrow.amount} satoshis</p>
    <p class="text-gray-700"><strong>Created:</strong> {new Date(escrow.created_at).toLocaleDateString()}</p>
    {#if escrow.description}
      <p class="text-gray-700"><strong>Description:</strong> {escrow.description}</p>
    {/if}
  </div>

  {#if error}
    <div class="rounded-md bg-red-50 p-3 mb-4">
      <p class="text-sm text-red-800">{error}</p>
    </div>
  {/if}

  <div class="flex gap-2">
    {#if getNextAction()}
      <button
        on:click={handleAction}
        disabled={loading}
        class="flex-1 px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 disabled:opacity-50 focus:outline-none focus:ring-2 focus:ring-blue-500"
      >
        {loading ? 'Processing...' : getNextAction()}
      </button>
    {/if}
    {#if escrow.status === 'AwaitingDelivery' || escrow.status === 'Funded'}
      <button
        on:click={handleDispute}
        disabled={loading}
        class="flex-1 px-4 py-2 bg-red-600 text-white rounded-md hover:bg-red-700 disabled:opacity-50 focus:outline-none focus:ring-2 focus:ring-red-500"
      >
        Dispute
      </button>
    {/if}
  </div>
</div>