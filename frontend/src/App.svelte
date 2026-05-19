<script>
  import { auth } from './stores/auth';
  import LoginForm from './components/LoginForm.svelte';
  import EscrowList from './components/EscrowList.svelte';
</script>

<main>
  {#if $auth.isAuthenticated}
    <EscrowList />
  {:else}
    <LoginForm />
  {/if}
</main>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu,
      Cantarell, 'Fira Sans', 'Droid Sans', 'Helvetica Neue', sans-serif;
  }
</style>