import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import test from 'node:test';

test('WSL Tauri dev command exposes the project Rust toolchain', async () => {
  const packageJson = JSON.parse(await readFile(new URL('../../package.json', import.meta.url), 'utf8'));

  assert.equal(packageJson.scripts['tauri:dev:wsl'], 'sh scripts/tauri-dev-wsl.sh');

  const script = await readFile(new URL('../../scripts/tauri-dev-wsl.sh', import.meta.url), 'utf8');

  assert.match(script, /export CARGO_HOME="\$\{CARGO_HOME:-\$ROOT_DIR\/\.cargo\}"/);
  assert.match(script, /export RUSTUP_HOME="\$\{RUSTUP_HOME:-\$ROOT_DIR\/\.rustup\}"/);
  assert.match(script, /export PATH="\$CARGO_HOME\/bin:\$PATH"/);
  assert.match(script, /WEBKIT_DISABLE_COMPOSITING_MODE/);
  assert.match(script, /exec "\$ROOT_DIR\/node_modules\/\.bin\/tauri" dev/);
});
