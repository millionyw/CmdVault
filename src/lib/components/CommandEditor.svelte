<script lang="ts">
  import { commands } from '../stores/commands';
  import type { Command } from '../stores/commands';

  interface Props {
    open: boolean;
    command?: Command;
    onclose: () => void;
  }

  let { open, command = undefined, onclose }: Props = $props();

  let name = $state('');
  let content = $state('');
  let description = $state('');
  let saving = $state(false);

  $effect(() => {
    if (open) {
      if (command) {
        name = command.name;
        content = command.content;
        description = command.description || '';
      } else {
        name = '';
        content = '';
        description = '';
      }
    }
  });

  const isEditing = $derived(!!command);

  async function handleSave() {
    if (!name.trim() || !content.trim()) return;

    saving = true;
    try {
      if (isEditing && command) {
        await commands.update(command.id, name.trim(), content.trim(), description.trim() || undefined);
      } else {
        await commands.create(name.trim(), content.trim(), description.trim() || undefined);
      }
      onclose();
    } finally {
      saving = false;
    }
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Escape') {
      onclose();
    } else if (event.key === 'Enter' && (event.ctrlKey || event.metaKey)) {
      handleSave();
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if open}
  <div class="modal-overlay" onclick={onclose} role="dialog" aria-modal="true">
    <div class="modal" onclick={(e) => e.stopPropagation()}>
      <div class="modal-header">
        <h2>{isEditing ? '编辑命令' : '新建命令'}</h2>
        <button class="close-btn" onclick={onclose} aria-label="关闭">×</button>
      </div>

      <div class="modal-body">
        <div class="form-group">
          <label for="name">名称 *</label>
          <input
            id="name"
            type="text"
            bind:value={name}
            placeholder="命令名称"
          />
        </div>

        <div class="form-group">
          <label for="content">内容 *</label>
          <textarea
            id="content"
            bind:value={content}
            placeholder="命令内容"
            rows="5"
          ></textarea>
        </div>

        <div class="form-group">
          <label for="description">描述</label>
          <input
            id="description"
            type="text"
            bind:value={description}
            placeholder="可选描述"
          />
        </div>
      </div>

      <div class="modal-footer">
        <button class="btn btn-secondary" onclick={onclose}>取消</button>
        <button
          class="btn btn-primary"
          onclick={handleSave}
          disabled={saving || !name.trim() || !content.trim()}
        >
          {saving ? '保存中...' : '保存'}
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.6);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    padding: 1rem;
  }

  .modal {
    background: var(--bg-primary, #1a1a1a);
    border-radius: 12px;
    width: 100%;
    max-width: 480px;
    max-height: 90vh;
    overflow: hidden;
    display: flex;
    flex-direction: column;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4);
  }

  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem 1.25rem;
    border-bottom: 1px solid var(--border-color, #333);
  }

  .modal-header h2 {
    margin: 0;
    font-size: 1.1rem;
    font-weight: 600;
    color: var(--text-primary, #fff);
  }

  .close-btn {
    background: transparent;
    border: none;
    font-size: 1.5rem;
    color: var(--text-secondary, #888);
    cursor: pointer;
    padding: 0;
    line-height: 1;
    width: 2rem;
    height: 2rem;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 4px;
  }

  .close-btn:hover {
    background: var(--bg-hover, #333);
    color: var(--text-primary, #fff);
  }

  .modal-body {
    padding: 1.25rem;
    overflow-y: auto;
  }

  .form-group {
    margin-bottom: 1rem;
  }

  .form-group:last-child {
    margin-bottom: 0;
  }

  label {
    display: block;
    font-size: 0.85rem;
    font-weight: 500;
    color: var(--text-secondary, #aaa);
    margin-bottom: 0.5rem;
  }

  input[type="text"],
  textarea {
    width: 100%;
    padding: 0.6rem 0.8rem;
    background: var(--bg-secondary, #2a2a2a);
    border: 1px solid var(--border-color, #333);
    border-radius: 6px;
    color: var(--text-primary, #fff);
    font-size: 0.9rem;
    font-family: inherit;
    resize: vertical;
  }

  input[type="text"]:focus,
  textarea:focus {
    outline: none;
    border-color: var(--primary, #3b82f6);
  }

  input[type="text"]::placeholder,
  textarea::placeholder {
    color: var(--text-muted, #666);
  }

  .modal-footer {
    display: flex;
    justify-content: flex-end;
    gap: 0.75rem;
    padding: 1rem 1.25rem;
    border-top: 1px solid var(--border-color, #333);
  }

  .btn {
    padding: 0.6rem 1.25rem;
    border-radius: 6px;
    font-size: 0.9rem;
    font-weight: 500;
    cursor: pointer;
    border: none;
    transition: background-color 0.15s ease;
  }

  .btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn-primary {
    background: var(--primary, #3b82f6);
    color: white;
  }

  .btn-primary:hover:not(:disabled) {
    background: var(--primary-hover, #2563eb);
  }

  .btn-secondary {
    background: var(--bg-secondary, #2a2a2a);
    color: var(--text-primary, #fff);
    border: 1px solid var(--border-color, #333);
  }

  .btn-secondary:hover {
    background: var(--bg-hover, #333);
  }
</style>