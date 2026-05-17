<script>
  import { auth } from '../stores/auth';
  import { v4 as uuidv4 } from 'uuid';

  let userId = '';
  let error = '';

  const handleLogin = async () => {
    if (!userId.trim()) {
      error = 'Please enter a user ID or generate a new one';
      return;
    }
    try {
      auth.login(userId, userId);
      error = '';
    } catch (e) {
      error = 'Login failed';
    }
  };

  const generateNewId = () => {
    userId = uuidv4();
  };
</script>

<div class="min-h-screen flex items-center justify-center bg-gray-50 py-12 px-4 sm:px-6 lg:px-8">
  <div class="max-w-md w-full space-y-8">
    <div>
      <h2 class="mt-6 text-center text-3xl font-extrabold text-gray-900">SatsEscrow</h2>
      <p class="mt-2 text-center text-sm text-gray-600">Secure Bitcoin Escrow Service</p>
    </div>
    <form class="mt-8 space-y-6" on:submit|preventDefault={handleLogin}>
      <div class="rounded-md shadow-sm -space-y-px">
        <label for="userId" class="sr-only">User ID</label>
        <input
          id="userId"
          type="text"
          placeholder="Enter your User ID or generate a new one"
          bind:value={userId}
          class="appearance-none rounded-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-md focus:outline-none focus:ring-blue-500 focus:border-blue-500 focus:z-10 sm:text-sm"
        />
      </div>

      {#if error}
        <div class="rounded-md bg-red-50 p-4">
          <p class="text-sm font-medium text-red-800">{error}</p>
        </div>
      {/if}

      <div class="flex gap-4">
        <button
          type="submit"
          class="group relative w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
        >
          Login
        </button>
        <button
          type="button"
          on:click={generateNewId}
          class="group relative w-full flex justify-center py-2 px-4 border border-gray-300 text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
        >
          Generate ID
        </button>
      </div>
    </form>
  </div>
</div>