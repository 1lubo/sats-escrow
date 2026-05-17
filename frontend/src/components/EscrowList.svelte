<script>
  import { onMount } from 'svelte';
  import { escrow } from '../stores/escrow';
  import { auth } from '../stores/auth';
  import EscrowCard from './EscrowCard.svelte';
  import CreateEscrowForm from './CreateEscrowForm.svelte';

  let showCreateForm = false;

  onMount(() => {
    escrow.fetch();
  });

  const handleLogout = () => {
    auth.logout();
  };
</script>

<div class="min-h-screen bg-gray-100">
  <nav class="bg-blue-600 text-white p-4 shadow-lg">
    <div class="max-w-6xl mx-auto flex justify-between items-center">
      <h1 class="text-2xl font-bold">SatsEscrow</h1>
      <button
        on:click={handleLogout}
        class="bg-red-500 hover:bg-red-600 px-4 py-2 rounded transition"
      >
        Logout
      </button>
    </div>
  </nav>

  <div class="max-w-6xl mx-auto p-8">
    <div class="flex justify-between items-center mb-8">
      <h2 class="text-3xl font-bold text-gray-800">Your Escrows</h2>
      <button
        on:click={() => (showCreateForm = !showCreateForm)}
        class="bg-green-600 text-white px-6 py-2 rounded-lg hover:bg-green-700 transition font-semibold"
      >
        {showCreateForm ? 'Cancel' : 'Create New Escrow'}
      </button>
    </div>

    {#if showCreateForm}
      <CreateEscrowForm on:created={() => (showCreateForm = false)} />
    {/if}

    {#if $escrow.error}
      <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
        {$escrow.error}
      </div>
    {/if}

    {#if $escrow.loading}
      <div class="text-center py-12">
        <p class="text-gray-600">Loading escrows...</p>
      </div>
    {:else if $escrow.escrows.length === 0}
      <div class="bg-white rounded-lg shadow p-8 text-center">
        <p class="text-gray-600 mb-4">No escrows yet. Create one to get started!</p>
      </div>
    {:else}
      <div class="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
        {#each $escrow.escrows as escrowItem (escrowItem.id)}
          <EscrowCard {escrowItem} />
        {/each}
      </div>
    {/if}
  </div>
</div>