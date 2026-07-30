import fs from 'fs';
import { run } from './utils.js';

fs.rmSync('pkg', { recursive: true, force: true });
fs.rmSync('node_modules/.vite', { recursive: true, force: true });

run('cargo update');
run('pnpm exec wasm-pack build --target web --dev');
run('pnpm exec vite --force');
