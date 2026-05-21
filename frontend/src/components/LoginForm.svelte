<script>
  import { auth } from '../stores/auth';

  let uuid = '';
  let error = '';

  const UUID_REGEX = /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i;

  const handleLogin = () => {
    if (!uuid.trim()) {
      error = 'Please enter a valid UUID';
      return;
    }
    if (!UUID_REGEX.test(uuid.trim())) {
      error = 'UUID must be in format xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx';
      return;
    }
    auth.login(uuid.trim());
  };
</script>

<div class="min-h-screen bg-gray-950 flex items-center justify-center">
  <div class="bg-white/5 border border-white/5 rounded-xl shadow-lg p-8 w-full max-w-md backdrop-blur-sm">
    <h1 class="text-3xl font-bold text-white mb-6 text-center">SatsEscrow</h1>
    <p class="text-gray-400 text-center mb-8">Enter your UUID to continue</p>

    <div class="space-y-4">
      <div class="relative group">
        <input
          type="text"
          placeholder="Your UUID"
          bind:value={uuid}
          on:keydown={(e) => e.key === 'Enter' && handleLogin()}
          aria-required="true"
          aria-invalid={!!error}
          title="Try: 550e8400-e29b-41d4-a716-446655440000"
          class="w-full px-4 py-2 bg-white/5 border border-white/10 rounded-lg text-white placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-orange-500 focus:border-transparent"
        />
        <div class="absolute left-0 right-0 mt-1 bg-gray-800 text-white text-xs rounded-md px-3 py-2 opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none z-10">
          Demo UUID: <span class="font-mono">550e8400-e29b-41d4-a716-446655440000</span>
        </div>
      </div>
      {#if error}
        <p class="text-red-400 text-sm" aria-live="assertive">{error}</p>
      {/if}
      <button
        on:click={handleLogin}
        class="w-full bg-orange-500 hover:bg-orange-600 text-white py-2 rounded-lg transition font-semibold shadow-lg shadow-orange-500/20"
      >
        Login
      </button>
    </div>
  </div>
</div>