<script>
  import { escrow } from '../stores/escrow';

  export let escrowItem;

  let actionLoading = false;
  let error = '';

  const getStatusColor = (state) => {
    switch (state) {
      case 'created':
        return 'bg-yellow-100 text-yellow-800';
      case 'funded':
        return 'bg-blue-100 text-blue-800';
      case 'awaiting_delivery':
        return 'bg-purple-100 text-purple-800';
      case 'released_to_seller':
      case 'released_to_buyer':
        return 'bg-green-100 text-green-800';
      case 'disputed':
        return 'bg-red-100 text-red-800';
      case 'cancelled':
        return 'bg-gray-100 text-gray-800';
      default:
        return 'bg-gray-100 text-gray-800';
    }
  };

  const canPerformAction = (state, action) => {
    const transitions = {
      fund: ['created'],
      deliver: ['funded'],
      confirm: ['awaiting_delivery'],
      cancel: ['created'],
      dispute: ['funded', 'awaiting_delivery'],
    };
    return transitions[action]?.includes(state);
  };

  const DESTRUCTIVE_ACTIONS = ['cancel', 'dispute', 'confirm'];
  const ACTION_LABELS = {
    cancel: 'Cancel this escrow',
    dispute: 'Open a dispute on this escrow',
    confirm: 'Confirm delivery and release funds',
  };

  const handleAction = async (action) => {
    if (DESTRUCTIVE_ACTIONS.includes(action)) {
      const confirmed = window.confirm(`Are you sure you want to: ${ACTION_LABELS[action]}?`);
      if (!confirmed) return;
    }

    actionLoading = true;
    error = '';
    try {
      await escrow.updateStatus(escrowItem.id, action);
    } catch (err) {
      error = err.response?.data?.message || `Failed to ${action} escrow`;
    } finally {
      actionLoading = false;
    }
  };
</script>

<div class="bg-white rounded-lg shadow-lg p-6 hover:shadow-xl transition">
  <div class="flex justify-between items-start mb-4">
    <h3 class="text-lg font-bold text-gray-800">Escrow {escrowItem.id.substring(0, 8)}</h3>
    <span class="px-3 py-1 rounded-full text-xs font-semibold {getStatusColor(escrowItem.state)}">
      {escrowItem.state}
    </span>
  </div>

  {#if error}
    <div class="bg-red-100 border border-red-400 text-red-700 px-3 py-2 rounded mb-4 text-sm" aria-live="assertive">
      {error}
    </div>
  {/if}

  <div class="space-y-2 text-sm text-gray-600 mb-6">
    <p><strong>Amount:</strong> {escrowItem.amount_sats} sats</p>
    <p><strong>Buyer:</strong> {escrowItem.buyer.substring(0, 8)}...</p>
    <p><strong>Seller:</strong> {escrowItem.seller.substring(0, 8)}...</p>
    <p><strong>Created:</strong> {new Date(escrowItem.created_at).toLocaleDateString()}</p>
  </div>

  <div class="space-y-2">
    {#if canPerformAction(escrowItem.state, 'fund')}
      <button
        on:click={() => handleAction('fund')}
        disabled={actionLoading}
        class="w-full bg-blue-600 text-white py-2 rounded hover:bg-blue-700 transition disabled:opacity-50"
      >
        {actionLoading ? 'Processing...' : 'Fund'}
      </button>
    {/if}

    {#if canPerformAction(escrowItem.state, 'deliver')}
      <button
        on:click={() => handleAction('deliver')}
        disabled={actionLoading}
        class="w-full bg-purple-600 text-white py-2 rounded hover:bg-purple-700 transition disabled:opacity-50"
      >
        {actionLoading ? 'Processing...' : 'Mark Delivered'}
      </button>
    {/if}

    {#if canPerformAction(escrowItem.state, 'confirm')}
      <button
        on:click={() => handleAction('confirm')}
        disabled={actionLoading}
        class="w-full bg-green-600 text-white py-2 rounded hover:bg-green-700 transition disabled:opacity-50"
      >
        {actionLoading ? 'Processing...' : 'Confirm & Release'}
      </button>
    {/if}

    {#if canPerformAction(escrowItem.state, 'cancel')}
      <button
        on:click={() => handleAction('cancel')}
        disabled={actionLoading}
        class="w-full bg-gray-600 text-white py-2 rounded hover:bg-gray-700 transition disabled:opacity-50"
      >
        {actionLoading ? 'Processing...' : 'Cancel'}
      </button>
    {/if}

    {#if canPerformAction(escrowItem.state, 'dispute')}
      <button
        on:click={() => handleAction('dispute')}
        disabled={actionLoading}
        class="w-full bg-red-600 text-white py-2 rounded hover:bg-red-700 transition disabled:opacity-50"
      >
        {actionLoading ? 'Processing...' : 'Open Dispute'}
      </button>
    {/if}
  </div>
</div>