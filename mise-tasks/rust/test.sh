#!/usr/bin/env bash
#MISE description="Run Rust test suite"
set -euo pipefail

export RUST_LOG="${RUST_LOG:-info}" RUST_BACKTRACE="${RUST_BACKTRACE:-1}"
cargo test --release --no-default-features -p html-to-markdown-rs
cargo test --release --workspace --exclude html-to-markdown-py --exclude html-to-markdown-php
