<script>
  import { onMount, createEventDispatcher } from 'svelte';
  import { escrow } from '../stores/escrow';
  import { auth } from '../stores/auth';
  import { navigate } from '../stores/router';
  import EscrowCard from './EscrowCard.svelte';
  import CreateEscrowForm from './CreateEscrowForm.svelte';
  import ThemeToggle from './ThemeToggle.svelte';

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

<div class="min-h-screen bg-gray-100 dark:bg-gray-900 transition-colors">
  <nav class="bg-blue-600 dark:bg-blue-800 text-white p-4 shadow-lg">
    <div class="max-w-6xl mx-auto flex justify-between items-center">
      <h1 class="text-2xl font-bold">₿ SatsEscrow</h1>
      <div class="flex items-center gap-3">
        <ThemeToggle />
        <button
          on:click={handleLogout}
          class="bg-red-500 hover:bg-red-600 px-4 py-2 rounded transition"
        >
          Logout
        </button>
      </div>
    </div>
  </nav>

  <div class="max-w-6xl mx-auto p-8">
    <div class="flex justify-between items-center mb-8">
      <h2 class="text-3xl font-bold text-gray-800 dark:text-gray-100">Your Escrows</h2>
      <button
        on:click={() => (showCreateForm = !showCreateForm)}
        class="bg-green-600 text-white px-6 py-2 rounded-lg hover:bg-green-700 transition font-semibold"
      >
        {showCreateForm ? 'Cancel' : 'Create New Escrow'}
      </button>
    </div>

    {#if showCreateForm}
      <CreateEscrowForm on:created={() => (showCreateForm = false)} on:close={() => (showCreateForm = false)} />
    {/if}

    {#if $escrow.error}
      <div class="bg-red-100 dark:bg-red-900/30 border border-red-400 text-red-700 dark:text-red-400 px-4 py-3 rounded mb-4">
        {$escrow.error}
      </div>
    {/if}

    {#if $escrow.loading}
      <div class="text-center py-12">
        <div class="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
          {#each [1, 2, 3] as _}
            <div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6 animate-pulse">
              <div class="flex justify-between items-start mb-4">
                <div class="h-6 bg-gray-200 dark:bg-gray-700 rounded w-32"></div>
                <div class="h-6 bg-gray-200 dark:bg-gray-700 rounded-full w-20"></div>
              </div>
              <div class="space-y-3">
                <div class="h-4 bg-gray-200 dark:bg-gray-700 rounded w-full"></div>
                <div class="h-4 bg-gray-200 dark:bg-gray-700 rounded w-3/4"></div>
                <div class="h-4 bg-gray-200 dark:bg-gray-700 rounded w-1/2"></div>
                <div class="h-4 bg-gray-200 dark:bg-gray-700 rounded w-2/3"></div>
              </div>
              <div class="mt-6 h-10 bg-gray-200 dark:bg-gray-700 rounded"></div>
            </div>
          {/each}
        </div>
      </div>
    {:else if $escrow.escrows.length === 0}
      <div class="bg-white dark:bg-gray-800 rounded-lg shadow p-12 text-center">
        <p class="text-5xl mb-4">📦</p>
        <h3 class="text-xl font-semibold text-gray-800 dark:text-gray-100 mb-2">No escrows yet</h3>
        <p class="text-gray-600 dark:text-gray-400 mb-6">Create your first escrow to get started with secure Bitcoin transactions.</p>
        <button
          on:click={() => (showCreateForm = true)}
          class="bg-green-600 text-white px-6 py-2 rounded-lg hover:bg-green-700 transition font-semibold"
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