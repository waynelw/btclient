import type { TaskDto } from '../api/types';
import {
  createTaskState as createTaskStateImpl,
  removeTask as removeTaskImpl,
  selectTask as selectTaskImpl,
  upsertTask as upsertTaskImpl
} from './tasks.mjs';

export interface TaskState {
  tasks: TaskDto[];
  selectedTaskId: string | null;
}

export function createTaskState(tasks: TaskDto[] = [], selectedTaskId: string | null = null): TaskState {
  return createTaskStateImpl(tasks, selectedTaskId);
}

export function upsertTask(state: TaskState, task: TaskDto): TaskState {
  return upsertTaskImpl(state, task);
}

export function selectTask(state: TaskState, taskId: string): TaskState {
  return selectTaskImpl(state, taskId);
}

export function removeTask(state: TaskState, taskId: string): TaskState {
  return removeTaskImpl(state, taskId);
}
