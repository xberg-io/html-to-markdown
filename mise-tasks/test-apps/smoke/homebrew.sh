#!/usr/bin/env bash
#MISE description="Smoke test the Homebrew CLI + FFI formulae"
#MISE dir="{{config_root}}/test_apps/homebrew"
set -euo pipefail

version="$(cd ../.. && "$MISE_CONFIG_ROOT/mise-tasks/.lib/cargo-version.sh")"
brew bundle install --file=Brewfile
test "$("$(brew --prefix html-to-markdown)/bin/html-to-markdown" --version)" = "html-to-markdown $version"
ffi_prefix="$(brew --prefix libhtml-to-markdown)"
probe_dir="$(mktemp -d)"
trap 'rm -rf "$probe_dir"' EXIT
printf '#include <stdio.h>\n#include <html_to_markdown.h>\nint main(void) { puts(htm_version()); return 0; }\n' |
  cc -x c -I"$ffi_prefix/include" -L"$ffi_prefix/lib" -Wl,-rpath,"$ffi_prefix/lib" \
    -lhtml_to_markdown_ffi -o "$probe_dir/version" -
test "$("$probe_dir/version")" = "$version"
