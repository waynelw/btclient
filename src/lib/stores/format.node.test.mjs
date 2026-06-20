import assert from 'node:assert/strict';
import test from 'node:test';

import { formatBytes, formatEta, formatSpeed } from './format.mjs';

test('formatBytes uses binary units', () => {
  assert.equal(formatBytes(0), '0 B');
  assert.equal(formatBytes(1024), '1.0 KiB');
  assert.equal(formatBytes(1536), '1.5 KiB');
  assert.equal(formatBytes(1024 * 1024), '1.0 MiB');
});

test('formatSpeed appends per-second suffix', () => {
  assert.equal(formatSpeed(0), '0 B/s');
  assert.equal(formatSpeed(2048), '2.0 KiB/s');
});

test('formatEta handles missing and long values', () => {
  assert.equal(formatEta(null), '—');
  assert.equal(formatEta(59), '59s');
  assert.equal(formatEta(61), '1m 1s');
  assert.equal(formatEta(3661), '1h 1m');
});
