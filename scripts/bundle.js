// build.js

import fs from 'fs';
import path from 'path';
import toml from 'toml';
import { execSync } from 'child_process';

// --------------------------------------------------
// Read Cargo.toml
// --------------------------------------------------

const cargo = toml.parse(
    fs.readFileSync('Cargo.toml', 'utf8')
);

const meta = cargo.package.metadata.bundle;

const appName = meta.app_name;
const binaryName = meta.binary_name;
const iconSource = meta.icon_source;

// --------------------------------------------------
// Paths
// --------------------------------------------------

const distDir = 'dist';

const appBundle = path.join(distDir, `${appName}.app`);

const contentsDir = path.join(appBundle, 'Contents');

const macosDir = path.join(contentsDir, 'MacOS');

const resourcesDir = path.join(contentsDir, 'Resources');

// --------------------------------------------------
// Create app bundle
// --------------------------------------------------

console.log(`Bundling "${appName}"...`);

fs.rmSync(appBundle, {
    recursive: true,
    force: true
});

fs.mkdirSync(macosDir, {
    recursive: true
});

fs.mkdirSync(resourcesDir, {
    recursive: true
});

// --------------------------------------------------
// Copy binary
// --------------------------------------------------

const binaryPath = path.join(distDir, binaryName);

if (!fs.existsSync(binaryPath)) {
    console.error(`Missing binary: ${binaryPath}`);
    process.exit(1);
}

const appBinaryPath = path.join(
    macosDir,
    binaryName
);

fs.copyFileSync(binaryPath, appBinaryPath);

fs.chmodSync(appBinaryPath, 0o755);

// --------------------------------------------------
// Generate icon
// --------------------------------------------------

if (fs.existsSync(iconSource)) {
    console.log('Generating macOS icons...');

    const iconsetDir = path.join(
        distDir,
        'AppIcon.iconset'
    );

    fs.rmSync(iconsetDir, {
        recursive: true,
        force: true
    });

    fs.mkdirSync(iconsetDir);

    const sizes = [16, 32, 64, 128, 256, 512];

    for (const size of sizes) {
        const innerSize = Math.floor(size * 0.8);

        const png1x = path.join(
            iconsetDir,
            `icon_${size}x${size}.png`
        );

        const png2x = path.join(
            iconsetDir,
            `icon_${size}x${size}@2x.png`
        );

        execSync(
            `rsvg-convert -w ${innerSize} -h ${innerSize} "${iconSource}" > "${png1x}"`
        );

        execSync(
            `sips -p ${size} ${size} "${png1x}" --out "${png1x}" > /dev/null`
        );

        const size2x = size * 2;

        const inner2x = Math.floor(size2x * 0.8);

        execSync(
            `rsvg-convert -w ${inner2x} -h ${inner2x} "${iconSource}" > "${png2x}"`
        );

        execSync(
            `sips -p ${size2x} ${size2x} "${png2x}" --out "${png2x}" > /dev/null`
        );
    }

    const icnsPath = path.join(
        resourcesDir,
        'AppIcon.icns'
    );

    execSync(
        `iconutil -c icns "${iconsetDir}" -o "${icnsPath}"`
    );

    fs.rmSync(iconsetDir, {
        recursive: true,
        force: true
    });
}

// --------------------------------------------------
// Copy Info.plist
// --------------------------------------------------

fs.copyFileSync(
    'Info.plist',
    path.join(contentsDir, 'Info.plist')
);

// --------------------------------------------------
// Create polished DMG
// --------------------------------------------------

console.log('Creating DMG...');

const dmgPath = path.join(
    distDir,
    `${appName}.dmg`
);

if (fs.existsSync(dmgPath)) {
    fs.unlinkSync(dmgPath);
}

execSync(
    `
npx create-dmg "${appBundle}" "${distDir}" \
  --overwrite \
  --no-code-sign \
  --dmg-title="${appName}" \
  --background="assets/dmg-background.png" \
  --window-size 660 400 \
  --icon-size 128 \
  --icon "${appName}.app" 180 170 \
  --app-drop-link 480 170
`,
    {
        stdio: 'inherit',
        shell: '/bin/bash'
    }
);

console.log(`Created ${dmgPath}`);
