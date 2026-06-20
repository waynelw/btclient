import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import test from 'node:test';

const components = [
  'AppShell.svelte',
  'Toolbar.svelte',
  'TaskTable.svelte',
  'TaskDetail.svelte',
  'SettingsDialog.svelte',
  'ErrorToast.svelte'
];

test('desktop components exist and expose core BT client UI controls', async () => {
  const contents = await Promise.all(
    components.map((name) => readFile(new URL(`./${name}`, import.meta.url), 'utf8'))
  );
  const combined = contents.join('\n');

  for (const label of ['添加 torrent', '开始', '暂停', '删除', '设置', '下载目录']) {
    assert.match(combined, new RegExp(label));
  }
  assert.doesNotMatch(combined, /landing|hero|marketing/i);
});

test('toolbar exposes bindable command handlers for the desktop command bridge', async () => {
  const toolbar = await readFile(new URL('./Toolbar.svelte', import.meta.url), 'utf8');
  const app = await readFile(new URL('../../App.svelte', import.meta.url), 'utf8');
  const shell = await readFile(new URL('./AppShell.svelte', import.meta.url), 'utf8');

  for (const handler of ['onAddTorrent', 'onStartTask', 'onPauseTask', 'onRemoveTask']) {
    assert.match(toolbar, new RegExp(`export let ${handler}`));
    assert.match(shell, new RegExp(handler));
    assert.match(app, new RegExp(handler));
  }

  assert.match(toolbar, /type="file"/);
  assert.match(toolbar, /torrent 路径或 magnet/);
  assert.match(app, /getAppSnapshot/);
  assert.match(app, /addTorrent/);
  assert.match(app, /startTask/);
  assert.match(app, /pauseTask/);
  assert.match(app, /removeTask/);
});
