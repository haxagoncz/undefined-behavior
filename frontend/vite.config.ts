import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import UnoCSS from 'unocss/vite'

export default defineConfig({
	plugins: [
		sveltekit(),
		UnoCSS({})
	],
	server: {
		proxy: {
			'/api': {
				target: 'http://localhost:3000',
				changeOrigin: true,
			}
		}
	}
});
