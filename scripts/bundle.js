import fs from 'fs';
import path from 'path';
import { execSync } from 'child_process';

// Parse Cargo.toml (simple TOML parser via regex)
const cargoToml = fs.readFileSync('Cargo.toml', 'utf8');

// We extract the metadata block
const metadataBlock = cargoToml.split('[package.metadata.bundle]')[1].split('[')[0];
const appNameMatch = metadataBlock.match(/app_name\s*=\s"(.*)"/);
const binaryNameMatch = metadataBlock.match(/binary_name\s*=\s"(.*)"/);
const iconSourceMatch = metadataBlock.match(/icon_source\s*=\s"(.*)"/);

if (!appNameMatch || !binaryNameMatch || !iconSourceMatch) {
    console.error('Missing required metadata in Cargo.toml');
    process.exit(1);
}

const appName = appNameMatch[1];
const binaryName = binaryNameMatch[1];
const iconSource = iconSourceMatch[1];

const distDir = 'dist';
const nativeDir = path.join(distDir, 'native');
const appBundle = path.join(nativeDir, `${appName}.app`);
const contentsDir = path.join(appBundle, 'Contents');
const macosDir = path.join(contentsDir, 'MacOS');
const resourcesDir = path.join(contentsDir, 'Resources');

console.log(`Bundling "${appName}"...`);

// 1. Create directory structure
fs.mkdirSync(macosDir, { recursive: true });
fs.mkdirSync(resourcesDir, { recursive: true });

// 2. Copy the binary
const binaryPath = path.join(nativeDir, binaryName);
if (fs.existsSync(binaryPath)) {
    fs.copyFileSync(binaryPath, path.join(macosDir, binaryName));
} else {
    console.error(`Error: Binary ${binaryPath} not found. Run build first.`);
    process.exit(1);
}

// 3. Create AppIcon.icons with macOS Margins
if (fs.existsSync(iconSource)) {
    console.log('Creating icon with native macOS margins...');
    const iconsetDir = path.join(nativeDir, 'AppIcon.iconset');
    if (!fs.existsSync(iconsetDir)) fs.mkdirSync(iconsetDir);

    const sizes = [16, 32, 64, 128, 256, 512, 1024];
    for (const size of sizes) {
        try {
            const pngPath = path.join(iconsetDir, `icon_${size}x${size}.png`);
            const pngPath2x = path.join(iconsetDir, `icon_${size}x${size}@2x.png`);
            
            const innerSize = Math.floor(size * 0.8);

            execSync(`rsvg-convert -w ${innerSize} -h ${innerSize} "${iconSource}" > "${pngPath}"`);
            execSync(`sips -p ${size} ${size} "${pngPath}" --out "${pngPath}" > /dev/null 2>&1`);

            if (size <= 512) {
                const innerSize2x = Math.floor(size * 2 * 0.8);
                execSync(`rsvg-convert -w ${innerSize2x} -h ${innerSize2x} "${iconSource}" > "${pngPath2x}"`);
                execSync(`sips -p ${size * 2} ${size * 2} "${pngPath2x}" --out "${pngPath2x}" > /dev/null 2>&1`);
            }
        } catch (e) {
            console.warn('Icon generation step failed. Ensure rsvg-convert is installed.');
            break;
        }
    }

    if (fs.existsSync(path.join(iconsetDir, 'icon_1024x1024.png'))) {
        execSync(`iconutil -c icns "${iconsetDir}" -o "${path.join(resourcesDir, 'AppIcon.icns')}"`);
        fs.rmSync(iconsetDir, { recursive: true, force: true });
    }
}

// 4. Copy Info.plist
fs.copyFileSync('Info.plist', path.join(contentsDir, 'Info.plist'));

console.log(`Successfully created ${appBundle}`);

// 5. Create DMG
const stagingDir = path.join(nativeDir, 'dmg');

fs.rmSync(stagingDir, { recursive: true, force: true });
fs.mkdirSync(stagingDir, { recursive: true });

try {
    const stagedApp = path.join(stagingDir, `${appName}.app`);
    fs.cpSync(appBundle, stagedApp, { recursive: true });

    const applicationsLink = path.join(stagingDir, 'Applications');
    fs.symlinkSync('/Applications', applicationsLink);

    const dmgPath = path.join(nativeDir, `${appName}.dmg`);

    if (fs.existsSync(dmgPath)) {
        fs.unlinkSync(dmgPath);
    }

    execSync(`
hdiutil create \\
  -volname "${appName}" \\
  -srcfolder "${stagingDir}" \\
  -ov \\
  -format UDZO \\
  "${dmgPath}"
`, { stdio: 'inherit' });

    console.log(`Created ${dmgPath}`);

} finally {
    fs.rmSync(stagingDir, { recursive: true, force: true });
}