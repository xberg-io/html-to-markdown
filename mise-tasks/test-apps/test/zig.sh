#!/usr/bin/env bash
#MISE description="Comprehensive tests for Zig package"
#MISE dir="{{config_root}}/test_apps/zig"
set -euo pipefail

version="$(cd ../.. && "$MISE_CONFIG_ROOT/mise-tasks/.lib/cargo-version.sh")"
(cd ../c && bash download_ffi.sh)
zig fetch --save=html_to_markdown \
  "https://github.com/xberg-io/html-to-markdown/releases/download/v$version/html-to-markdown-rs-zig-v$version.tar.gz"
zig build test
