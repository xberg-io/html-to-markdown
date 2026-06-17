#!/usr/bin/env bash
#
# Populates the cgo FFI library inside the module cache for the
# kreuzberg-dev/html-to-markdown/packages/go/v3 binding, downloading the
# matching pre-built artifact from the GitHub release.
#
# The binding's binding.go declares `#cgo LDFLAGS: -L${SRCDIR}/.lib/<platform>/`
# where ${SRCDIR} is the binding source directory in the Go module cache. The
# tarball published with each h2m release ships the static + shared libs which
# we drop into that directory so cgo links cleanly without any env overrides.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

MODULE_VERSION="$(awk '/github.com\/kreuzberg-dev\/html-to-markdown\/packages\/go\/v3[[:space:]]+v[0-9]/ {gsub(/^v/, "", $2); print $2; exit}' "$SCRIPT_DIR/go.mod")"
if [ -z "${MODULE_VERSION:-}" ]; then
  echo "Could not determine MODULE_VERSION from go.mod" >&2
  exit 1
fi

case "$(uname -s)" in
Darwin)
  case "$(uname -m)" in
  arm64)
    PLATFORM_RUST="aarch64-apple-darwin"
    PLATFORM="macos-arm64"
    ;;
  x86_64)
    PLATFORM_RUST="x86_64-apple-darwin"
    PLATFORM="macos-x86_64"
    ;;
  *)
    echo "Unsupported Darwin arch: $(uname -m)" >&2
    exit 1
    ;;
  esac
  ;;
Linux)
  case "$(uname -m)" in
  x86_64)
    PLATFORM_RUST="x86_64-unknown-linux-gnu"
    PLATFORM="linux-x86_64"
    ;;
  aarch64)
    PLATFORM_RUST="aarch64-unknown-linux-gnu"
    PLATFORM="linux-aarch64"
    ;;
  *)
    echo "Unsupported Linux arch: $(uname -m)" >&2
    exit 1
    ;;
  esac
  ;;
*)
  echo "Unsupported OS: $(uname -s)" >&2
  exit 1
  ;;
esac

GOMODCACHE="$(go env GOMODCACHE)"
BINDING_DIR="$GOMODCACHE/github.com/kreuzberg-dev/html-to-markdown/packages/go/v3@v${MODULE_VERSION}"

if [ ! -d "$BINDING_DIR" ]; then
  cd "$SCRIPT_DIR"
  go mod download github.com/kreuzberg-dev/html-to-markdown/packages/go/v3
fi

if [ ! -d "$BINDING_DIR" ]; then
  echo "Binding directory not found after go mod download: $BINDING_DIR" >&2
  exit 1
fi

LIB_DIR="$BINDING_DIR/.lib/$PLATFORM"
EXPECTED_LIB="$LIB_DIR/libhtml_to_markdown_ffi.a"

if [ -f "$EXPECTED_LIB" ]; then
  echo "FFI library already present at $EXPECTED_LIB"
  exit 0
fi

# Module cache is read-only by default; flip it for this binding only.
chmod -R u+w "$BINDING_DIR"

mkdir -p "$LIB_DIR"

CACHE_DIR="${XDG_CACHE_HOME:-$HOME/.cache}/html-to-markdown-ffi/v${MODULE_VERSION}/$PLATFORM"
mkdir -p "$CACHE_DIR"

if [ ! -f "$CACHE_DIR/libhtml_to_markdown_ffi.a" ]; then
  URL="https://github.com/kreuzberg-dev/html-to-markdown/releases/download/v${MODULE_VERSION}/html-to-markdown-rs-ffi-v${MODULE_VERSION}-${PLATFORM_RUST}.tar.gz"
  echo "Downloading FFI library from: $URL"

  TMPDIR="$(mktemp -d)"
  trap 'rm -rf "$TMPDIR"' EXIT

  curl --fail --location --silent --show-error --output "$TMPDIR/ffi.tar.gz" "$URL"
  tar -xzf "$TMPDIR/ffi.tar.gz" -C "$TMPDIR"

  EXTRACTED_DIR="$TMPDIR/html-to-markdown-rs-ffi-v${MODULE_VERSION}-${PLATFORM_RUST}"
  if [ ! -d "$EXTRACTED_DIR" ]; then
    echo "Tarball did not contain expected directory: $EXTRACTED_DIR" >&2
    exit 1
  fi

  cp "$EXTRACTED_DIR/lib/libhtml_to_markdown_ffi.a" "$CACHE_DIR/"
  if [ -f "$EXTRACTED_DIR/lib/libhtml_to_markdown_ffi.dylib" ]; then
    cp "$EXTRACTED_DIR/lib/libhtml_to_markdown_ffi.dylib" "$CACHE_DIR/"
  fi
fi

cp "$CACHE_DIR/libhtml_to_markdown_ffi.a" "$LIB_DIR/libhtml_to_markdown_ffi.a"
if [ -f "$CACHE_DIR/libhtml_to_markdown_ffi.dylib" ]; then
  cp "$CACHE_DIR/libhtml_to_markdown_ffi.dylib" "$LIB_DIR/libhtml_to_markdown_ffi.dylib"
fi

echo "FFI library staged at $LIB_DIR"
