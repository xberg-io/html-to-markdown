#!/usr/bin/env bash
set -euo pipefail

REPO="xberg-io/html-to-markdown"
# ~keep Rewritten by scripts/sync_plugin_version.py on every release. The plugin only accepts a
# ~keep binary at or above this version, and downloads exactly this release, so a plugin update
# ~keep cannot keep running an older html-to-markdown that happens to be installed.
PINNED_VERSION="3.23.1"

log() { printf 'html-to-markdown-launch: %s\n' "$*" >&2; }
die() {
  log "error: $*"
  exit 1
}

LAUNCHER="${HTML_TO_MARKDOWN_LAUNCHER:-auto}"
case "$LAUNCHER" in
auto | brew | download) ;;
*) die "invalid HTML_TO_MARKDOWN_LAUNCHER='$LAUNCHER' (expected auto|brew|download)" ;;
esac

want() { [ "$LAUNCHER" = "auto" ] || [ "$LAUNCHER" = "$1" ]; }

PLUGIN_ROOT="${CLAUDE_PLUGIN_ROOT:-}"
if [ -z "$PLUGIN_ROOT" ]; then
  PLUGIN_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
fi

BINARY_NAME="html-to-markdown"
case "$(uname -s)" in
MINGW* | MSYS* | CYGWIN* | Windows_NT) BINARY_NAME="html-to-markdown.exe" ;;
esac
BIN_DIR="$PLUGIN_ROOT/bin"
BIN="$BIN_DIR/$BINARY_NAME"

have() { command -v "$1" >/dev/null 2>&1; }

# True when $1 runs and reports a version at or above PINNED_VERSION. A prerelease of a version
# sorts below that version (SemVer), so 3.23.0-rc.1 does not satisfy a 3.23.0 pin. ~keep
runnable() {
  local out found base
  [ -x "$1" ] || return 1
  out="$("$1" --version 2>/dev/null)" || return 1
  found="$(printf '%s\n' "$out" | grep -Eo '[0-9]+\.[0-9]+\.[0-9]+(-[0-9A-Za-z.-]+)?' | head -n 1)"
  [ -n "$found" ] || return 1
  base="${found%%-*}"
  if [ "$(printf '%s\n%s\n' "$PINNED_VERSION" "$base" | sort -V | head -n 1)" != "$PINNED_VERSION" ] ||
    { [ "$base" = "$PINNED_VERSION" ] && [ "$found" != "$base" ]; }; then
    log "ignoring $1: version $found is older than the pinned $PINNED_VERSION"
    return 1
  fi
}

if runnable "$BIN"; then
  log "using cached html-to-markdown at $BIN"
  exec "$BIN" "$@"
fi
if have "$BINARY_NAME"; then
  PATH_BIN="$(command -v "$BINARY_NAME")"
  if runnable "$PATH_BIN"; then
    log "using html-to-markdown on PATH ($PATH_BIN)"
    exec "$PATH_BIN" "$@"
  fi
fi

if want brew && have brew; then
  log "installing via 'brew install xberg-io/tap/html-to-markdown' ..."
  if brew install xberg-io/tap/html-to-markdown >&2; then
    if have "$BINARY_NAME"; then
      BREW_BIN="$(command -v "$BINARY_NAME")"
      runnable "$BREW_BIN" && exec "$BREW_BIN" "$@"
    fi
    log "brew install reported success but html-to-markdown is not on PATH; falling through"
  else
    log "brew install failed; falling through"
  fi
fi

