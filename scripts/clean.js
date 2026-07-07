import fs from 'fs';

fs.rmSync('dist', {
    recursive: true,
    force: true
});

fs.rmSync('pkg', {
    recursive: true,
    force: true
});

fs.rmSync('docs', {
    recursive: true,
    force: true
});

fs.rmSync('target', {
    recursive: true,
    force: true
});
