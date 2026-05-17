<script>
  import { auth } from '../stores/auth';

  let uuid = '';
  let error = '';

  const handleLogin = () => {
    if (!uuid.trim()) {
      error = 'Please enter a valid UUID';
      return;
    }
    auth.login(uuid);
  };
</script>

<div class="min-h-screen bg-gradient-to-br from-blue-600 to-blue-800 flex items-center justify-center">
  <div class="bg-white rounded-lg shadow-lg p-8 w-full max-w-md">
    <h1 class="text-3xl font-bold text-gray-800 mb-6 text-center">SatsEscrow</h1>
    <p class="text-gray-600 text-center mb-8">Enter your UUID to continue</p>
    
    <div class="space-y-4">
      <input
        type="text"
        placeholder="Your UUID"
        bind:value={uuid}
        class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
      />
      {#if error}
        <p class="text-red-600 text-sm">{error}</p>
      {/if}
      <button
        on:click={handleLogin}
        class="w-full bg-blue-600 text-white py-2 rounded-lg hover:bg-blue-700 transition font-semibold"
      >
        Login
      </button>
    </div>
  </div>
</div>