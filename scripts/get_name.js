import fs from 'fs';

// ---- canonical sources ----

const cargoToml = fs.readFileSync('Cargo.toml', 'utf8');
let pkgJson = {};
try {
    pkgJson = JSON.parse(fs.readFileSync('package.json', 'utf8'));
} catch {
    // package.json is optional for the naming extraction
}

// ---- helpers ----

function extractSection(text, header) {
    const idx = text.indexOf(`[${header}]`);
    if (idx === -1) return '';
    const after = text.slice(idx + header.length + 2);
    const end = after.search(/\n\s*\[/);
    return (end === -1 ? after : after.slice(0, end)).trim();
}

function getField(section, key) {
    const re = new RegExp(`^${key}\\s*=\\s*"(.+)"`, 'm');
    const m = section.match(re);
    return m ? m[1] : '';
}

// ---- extract ----

const pkgSection = extractSection(cargoToml, 'package');
const htmlSection = extractSection(cargoToml, 'package.metadata.html');
const bundleSection = extractSection(cargoToml, 'package.metadata.bundle');

const cargoName = getField(pkgSection, 'name');
const appName   = getField(bundleSection, 'app_name');
const npmName   = pkgJson.name || cargoName.replace(/_/g, '-');

// tagline: strip "AppName :: " prefix from the html title
let title = getField(htmlSection, 'title');
let tagline = title;
if (appName && title.startsWith(appName + ' :: ')) {
    tagline = title.slice(appName.length + 4);
}

const keywords    = getField(htmlSection, 'keywords');
const description = getField(pkgSection, 'description');

// ---- build command ----

const args = [];
args.push(`--name ${cargoName}`);
if (appName  && appName  !== cargoName) args.push(`--app-name "${appName}"`);
if (npmName  && npmName  !== cargoName.replace(/_/g, '-')) args.push(`--npm-name "${npmName}"`);
if (tagline  && tagline  !== appName)   args.push(`--tagline "${tagline}"`);
if (keywords)                            args.push(`--keywords "${keywords}"`);
if (description)                         args.push(`--description "${description}"`);

console.log(`node scripts/set_name.js ${args.join(' ')}`);
