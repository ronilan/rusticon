// scripts/build_release.js
//
// Local equivalent of the "Create Downloadable Binaries" GitHub Actions
// workflow (downloadable_binaries.yml). Builds whatever this machine is
// actually capable of building — GitHub cross-builds four platforms using
// four different runners; a single local machine can only build for
// itself, since none of the build_*.js scripts cross-compile.
//
//   macOS (arm64)   -> terminal + macos-native, zipped
//   macOS (x64)     -> terminal + macos-native, zipped
//   Windows         -> terminal + windows-native, zipped
//   Linux           -> terminal only, zipped
//
// Usage:
//   node scripts/build_release.js            build + zip only
//   node scripts/build_release.js --publish   also upload to the "latest"
//                                              GitHub Release (requires the
//                                              `gh` CLI, already authenticated)
//
// This intentionally shells out to the platform's native zip tool (`zip`
// on macOS/Linux, PowerShell's Compress-Archive on Windows) rather than
// adding an npm zip dependency, to mirror exactly what the workflow does.

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
    const match = cargoToml.match(/\[package\.metadata\.bundle\][^\[]*app_name\s*=\s*"(.*)"/);
    return match ? match[1] : 'Incredible App';
}

// ---------------------------------------------------------------------------
// Zip a single file. Mirrors the two zip steps split by platform in the
// workflow: `zip` on macOS/Linux, Compress-Archive on Windows. Zips from
// inside the containing directory so the archive holds just the bare file,
// not a wrapping folder.
// ---------------------------------------------------------------------------
function zipFile(dir, filename, zipName) {
    const zipPath = path.resolve(dir, '..', zipName);
    if (fs.existsSync(zipPath)) fs.rmSync(zipPath);

    if (process.platform === 'win32') {
        execSync(
            `powershell -Command "Compress-Archive -Path '${filename}' -DestinationPath '${zipPath}'"`,
            { cwd: dir, stdio: 'inherit' }
        );
    } else {
        execSync(`zip -r "${zipPath}" "${filename}"`, { cwd: dir, stdio: 'inherit' });
    }

    // Move to release directory
    const releaseDir = path.resolve(dir, '..', 'release');
    fs.mkdirSync(releaseDir, { recursive: true });

    const releasePath = path.join(releaseDir, zipName);

    if (fs.existsSync(releasePath)) {
        fs.rmSync(releasePath);
    }

    fs.renameSync(zipPath, releasePath);

    console.log(`Zipped: ${releasePath}`);
    return releasePath;
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------
const releaseAssets = [];

console.log(`\n=== Building for ${PLATFORM} ===\n`);

// Clean, matching the workflow's explicit "don't trust the working tree"
// step — dist/ has previously been committed to this repo's history.
fs.rmSync('dist', { recursive: true, force: true });
fs.rmSync('release', { recursive: true, force: true });

// --- Terminal binary (always built) -----------------------------------
run('pnpm run build:terminal');

{
    // Dynamically find the first file in the dist folder, matching the GHA 'ls | head -n 1' logic
    const files = fs.readdirSync('dist');
    if (files.length === 0) throw new Error("No terminal binary found in dist/");
    
    const exe = files[0];
    const base = path.basename(exe, path.extname(exe));
    const zipName = `${base}-terminal-${PLATFORM}.zip`;
    releaseAssets.push(zipFile('dist', exe, zipName));
}

// --- macOS native (only on macOS) --------------------------------------
if (process.platform === 'darwin') {
    run('pnpm run build:macos');
    run('pnpm run bundle:macos');

    // Dynamically find the built .dmg file
    const originalDmg = fs.readdirSync('dist').find(f => f.endsWith('.dmg'));
    if (!originalDmg) throw new Error("No .dmg bundle found in dist/");

    // Force snake_case naming to mirror CI exactly: "Incredible Template.dmg" -> "incredible_template.dmg"
    const sanitizedDmg = originalDmg.toLowerCase().replace(/\s+/g, '_');
    
    if (originalDmg !== sanitizedDmg) {
        fs.renameSync(path.join('dist', originalDmg), path.join('dist', sanitizedDmg));
    }

    const base = path.basename(sanitizedDmg, '.dmg');
    const zipName = `${base}-macos-native-${PLATFORM}.zip`;
    releaseAssets.push(zipFile('dist', sanitizedDmg, zipName));
}

// --- Windows native (only on Windows) -----------------------------------
if (process.platform === 'win32') {
    run('pnpm run build:windows');

    // Dynamically find the built native .exe file
    const originalExe = fs.readdirSync('dist').find(f => f.endsWith('.exe') && !f.includes('template'));
    if (!originalExe) throw new Error("No native .exe found in dist/");

    // Force snake_case naming to mirror CI exactly: "Incredible Template.exe" -> "incredible_template.exe"
    const sanitizedExe = originalExe.toLowerCase().replace(/\s+/g, '_');

    if (originalExe !== sanitizedExe) {
        fs.renameSync(path.join('dist', originalExe), path.join('dist', sanitizedExe));
    }

    const base = path.basename(sanitizedExe, '.exe');
    const zipName = `${base}-windows-native.zip`;
    releaseAssets.push(zipFile('dist', sanitizedExe, zipName));
}

console.log('\n=== Done. Produced: ===');
releaseAssets.forEach((f) => console.log(`  ${f}`));
