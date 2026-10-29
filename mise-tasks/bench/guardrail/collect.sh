#!/usr/bin/env bash
#MISE description="Capture one prebuilt benchmark with the same continuous telemetry policy as calibration"
#USAGE flag "--artifact-dir <dir>" env="ARTIFACT_DIR" default="benchmark-guardrail"
#USAGE flag "--bench-bin <path>" env="BENCH_BIN" default="target/release/htmbench"
# shellcheck disable=SC2154 # usage_* are set by mise from the #USAGE lines
set -euo pipefail

if [ "$(uname -s)" != Linux ]; then
  echo "bench:guardrail:collect reads /proc and runs on Linux only" >&2
  exit 1
fi
export HTMBENCH_LOG=htmbench=info,html_to_markdown=error
artifact_dir="$usage_artifact_dir"
python3 scripts/benchmark_telemetry.py collect \
  --telemetry "$artifact_dir/logs/telemetry.jsonl" --runs-dir "$artifact_dir/runs" \
  --binary "$usage_bench_bin" --count 1
python3 scripts/benchmark_telemetry.py validate \
  --telemetry "$artifact_dir/logs/telemetry.jsonl" --runs-dir "$artifact_dir/runs" --count 1
mkdir -p tools/benchmark-harness/results
cp "$artifact_dir/runs/0001.json" tools/benchmark-harness/results/latest.json
