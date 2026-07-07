import fs from 'fs';
import path from 'path';

import { run } from './utils.js';

run('cargo build --release --bin rusticon');

const exe =
    process.platform === 'win32'
        ? 'rusticon.exe'
        : 'rusticon';

fs.mkdirSync('dist', {
    recursive: true
});

fs.copyFileSync(
    path.join('target', 'release', exe),
    path.join('dist', exe)
);
