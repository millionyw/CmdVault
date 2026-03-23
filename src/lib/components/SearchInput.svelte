<script lang="ts">
  import { onMount } from 'svelte';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { searchQuery } from '../stores/commands';

  let inputElement: HTMLInputElement;

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Escape') {
      searchQuery.set('');
    }
  }

  function focusInput() {
    inputElement?.focus();
  }

  onMount(() => {
    // Focus on mount
    focusInput();

    // Focus when window gains focus
    const appWindow = getCurrentWindow();
    const unlisten = appWindow.onFocusChanged(({ payload: focused }) => {
      if (focused) {
        focusInput();
      }
    });

    return () => {
      unlisten.then(fn => fn());
    };
  });
</script>

<div class="search-container">
  <span class="search-icon">&#x1F50D;</span>
  <input
    type="text"
    placeholder="Search commands..."
    bind:value={$searchQuery}
    bind:this={inputElement}
    onkeydown={handleKeydown}
  />
  {#if $searchQuery}
    <button class="clear-btn" onclick={() => searchQuery.set('')}>x</button>
  {/if}
</div>

<style>
  .search-container {
    display: flex;
    align-items: center;
    background: var(--bg-secondary, #2a2a2a);
    border-radius: 8px;
    padding: 0.5rem 1rem;
    margin-bottom: 1rem;
    border: 1px solid var(--border-color, #333);
  }

  .search-icon {
    margin-right: 0.5rem;
    font-size: 1rem;
  }

  input {
    flex: 1;
    background: transparent;
    border: none;
    outline: none;
    color: var(--text-primary, #fff);
    font-size: 0.95rem;
  }

  input::placeholder {
    color: var(--text-secondary, #888);
  }

  .clear-btn {
    background: transparent;
    border: none;
    color: var(--text-secondary, #888);
    cursor: pointer;
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
    font-size: 0.8rem;
  }

  .clear-btn:hover {
    background: var(--bg-hover, #333);
    color: var(--text-primary, #fff);
  }
</style>