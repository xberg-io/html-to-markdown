#!/usr/bin/env bash
#MISE description="Calibrate baseline and fixture floors from exactly 40 quiet full-corpus runs"
#USAGE flag "--runs-dir <dir>" env="RUNS_DIR" required=#true help="Directory of calibration captures"
#USAGE flag "--accept-output-change" env="ACCEPT_OUTPUT_CHANGE" help="Accept a reviewed, intended output change"
# shellcheck disable=SC2154 # usage_* are set by mise from the #USAGE lines
set -euo pipefail

extra=()
if [ -n "${usage_accept_output_change:-}" ] && [ "$usage_accept_output_change" != false ]; then
  extra=(--accept-output-change)
fi
python3 scripts/benchmark_telemetry.py validate \
  --telemetry "$usage_runs_dir/../logs/telemetry.jsonl" --runs-dir "$usage_runs_dir" --count 40
cargo run --release -p html-to-markdown-bench -- calibrate --runs-dir "$usage_runs_dir" \
  --baseline tools/benchmark-harness/baselines/baseline.json \
  --guardrails tools/benchmark-harness/guardrails.json "${extra[@]}"
