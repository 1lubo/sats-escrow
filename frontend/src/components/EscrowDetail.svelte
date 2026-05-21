<script>
  import { createEventDispatcher } from 'svelte';
  import { escrow } from '../stores/escrow';

  export let escrowItem;

  const dispatch = createEventDispatcher();
  let actionLoading = false;
  let error = '';

  const getStatusColor = (state) => {
    switch (state) {
      case 'created': return 'bg-yellow-100 text-yellow-800';
      case 'funded': return 'bg-orange-100 text-orange-800';
      case 'awaiting_delivery': return 'bg-purple-100 text-purple-800';
      case 'released_to_seller':
      case 'released_to_buyer': return 'bg-green-100 text-green-800';
      case 'disputed': return 'bg-red-100 text-red-800';
      case 'cancelled': return 'bg-gray-100 text-gray-800';
      default: return 'bg-gray-100 text-gray-800';
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

  const TIMELINE_STEPS = ['created', 'funded', 'awaiting_delivery', 'confirmed'];
  const TIMELINE_LABELS = { created: 'Created', funded: 'Funded', awaiting_delivery: 'Delivery', confirmed: 'Confirmed' };

  const getStepState = (step, currentState) => {
    const order = ['created', 'funded', 'awaiting_delivery', 'confirmed'];
    const terminalMap = { released_to_seller: 3, released_to_buyer: 3, disputed: -1, cancelled: -1 };
    const currentIdx = order.indexOf(currentState);
    const stepIdx = order.indexOf(step);
    if (currentState === 'disputed' && step === 'awaiting_delivery') return 'current';
    if (currentState === 'disputed') return stepIdx < 2 ? 'done' : 'future';
    if (terminalMap[currentState] !== undefined) return stepIdx <= terminalMap[currentState] ? 'done' : 'future';
    if (stepIdx < currentIdx) return 'done';
    if (stepIdx === currentIdx) return 'current';
    return 'future';
  };

  $: btcAmount = (escrowItem.amount_sats / 100_000_000).toFixed(8);
</script>

<div class="bg-white/5 border border-white/5 rounded-xl p-8 max-w-2xl mx-auto backdrop-blur-sm">
  <button on:click={() => dispatch('back')} class="text-gray-400 hover:text-white mb-6 flex items-center gap-1 text-sm font-medium transition">
    ← Back to list
  </button>

  <div class="flex justify-between items-start mb-6">
    <h2 class="text-xl font-bold text-white break-all">Escrow {escrowItem.id}</h2>
    <span class="px-3 py-1 rounded-full text-xs font-semibold whitespace-nowrap ml-4 {getStatusColor(escrowItem.state)}">
      {escrowItem.state}
    </span>
  </div>

  {#if error}
    <div class="bg-red-500/10 border border-red-500/20 text-red-400 px-4 py-3 rounded-lg mb-6 text-sm" aria-live="assertive">{error}</div>
  {/if}

  <div class="grid grid-cols-1 md:grid-cols-2 gap-4 text-sm text-gray-400 mb-8">
    <div><strong class="text-gray-300">Amount:</strong> {escrowItem.amount_sats} sats ({btcAmount} BTC)</div>
    <div><strong class="text-gray-300">Created:</strong> {new Date(escrowItem.created_at).toLocaleString()}</div>
    <div class="break-all"><strong class="text-gray-300">Buyer:</strong> {escrowItem.buyer}</div>
    <div class="break-all"><strong class="text-gray-300">Seller:</strong> {escrowItem.seller}</div>
    {#if escrowItem.description}
      <div class="md:col-span-2"><strong class="text-gray-300">Description:</strong> {escrowItem.description}</div>
    {/if}
    {#if escrowItem.deposit_address}
      <div class="md:col-span-2 break-all"><strong class="text-gray-300">Deposit Address:</strong> {escrowItem.deposit_address}</div>
    {/if}
    {#if escrowItem.funded_at}
      <div><strong class="text-gray-300">Funded:</strong> {new Date(escrowItem.funded_at).toLocaleString()}</div>
    {/if}
  </div>

  <div class="mb-8">
    <h3 class="text-sm font-semibold text-gray-500 uppercase tracking-widest mb-3">State Timeline</h3>
    <div class="flex items-center justify-between">
      {#each TIMELINE_STEPS as step, i}
        {@const state = getStepState(step, escrowItem.state)}
        <div class="flex flex-col items-center flex-1">
          <div class="w-8 h-8 rounded-full flex items-center justify-center text-xs font-bold
            {state === 'done' ? 'bg-green-500/20 text-green-400' : state === 'current' ? 'bg-orange-500/20 text-orange-400 ring-4 ring-orange-500/10' : 'bg-white/5 text-gray-600'}">
            {state === 'done' ? '✓' : i + 1}
          </div>
          <span class="mt-1 text-xs {state === 'future' ? 'text-gray-600' : 'text-gray-400'}">{TIMELINE_LABELS[step]}</span>
        </div>
        {#if i < TIMELINE_STEPS.length - 1}
          <div class="h-0.5 flex-1 mx-1 {getStepState(TIMELINE_STEPS[i + 1], escrowItem.state) !== 'future' ? 'bg-green-500/30' : 'bg-white/5'}"></div>
        {/if}
      {/each}
      {#if escrowItem.state === 'disputed'}
        <div class="flex flex-col items-center flex-1">
          <div class="w-8 h-8 rounded-full flex items-center justify-center text-xs font-bold bg-red-500/20 text-red-400 ring-4 ring-red-500/10">!</div>
          <span class="mt-1 text-xs text-red-400">Disputed</span>
        </div>
      {/if}
    </div>
  </div>

  <div class="space-y-3">
    {#if canPerformAction(escrowItem.state, 'fund')}
      <button on:click={() => handleAction('fund')} disabled={actionLoading}
        class="w-full bg-orange-500 hover:bg-orange-600 text-white py-3 rounded-lg text-lg font-semibold transition disabled:opacity-50 shadow-lg shadow-orange-500/20">
        {actionLoading ? 'Processing...' : 'Fund Escrow'}
      </button>
    {/if}
    {#if canPerformAction(escrowItem.state, 'deliver')}
      <button on:click={() => handleAction('deliver')} disabled={actionLoading}
        class="w-full bg-purple-500/20 text-purple-400 border border-purple-500/20 py-3 rounded-lg text-lg font-semibold hover:bg-purple-500/30 transition disabled:opacity-50">
        {actionLoading ? 'Processing...' : 'Mark as Delivered'}
      </button>
    {/if}
    {#if canPerformAction(escrowItem.state, 'confirm')}
      <button on:click={() => handleAction('confirm')} disabled={actionLoading}
        class="w-full bg-green-500/20 text-green-400 border border-green-500/20 py-3 rounded-lg text-lg font-semibold hover:bg-green-500/30 transition disabled:opacity-50">
        {actionLoading ? 'Processing...' : 'Confirm & Release Funds'}
      </button>
    {/if}
    {#if canPerformAction(escrowItem.state, 'cancel')}
      <button on:click={() => handleAction('cancel')} disabled={actionLoading}
        class="w-full bg-white/5 text-gray-400 border border-white/10 py-3 rounded-lg text-lg font-semibold hover:bg-white/10 transition disabled:opacity-50">
        {actionLoading ? 'Processing...' : 'Cancel Escrow'}
      </button>
    {/if}
    {#if canPerformAction(escrowItem.state, 'dispute')}
      <button on:click={() => handleAction('dispute')} disabled={actionLoading}
        class="w-full bg-red-500/20 text-red-400 border border-red-500/20 py-3 rounded-lg text-lg font-semibold hover:bg-red-500/30 transition disabled:opacity-50">
        {actionLoading ? 'Processing...' : 'Open Dispute'}
      </button>
    {/if}
  </div>
</div>
