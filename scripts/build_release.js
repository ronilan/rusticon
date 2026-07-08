import fs from 'fs';
import path from 'path';
import { execSync } from 'child_process';

import { run } from './utils.js';

const PUBLISH = process.argv.includes('--publish');

// ---------------------------------------------------------------------------
// Figure out which "matrix leg" this machine corresponds to, matching the
// naming used in downloadable_binaries.yml (macos-arm, macos-intel, windows,
// linux), so zip filenames line up with what CI produces.
// ---------------------------------------------------------------------------
function platformName() {
    if (process.platform === 'darwin') {
        return process.arch === 'arm64' ? 'macos-arm' : 'macos-intel';
    }
    if (process.platform === 'win32') return 'windows';
    return 'linux';
}

const PLATFORM = platformName();

// ---------------------------------------------------------------------------
// Read app_name from Cargo.toml, same regex approach used by
// build_macos.js / build_windows.js / bundle.js, so the native build zip
// names match what those scripts already produce.
// ---------------------------------------------------------------------------
function getAppName() {
    const cargoToml = fs.readFileSync('Cargo.toml', 'utf8');
    const match = cargoToml.match(/\[package\.metadata\.bundle\][^\[]*app_name\s*=\s\"(.*)\"/);
    return match ? match[1] : 'Rusticon';
}

// ---------------------------------------------------------------------------
// Zip a single file. Mirrors the two zip steps split by platform in the
// workflow: zip on macOS/Linux, Compress-Archive on Windows. Zips from
// inside the containing directory so the archive holds just the bare file,
// not a wrapping folder.
// ---------------------------------------------------------------------------
function zipFile(dir, filename, zipName) {
    const zipPath = path.resolve(dir, '..', zipName);
    if (fs.existsSync(zipPath)) fs.rmSync(zipPath);

    if (process.platform === 'win32') {
        execSync(
            powershell -Command \"Compress-Archive -Path '' -DestinationPath ''\",
            { cwd: dir, stdio: 'inherit' }
        );
    } else {
        execSync(zip -r \"\" \"\", { cwd: dir, stdio: 'inherit' });
    }

    // Move to release directory
    const releaseDir = path.resolve(dir, '..', 'release');
    fs.mkdirSync(releaseDir, { recursive: true });

    const releasePath = path.join(releaseDir, zipName);

    if (fs.existsSync(releasePath)) {
        fs.rmSync(releasePath);
    }

    fs.renameSync(zipPath, releasePath);

    console.log(Zipped: );
    return releasePath;
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------
const releaseAssets = [];

console.log(\n=== Building for  ===\n);

// Clean, matching the workflow's explicit "don't trust the working tree"
// step � dist/ has previously been committed to this repo's history.
fs.rmSync('dist', { recursive: true, force: true });
fs.rmSync('release', { recursive: true, force: true });

// --- Terminal binary (always built) -----------------------------------
run('pnpm run build:terminal');

{
    // Dynamically find the first file in dist/cli folder
    const files = fs.readdirSync(path.join('dist', 'cli'));
    if (files.length === 0) throw new Error('No terminal binary found in dist/cli/');
    
    const exe = files[0];
    const base = path.basename(exe, path.extname(exe));
    const zipName = ${base}-terminal-.zip;
    releaseAssets.push(zipFile(path.join('dist', 'cli'), exe, zipName));
}

// --- macOS native (only on macOS) --------------------------------------
if (process.platform === 'darwin') {
    run('pnpm run build:macos');
    run('pnpm run bundle:macos');

    // Find the .app bundle in dist/
    const appBundle = fs.readdirSync(path.join('dist', 'native')).find(f => f.endsWith('.app'));
    if (!appBundle) throw new Error('No .app bundle found in dist/');

    const sanitizedApp = appBundle.toLowerCase().replace(/\s+/g, '_');
    
    if (appBundle !== sanitizedApp) {
        fs.renameSync(path.join('dist', appBundle), path.join('dist', sanitizedApp));
    }

    const base = path.basename(sanitizedApp, '.app');
    const zipName = ${base}-macos-native-.zip;
    releaseAssets.push(zipFile(path.join('dist', 'native'), sanitizedApp, zipName));
}

// --- Windows native (only on Windows) -----------------------------------
if (process.platform === 'win32') {
    run('pnpm run build:windows');
    run('pnpm run bundle:windows');

    // Dynamically find the built native .exe file
    const appName = getAppName();
    const originalExe = path.join('dist', 'native', ${appName}.exe);
    if (!fs.existsSync(originalExe)) throw new Error(No native .exe found at );

    // On Windows, use the app name as-is for the zip
    const base = appName;
    const zipName = ${base.toLowerCase().replace(/\s+/g, '_')}-windows-native.zip;
    releaseAssets.push(zipFile(path.join('dist', 'native'), ${appName}.exe, zipName));
}

console.log('\n=== Done. Produced: ===');
releaseAssets.forEach((f) => console.log(  ));
