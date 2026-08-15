## Development

This project builds and packages for four platforms from a single codebase: **Terminal** (native binary), **Web** (WASM on GitHub Pages), **macOS** (native GUI), and **Windows** (native GUI). Make sure you meet the [development prerequisites](DEVELOPMENT_PREREQUISITES.md) first.

**Build the tools** (from the repo root):

```bash
cargo build-tools
tools/copy.js
```

This builds all three tools (config, package, run) and copies the binaries to the project root.

**Configure the app:**

```bash
./config
```

Sets up the app (name, icon, etc.). Can be re-run at any time to change values. App code lives in `src/` — edit `app.rs` and `platform.rs`.

**Run / develop:**

```bash
./run
```

Launches an interactive menu to build and run for a chosen platform (Terminal, Web/WASM, macOS, or Windows).

**Package & publish:**

```bash
./package
```

Opens the interactive menu to bundle (and optionally publish) the app for your chosen targets.

**Publish with GitHub Actions:**

Two workflows in `.github/workflows/` build and distribute for you on GitHub's servers:

- **Create Downloadable Binaries** — builds the Terminal binary for all four platforms (macOS ARM, macOS Intel, Windows, Linux) plus the macOS and Windows native apps, and attaches them as `.zip` assets to a GitHub Release (or a rolling `latest` tag on manual runs).
- **Deploy to GitHub Pages** — builds the WASM/web version and deploys it as a static site, automatically on every push to `main` (or manually).

> Note: the workflows require an `INCREDIBLE_ALPHA` secret (a GitHub token with read access to the private crate) under Settings > Secrets and variables > Actions.
