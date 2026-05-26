#!/usr/bin/env bash
# Copy the freshly built FFI library into the C# package runtimes/ tree at
# the RID + native filename .NET expects for the host platform. The package
# csproj packs `runtimes/**` verbatim, so the RID directory must match what
# the runtime probes for at NativeLibrary.Load time.
set -euo pipefail

os=$(uname -s)
arch=$(uname -m)

case "$os" in
  Darwin)
    case "$arch" in
      arm64)   rid="osx-arm64" ;;
      x86_64)  rid="osx-x64" ;;
      *)       echo "unsupported macOS arch: $arch" >&2; exit 1 ;;
    esac
    src="target/release/libhtml_to_markdown_ffi.dylib"
    dst_name="html_to_markdown_ffi.dylib"
    ;;
  Linux)
    case "$arch" in
      x86_64)         rid="linux-x64" ;;
      aarch64|arm64)  rid="linux-arm64" ;;
      *)              echo "unsupported Linux arch: $arch" >&2; exit 1 ;;
    esac
    src="target/release/libhtml_to_markdown_ffi.so"
    dst_name="libhtml_to_markdown_ffi.so"
    ;;
  MINGW*|MSYS*|CYGWIN*|Windows_NT)
    case "$arch" in
      x86_64|AMD64)   rid="win-x64" ;;
      aarch64|ARM64)  rid="win-arm64" ;;
      *)              echo "unsupported Windows arch: $arch" >&2; exit 1 ;;
    esac
    src="target/release/html_to_markdown_ffi.dll"
    dst_name="html_to_markdown_ffi.dll"
    ;;
  *)
    echo "unsupported OS: $os" >&2
    exit 1
    ;;
esac

dst_dir="packages/csharp/HtmlToMarkdown/runtimes/$rid/native"
mkdir -p "$dst_dir"
cp "$src" "$dst_dir/$dst_name"
echo "copied $src -> $dst_dir/$dst_name"
