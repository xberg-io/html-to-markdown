#!/usr/bin/env bash
# Stage the locally built FFI cdylib where packages/go/binding.go's cgo LDFLAGS look for it.
#
# The published module resolves this through `cmd/setup`, which downloads the release asset for
# the module version. CI must link the working tree instead, so it copies target/release straight
# into .lib/<platform>/. ~keep
set -euo pipefail

os=$(uname -s)
arch=$(uname -m)

case "$os" in
Darwin)
  case "$arch" in
  arm64) platform="macos-arm64" ;;
  x86_64) platform="macos-x86_64" ;;
  *)
    echo "unsupported macOS arch: $arch" >&2
    exit 1
    ;;
  esac
  src="target/release/libhtml_to_markdown_ffi.dylib"
  dst_name="libhtml_to_markdown_ffi.dylib"
  ;;
Linux)
  case "$arch" in
  x86_64) platform="linux-x86_64" ;;
  aarch64 | arm64) platform="linux-aarch64" ;;
  *)
    echo "unsupported Linux arch: $arch" >&2
    exit 1
    ;;
  esac
  src="target/release/libhtml_to_markdown_ffi.so"
  dst_name="libhtml_to_markdown_ffi.so"
  ;;
MINGW* | MSYS* | CYGWIN* | Windows_NT)
  case "$arch" in
  x86_64 | AMD64) platform="windows-x86_64" ;;
  *)
    echo "unsupported Windows arch: $arch" >&2
    exit 1
    ;;
  esac
  src="target/release/html_to_markdown_ffi.dll"
  dst_name="html_to_markdown_ffi.dll"
  ;;
*)
  echo "unsupported OS: $os" >&2
  exit 1
  ;;
esac

dst_dir="packages/go/.lib/$platform"
mkdir -p "$dst_dir"
cp "$src" "$dst_dir/$dst_name"
echo "copied $src -> $dst_dir/$dst_name"
