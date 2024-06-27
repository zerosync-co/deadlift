import { sveltekit } from '@sveltejs/kit/vite';
import wasm from 'vite-plugin-wasm';

/** @type {import('vite').UserConfig} */
const config = {
	plugins: [sveltekit(), wasm()]
};

export default config;
