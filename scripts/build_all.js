import { run } from './utils.js';

run('cargo update');

run('pnpm run build:web');
run('pnpm run build:terminal');

if (process.platform === 'darwin') {
    run('pnpm run build:macos');
    run('pnpm run bundle:macos');
}

if (process.platform === 'win32') {
    run('pnpm run build:windows');
}
