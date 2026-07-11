import fs from 'fs';
import path from 'path';

const args = process.argv.slice(2);
let name, appName, npmName, description, tagline, keywords;

for (let i = 0; i < args.length; i++) {
    if (args[i] === '--name' && args[i + 1]) {
        name = args[++i];
    } else if (args[i] === '--app-name' && args[i + 1]) {
        appName = args[++i];
    } else if (args[i] === '--npm-name' && args[i + 1]) {
        npmName = args[++i];
    } else if (args[i] === '--description' && args[i + 1]) {
        description = args[++i];
    } else if (args[i] === '--tagline' && args[i + 1]) {
        tagline = args[++i];
    } else if (args[i] === '--keywords' && args[i + 1]) {
        keywords = args[++i];
    } else if (!args[i].startsWith('-')) {
        if (!name) name = args[i];
        else if (!appName) appName = args[i];
    }
}

if (!name) {
    console.error('Usage: node scripts/set_name.js --name <snake_case_name> [--app-name "<Display Name>"] [--npm-name "<npm_name>"] [--tagline "<HTML title>"] [--keywords "<comma,separated,keywords>"] [--description "<description>"]');
    console.error('Example: node scripts/set_name.js --name my_app --app-name "My App" --tagline "My App" --keywords "tui,rust" --description "A minimal My App for terminal, wasm and native mac os targets."');
    process.exit(1);
}

if (!appName) appName = name;
if (!npmName) npmName = name.includes('_') ? name.replace(/_/g, '-') : name;

const displayTitle = tagline ? `${appName} :: ${tagline}` : appName;

function replaceFieldValue(line, key, value) {
    return line.replace(
        new RegExp(`^(\\s*)${key}\\s*=\\s*".*?"`),
        `$1${key} = "${value}"`
    );
}

function transform(content, file) {
    switch (file) {
        case 'Cargo.toml':
            return transformCargoToml(content);
        case 'src/main.rs':
            return transformMainRs(content);
        case 'src/macos.rs':
        case 'src/windows.rs':
            return transformPlatformRs(content);
        case 'src/main.ts':
            return transformMainTs(content);
        case 'Info.plist':
            return transformInfoPlist(content);
        case 'run':
        case 'run.bat':
            return transformRun(content);
        case 'scripts/build_macos.js':
            return transformBuildMacos(content);
        case 'scripts/build_terminal.js':
            return transformBuildTerminal(content);
        case 'scripts/build_windows.js':
            return transformBuildWindows(content);
        case 'pkg/package.json':
            return transformPkgPackageJson(content);
        case 'docs/index.html':
        case 'web/index.html':
            return transformDocsHtml(content);
        case 'package.json':
            return transformRootPackageJson(content);
        default:
            return content;
    }
}

