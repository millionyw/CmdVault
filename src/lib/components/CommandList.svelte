<script lang="ts">
  import { filteredCommands, commands } from '../stores/commands';
  import CommandItem from './CommandItem.svelte';
  import EmptyState from './EmptyState.svelte';

  let selectedIndex = $state(0);

  $effect(() => {
    // Reset selection when filtered commands change
    $filteredCommands;
    selectedIndex = 0;
  });

  function handleKeydown(event: KeyboardEvent) {
    const count = $filteredCommands.length;

    if (event.key === 'ArrowDown') {
      event.preventDefault();
      selectedIndex = Math.min(selectedIndex + 1, count - 1);
    } else if (event.key === 'ArrowUp') {
      event.preventDefault();
      selectedIndex = Math.max(selectedIndex - 1, 0);
    } else if (event.key === 'Enter' && count > 0) {
      event.preventDefault();
      const selected = $filteredCommands[selectedIndex];
      if (selected) {
        handleCopy(selected.id);
      }
    }
  }

  async function handleCopy(id: string) {
    const { commands } = await import('../stores/commands');
    await commands.copy(id);
  }

  function handleCreateSample() {
    commands.create('Sample Command', 'echo "Hello, World!"', 'A sample command to get started');
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="command-list" role="listbox" aria-label="Command list">
  {#if $filteredCommands.length === 0}
    <EmptyState oncreate={handleCreateSample} />
  {:else}
    {#each $filteredCommands as command, index (command.id)}
      <CommandItem
        {command}
        selected={index === selectedIndex}
        onclick={() => handleCopy(command.id)}
      />
    {/each}
  {/if}
</div>

<style>
  .command-list {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    overflow-y: auto;
    max-height: calc(100vh - 200px);
    padding-right: 0.25rem;
  }

  .command-list::-webkit-scrollbar {
    width: 6px;
  }

  .command-list::-webkit-scrollbar-track {
    background: transparent;
  }

  .command-list::-webkit-scrollbar-thumb {
    background: var(--scrollbar-thumb, #444);
    border-radius: 3px;
  }

  .command-list::-webkit-scrollbar-thumb:hover {
    background: var(--scrollbar-thumb-hover, #555);
  }
</style>