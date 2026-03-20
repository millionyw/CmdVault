// src/lib/stores/commands.ts
import { writable, derived } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

export interface Command {
  id: string;
  name: string;
  content: string;
  description?: string;
  created_at: string;
  updated_at: string;
  usage_count: number;
}

function createCommandStore() {
  const { subscribe, set, update } = writable<Command[]>([]);

  return {
    subscribe,
    async load() {
      const commands = await invoke<Command[]>('get_all_commands');
      set(commands);
    },
    async create(name: string, content: string, description?: string) {
      const cmd = await invoke<Command>('create_command', { name, content, description });
      update(cmds => [cmd, ...cmds]);
      return cmd;
    },
    async update(id: string, name: string, content: string, description?: string) {
      const cmd = await invoke<Command>('update_command', { id, name, content, description });
      update(cmds => cmds.map(c => c.id === id ? cmd : c));
      return cmd;
    },
    async delete(id: string) {
      await invoke('delete_command', { id });
      update(cmds => cmds.filter(c => c.id !== id));
    },
    async copy(id: string) {
      await invoke('copy_command', { id });
      update(cmds => cmds.map(c =>
        c.id === id ? { ...c, usage_count: c.usage_count + 1 } : c
      ));
    },
    async export() {
      return await invoke<string>('export_commands');
    },
    async import(json: string) {
      const count = await invoke<number>('import_commands', { json });
      await this.load();
      return count;
    }
  };
}

export const commands = createCommandStore();

// 搜索 store
export const searchQuery = writable('');

// 过滤后的命令列表
export const filteredCommands = derived(
  [commands, searchQuery],
  ([$commands, $query]) => {
    if (!$query.trim()) return $commands;
    const q = $query.toLowerCase();
    return $commands.filter(c =>
      c.name.toLowerCase().includes(q) ||
      c.content.toLowerCase().includes(q) ||
      c.description?.toLowerCase().includes(q)
    );
  }
);