#!/usr/bin/env bash
#MISE description="Generate code coverage report (lcov format)"
#USAGE flag "--fail-under-lines <percent>" help="Fail when workspace line coverage is below this floor"
# shellcheck disable=SC2154 # usage_* are set by mise from the #USAGE lines
set -euo pipefail

floor=()
if [ -n "${usage_fail_under_lines:-}" ]; then
  floor=(--fail-under-lines "$usage_fail_under_lines")
fi
RUST_LOG="${RUST_LOG:-info}" cargo llvm-cov --all-features --workspace --exclude html-to-markdown-py \
  --exclude html-to-markdown-php --exclude benchmark-harness --exclude html-to-markdown-wasm-wasmtime-tests \
  --lcov --output-path rust-coverage.lcov "${floor[@]}"
