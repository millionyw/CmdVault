// src/lib/stores/settings.ts
import { writable } from 'svelte/store';

export interface Settings {
  shortcut: string;
  githubToken?: string;
  gistId?: string;
  theme: 'light' | 'dark' | 'system';
}

const defaultSettings: Settings = {
  shortcut: 'CommandOrControl+Shift+C',
  theme: 'system'
};

function createSettingsStore() {
  const { subscribe, set, update } = writable<Settings>(defaultSettings);

  return {
    subscribe,
    async load() {
      // TODO: 从后端加载设置
      set(defaultSettings);
    },
    async updateSettings(settings: Partial<Settings>) {
      update(s => ({ ...s, ...settings }));
      // TODO: 保存到后端
    }
  };
}

export const settings = createSettingsStore();