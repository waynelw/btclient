import assert from 'node:assert/strict';
import test from 'node:test';

import { createTaskState, removeTask, selectTask, upsertTask } from './tasks.mjs';

function task(id, name = id) {
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

test('upsertTask inserts and updates without reordering', () => {
  let state = createTaskState();
  state = upsertTask(state, task('a', 'first'));
  state = upsertTask(state, task('b', 'second'));
  state = upsertTask(state, { ...task('a', 'updated'), status: 'downloading' });

  assert.deepEqual(
    state.tasks.map((item) => item.id),
    ['a', 'b']
  );
  assert.equal(state.tasks[0].name, 'updated');
  assert.equal(state.tasks[0].status, 'downloading');
});

test('removeTask clears selected task when removed', () => {
  let state = createTaskState([task('a'), task('b')]);
  state = selectTask(state, 'b');
  state = removeTask(state, 'b');

  assert.equal(state.selectedTaskId, null);
  assert.deepEqual(
    state.tasks.map((item) => item.id),
    ['a']
  );
});

test('removeTask preserves selection for other task', () => {
  let state = createTaskState([task('a'), task('b')], 'b');
  state = removeTask(state, 'a');

  assert.equal(state.selectedTaskId, 'b');
  assert.deepEqual(
    state.tasks.map((item) => item.id),
    ['b']
  );
});
