import { listen } from '@tauri-apps/api/event';
import type { TaskDto } from './types';

export function listenTaskUpdated(handler: (task: TaskDto) => void): Promise<() => void> {
  return listen<TaskDto>('task.updated', (event) => handler(event.payload));
}
