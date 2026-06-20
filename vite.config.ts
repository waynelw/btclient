import { svelte } from '@sveltejs/vite-plugin-svelte';
import { defineConfig } from 'vite';

export default defineConfig({
  plugins: [svelte()],
  clearScreen: false,
  test: {
    environment: 'node',
    include: ['src/**/*.test.ts']
  },
  server: {
    strictPort: true,
    port: 1420
  }
});
