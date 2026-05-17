<script>
  import { auth } from './stores/auth';
  import LoginForm from './components/LoginForm.svelte';
  import EscrowList from './components/EscrowList.svelte';

  let currentAuth;

  auth.subscribe((value) => {
    currentAuth = value;
  });
</script>

<div class="bg-gray-100 min-h-screen">
  {#if !currentAuth.token}
    <LoginForm />
  {:else}
    <nav class="bg-white shadow-sm">
      <div class="max-w-6xl mx-auto px-4 py-4 flex justify-between items-center">
        <h1 class="text-xl font-bold text-gray-900">SatsEscrow</h1>
        <button
          on:click={() => auth.logout()}
          class="px-4 py-2 text-gray-700 hover:text-gray-900 focus:outline-none"
        >
          Logout
        </button>
      </div>
    </nav>
    <EscrowList />
  {/if}
</div>

<style global>
  * {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
  }

  body {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', 'Oxygen', 'Ubuntu',
      'Cantarell', 'Fira Sans', 'Droid Sans', 'Helvetica Neue', sans-serif;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
  }
</style>