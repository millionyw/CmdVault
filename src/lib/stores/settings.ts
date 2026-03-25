// src/lib/stores/settings.ts
import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

export interface Shortcuts {
  global: string;
  newCommand: string;
  editSelected: string;
  deleteSelected: string;
  openSettings: string;
  export: string;
  import: string;
  close: string;
}

export interface SyncStatus {
  connected: boolean;
  username?: string;
  gist_id?: string;
  device_name?: string;
  last_sync?: string;
}

export interface Settings {
  shortcuts: Shortcuts;
  sync: SyncStatus;
  theme: 'light' | 'dark' | 'system';
  autostart: boolean;
}

const defaultShortcuts: Shortcuts = {
  global: 'CommandOrControl+Shift+C',
  newCommand: 'CommandOrControl+N',
  editSelected: 'CommandOrControl+E',
  deleteSelected: 'Delete',
  openSettings: 'CommandOrControl+,',
  export: 'CommandOrControl+Shift+E',
  import: 'CommandOrControl+Shift+I',
  close: 'Escape',
};

const defaultSettings: Settings = {
  shortcuts: defaultShortcuts,
  sync: { connected: false },
  theme: 'system',
  autostart: false,
};

function createSettingsStore() {
  const { subscribe, set, update } = writable<Settings>(defaultSettings);

  return {
    subscribe,

    async load() {
      try {
        const shortcuts = await invoke<Shortcuts>('get_shortcuts');
        const syncStatus = await invoke<SyncStatus>('get_sync_status');
        const autostart = await invoke<boolean>('get_autostart_enabled');
        set({
          ...defaultSettings,
          shortcuts,
          sync: syncStatus,
          autostart,
        });
      } catch (e) {
        console.error('Failed to load settings:', e);
        set(defaultSettings);
      }
    },

    async updateShortcut(key: string, value: string) {
      try {
        await invoke('update_shortcut', { key, value });
        update(s => ({
          ...s,
          shortcuts: { ...s.shortcuts, [key]: value },
        }));
      } catch (e) {
        console.error('Failed to update shortcut:', e);
        throw e;
      }
    },

    async resetShortcuts() {
      try {
        await invoke('reset_shortcuts');
        update(s => ({
          ...s,
          shortcuts: defaultShortcuts,
        }));
      } catch (e) {
        console.error('Failed to reset shortcuts:', e);
        throw e;
      }
    },

    async getSyncStatus(): Promise<SyncStatus> {
      try {
        return await invoke<SyncStatus>('get_sync_status');
      } catch (e) {
        console.error('Failed to get sync status:', e);
        return { connected: false };
      }
    },

    async connectWithToken(token: string, deviceName: string): Promise<SyncStatus> {
      const status = await invoke<SyncStatus>('connect_with_token', { token, deviceName });
      update(s => ({ ...s, sync: status }));
      return status;
    },

    async disconnect(): Promise<void> {
      await invoke('disconnect');
      update(s => ({ ...s, sync: { connected: false } }));
    },

    async pushToGist(): Promise<{ message: string }> {
      try {
        const result = await invoke<{ message: string }>('push_to_gist');
        return result;
      } catch (e) {
        console.error('Push failed:', e);
        throw new Error(typeof e === 'string' ? e : (e as Error).message || '未知错误');
      }
    },

    async pullFromGist(): Promise<{ message: string }> {
      try {
        const result = await invoke<{ message: string }>('pull_from_gist');
        return result;
      } catch (e) {
        console.error('Pull failed:', e);
        throw new Error(typeof e === 'string' ? e : (e as Error).message || '未知错误');
      }
    },

    async linkGist(gistId: string): Promise<void> {
      await invoke('link_gist', { gistId });
    },

    async copyGistId(): Promise<string> {
      return await invoke<string>('copy_gist_id');
    },

    async toggleAutostart(enabled: boolean): Promise<void> {
      await invoke('set_autostart_enabled', { enabled });
      update(s => ({ ...s, autostart: enabled }));
    },
  };
}

export const settings = createSettingsStore();