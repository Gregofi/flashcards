import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

// https://vitejs.dev/config/
export default defineConfig(async () => ({
    resolve: {
        alias: {
            '@api': '/src/api',
            '@stores': '/src/stores'
        }
    },
    clearScreen: false,
    server: {
        port: 1420,
        strictPort: true,
        fs: {
            allow: ['./external']
        }
    },
    envPrefix: ['VITE_', 'TAURI_'],
    plugins: [sveltekit()]
}));
