import fs from 'fs';
import path from 'path';
import { execSync } from 'child_process';

import { run } from './utils.js';

function sleep(ms) {
    execSync(`node -e "setTimeout(() => {}, ${ms})"`);
}

function waitUntilAssetDeleted(tag, filename) {
    const lower = filename.toLowerCase();

    while (true) {
        const assets = JSON.parse(
            execSync(`gh release view ${tag} --json assets`, {
                encoding: 'utf8'
            })
        ).assets;

        if (!assets.some(a => a.name.toLowerCase() === lower))
            return;

        sleep(500);
    }
}

console.log('\n=== Publishing to "latest" release via gh CLI ===\n');

// Ensure release exists.
try {
    execSync('gh release view latest', { stdio: 'ignore' });
} catch {
    console.log('-> Creating "latest" release...');
    run('gh release create latest --title "Latest Build" --prerelease --notes "Automated local build artifacts."');
}

// Read existing assets once.
const existingAssets = new Map(
    JSON.parse(
        execSync('gh release view latest --json assets', {
            encoding: 'utf8'
        })
    ).assets.map(a => [a.name.toLowerCase(), a.name])
);

const releaseDir = path.resolve('release');

for (const file of fs.readdirSync(releaseDir)) {
    const asset = path.join(releaseDir, file);

    if (!fs.statSync(asset).isFile())
        continue;

    const existing = existingAssets.get(file.toLowerCase());

    if (existing) {
        console.log(`-> Removing existing asset "${existing}"...`);

        execSync(
            `gh release delete-asset latest "${existing}" -y`,
            { stdio: 'inherit' }
        );

        waitUntilAssetDeleted('latest', existing);
        existingAssets.delete(file.toLowerCase());
    }

    console.log(`-> Uploading "${file}"...`);
    run(`gh release upload latest "${asset}"`);
}

console.log('\nDone publishing.');