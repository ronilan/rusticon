import { run } from './utils.js';

run('cargo update');
run('pnpm run build:wasm:dev');
run('vite');
