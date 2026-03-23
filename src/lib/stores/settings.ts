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
};

function createSettingsStore() {
  const { subscribe, set, update } = writable<Settings>(defaultSettings);

  return {
    subscribe,

    async load() {
      try {
        const shortcuts = await invoke<Shortcuts>('get_shortcuts');
        set({
          ...defaultSettings,
          shortcuts,
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
  };
}

export const settings = createSettingsStore();