import fs from 'fs';
import path from 'path';
import { execSync } from 'child_process';

// Parse Cargo.toml (simple TOML parser via regex)
const cargoToml = fs.readFileSync('Cargo.toml', 'utf8');

// We extract the metadata block
const metadataBlock = cargoToml.split('[package.metadata.bundle]')[1].split('[')[0];
const appName = metadataBlock.match(/app_name\s*=\s*"(.*)"/)[1];
const binaryName = metadataBlock.match(/binary_name\s*=\s*"(.*)"/)[1];
const iconSource = metadataBlock.match(/icon_source\s*=\s*"(.*)"/)[1];

const distDir = "dist";
const appBundle = path.join(distDir, `${appName}.app`);
const contentsDir = path.join(appBundle, "Contents");
const macosDir = path.join(contentsDir, "MacOS");
const resourcesDir = path.join(contentsDir, "Resources");

console.log(`Bundling "${appName}"...`);

// 1. Create directory structure
fs.mkdirSync(macosDir, { recursive: true });
fs.mkdirSync(resourcesDir, { recursive: true });

// 2. Copy the binary
const binaryPath = path.join(distDir, binaryName);
if (fs.existsSync(binaryPath)) {
    fs.copyFileSync(binaryPath, path.join(macosDir, binaryName));
} else {
    console.error(`Error: Binary ${binaryPath} not found. Run build first.`);
    process.exit(1);
}

// 3. Create AppIcon.icns with macOS Margins
if (fs.existsSync(iconSource)) {
    console.log("Creating icon with native macOS margins...");
    const iconsetDir = path.join(distDir, "AppIcon.iconset");
    if (!fs.existsSync(iconsetDir)) fs.mkdirSync(iconsetDir);

    const sizes = [16, 32, 64, 128, 256, 512, 1024];
    for (const size of sizes) {
        try {
            const pngPath = path.join(iconsetDir, `icon_${size}x${size}.png`);
            const pngPath2x = path.join(iconsetDir, `icon_${size}x${size}@2x.png`);
            
            // Standard macOS "Safe Area" is ~80%. 
            // We render smaller and then pad with sips (native macOS tool).
            const innerSize = Math.floor(size * 0.8);

            execSync(`rsvg-convert -w ${innerSize} -h ${innerSize} "${iconSource}" > "${pngPath}"`);
            execSync(`sips -p ${size} ${size} "${pngPath}" --out "${pngPath}" > /dev/null 2>&1`);

            if (size <= 512) {
                const innerSize2x = Math.floor(size * 2 * 0.8);
                execSync(`rsvg-convert -w ${innerSize2x} -h ${innerSize2x} "${iconSource}" > "${pngPath2x}"`);
                execSync(`sips -p ${size * 2} ${size * 2} "${pngPath2x}" --out "${pngPath2x}" > /dev/null 2>&1`);
            }
        } catch (e) {
            console.warn("Icon generation step failed. Ensure rsvg-convert is installed.");
            break;
        }
    }

    if (fs.existsSync(path.join(iconsetDir, "icon_1024x1024.png"))) {
        execSync(`iconutil -c icns "${iconsetDir}" -o "${path.join(resourcesDir, "AppIcon.icns")}"`);
        fs.rmSync(iconsetDir, { recursive: true, force: true });
    }
}

// 4. Copy Info.plist
fs.copyFileSync("Info.plist", path.join(contentsDir, "Info.plist"));

console.log(`Successfully created ${appBundle}`);
