<script>
  import { createEventDispatcher } from 'svelte';

  const dispatch = createEventDispatcher();

  const generateUUID = () =>
    'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, (c) => {
      const r = (Math.random() * 16) | 0;
      return (c === 'x' ? r : (r & 0x3) | 0x8).toString(16);
    });

  const handleTryDemo = () => {
    const uuid = generateUUID();
    dispatch('tryDemo', { uuid });
  };

  const steps = [
    { emoji: '📝', label: 'Create', desc: 'Buyer creates an escrow contract' },
    { emoji: '💰', label: 'Fund', desc: 'Bitcoin is deposited into escrow' },
    { emoji: '📦', label: 'Deliver', desc: 'Seller delivers goods or services' },
    { emoji: '✅', label: 'Confirm', desc: 'Buyer confirms and funds release' },
  ];
</script>

<div class="min-h-screen bg-gradient-to-br from-blue-600 to-blue-800 text-white">
  <!-- Hero -->
  <div class="max-w-4xl mx-auto px-6 pt-20 pb-16 text-center">
    <h1 class="text-5xl font-bold mb-4">SatsEscrow</h1>
    <p class="text-xl text-blue-100 mb-4">Secure Bitcoin escrow for peer-to-peer transactions</p>
    <p class="text-blue-200 max-w-2xl mx-auto mb-10">
      Trade with confidence. SatsEscrow holds Bitcoin in a secure escrow until both parties
      fulfill their obligations — no trust required.
    </p>

    <div class="flex justify-center gap-4">
      <button
        on:click={handleTryDemo}
        class="bg-green-500 hover:bg-green-600 text-white px-8 py-3 rounded-lg text-lg font-semibold transition shadow-lg"
      >
        Try Demo
      </button>
      <button
        on:click={() => dispatch('goToLogin')}
        class="bg-white/20 hover:bg-white/30 text-white px-8 py-3 rounded-lg text-lg font-semibold transition border border-white/30"
      >
        Login
      </button>
    </div>
  </div>

  <!-- Escrow Flow Steps -->
  <div class="max-w-4xl mx-auto px-6 pb-16">
    <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
      {#each steps as step, i}
        <div class="bg-white/10 rounded-lg p-6 text-center backdrop-blur-sm">
          <div class="text-4xl mb-3">{step.emoji}</div>
          <div class="font-bold text-lg mb-1">{step.label}</div>
          <p class="text-blue-200 text-sm">{step.desc}</p>
          {#if i < steps.length - 1}
            <div class="hidden md:block absolute right-0 top-1/2 -translate-y-1/2 text-blue-300 text-2xl">→</div>
          {/if}
        </div>
      {/each}
    </div>
  </div>

  <!-- How It Works -->
  <div class="bg-white/10 backdrop-blur-sm">
    <div class="max-w-4xl mx-auto px-6 py-16">
      <h2 class="text-3xl font-bold text-center mb-10">How It Works</h2>

      <div class="space-y-6 max-w-2xl mx-auto">
        <div class="flex gap-4 items-start">
          <span class="bg-blue-500 text-white w-8 h-8 rounded-full flex items-center justify-center font-bold shrink-0">1</span>
          <div>
            <h3 class="font-semibold text-lg">Buyer Creates Escrow</h3>
            <p class="text-blue-200 text-sm">Set the amount in satoshis, describe the deal, and specify the seller.</p>
          </div>
        </div>

        <div class="flex gap-4 items-start">
          <span class="bg-blue-500 text-white w-8 h-8 rounded-full flex items-center justify-center font-bold shrink-0">2</span>
          <div>
            <h3 class="font-semibold text-lg">Deposit Bitcoin</h3>
            <p class="text-blue-200 text-sm">The buyer funds the escrow with Bitcoin. Funds are held securely until the deal completes.</p>
          </div>
        </div>

        <div class="flex gap-4 items-start">
          <span class="bg-blue-500 text-white w-8 h-8 rounded-full flex items-center justify-center font-bold shrink-0">3</span>
          <div>
            <h3 class="font-semibold text-lg">Seller Delivers</h3>
            <p class="text-blue-200 text-sm">The seller provides the agreed goods or services and marks the delivery as complete.</p>
          </div>
        </div>

        <div class="flex gap-4 items-start">
          <span class="bg-blue-500 text-white w-8 h-8 rounded-full flex items-center justify-center font-bold shrink-0">4</span>
          <div>
            <h3 class="font-semibold text-lg">Buyer Confirms & Funds Release</h3>
            <p class="text-blue-200 text-sm">The buyer confirms receipt, and the escrowed Bitcoin is released to the seller automatically.</p>
          </div>
        </div>
      </div>
    </div>
  </div>

  <!-- Footer CTA -->
  <div class="max-w-4xl mx-auto px-6 py-12 text-center">
    <p class="text-blue-200 mb-4">Ready to get started?</p>
    <button
      on:click={handleTryDemo}
      class="bg-green-500 hover:bg-green-600 text-white px-8 py-3 rounded-lg text-lg font-semibold transition shadow-lg"
    >
      Try Demo
    </button>
  </div>
</div>
