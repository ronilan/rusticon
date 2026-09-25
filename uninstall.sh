#!/usr/bin/env bash
#
# uninstall.sh — remove this project's terminal binary that install.sh put on
# the PATH. Designed to be run standalone — download (or curl | bash) and run.
#
# Usage:
#   bash uninstall.sh                 # standard uninstall
#   bash uninstall.sh <binary-name>   # fallback if the release lookup fails
#
# Requires: gh (optional; used when available), sudo.

set -euo pipefail

# --- The only project-specific setting ---------------------------------------
# The GitHub repository (owner/name) whose releases carry the
# `<binary>-terminal-<platform>.zip` assets. Must match install.sh.
REPO="ronilan/rusticon"
# ------------------------------------------------------------------------------

DEST="/usr/local/bin"

# --- 1. Determine the binary name --------------------------------------------
# Preferred: derive it from the latest release assets (same convention as
# install.sh: the asset is "<binary>-terminal-<platform>.zip").
# Fallback: pass the binary name explicitly as the first argument — useful if
# the release is gone or there is no network access.
if [ "${1:-}" ]; then
  bin_name="$1"
else
  echo "Looking up binary name from the latest release of ${REPO}..."
  assets=$(curl -fsSL --retry 3 "https://api.github.com/repos/${REPO}/releases?per_page=1") \
    || assets=""
  if [ -z "$assets" ] && command -v gh >/dev/null 2>&1 && gh auth status >/dev/null 2>&1; then
    assets=$(gh api "repos/${REPO}/releases?per_page=1") || assets=""
  fi
  bin_name=$(printf '%s' "$assets" \
    | grep -o '"name": *"[^"]*-terminal-[^"]*\.zip"' \
    | head -n 1 | cut -d'"' -f4 \
    | sed -E 's/-terminal-[^.]*\.zip$//') || true
  [ -n "$bin_name" ] || {
    echo "error: could not determine the binary name from the latest release." >&2
    echo "If the repository is not publicly accessible, install/authenticate the GitHub CLI (gh auth login)." >&2
    echo "Pass it explicitly: $0 <binary-name>   (e.g. the name you installed)" >&2
    exit 1
  }
fi

# --- 2. Remove the binary ------------------------------------------------------
target="${DEST}/${bin_name}"
if [ ! -f "$target" ]; then
  echo "Nothing to do: $target does not exist."
  exit 0
fi
sudo rm -f "$target"
echo "Removed: $target"
