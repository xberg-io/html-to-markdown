#!/usr/bin/env bash
#MISE description="Run CI tests; generate coverage on Linux and macOS"
set -euo pipefail

export RUST_LOG="${RUST_LOG:-info}" RUST_BACKTRACE="${RUST_BACKTRACE:-1}"
case "$(uname -s)" in
MINGW* | MSYS* | CYGWIN*)
  # ~keep llvm-cov's object list exceeds Windows' command-line limit; Linux and macOS collect coverage.
  cargo test --features metadata,visitor,inline-images --workspace --exclude html-to-markdown-py \
    --exclude html-to-markdown-php --exclude html-to-markdown-rs-swift --exclude html-to-markdown-rs-dart \
    --exclude html-to-markdown-bench
  ;;
*)
  coverage=(cargo llvm-cov --all-features --workspace --exclude html-to-markdown-py
    --exclude html-to-markdown-php --exclude html-to-markdown-bench)
  "${coverage[@]}" --lcov --output-path rust-coverage.lcov
  "${coverage[@]}" --summary-only
  ;;
esac
