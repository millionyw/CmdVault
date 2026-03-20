<script lang="ts">
  import { settings } from '../stores/settings';
  import { commands } from '../stores/commands';

  interface Props {
    open: boolean;
    onclose: () => void;
  }

  let { open, onclose }: Props = $props();

  async function handleExport() {
    try {
      const json = await commands.export();
      const blob = new Blob([json], { type: 'application/json' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = `commands-${new Date().toISOString().split('T')[0]}.json`;
      a.click();
      URL.revokeObjectURL(url);
    } catch (e) {
      console.error('Export failed:', e);
    }
  }

  async function handleImport() {
    const input = document.createElement('input');
    input.type = 'file';
    input.accept = '.json';
    input.onchange = async (e) => {
      const file = (e.target as HTMLInputElement).files?.[0];
      if (!file) return;

      const reader = new FileReader();
      reader.onload = async (event) => {
        try {
          const json = event.target?.result as string;
          const count = await commands.import(json);
          alert(`成功导入 ${count} 条命令`);
        } catch (e) {
          console.error('Import failed:', e);
          alert('导入失败，请检查文件格式');
        }
      };
      reader.readAsText(file);
    };
    input.click();
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Escape') {
      onclose();
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if open}
  <div class="modal-overlay" onclick={onclose} role="dialog" aria-modal="true">
    <div class="modal" onclick={(e) => e.stopPropagation()}>
      <div class="modal-header">
        <h2>设置</h2>
        <button class="close-btn" onclick={onclose} aria-label="关闭">×</button>
      </div>

      <div class="modal-body">
        <section class="settings-section">
          <h3>快捷键</h3>
          <div class="setting-item">
            <span>全局快捷键</span>
            <code class="shortcut-badge">{$settings.shortcut}</code>
          </div>
          <p class="hint">按下快捷键可快速打开应用</p>
        </section>

        <section class="settings-section">
          <h3>数据管理</h3>
          <div class="button-group">
            <button class="btn btn-secondary" onclick={handleExport}>
              <span class="btn-icon">&#x1F4E4;</span>
              导出
            </button>
            <button class="btn btn-secondary" onclick={handleImport}>
              <span class="btn-icon">&#x1F4E5;</span>
              导入
            </button>
          </div>
          <p class="hint">导出为 JSON 格式，可用于备份或迁移</p>
        </section>

        <section class="settings-section">
          <h3>GitHub 同步</h3>
          <div class="setting-item disabled">
            <span>GitHub Token</span>
            <input type="password" placeholder="未配置" disabled />
          </div>
          <div class="setting-item disabled">
            <span>Gist ID</span>
            <input type="text" placeholder="未配置" disabled />
          </div>
          <button class="btn btn-primary" disabled>
            开启同步 (即将推出)
          </button>
          <p class="hint">通过 GitHub Gist 同步命令到云端</p>
        </section>
      </div>

      <div class="modal-footer">
        <button class="btn btn-secondary" onclick={onclose}>关闭</button>
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

  .settings-section {
    margin-bottom: 1.5rem;
    padding-bottom: 1.5rem;
    border-bottom: 1px solid var(--border-color, #333);
  }

  .settings-section:last-child {
    margin-bottom: 0;
    padding-bottom: 0;
    border-bottom: none;
  }

  .settings-section h3 {
    margin: 0 0 1rem 0;
    font-size: 0.95rem;
    font-weight: 600;
    color: var(--text-primary, #fff);
  }

  .setting-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.75rem 0;
    color: var(--text-secondary, #aaa);
  }

  .setting-item.disabled {
    opacity: 0.5;
  }

  .setting-item input {
    padding: 0.4rem 0.6rem;
    background: var(--bg-secondary, #2a2a2a);
    border: 1px solid var(--border-color, #333);
    border-radius: 4px;
    color: var(--text-primary, #fff);
    font-size: 0.85rem;
    width: 150px;
    text-align: right;
  }

  .shortcut-badge {
    background: var(--bg-tertiary, #333);
    padding: 0.3rem 0.6rem;
    border-radius: 4px;
    font-size: 0.8rem;
    font-family: monospace;
  }

  .button-group {
    display: flex;
    gap: 0.75rem;
    margin-bottom: 0.75rem;
  }

  .btn {
    padding: 0.6rem 1rem;
    border-radius: 6px;
    font-size: 0.85rem;
    font-weight: 500;
    cursor: pointer;
    border: none;
    transition: background-color 0.15s ease;
    display: inline-flex;
    align-items: center;
    gap: 0.4rem;
  }

  .btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn-icon {
    font-size: 1rem;
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

  .btn-secondary:hover:not(:disabled) {
    background: var(--bg-hover, #333);
  }

  .hint {
    font-size: 0.75rem;
    color: var(--text-muted, #666);
    margin: 0.5rem 0 0 0;
  }

  .modal-footer {
    display: flex;
    justify-content: flex-end;
    gap: 0.75rem;
    padding: 1rem 1.25rem;
    border-top: 1px solid var(--border-color, #333);
  }
</style>