try_download() {
  local arch triple ext base_url asset asset_url tmp ex src_dir
  local sums_asset sums_url sums_fetched expected_sum actual_sum
  arch="$(uname -m)"
  case "$(uname -s)" in
  Darwin)
    case "$arch" in
    arm64 | aarch64) triple="aarch64-apple-darwin" ;;
    *)
      log "no prebuilt macOS archive for $arch (only Apple Silicon is published); falling through"
      return 1
      ;;
    esac
    ;;
  Linux)
    case "$arch" in
    aarch64 | arm64) triple="aarch64-unknown-linux-gnu" ;;
    x86_64) triple="x86_64-unknown-linux-gnu" ;;
    *)
      log "no prebuilt Linux archive for $arch; falling through"
      return 1
      ;;
    esac
    ;;
  MINGW* | MSYS* | CYGWIN* | Windows_NT) triple="x86_64-pc-windows-msvc" ;;
  *)
    log "no prebuilt archive for $(uname -s)/$arch; falling through"
    return 1
    ;;
  esac
  case "$triple" in
  *windows*) ext="zip" ;;
  *) ext="tar.gz" ;;
  esac

  base_url="https://github.com/${REPO}/releases/download/v${PINNED_VERSION}"
  asset="cli-${triple}.${ext}"
  asset_url="${base_url}/${asset}"
  sums_asset="cli-SHA256SUMS"
  sums_url="${base_url}/${sums_asset}"

  if have curl; then
    fetch() { curl -fsSL --retry 3 -o "$2" "$1"; }
  elif have wget; then
    fetch() { wget -q -O "$2" "$1"; }
  else
    log "no curl or wget available for download; falling through"
    return 1
  fi

  tmp="$(mktemp -d)"
  # shellcheck disable=SC2064  # expand $tmp now so the trap removes this exact dir
  trap "rm -rf '$tmp'" RETURN

  log "downloading $asset from release v$PINNED_VERSION ..."
  if ! fetch "$asset_url" "$tmp/$asset"; then
    log "download failed or asset not found: $asset_url; falling through"
    return 1
  fi

  # Right after a release the checksums asset can briefly 404 while GitHub
  # finishes propagating release assets; retry a few times before giving up.
  sums_fetched=0
  for _attempt in 1 2 3 4 5; do
    if fetch "$sums_url" "$tmp/$sums_asset" 2>/dev/null; then
      sums_fetched=1
      break
    fi
    sleep 2
  done
  if [ "$sums_fetched" -ne 1 ]; then
    log "could not download $sums_asset after retries; refusing to execute an unverified binary; falling through"
    return 1
  fi

  expected_sum="$(awk -v asset="$asset" '$2 == asset { print $1 }' "$tmp/$sums_asset")"
  if [ -z "$expected_sum" ]; then
    log "no checksum entry for $asset in $sums_asset; refusing to execute an unverified binary; falling through"
    return 1
  fi

  if have sha256sum; then
    actual_sum="$(sha256sum "$tmp/$asset" | awk '{print $1}')"
  elif have shasum; then
    actual_sum="$(shasum -a 256 "$tmp/$asset" | awk '{print $1}')"
  else
    log "no sha256sum or shasum available to verify $asset; refusing to execute an unverified binary; falling through"
    return 1
  fi

  if [ "$actual_sum" != "$expected_sum" ]; then
    die "checksum mismatch for $asset (expected $expected_sum, got $actual_sum) — refusing to execute"
  fi
  log "checksum verified for $asset"

  ex="$tmp/extracted"
  mkdir -p "$ex"
  case "$ext" in
  tar.gz) tar -xzf "$tmp/$asset" -C "$ex" || {
    log "extraction failed; falling through"
    return 1
  } ;;
  zip)
    if have unzip; then
      unzip -qo "$tmp/$asset" -d "$ex" || {
        log "extraction failed; falling through"
        return 1
      }
    else
      log "need unzip to extract $asset; falling through"
      return 1
    fi
    ;;
  esac

  src_dir="$ex/cli-${triple}"
  [ -d "$src_dir" ] || src_dir="$ex"
  if [ ! -f "$src_dir/$BINARY_NAME" ]; then
    log "binary $BINARY_NAME not found inside $asset; falling through"
    return 1
  fi

  rm -rf "$BIN_DIR"
  mkdir -p "$BIN_DIR"
  mv "$src_dir"/* "$BIN_DIR"/
  chmod +x "$BIN"
  log "installed html-to-markdown to $BIN"
  return 0
}

if want download; then
  if try_download; then
    exec "$BIN" "$@"
  fi
fi

die "could not locate or install html-to-markdown. Install it manually with one of:
  brew install xberg-io/tap/html-to-markdown
  cargo install html-to-markdown-cli
  or download a prebuilt archive from https://github.com/${REPO}/releases/tag/v${PINNED_VERSION}
then ensure 'html-to-markdown' is on PATH and retry."
