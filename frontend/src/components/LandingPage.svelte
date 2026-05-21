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

  const apiUrl = import.meta.env.VITE_API_URL || 'http://localhost:8000';
  const swaggerUrl = `${apiUrl}/swagger-ui`;
</script>

<div class="landing-bg min-h-screen bg-gray-950 text-white relative">
  <!-- Top Nav -->
  <nav class="fixed top-0 inset-x-0 z-50 bg-gray-950/80 backdrop-blur-md border-b border-white/5">
    <div class="max-w-5xl mx-auto px-6 h-16 flex items-center justify-between">
      <span class="text-xl font-bold tracking-tight">₿ SatsEscrow</span>
      <div class="flex items-center gap-3">
        <button on:click={handleTryDemo}
          class="bg-orange-500 hover:bg-orange-600 text-white px-5 py-2 rounded-lg text-sm font-semibold transition shadow-lg shadow-orange-500/20">
          Try Demo
        </button>
        <button on:click={() => dispatch('goToLogin')}
          class="text-gray-300 hover:text-white px-4 py-2 rounded-lg text-sm font-medium transition border border-white/10 hover:border-white/20">
          Login
        </button>
      </div>
    </div>
  </nav>

  <!-- Hero -->
  <div class="max-w-3xl mx-auto px-6 pt-36 pb-24 text-center relative z-10">
    <h1 class="text-6xl md:text-7xl font-bold tracking-tight mb-6 leading-[1.1]">SatsEscrow</h1>
    <p class="text-xl md:text-2xl text-gray-400 mb-4 font-light">Secure Bitcoin escrow for peer-to-peer transactions</p>
    <p class="text-gray-500 max-w-xl mx-auto leading-relaxed">
      Trade with confidence. SatsEscrow holds Bitcoin in a secure escrow until both
      parties fulfill their obligations — no trust required.
    </p>
  </div>

  <!-- How It Works -->
  <div class="max-w-4xl mx-auto px-6 pb-20 relative z-10">
    <h2 class="text-sm font-semibold uppercase tracking-widest text-gray-500 text-center mb-10">How It Works</h2>
    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-5">
      {#each steps as step, i}
        <div class="bg-white/5 border border-white/5 rounded-xl p-6 text-center backdrop-blur-sm hover:bg-white/[0.08] transition">
          <div class="w-10 h-10 rounded-full bg-orange-500/20 text-orange-400 font-bold text-sm flex items-center justify-center mx-auto mb-4">{i + 1}</div>
          <div class="text-3xl mb-3">{step.emoji}</div>
          <h3 class="font-semibold text-white mb-1">{step.label}</h3>
          <p class="text-gray-500 text-sm leading-relaxed">{step.desc}</p>
        </div>
      {/each}
    </div>
  </div>

  <!-- Minimal Footer -->
  <footer class="border-t border-white/5 py-3 text-center relative z-10">
    <a href={swaggerUrl} target="_blank" rel="noopener noreferrer"
      class="text-gray-600 hover:text-gray-400 text-xs transition inline-flex items-center gap-1">
      📖 API Docs
    </a>
  </footer>
</div>

<style>
  .landing-bg::before {
    content: '';
    position: absolute;
    inset: 0;
    z-index: 0;
    opacity: 0.04;
    pointer-events: none;
    background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='120' height='120' viewBox='0 0 120 120'%3E%3Ctext x='60' y='68' text-anchor='middle' font-size='42' font-family='system-ui' fill='%23ffffff'%3E₿%3C/text%3E%3C/svg%3E");
    background-repeat: repeat;
    background-size: 120px 120px;
  }
</style>
