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

  // Sync state
  let token = $state('');
  let deviceName = $state('My Device');
  let manualGistId = $state('');
  let syncLoading = $state(false);

  // Load sync status on mount
  $effect(() => {
    if (open) {
      settings.getSyncStatus().then(status => {
        if (status.device_name) {
          deviceName = status.device_name;
        }
      });
    }
  });

  async function handleConnect() {
    if (!token.trim()) {
      alert('请输入 GitHub Token');
      return;
    }
    syncLoading = true;
    try {
      await settings.connectWithToken(token, deviceName);
      token = '';
      alert('连接成功');
    } catch (e) {
      alert('连接失败: ' + (e as Error).message);
    } finally {
      syncLoading = false;
    }
  }

  async function handleDisconnect() {
    if (confirm('确定断开 GitHub 连接？')) {
      try {
        await settings.disconnect();
        alert('已断开连接');
      } catch (e) {
        alert('断开失败');
      }
    }
  }

  async function handlePush() {
    syncLoading = true;
    try {
      await settings.pushToGist();
      alert('推送成功');
    } catch (e) {
      alert('推送失败: ' + (e as Error).message);
    } finally {
      syncLoading = false;
    }
  }

  async function handlePull() {
    syncLoading = true;
    try {
      await settings.pullFromGist();
      alert('拉取成功');
    } catch (e) {
      alert('拉取失败: ' + (e as Error).message);
    } finally {
      syncLoading = false;
    }
  }

  async function handleLinkGist() {
    if (!manualGistId.trim()) {
      alert('请输入 Gist ID');
      return;
    }
    try {
      await settings.linkGist(manualGistId);
      alert('关联成功');
      manualGistId = '';
    } catch (e) {
      alert('关联失败');
    }
  }

  async function handleCopyGistId() {
    try {
      const id = await settings.copyGistId();
      await navigator.clipboard.writeText(id);
      alert('已复制 Gist ID');
    } catch (e) {
      alert('复制失败');
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

          {#if $settings.sync.connected}
            <div class="sync-status connected">
              <span class="status-dot"></span>
              <span>已连接</span>
            </div>

            {#if $settings.sync.gist_id}
              <div class="setting-item">
                <span>Gist ID</span>
                <code class="gist-id">{$settings.sync.gist_id.slice(0, 12)}...</code>
              </div>
            {/if}

            {#if $settings.sync.device_name}
              <div class="setting-item">
                <span>设备</span>
                <span>{$settings.sync.device_name}</span>
              </div>
            {/if}

            {#if $settings.sync.last_sync}
              <div class="setting-item">
                <span>最后同步</span>
                <span>{new Date($settings.sync.last_sync).toLocaleString()}</span>
              </div>
            {/if}

            <div class="button-group">
              <button class="btn btn-secondary" onclick={handlePush} disabled={syncLoading}>
                手动推送
              </button>
              <button class="btn btn-secondary" onclick={handlePull} disabled={syncLoading}>
                手动拉取
              </button>
            </div>

            <div class="setting-item">
              <span>手动关联 Gist</span>
              <input
                type="text"
                bind:value={manualGistId}
                placeholder="输入 Gist ID"
              />
            </div>
            <button class="btn btn-secondary btn-small" onclick={handleLinkGist}>
              关联
            </button>

            <div class="button-group">
              <button class="btn btn-secondary btn-small" onclick={handleCopyGistId}>
                复制 Gist ID
              </button>
              <button class="btn btn-danger btn-small" onclick={handleDisconnect}>
                断开连接
              </button>
            </div>
          {:else}
            <div class="sync-status disconnected">
              <span class="status-dot"></span>
              <span>未连接</span>
            </div>

            <div class="setting-item">
              <span>Token</span>
              <input
                type="password"
                bind:value={token}
                placeholder="ghp_xxxx..."
              />
            </div>

            <div class="setting-item">
              <span>设备名称</span>
              <input
                type="text"
                bind:value={deviceName}
                placeholder="My Device"
              />
            </div>

            <button class="btn btn-primary" onclick={handleConnect} disabled={syncLoading}>
              连接
            </button>
          {/if}

          <p class="hint">需要 gist 权限的 GitHub Personal Access Token</p>
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

  .sync-status {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin-bottom: 1rem;
    padding: 0.5rem;
    background: #2a2a2a;
    border-radius: 4px;
  }

  .sync-status.connected {
    color: #22c55e;
  }

  .sync-status.disconnected {
    color: #888;
  }

  .status-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: currentColor;
  }

  .gist-id {
    font-family: monospace;
    font-size: 0.8rem;
    background: #333;
    padding: 0.2rem 0.4rem;
    border-radius: 3px;
  }

  .btn-small {
    padding: 0.4rem 0.8rem;
    font-size: 0.8rem;
  }

  .btn-danger {
    background: #dc2626;
    color: white;
  }

  .btn-danger:hover:not(:disabled) {
    background: #b91c1c;
  }
</style>