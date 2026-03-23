// src/lib/utils/fileOperations.ts
import { save, open } from '@tauri-apps/plugin-dialog';
import { writeTextFile, readTextFile } from '@tauri-apps/plugin-fs';
import { commands } from '../stores/commands';

export async function exportToFile(onSuccess: (message: string) => void, onError: (message: string) => void) {
  try {
    const path = await save({
      defaultPath: `commands-${new Date().toISOString().split('T')[0]}.json`,
      filters: [{ name: 'JSON', extensions: ['json'] }]
    });

    if (path) {
      const json = await commands.export();
      await writeTextFile(path, json);
      onSuccess('导出成功');
    }
  } catch (e) {
    console.error('Export failed:', e);
    onError('导出失败');
  }
}

export async function importFromFile(onSuccess: (message: string) => void, onError: (message: string) => void) {
  try {
    const path = await open({
      filters: [{ name: 'JSON', extensions: ['json'] }]
    });

    if (path && typeof path === 'string') {
      const json = await readTextFile(path);
      const count = await commands.import(json);
      onSuccess(`导入成功，共 ${count} 条`);
    }
  } catch (e) {
    console.error('Import failed:', e);
    onError('导入失败，请检查文件格式');
  }
}