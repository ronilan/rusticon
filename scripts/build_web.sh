#!/bin/bash
#
# Build the WASM/web version and assemble docs/ — no node, no bundler.
#
# Mirrors what the `package` tool does on the main branch: wasm-pack
# compiles Rust -> WebAssembly into pkg/, then the static assets, the
# entry script and pkg/ are copied into docs/, and HTML metadata from
# Cargo.toml ([package.metadata.html]) is injected into index.html.
#
# Usage:
#   scripts/build_web.sh               # release build
#   scripts/build_web.sh --dev         # dev (debug) build
#   scripts/build_web.sh --serve       # release build + serve docs/ at :4627
#   scripts/build_web.sh --dev --serve # dev build + serve docs/ at :4627

set -euo pipefail

cd "$(dirname "$0")/.."

DEV=false
SERVE=false
for arg in "$@"; do
  case "$arg" in
    --dev) DEV=true ;;
    --serve) SERVE=true ;;
    *)
      echo "Unknown option: $arg" >&2
      echo "Usage: $0 [--dev] [--serve]" >&2
      exit 1
      ;;
  esac
done

# --- 1. Compile Rust -> WebAssembly -------------------------------------
if [ "$DEV" = true ]; then
  wasm-pack build --target web --dev
else
  wasm-pack build --target web --release
fi

# --- 2. Recreate docs/ ---------------------------------------------------
rm -rf docs
mkdir -p docs

# Static assets
for asset in style.css favicon.svg; do
  if [ -f "web/$asset" ]; then
    cp "web/$asset" "docs/$asset"
  fi
done

if [ -d web/fonts ]; then
  cp -R web/fonts docs/fonts
fi

# Entry script — lives at docs/ root, imports ./pkg/rusticon.js
if [ -f src/main.js ]; then
  cp src/main.js docs/main.js
fi

# wasm-pack output
if [ -d pkg ]; then
  cp -R pkg docs/pkg
fi

# --- 3. Inject metadata from Cargo.toml into docs/index.html -------------
# Read a single-line string value from a TOML section:
#   toml_string <section> <key>
toml_string() {
  awk -v section="$1" -v key="$2" '
    $0 ~ "^\\[" { in_section = ($0 == "[" section "]") }
    in_section && $1 == key {
      line = $0
      sub(/^[^=]*=/, "", line)          # drop "key ="
      gsub(/^[ \t]+|[ \t]+$/, "", line) # trim
      sub(/^"/, "", line)               # strip opening quote
      sub(/"[ \t]*$/, "", line)         # strip closing quote
      print line
    }
  ' Cargo.toml
}

HTML_META_TITLE=$(toml_string "package.metadata.html" "title")
HTML_META_DESCRIPTION=$(toml_string "package.metadata.html" "description")
HTML_META_KEYWORDS=$(toml_string "package.metadata.html" "keywords")
HTML_META_MIN_WIDTH=$(toml_string "package.metadata.html" "mobile-min-width")
HTML_META_MIN_HEIGHT=$(toml_string "package.metadata.html" "mobile-min-height")
HTML_META_CNAME=$(toml_string "package.metadata.html" "cname")
PACKAGE_VERSION=$(toml_string "package" "version")

cp web/index.html docs/index.html
INDEX=docs/index.html
TMP="$(mktemp)"
trap 'rm -f "$TMP"' EXIT

if [ -n "$HTML_META_TITLE" ]; then
  sed "s|<title>.*</title>|<title>${HTML_META_TITLE}</title>|" "$INDEX" > "$TMP" && mv "$TMP" "$INDEX"
fi
if [ -n "$HTML_META_MIN_WIDTH" ]; then
  sed "s|data-min-width=\"[^\"]*\"|data-min-width=\"${HTML_META_MIN_WIDTH}\"|" "$INDEX" > "$TMP" && mv "$TMP" "$INDEX"
fi
if [ -n "$HTML_META_MIN_HEIGHT" ]; then
  sed "s|data-min-height=\"[^\"]*\"|data-min-height=\"${HTML_META_MIN_HEIGHT}\"|" "$INDEX" > "$TMP" && mv "$TMP" "$INDEX"
fi

EXTRA=""
if [ -n "$HTML_META_DESCRIPTION" ]; then
  EXTRA="${EXTRA}<meta name=\"description\" content=\"${HTML_META_DESCRIPTION}\">\n"
fi
if [ -n "$HTML_META_KEYWORDS" ]; then
  EXTRA="${EXTRA}<meta name=\"keywords\" content=\"${HTML_META_KEYWORDS}\">\n"
fi
if [ -n "$PACKAGE_VERSION" ]; then
  EXTRA="${EXTRA}<meta name=\"version\" content=\"${PACKAGE_VERSION}\">\n"
fi
if [ -n "$EXTRA" ]; then
  # Insert the extra <meta> tags just before </head>.
  awk -v extra="$EXTRA" '
    /<\/head>/ { printf "%s%s\n", extra, $0; next }
    { print }
  ' "$INDEX" > "$TMP" && mv "$TMP" "$INDEX"
fi

if [ -n "$HTML_META_CNAME" ]; then
  printf '%s\n' "$HTML_META_CNAME" > docs/CNAME
fi

echo "Web build complete. Wrote docs/"

# --- 4. Serve (optional) -------------------------------------------------
if [ "$SERVE" = true ]; then
  echo "Serving docs/ at http://localhost:4627"
  python3 -m http.server 4627 --directory docs
fi