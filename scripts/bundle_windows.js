import path from 'path';
import fs from 'fs';
import { execSync } from 'child_process';
import sharp from 'sharp';
import pngToIco from 'png-to-ico';
import { rcedit } from 'rcedit';

async function main() {
    // Parse Cargo.toml
    const cargoToml = fs.readFileSync('Cargo.toml', 'utf8');
    const metadataBlock = cargoToml.split('[package.metadata.bundle]')[1].split('[')[0];
    const appNameMatch = metadataBlock.match(/app_name\s*=\s\"(.*)\"/);
    const appName = appNameMatch ? appNameMatch[1] : 'Rusticon';
    const iconSourceMatch = metadataBlock.match(/icon_source\s*=\s\"(.*)\"/);
    const iconSource = iconSourceMatch ? iconSourceMatch[1] : 'favicon.svg';

    const distDir = 'dist';
    const nativeDir = path.join(distDir, 'native');
    fs.mkdirSync(nativeDir, { recursive: true });

    console.log('Bundling Windows version for: ' + appName);

    const exePath = path.join(nativeDir, appName + '.exe');
    
    // The executable should already be in dist/native/ from build_windows.js
    if (!fs.existsSync(exePath)) {
        console.error('Error: Binary ' + exePath + ' not found. Run build:windows first.');
        process.exit(1);
    }

    if (fs.existsSync(iconSource)) {
        console.log('Generating .ico from ' + iconSource + '...');
        
        try {
            // Read SVG and render as PNG using sharp
            const pngBuffer = await sharp(iconSource)
                .resize(256, 256, { fit: 'contain', background: { r: 0, g: 0, b: 0, alpha: 0 } })
                .png()
                .toBuffer();
            
            // Convert PNG to ICO
            const icoBuffer = await pngToIco(pngBuffer);
            
            const iconPath = path.join(distDir, 'icon.ico');
            fs.writeFileSync(iconPath, icoBuffer);
            
            console.log('Injecting icon into ' + exePath + '...');
            await rcedit(exePath, { icon: iconPath });
            
            // Clean up the temporary .ico file
            fs.unlinkSync(iconPath);
            
        } catch (e) {
            console.error('Failed to generate or inject icon:', e);
        }
    }

    console.log('Successfully bundled Windows version for: ' + appName);
}

main().catch(e => {
    console.error(e);
    process.exit(1);
});