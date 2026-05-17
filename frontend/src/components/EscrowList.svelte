<script>
  import { onMount } from 'svelte';
  import { escrowStore } from '../stores/escrow';
  import EscrowCard from './EscrowCard.svelte';
  import CreateEscrowForm from './CreateEscrowForm.svelte';

  let showCreateForm = false;

  onMount(() => {
    escrowStore.fetch();
  });
</script>

<div class="max-w-6xl mx-auto px-4 py-8">
  <div class="flex justify-between items-center mb-8">
    <h1 class="text-3xl font-bold text-gray-900">My Escrows</h1>
    <button
      on:click={() => (showCreateForm = !showCreateForm)}
      class="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500"
    >
      {showCreateForm ? 'Cancel' : 'Create New'}
    </button>
  </div>

  {#if showCreateForm}
    <div class="mb-8">
      <CreateEscrowForm
        onCreated={() => {
          showCreateForm = false;
          escrowStore.fetch();
        }}
      />
    </div>
  {/if}

  {#if $escrowStore.loading}
    <div class="text-center py-12">
      <p class="text-gray-600">Loading escrows...</p>
    </div>
  {:else if $escrowStore.error}
    <div class="rounded-md bg-red-50 p-4">
      <p class="text-sm font-medium text-red-800">{$escrowStore.error}</p>
    </div>
  {:else if $escrowStore.escrows.length === 0}
    <div class="text-center py-12">
      <p class="text-gray-600">No escrows yet. Create one to get started!</p>
    </div>
  {:else}
    <div class="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
      {#each $escrowStore.escrows as escrow (escrow.id)}
        <EscrowCard {escrow} />
      {/each}
    </div>
  {/if}
</div>