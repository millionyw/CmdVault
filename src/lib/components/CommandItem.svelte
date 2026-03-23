<script lang="ts">
  import type { Command } from '../stores/commands';

  interface Props {
    command: Command;
    selected?: boolean;
    onclick?: () => void;
    onmouseenter?: () => void;
  }

  let { command, selected = false, onclick, onmouseenter }: Props = $props();

  function formatPreview(content: string, maxLength: number = 60): string {
    const trimmed = content.trim().replace(/\n/g, ' ');
    if (trimmed.length <= maxLength) return trimmed;
    return trimmed.substring(0, maxLength) + '...';
  }
</script>

<div
  class="command-item"
  class:selected
  role="button"
  tabindex="0"
  {onclick}
  {onmouseenter}
>
  <div class="item-content">
    <div class="item-header">
      <span class="name">{command.name}</span>
      <span class="usage-count">{command.usage_count}x</span>
    </div>
    <div class="preview">{formatPreview(command.content)}</div>
    {#if command.description}
      <div class="description">{command.description}</div>
    {/if}
  </div>
</div>

<style>
  .command-item {
    display: flex;
    align-items: center;
    padding: 0.75rem 1rem;
    background: var(--bg-secondary, #2a2a2a);
    border-radius: 8px;
    cursor: pointer;
    transition: background-color 0.15s ease;
    position: relative;
    border-left: 3px solid transparent;
    margin-bottom: 0.5rem;
  }

  .command-item:hover {
    background: var(--bg-hover, #333);
  }

  .command-item.selected {
    background: var(--bg-hover, #333);
    border-left-color: var(--primary, #3b82f6);
  }

  .command-item:focus {
    outline: 2px solid var(--primary, #3b82f6);
    outline-offset: 2px;
  }

  .item-content {
    flex: 1;
    min-width: 0;
  }

  .item-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.25rem;
  }

  .name {
    font-weight: 600;
    color: var(--text-primary, #fff);
    font-size: 0.95rem;
  }

  .usage-count {
    font-size: 0.75rem;
    color: var(--text-secondary, #888);
    background: var(--bg-tertiary, #1a1a1a);
    padding: 0.15rem 0.4rem;
    border-radius: 4px;
  }

  .preview {
    font-size: 0.85rem;
    color: var(--text-secondary, #aaa);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .description {
    font-size: 0.75rem;
    color: var(--text-muted, #666);
    margin-top: 0.25rem;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
</style>