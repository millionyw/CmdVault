<script lang="ts">
  import { onMount } from 'svelte';

  interface Props {
    message?: string;
    duration?: number;
    onclose?: () => void;
  }

  let { message = '\u{5DF2}\u{590D}\u{5236}\u{5230}\u{526A}\u{8D34}\u{677F}', duration = 1500, onclose }: Props = $props();

  let visible = $state(false);

  onMount(() => {
    visible = true;
    const timer = setTimeout(() => {
      visible = false;
      setTimeout(() => onclose?.(), 300);
    }, duration);

    return () => clearTimeout(timer);
  });
</script>

<div class="toast" class:visible>
  <span class="toast-icon">\u{2705}</span>
  <span class="toast-message">{message}</span>
</div>

<style>
  .toast {
    position: fixed;
    bottom: 2rem;
    left: 50%;
    transform: translateX(-50%) translateY(20px);
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.75rem 1.25rem;
    background: var(--bg-toast, #1a1a1a);
    border: 1px solid var(--border-color, #333);
    border-radius: 8px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
    opacity: 0;
    transition: opacity 0.3s ease, transform 0.3s ease;
    z-index: 1000;
  }

  .toast.visible {
    opacity: 1;
    transform: translateX(-50%) translateY(0);
  }

  .toast-icon {
    font-size: 1rem;
  }

  .toast-message {
    font-size: 0.9rem;
    color: var(--text-primary, #fff);
    white-space: nowrap;
  }
</style>