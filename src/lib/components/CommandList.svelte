<script lang="ts">
  import { filteredCommands, commands } from '../stores/commands';
  import CommandItem from './CommandItem.svelte';
  import EmptyState from './EmptyState.svelte';

  interface Props {
    selectedIndex?: number;
    onSelectedIndexChange?: (index: number) => void;
    onCopy?: (id: string) => void;
  }

  let { selectedIndex = 0, onSelectedIndexChange, onCopy }: Props = $props();

  $effect(() => {
    // Reset selection when filtered commands change
    $filteredCommands;
    onSelectedIndexChange?.(0);
  });

  function handleKeydown(event: KeyboardEvent) {
    const count = $filteredCommands.length;

    if (event.key === 'ArrowDown') {
      event.preventDefault();
      const newIndex = Math.min(selectedIndex + 1, count - 1);
      onSelectedIndexChange?.(newIndex);
    } else if (event.key === 'ArrowUp') {
      event.preventDefault();
      const newIndex = Math.max(selectedIndex - 1, 0);
      onSelectedIndexChange?.(newIndex);
    } else if (event.key === 'Enter' && count > 0) {
      event.preventDefault();
      const selected = $filteredCommands[selectedIndex];
      if (selected) {
        onCopy?.(selected.id);
      }
    }
  }

  function handleItemClick(index: number, id: string) {
    onSelectedIndexChange?.(index);
    onCopy?.(id);
  }

  function handleItemHover(index: number) {
    onSelectedIndexChange?.(index);
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
        onclick={() => handleItemClick(index, command.id)}
        onmouseenter={() => handleItemHover(index)}
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