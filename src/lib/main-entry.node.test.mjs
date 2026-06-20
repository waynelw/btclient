import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import test from 'node:test';

test('main entry mounts the Svelte 5 app into #app', async () => {
  const main = await readFile(new URL('../main.ts', import.meta.url), 'utf8');

  assert.match(main, /import\s+\{\s*mount\s*\}\s+from\s+['"]svelte['"]/);
  assert.match(main, /mount\(App,\s*\{\s*target:\s*document\.getElementById\(['"]app['"]\)/s);
  assert.doesNotMatch(main, /new\s+App\s*\(/);
});
