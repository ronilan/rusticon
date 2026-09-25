#!/usr/bin/env bash
#
# install.sh — globally install this project's terminal binary from its latest
# GitHub release. Designed to be run standalone — download and run; no git
# clone or checkout required. Downloads the release asset matching your
# platform and installs the binary into /usr/local/bin.
#
# If `gh` (the GitHub CLI) is installed and authenticated, it is used as a
# fallback for private repos; otherwise anonymous access is used, which works
# for public repos with nothing installed.
#
# Usage:
#   bash install.sh                 # standard install
#   bash install.sh owner/repo      # optional override (e.g. testing a fork)
#
# Requires: gh (optional; used when available), unzip, sudo.

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
# Anonymous curl works for public repos, so it comes first - no account,
# no gh needed. Authenticated gh is only a fallback (e.g. private repos).
use_gh=false
if command -v gh >/dev/null 2>&1 && gh auth status >/dev/null 2>&1; then
  use_gh=true
fi

echo "Looking up latest release for ${REPO}..."
assets=$(curl -fsSL --retry 3 "https://api.github.com/repos/${REPO}/releases?per_page=1") || assets=""
if [ -z "$assets" ] && [ "$use_gh" = true ]; then
  echo "(anonymous lookup failed, retrying authenticated...)"
  assets=$(gh api "repos/${REPO}/releases?per_page=1") || assets=""
fi

asset=$(printf '%s' "$assets" \
  | grep -o "\"name\": *\"[^\"]*-terminal-${platform}\.zip\"" \
  | head -n 1 | cut -d'"' -f4) || true
[ -n "$asset" ] || {
  echo "error: no *-terminal-${platform}.zip asset found in the latest release." >&2
  echo "If the repository is not publicly accessible, install the GitHub CLI" >&2
  echo "and authenticate:  https://cli.github.com   then:   gh auth login" >&2
  exit 1
}

# The asset is "<binary>-terminal-<platform>.zip" containing the bare binary,
# so the binary name is the asset name with the suffix stripped.
bin_name="${asset%-terminal-${platform}.zip}"

# The release tag — gh release download needs it explicitly (the tag may be a
# prerelease like "latest", which gh's implicit "latest" lookup would skip).
release_tag=$(printf '%s' "$assets" \
  | grep -o '"tag_name": *"[^"]*"' \
  | head -n 1 | cut -d'"' -f4) || true

# --- 3. Download and unzip into a temp dir -----------------------------------
# Direct download URL first (no auth needed for public repos); authenticated
# gh download only as fallback.
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT
url=$(printf '%s' "$assets" \
  | grep -o "\"browser_download_url\": *\"[^\"]*/${asset}\"" \
  | cut -d'"' -f4) || true
if [ -n "$url" ] && curl -fSL --retry 3 --progress-bar -o "${tmp}/${asset}" "$url"; then
  :
elif [ "$use_gh" = true ]; then
  gh release download "${release_tag}" --repo "${REPO}" --pattern "${asset}" --dir "${tmp}" --clobber
else
  echo "error: download URL for ${asset} not found" >&2
  exit 1
fi
unzip -o "${tmp}/${asset}" -d "$tmp"

# --- 4. Install onto the PATH -------------------------------------------------
sudo mv "$tmp/${bin_name}" "${DEST}/${bin_name}"
sudo chmod +x "${DEST}/${bin_name}"

echo "Installed: ${DEST}/${bin_name}"
echo "Verify it works: open a new terminal anywhere and run '${bin_name}'"
