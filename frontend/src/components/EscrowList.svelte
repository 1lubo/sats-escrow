<script>
  import { onMount, createEventDispatcher } from 'svelte';
  import { escrow } from '../stores/escrow';
  import { auth } from '../stores/auth';
  import { navigate } from '../stores/router';
  import EscrowCard from './EscrowCard.svelte';
  import CreateEscrowForm from './CreateEscrowForm.svelte';

  const dispatch = createEventDispatcher();

  let showCreateForm = false;

  onMount(() => {
    escrow.fetch();
  });

  const handleLogout = () => {
    auth.logout();
    navigate('/');
  };

  function handleSelectEscrow(e) {
    dispatch('selectEscrow', { id: e.detail.id });
  }
</script>

<div class="min-h-screen bg-gray-950 text-white">
  <nav class="fixed top-0 inset-x-0 z-50 bg-gray-950/80 backdrop-blur-md border-b border-white/5">
    <div class="max-w-5xl mx-auto px-6 h-16 flex items-center justify-between">
      <h1 class="text-xl font-bold tracking-tight">₿ SatsEscrow</h1>
      <div class="flex items-center gap-3">
        <button
          on:click={handleLogout}
          class="text-gray-300 hover:text-white px-4 py-2 rounded-lg text-sm font-medium transition border border-white/10 hover:border-white/20"
        >
          Logout
        </button>
      </div>
    </div>
  </nav>

  <div class="max-w-5xl mx-auto px-6 pt-24 pb-12">
    <div class="flex justify-between items-center mb-8">
      <h2 class="text-3xl font-bold text-white">Your Escrows</h2>
      <button
        on:click={() => (showCreateForm = !showCreateForm)}
        class="bg-orange-500 hover:bg-orange-600 text-white px-5 py-2 rounded-lg text-sm font-semibold transition shadow-lg shadow-orange-500/20"
      >
        {showCreateForm ? 'Cancel' : 'Create New Escrow'}
      </button>
    </div>

    {#if showCreateForm}
      <CreateEscrowForm on:created={() => (showCreateForm = false)} on:close={() => (showCreateForm = false)} />
    {/if}

    {#if $escrow.error}
      <div class="bg-red-500/10 border border-red-500/20 text-red-400 px-4 py-3 rounded-lg mb-4">
        {$escrow.error}
      </div>
    {/if}

    {#if $escrow.loading}
      <div class="text-center py-12">
        <div class="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
          {#each [1, 2, 3] as _}
            <div class="bg-white/5 border border-white/5 rounded-xl p-6 animate-pulse">
              <div class="flex justify-between items-start mb-4">
                <div class="h-6 bg-white/10 rounded w-32"></div>
                <div class="h-6 bg-white/10 rounded-full w-20"></div>
              </div>
              <div class="space-y-3">
                <div class="h-4 bg-white/10 rounded w-full"></div>
                <div class="h-4 bg-white/10 rounded w-3/4"></div>
                <div class="h-4 bg-white/10 rounded w-1/2"></div>
                <div class="h-4 bg-white/10 rounded w-2/3"></div>
              </div>
              <div class="mt-6 h-10 bg-white/10 rounded"></div>
            </div>
          {/each}
        </div>
      </div>
    {:else if $escrow.escrows.length === 0}
      <div class="bg-white/5 border border-white/5 rounded-xl p-12 text-center backdrop-blur-sm">
        <p class="text-5xl mb-4">📦</p>
        <h3 class="text-xl font-semibold text-white mb-2">No escrows yet</h3>
        <p class="text-gray-400 mb-6">Create your first escrow to get started with secure Bitcoin transactions.</p>
        <button
          on:click={() => (showCreateForm = true)}
          class="bg-orange-500 hover:bg-orange-600 text-white px-5 py-2 rounded-lg text-sm font-semibold transition shadow-lg shadow-orange-500/20"
        >
          Create New Escrow
        </button>
      </div>
    {:else}
      <div class="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
        {#each $escrow.escrows as escrowItem (escrowItem.id)}
          <EscrowCard {escrowItem} on:select={handleSelectEscrow} />
        {/each}
      </div>
    {/if}
  </div>
</div>