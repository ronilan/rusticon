import fs from 'fs';
import { execSync } from 'child_process';
import path from 'path';

const cargoToml = fs.readFileSync('Cargo.toml', 'utf8');
const metadataMatch =
    cargoToml.match(/\[package\.metadata\.bundle\][^\[]*app_name\s*=\s\"(.*)\"/);
const appName = metadataMatch ? metadataMatch[1] : 'Rusticon';

console.log('Building Windows version for: ' + appName);
process.env.APP_NAME = appName;

try {
    execSync('cargo build --release --bin rusticon_windows --features windows-native', { stdio: 'inherit' });
    fs.mkdirSync(path.join('dist', 'native'), { recursive: true });
    fs.copyFileSync(
        path.join('target', 'release', 'rusticon_windows.exe'),
        path.join('dist', 'native', appName + '.exe')
    );
    console.log('Build successful! Binary copied to dist/native/' + appName + '.exe');
} catch (error) {
    console.error('Build failed:', error.message);
    process.exit(1);
}
