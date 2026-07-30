import { run } from './utils.js';

run('pnpm exec wasm-pack build --target web --release');
run('pnpm exec vite build');
