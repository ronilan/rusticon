import fs from "node:fs";

const names = ["config", "package", "run"];
const name = process.argv[2];

if (name) {
    names.push(name);
}

const toCopy = [...new Set(names)];

for (const n of toCopy) {
    const src = process.platform === "win32"
        ? `target/release/${n}.exe`
        : `target/release/${n}`;

    const dest = process.platform === "win32"
        ? `./${n}.exe`
        : `./${n}`;

    fs.copyFileSync(src, dest);

    if (process.platform !== "win32") {
        fs.chmodSync(dest, 0o755);
    }
}