function transformCargoToml(content) {
    const lines = content.split('\n');
    let inPackage = false;
    let inMetadataHtml = false;
    let inMetadataBundle = false;
    let inBin = false;
    let binPath = '';
    let pendingBinNameIdx = -1;

    for (let i = 0; i < lines.length; i++) {
        const trimmed = lines[i].trim();

        if (trimmed === '[package]') {
            inPackage = true;
            inMetadataHtml = false;
            inMetadataBundle = false;
            inBin = false;
        } else if (trimmed === '[package.metadata.html]') {
            inPackage = false;
            inMetadataHtml = true;
            inMetadataBundle = false;
            inBin = false;
        } else if (trimmed === '[package.metadata.bundle]') {
            inPackage = false;
            inMetadataHtml = false;
            inMetadataBundle = true;
            inBin = false;
        } else if (trimmed === '[[bin]]') {
            inPackage = false;
            inMetadataHtml = false;
            inMetadataBundle = false;
            inBin = true;
            binPath = '';
            pendingBinNameIdx = -1;
        } else if (/^\[/.test(trimmed)) {
            inPackage = false;
            inMetadataHtml = false;
            inMetadataBundle = false;
            inBin = false;
        }

        if (inPackage) {
            lines[i] = replaceFieldValue(lines[i], 'name', name);
            lines[i] = replaceFieldValue(lines[i], 'default-run', name);
        }
        if (inMetadataHtml) {
            lines[i] = replaceFieldValue(lines[i], 'title', displayTitle);
            if (description) lines[i] = replaceFieldValue(lines[i], 'description', description);
            if (keywords) lines[i] = replaceFieldValue(lines[i], 'keywords', keywords);
        }
        if (inMetadataBundle) {
            lines[i] = replaceFieldValue(lines[i], 'app_name', appName);
            lines[i] = replaceFieldValue(lines[i], 'binary_name', `${name}_macos`);
        }
        if (inBin) {
            if (/^\s*path\s*=/.test(lines[i])) {
                const match = lines[i].match(/^\s*path\s*=\s*"([^"]+)"/);
                const binPath = match ? match[1] : '';

                if (pendingBinNameIdx >= 0) {
                    let bn = null;

                    // only auto-rename known platform bins
                    if (binPath.includes('macos.rs')) {
                        bn = `${name}_macos`;
                    } else if (binPath.includes('windows.rs')) {
                        bn = `${name}_windows`;
                    } else if (binPath.includes('main.rs')) {
                        bn = name;
                    }

                    // only apply if to recognized
                    if (bn) {
                        lines[pendingBinNameIdx] = replaceFieldValue(
                            lines[pendingBinNameIdx],
                            'name',
                            bn
                        );
                    }

                    pendingBinNameIdx = -1;
                }
            }

            if (/^\s*name\s*=/.test(lines[i])) {
                pendingBinNameIdx = i;
            }
        }
    }
    return lines.join('\n');
}

function transformMainRs(content) {
    let r = content;
    r = r.replace(/'[^']+'(?=\s+binary is for Terminal)/, `'${name}'`);
    r = r.replace(/'[^']+'(?=\s+for the macOS version)/, `'${name}_macos'`);
    return r;
}

