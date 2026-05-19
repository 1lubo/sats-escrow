<script>
  import { onMount } from 'svelte';

  let dark = false;

  onMount(() => {
    const stored = localStorage.getItem('theme');
    if (stored) {
      dark = stored === 'dark';
    } else {
      dark = window.matchMedia('(prefers-color-scheme: dark)').matches;
    }
    applyTheme();
  });

  function applyTheme() {
    if (dark) {
      document.documentElement.classList.add('dark');
    } else {
      document.documentElement.classList.remove('dark');
    }
    localStorage.setItem('theme', dark ? 'dark' : 'light');
  }

  function toggle() {
    dark = !dark;
    applyTheme();
  }
</script>

<button
  on:click={toggle}
  aria-label="Toggle dark mode"
  class="p-2 rounded-lg hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors"
>
  {dark ? '☀️' : '🌙'}
</button>
