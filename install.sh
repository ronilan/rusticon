#!/usr/bin/env bash
#
# install.sh — globally install this project's terminal binary from its latest
# GitHub release. Designed to be run standalone — download and run; no git
# clone or checkout required. Downloads the release asset matching your
# platform and installs the binary into /usr/local/bin.
#
# The repository and its release assets must be publicly accessible.
#
# Usage:
#   bash install.sh                 # standard install
#   bash install.sh owner/repo      # optional override (e.g. testing a public fork)
#
# Requires: curl, unzip, sudo.

set -euo pipefail

# --- The only project-specific setting ---------------------------------------
# The GitHub repository (owner/name) whose releases carry the
# `<binary>-terminal-<platform>.zip` assets built by
# .github/workflows/downloadable_binaries.yml.
REPO="ronilan/rusticon"
# ------------------------------------------------------------------------------

DEST="/usr/local/bin"

# Optional one-off override: ./install.sh owner/repo
[ "${1:-}" ] && REPO="$1"

# --- 1. Detect platform and map it to the release asset platform suffix -----
case "$(uname -s)-$(uname -m)" in
  Darwin-arm64)  platform="macos-arm" ;;
  Darwin-x86_64) platform="macos-intel" ;;
  Linux-x86_64)  platform="linux" ;;
  *)
    echo "error: unsupported platform '$(uname -s)-$(uname -m)'" >&2
    echo "(release binaries exist for macOS arm/intel and Linux x86_64)" >&2
    exit 1
    ;;
esac

# --- 2. Fetch the latest release metadata ------------------------------------
# Note: /releases/latest excludes drafts and prereleases (404s if the most
# recent release is one), so we list /releases and take the most recent.
#
echo "Looking up latest release for ${REPO}..."
assets=$(curl -fsSL --retry 3 "https://api.github.com/repos/${REPO}/releases?per_page=1") || assets=""

asset=$(printf '%s' "$assets" \
  | grep -o "\"name\": *\"[^\"]*-terminal-${platform}\.zip\"" \
  | head -n 1 | cut -d'"' -f4) || true
[ -n "$asset" ] || {
  echo "error: no *-terminal-${platform}.zip asset found in the latest release." >&2
  echo "Verify that ${REPO} and its release assets are publicly accessible." >&2
  exit 1
}

# The asset is "<binary>-terminal-<platform>.zip" containing the bare binary,
# so the binary name is the asset name with the suffix stripped.
bin_name="${asset%-terminal-${platform}.zip}"

# --- 3. Download and unzip into a temp dir -----------------------------------
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT
url=$(printf '%s' "$assets" \
  | grep -o "\"browser_download_url\": *\"[^\"]*/${asset}\"" \
  | cut -d'"' -f4) || true
if [ -z "$url" ]; then
  echo "error: download URL for ${asset} not found" >&2
  exit 1
fi
curl -fSL --retry 3 --progress-bar -o "${tmp}/${asset}" "$url"
unzip -o "${tmp}/${asset}" -d "$tmp"

# --- 4. Install onto the PATH -------------------------------------------------
sudo mv "$tmp/${bin_name}" "${DEST}/${bin_name}"
sudo chmod +x "${DEST}/${bin_name}"

echo "Installed: ${DEST}/${bin_name}"
echo "Verify it works: open a new terminal anywhere and run '${bin_name}'"
