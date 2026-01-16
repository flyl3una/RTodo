import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';
import path from 'path';

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [vue()],
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    host: true,
  },
  envPrefix: ['VITE_', 'TAURI_'],
  resolve: {
    alias: {
      '@': path.posix.resolve('/', './src'),
    },
  },
  // Tauri 2.0 requires this for development mode
  define: {
    __TAURI_PLATFORM__: '"windows"',
    __TAURI_ARCH__: '"x86_64"',
  },
}));
