<script>
  import { auth } from './stores/auth';
  import { router, navigate } from './stores/router';
  import LoginForm from './components/LoginForm.svelte';
  import EscrowList from './components/EscrowList.svelte';
  import EscrowDetail from './components/EscrowDetail.svelte';
  import LandingPage from './components/LandingPage.svelte';
  import { escrow } from './stores/escrow';

  // Auto-redirect to dashboard when authenticated and on landing/login
  $: if ($auth.isAuthenticated && ($router.path === '/' || $router.path === '/login')) {
    navigate('/dashboard');
  }

  // Redirect to landing if not authenticated and trying to access protected routes
  $: if (!$auth.isAuthenticated && $router.path !== '/' && $router.path !== '/login') {
    navigate('/');
  }

  function handleTryDemo(e) {
    auth.login(e.detail.uuid);
    navigate('/dashboard');
  }

  function handleGoToLogin() {
    navigate('/login');
  }

  async function handleSelectEscrow(e) {
    navigate(`/escrow/${e.detail.id}`);
  }

  function handleBack() {
    navigate('/dashboard');
  }

  // Find escrow for detail view
  let selectedEscrow = null;
  $: if ($router.path === '/escrow/:id' && $router.params.id) {
    selectedEscrow = $escrow.escrows.find(e => e.id === $router.params.id) || null;
    // If not in store, try fetching
    if (!selectedEscrow && !$escrow.loading) {
      escrow.fetch();
    }
  }
</script>

<main class="bg-gray-950 text-white min-h-screen">
  {#if !$auth.isAuthenticated}
    {#if $router.path === '/login'}
      <LoginForm />
    {:else}
      <LandingPage on:tryDemo={handleTryDemo} on:goToLogin={handleGoToLogin} />
    {/if}
  {:else if $router.path === '/escrow/:id' && selectedEscrow}
    <EscrowDetail escrowItem={selectedEscrow} on:back={handleBack} />
  {:else}
    <EscrowList on:selectEscrow={handleSelectEscrow} />
  {/if}
</main>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    font-family: 'Inter', system-ui, -apple-system, sans-serif;
  }
</style>