function transformPlatformRs(content) {
    // Real Rust syntax is option_env!("APP_NAME").unwrap_or("...") — the
    // parentheses around both the macro arg and unwrap_or arg must be part
    // of the pattern, or this silently matches nothing.
    return content.replace(
        /(option_env!\("APP_NAME"\)\.unwrap_or\(")[^"]*("\))/g,
        `$1${appName}$2`
    );
}

function transformMainTs(content) {
    return content.replace(
        /(from\s+"\.\.\/pkg\/)[^"]+(\.js")/g,
        `$1${name}$2`
    );
}

function transformInfoPlist(content) {
    let r = content;
    r = r.replace(
        /(<key>CFBundleExecutable<\/key>\s*<string>)[^<]+(<\/string>)/,
        `$1${name}_macos$2`
    );
    r = r.replace(
        /(<key>CFBundleName<\/key>\s*<string>)[^<]+(<\/string>)/,
        `$1${appName}$2`
    );
    return r;
}

function transformRun(content) {
    // Match any --bin token and preserve a known _macos/_windows suffix
    // rather than relying on a lookahead, which can't tell "end of the
    // base name" apart from "end of the suffix" when both are \w-chars.
    return content.replace(/(--bin\s+)(\S+)/g, (match, prefix, token) => {
        let suffix = '';
        if (token.endsWith('_macos')) suffix = '_macos';
        else if (token.endsWith('_windows')) suffix = '_windows';
        return `${prefix}${name}${suffix}`;
    });
}

function transformBuildMacos(content) {
    let r = content;
    r = r.replace(
        /(--bin\s+)\w+( --features macos-native)/,
        `$1${name}_macos$2`
    );
    r = r.replace(
        /(path\.join\('target',\s*'release',\s*')[^']+('\))/,
        `$1${name}_macos$2`
    );
    r = r.replace(
        /(path\.join\('dist',\s*')[^']+('\))/,
        `$1${name}_macos$2`
    );
    r = r.replace(
        /('Build successful! Binary copied to dist\/)[^']+(')/,
        `$1${name}_macos$2`
    );
    return r;
}

function transformBuildTerminal(content) {
    let r = content;
    r = r.replace(
        /(run\('cargo build --release --bin\s+)[^']+('\))/,
        `$1${name}$2`
    );
    // Whitespace/newline-tolerant: matches the ternary whether it's on one
    // line or spread across multiple lines (as in the real build_terminal.js).
    // Note: group 2 already contains the literal ".exe'" text, so it must
    // NOT be prefixed with an extra ${name}.exe or "*.exe.exe" results.
    r = r.replace(
        /(\?\s*')[^']+(\.exe'\s*:\s*')[^']+(')/,
        `$1${name}$2${name}$3`
    );
    return r;
}

function transformBuildWindows(content) {
    let r = content;
    // Anchor on the fixed "cargo build --release --bin " prefix inside the
    // execSync string, not on the current binary name value.
    r = r.replace(
        /('cargo build --release --bin\s+)[^']+(')/,
        `$1${name}_windows$2`
    );
    // Anchor on the two preceding fixed path.join args
    // ('target', 'release', ...), not on the current binary filename value.
    // \s* tolerates the multi-line formatting used in the real file.
    r = r.replace(
        /(path\.join\(\s*'target',\s*'release',\s*')[^']+(\.exe'\s*\))/,
        `$1${name}_windows$2`
    );
    return r;
}

function transformPkgPackageJson(content) {
    let r = content;
    r = r.replace(
        /("name"\s*:\s*")[^"]+(",)/,
        `$1${name}$2`
    );
    r = r.replace(
        /"[\w_]+_bg\.wasm"/g,
        `"${name}_bg.wasm"`
    );
    r = r.replace(
        /"[\w_]+\.js"/g,
        `"${name}.js"`
    );
    r = r.replace(
        /"[\w_]+\.d\.ts"/g,
        `"${name}.d.ts"`
    );
    r = r.replace(
        /("main"\s*:\s*")[^"]+(",)/,
        `$1${name}.js$2`
    );
    r = r.replace(
        /("types"\s*:\s*")[^"]+(",)/,
        `$1${name}.d.ts$2`
    );
    return r;
}

function transformDocsHtml(content) {
    let r = content;
    r = r.replace(/<title>.*?<\/title>/, `<title>${displayTitle}</title>`);
    if (description) {
        r = r.replace(
            /(<meta name="description" content=")[^"]*(">)/,
            `$1${description}$2`
        );
    }
    if (keywords) {
        r = r.replace(
            /(<meta name="keywords" content=")[^"]*(">)/,
            `$1${keywords}$2`
        );
    }
    return r;
}

function transformRootPackageJson(content) {
    return content.replace(
        /("name"\s*:\s*")[^"]+(",)/,
        `$1${npmName}$2`
    );
}

const files = [
    'Cargo.toml',
    'src/main.rs',
    'src/macos.rs',
    'src/windows.rs',
    'src/main.ts',
    'Info.plist',
    'run',
    'run.bat',
    'scripts/build_macos.js',
    'scripts/build_terminal.js',
    'scripts/build_windows.js',
    'pkg/package.json',
    'docs/index.html',
    'web/index.html',
    'package.json',
];

let updatedCount = 0;

for (const file of files) {
    const filePath = path.resolve(file);
    if (!fs.existsSync(filePath)) {
        console.warn(`Skip missing: ${file}`);
        continue;
    }

    const original = fs.readFileSync(filePath, 'utf8');
    const updated = transform(original, file);

    if (updated !== original) {
        fs.writeFileSync(filePath, updated, 'utf8');
        console.log(`Updated: ${file}`);
        updatedCount++;
    } else {
        console.log(`Unchanged: ${file}`);
    }
}

console.log(`\nDone! ${updatedCount} file(s) updated.`);
console.log('Next steps:');
console.log('  1. Rebuild your binaries / wasm / web assets to refresh generated artifacts.');
console.log('  2. Consider reviewing and updating docs/CNAME, favicon, and any URL slugs on your hosting platform.');
