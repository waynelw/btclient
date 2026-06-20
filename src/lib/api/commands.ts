import { invoke } from '@tauri-apps/api/core';
import type { SettingsDto, TaskDto } from './types';

export function getAppSnapshot(): Promise<{ tasks: TaskDto[]; settings: SettingsDto }> {
  return invoke('get_app_snapshot');
}

export function addTorrent(torrentPath: string, downloadDir: string): Promise<TaskDto> {
  return invoke('add_torrent', { torrentPath, downloadDir });
}

export function startTask(taskId: string): Promise<TaskDto> {
  return invoke('start_task', { taskId });
}

export function pauseTask(taskId: string): Promise<TaskDto> {
  return invoke('pause_task', { taskId });
}

export function removeTask(taskId: string, deleteFiles: boolean): Promise<void> {
  return invoke('remove_task', { taskId, deleteFiles });
}
