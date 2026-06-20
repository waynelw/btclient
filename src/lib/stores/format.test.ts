import { describe, expect, it } from 'vitest';
import { formatBytes, formatEta, formatSpeed } from './format';

describe('format helpers', () => {
  it('formats bytes using binary units', () => {
    expect(formatBytes(0)).toBe('0 B');
    expect(formatBytes(1024)).toBe('1.0 KiB');
    expect(formatBytes(1536)).toBe('1.5 KiB');
    expect(formatBytes(1024 * 1024)).toBe('1.0 MiB');
  });

  it('formats speed as bytes per second', () => {
    expect(formatSpeed(0)).toBe('0 B/s');
    expect(formatSpeed(2048)).toBe('2.0 KiB/s');
  });

  it('formats nullable eta values', () => {
    expect(formatEta(null)).toBe('—');
    expect(formatEta(59)).toBe('59s');
    expect(formatEta(61)).toBe('1m 1s');
    expect(formatEta(3661)).toBe('1h 1m');
  });
});
