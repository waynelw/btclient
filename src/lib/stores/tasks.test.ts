import { describe, expect, it } from 'vitest';
import type { TaskDto } from '../api/types';
import { createTaskState, removeTask, selectTask, upsertTask } from './tasks';

function task(id: string, name = id): TaskDto {
  return {
    id,
    name,
    status: 'queued',
    torrent_path: `/tmp/${id}.torrent`,
    download_dir: '/tmp/downloads',
    total_bytes: 1024,
    downloaded_bytes: 0,
    progress: 0,
    download_speed_bps: 0,
    upload_speed_bps: 0,
    peers_connected: 0,
    peers_total: 0,
    eta_seconds: null,
    created_at: '1970-01-01T00:00:00Z',
    updated_at: '1970-01-01T00:00:00Z',
    error: null
  };
}

describe('task state helpers', () => {
  it('upserts new and existing tasks without changing order', () => {
    let state = createTaskState();
    state = upsertTask(state, task('a', 'first'));
    state = upsertTask(state, task('b', 'second'));
    state = upsertTask(state, { ...task('a', 'updated'), status: 'downloading' });

    expect(state.tasks.map((item) => item.id)).toEqual(['a', 'b']);
    expect(state.tasks[0].name).toBe('updated');
    expect(state.tasks[0].status).toBe('downloading');
  });

  it('selects and clears selected task when removed', () => {
    let state = createTaskState([task('a'), task('b')]);
    state = selectTask(state, 'b');
    state = removeTask(state, 'b');

    expect(state.selectedTaskId).toBeNull();
    expect(state.tasks.map((item) => item.id)).toEqual(['a']);
  });

  it('keeps selection when removing a different task', () => {
    let state = createTaskState([task('a'), task('b')], 'b');
    state = removeTask(state, 'a');

    expect(state.selectedTaskId).toBe('b');
    expect(state.tasks.map((item) => item.id)).toEqual(['b']);
  });
});
