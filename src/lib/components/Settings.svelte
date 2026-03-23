<script lang="ts">
  import { settings } from '../stores/settings';
  import { commands } from '../stores/commands';
  import ShortcutRecorder from './ShortcutRecorder.svelte';
  import { exportToFile, importFromFile } from '../utils/fileOperations';

  interface Props {
    open: boolean;
    onclose: () => void;
  }

  let { open, onclose }: Props = $props();

  // Shortcut configuration
  const shortcutItems = [
    { key: 'global', label: '全局快捷键（打开主窗口）' },
    { key: 'newCommand', label: '新建命令' },
    { key: 'editSelected', label: '编辑选中' },
    { key: 'deleteSelected', label: '删除选中' },
    { key: 'openSettings', label: '打开设置' },
    { key: 'export', label: '导出' },
    { key: 'import', label: '导入' },
    { key: 'close', label: '关闭窗口/弹窗' },
  ] as const;

  async function handleShortcutChange(key: string, value: string) {
    try {
      await settings.updateShortcut(key, value);
    } catch (e) {
      alert('更新快捷键失败');
    }
  }

  async function handleResetShortcuts() {
    if (confirm('确定恢复默认快捷键设置？')) {
      try {
        await settings.resetShortcuts();
      } catch (e) {
        alert('恢复默认设置失败');
      }
    }
  }

  async function handleExport() {
    await exportToFile(
      (msg) => alert(msg),
      (msg) => alert(msg)
    );
  }

  async function handleImport() {
    await importFromFile(
      (msg) => alert(msg),
      (msg) => alert(msg)
    );
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
          <div class="shortcut-list">
            {#each shortcutItems as item}
              <div class="shortcut-item">
                <span class="shortcut-label">{item.label}</span>
                <ShortcutRecorder
                  value={$settings.shortcuts[item.key]}
                  disabled={item.key === 'global'}
                  onchange={(val: string) => handleShortcutChange(item.key, val)}
                />
              </div>
            {/each}
          </div>
          <button class="btn btn-secondary" onclick={handleResetShortcuts}>
            恢复默认
          </button>
          <p class="hint">全局快捷键需要在应用重启后生效</p>
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
    background: rgba(0, 0, 0, 0.85);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    padding: 1rem;
  }

  .modal {
    background: #1a1a1a;
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
    border-bottom: 1px solid #333;
    background: #1a1a1a;
  }

  .modal-header h2 {
    margin: 0;
    font-size: 1.1rem;
    font-weight: 600;
    color: #fff;
  }

  .close-btn {
    background: transparent;
    border: none;
    font-size: 1.5rem;
    color: #888;
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
    background: #333;
    color: #fff;
  }

  .modal-body {
    padding: 1.25rem;
    overflow-y: auto;
    background: #1a1a1a;
  }

  .settings-section {
    margin-bottom: 1.5rem;
    padding-bottom: 1.5rem;
    border-bottom: 1px solid #333;
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
    color: #fff;
  }

  .setting-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.75rem 0;
    color: #aaa;
  }

  .setting-item.disabled {
    opacity: 0.5;
  }

  .shortcut-list {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    margin-bottom: 1rem;
  }

  .shortcut-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.5rem 0;
    border-bottom: 1px solid #2a2a2a;
  }

  .shortcut-label {
    color: #aaa;
    font-size: 0.9rem;
  }

  .setting-item input {
    padding: 0.4rem 0.6rem;
    background: #2a2a2a;
    border: 1px solid #333;
    border-radius: 4px;
    color: #fff;
    font-size: 0.85rem;
    width: 150px;
    text-align: right;
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
    background: #3b82f6;
    color: white;
  }

  .btn-primary:hover:not(:disabled) {
    background: #2563eb;
  }

  .btn-secondary {
    background: #2a2a2a;
    color: #fff;
    border: 1px solid #333;
  }

  .btn-secondary:hover:not(:disabled) {
    background: #333;
  }

  .hint {
    font-size: 0.75rem;
    color: #666;
    margin: 0.5rem 0 0 0;
  }

  .modal-footer {
    display: flex;
    justify-content: flex-end;
    gap: 0.75rem;
    padding: 1rem 1.25rem;
    border-top: 1px solid #333;
    background: #1a1a1a;
  }
</style>