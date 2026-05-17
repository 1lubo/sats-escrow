<script>
  import { escrow } from '../stores/escrow';

  export let escrowItem;

  const getStatusColor = (status) => {
    switch (status) {
      case 'Created':
        return 'bg-yellow-100 text-yellow-800';
      case 'Funded':
        return 'bg-blue-100 text-blue-800';
      case 'AwaitingDelivery':
        return 'bg-purple-100 text-purple-800';
      case 'Released':
        return 'bg-green-100 text-green-800';
      case 'Disputed':
        return 'bg-red-100 text-red-800';
      default:
        return 'bg-gray-100 text-gray-800';
    }
  };

  const canPerformAction = (status, action) => {
    const transitions = {
      fund: ['Created'],
      deliver: ['Funded'],
      confirm: ['AwaitingDelivery'],
      cancel: ['Created'],
      dispute: ['Funded', 'AwaitingDelivery'],
    };
    return transitions[action]?.includes(status);
  };

  const handleAction = async (action) => {
    try {
      await escrow.updateStatus(escrowItem.id, action);
    } catch (error) {
      console.error(`Failed to ${action}:`, error);
    }
  };
</script>

<div class="bg-white rounded-lg shadow-lg p-6 hover:shadow-xl transition">
  <div class="flex justify-between items-start mb-4">
    <h3 class="text-lg font-bold text-gray-800">Escrow {escrowItem.id.substring(0, 8)}</h3>
    <span class="px-3 py-1 rounded-full text-xs font-semibold {getStatusColor(escrowItem.status)}">
      {escrowItem.status}
    </span>
  </div>

  <div class="space-y-2 text-sm text-gray-600 mb-6">
    <p><strong>Amount:</strong> {escrowItem.amount} sats</p>
    <p><strong>Buyer:</strong> {escrowItem.buyer_id.substring(0, 8)}...</p>
    <p><strong>Seller:</strong> {escrowItem.seller_id.substring(0, 8)}...</p>
    <p><strong>Created:</strong> {new Date(escrowItem.created_at).toLocaleDateString()}</p>
  </div>

  <div class="space-y-2">
    {#if canPerformAction(escrowItem.status, 'fund')}
      <button
        on:click={() => handleAction('fund')}
        disabled={$escrow.loading}
        class="w-full bg-blue-600 text-white py-2 rounded hover:bg-blue-700 transition disabled:opacity-50"
      >
        Fund
      </button>
    {/if}

    {#if canPerformAction(escrowItem.status, 'deliver')}
      <button
        on:click={() => handleAction('deliver')}
        disabled={$escrow.loading}
        class="w-full bg-purple-600 text-white py-2 rounded hover:bg-purple-700 transition disabled:opacity-50"
      >
        Mark Delivered
      </button>
    {/if}

    {#if canPerformAction(escrowItem.status, 'confirm')}
      <button
        on:click={() => handleAction('confirm')}
        disabled={$escrow.loading}
        class="w-full bg-green-600 text-white py-2 rounded hover:bg-green-700 transition disabled:opacity-50"
      >
        Confirm & Release
      </button>
    {/if}

    {#if canPerformAction(escrowItem.status, 'cancel')}
      <button
        on:click={() => handleAction('cancel')}
        disabled={$escrow.loading}
        class="w-full bg-gray-600 text-white py-2 rounded hover:bg-gray-700 transition disabled:opacity-50"
      >
        Cancel
      </button>
    {/if}

    {#if canPerformAction(escrowItem.status, 'dispute')}
      <button
        on:click={() => handleAction('dispute')}
        disabled={$escrow.loading}
        class="w-full bg-red-600 text-white py-2 rounded hover:bg-red-700 transition disabled:opacity-50"
      >
        Open Dispute
      </button>
    {/if}
  </div>
</div>