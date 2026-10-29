#!/usr/bin/env bash
#MISE description="Run the prebuilt benchmark harness without compiling"
#USAGE flag "--results-file <path>" env="RESULTS_FILE" default="tools/benchmark-harness/results/latest.json"
#USAGE flag "--bench-bin <path>" env="BENCH_BIN" default="target/release/htmbench"
# shellcheck disable=SC2154 # usage_* are set by mise from the #USAGE lines
set -euo pipefail

export HTMBENCH_LOG=htmbench=info,html_to_markdown=error
test -x "$usage_bench_bin" || {
  echo "Build the benchmark harness with mise run bench:build before timing" >&2
  exit 1
}
mkdir -p tools/benchmark-harness/results
"$usage_bench_bin" run --output "$usage_results_file"
