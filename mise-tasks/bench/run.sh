#!/usr/bin/env bash
#MISE description="Run the benchmark harness over the fixture corpus"
#USAGE flag "--results-file <path>" env="RESULTS_FILE" default="tools/benchmark-harness/results/latest.json"
# shellcheck disable=SC2154 # usage_* are set by mise from the #USAGE lines
set -euo pipefail

export HTMBENCH_LOG=htmbench=info,html_to_markdown=error
mkdir -p tools/benchmark-harness/results
cargo run --release -p html-to-markdown-bench -- run --output "$usage_results_file"
