<script lang="ts">
  import { onMount } from 'svelte';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { listen } from '@tauri-apps/api/event';
  import SearchInput from './lib/components/SearchInput.svelte';
  import CommandList from './lib/components/CommandList.svelte';
  import CommandEditor from './lib/components/CommandEditor.svelte';
  import Settings from './lib/components/Settings.svelte';
  import Toast from './lib/components/Toast.svelte';
  import { commands, filteredCommands } from './lib/stores/commands';
  import type { Command } from './lib/stores/commands';

  const appWindow = getCurrentWindow();

  let editorOpen = $state(false);
  let settingsOpen = $state(false);
  let editingCommand = $state<Command | undefined>(undefined);
  let toastMessage = $state('');
  let showToast = $state(false);
  let selectedIndex = $state(0);

  // Load commands on mount
  onMount(async () => {
    try {
      await commands.load();
    } catch (e) {
      console.error('Failed to load commands:', e);
    }

    // Listen for tray events
    const unlistenNewCommand = await listen('tray:new-command', () => {
      editingCommand = undefined;
      editorOpen = true;
    });

    const unlistenSettings = await listen('tray:settings', () => {
      settingsOpen = true;
    });

    return () => {
      unlistenNewCommand();
      unlistenSettings();
    };
  });

  // Handle window focus to reload commands
  $effect(() => {
    const unlisten = appWindow.onFocusChanged(({ payload: focused }) => {
      if (focused) {
        commands.load();
      }
    });

    return () => {
      unlisten.then(fn => fn());
    };
  });

  // Global keyboard shortcuts
  async function handleKeydown(event: KeyboardEvent) {
    // Don't handle shortcuts when modal is open
    if (editorOpen || settingsOpen) return;

    // Escape: hide window
    if (event.key === 'Escape') {
      event.preventDefault();
      hideWindow();
      return;
    }

    // Ctrl+N: open editor (new command)
    if (event.key === 'n' && (event.ctrlKey || event.metaKey)) {
      event.preventDefault();
      editingCommand = undefined;
      editorOpen = true;
      return;
    }

    // Ctrl+E: edit selected
    if (event.key === 'e' && (event.ctrlKey || event.metaKey)) {
      event.preventDefault();
      if ($filteredCommands.length > 0) {
        editingCommand = $filteredCommands[selectedIndex];
        editorOpen = true;
      }
      return;
    }

    // Ctrl+,: open settings
    if (event.key === ',' && (event.ctrlKey || event.metaKey)) {
      event.preventDefault();
      settingsOpen = true;
      return;
    }

    // Delete: delete selected command
    if (event.key === 'Delete') {
      event.preventDefault();
      if ($filteredCommands.length > 0) {
        const cmd = $filteredCommands[selectedIndex];
        if (confirm(`确定删除命令 "${cmd.name}"？`)) {
          try {
            await commands.delete(cmd.id);
            showToastMessage('已删除');
            // Auto-select next or previous
            if (selectedIndex >= $filteredCommands.length) {
              selectedIndex = Math.max(0, $filteredCommands.length - 1);
            }
          } catch (e) {
            console.error('Failed to delete:', e);
            showToastMessage('删除失败');
          }
        }
      }
      return;
    }
  }

  async function copyCommand(id: string) {
    try {
      await commands.copy(id);
      showToastMessage('已复制到剪贴板');
      // Hide window after copy
      setTimeout(() => hideWindow(), 500);
    } catch (e) {
      console.error('Failed to copy:', e);
      showToastMessage('复制失败');
    }
  }

  function showToastMessage(message: string) {
    toastMessage = message;
    showToast = true;
  }

  function handleToastClose() {
    showToast = false;
  }

  function openEditorNew() {
    editingCommand = undefined;
    editorOpen = true;
  }

  function closeEditor() {
    editorOpen = false;
    editingCommand = undefined;
  }

  async function hideWindow() {
    try {
      await appWindow.hide();
    } catch (e) {
      console.error('Failed to hide window:', e);
    }
  }

  async function startDragging(e: MouseEvent) {
    // Only allow left mouse button
    if (e.buttons !== 1) return;
    // Don't drag if clicking on interactive elements
    if ((e.target as HTMLElement).closest('button, input, a')) return;
    await appWindow.startDragging();
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="app-container">
  <header class="app-header" onmousedown={startDragging}>
    <h1>CommandRepo</h1>
  </header>

  <main class="app-main">
    <SearchInput />
    <div class="command-list-wrapper">
      <CommandList
        selectedIndex={selectedIndex}
        onSelectedIndexChange={(index) => selectedIndex = index}
        onCopy={copyCommand}
      />
    </div>
  </main>

  <footer class="app-footer">
    <div class="shortcuts">
      <span class="shortcut"><kbd>Ctrl</kbd>+<kbd>N</kbd> 新建</span>
      <span class="shortcut"><kbd>Ctrl</kbd>+<kbd>E</kbd> 编辑</span>
      <span class="shortcut"><kbd>Del</kbd> 删除</span>
      <span class="shortcut"><kbd>Ctrl</kbd>+<kbd>,</kbd> 设置</span>
      <span class="shortcut"><kbd>Esc</kbd> 隐藏</span>
    </div>
  </footer>
</div>

<CommandEditor open={editorOpen} command={editingCommand} onclose={closeEditor} />
<Settings open={settingsOpen} onclose={() => settingsOpen = false} />

{#if showToast}
  <Toast message={toastMessage} onclose={handleToastClose} />
{/if}

<style>
  .app-container {
    display: flex;
    flex-direction: column;
    height: 100vh;
    max-height: 100vh;
    overflow: hidden;
  }

  .app-header {
    padding: 0.75rem 1rem;
    border-bottom: 1px solid var(--border);
    flex-shrink: 0;
  }

  .app-header h1 {
    margin: 0;
    font-size: 1.1rem;
    font-weight: 600;
    color: var(--text-primary);
  }

  .app-main {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    padding: 0.75rem;
  }

  .command-list-wrapper {
    flex: 1;
    overflow: hidden;
  }

  .app-footer {
    padding: 0.5rem 1rem;
    border-top: 1px solid var(--border);
    background: rgba(42, 42, 42, 0.98);
    flex-shrink: 0;
  }

  .shortcuts {
    display: flex;
    gap: 1rem;
    justify-content: center;
    flex-wrap: wrap;
  }

  .shortcut {
    font-size: 0.75rem;
    color: rgba(255, 255, 255, 0.9);
  }

  .shortcut kbd {
    display: inline-block;
    padding: 0.15rem 0.35rem;
    font-size: 0.7rem;
    font-family: monospace;
    background: rgba(68, 68, 68, 0.9);
    border: 1px solid rgba(85, 85, 85, 0.8);
    border-radius: 3px;
    margin-right: 0.15rem;
    color: rgba(255, 255, 255, 0.95);
  }
</style>