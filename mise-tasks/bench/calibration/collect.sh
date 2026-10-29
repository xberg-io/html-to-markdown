#!/usr/bin/env bash
#MISE description="Collect exactly 40 quiet schema-v2 full-corpus calibration captures"
#USAGE flag "--artifact-dir <dir>" env="ARTIFACT_DIR" default="benchmark-calibration"
#USAGE flag "--bench-bin <path>" env="BENCH_BIN" default="target/release/htmbench"
# shellcheck disable=SC2154 # usage_* are set by mise from the #USAGE lines
set -euo pipefail

if [ "$(uname -s)" != Linux ]; then
  echo "bench:calibration:collect reads /proc and lscpu and runs on Linux only" >&2
  exit 1
fi
export HTMBENCH_LOG=htmbench=info,html_to_markdown=error
artifact_dir="$usage_artifact_dir"
bench_bin="$usage_bench_bin"
runs_dir="$artifact_dir/runs"
logs_dir="$artifact_dir/logs"
mkdir -p "$runs_dir" "$logs_dir"
if find "$runs_dir" -mindepth 1 -print -quit | grep -q .; then
  echo "calibration runs directory must be empty: $runs_dir" >&2
  exit 1
fi
{
  date --iso-8601=seconds
  uname -a
  lscpu
  rustc -Vv
  cargo -V
  git rev-parse HEAD
  printf 'ImageOS=%s\n' "${ImageOS:-}"
  printf 'ImageVersion=%s\n' "${ImageVersion:-}"
  printf 'RUNNER_ARCH=%s\n' "${RUNNER_ARCH:-}"
  printf 'RUNNER_ENVIRONMENT=%s\n' "${RUNNER_ENVIRONMENT:-}"
  printf 'HTMBENCH_RUNNER_CLASS=%s\n' "${HTMBENCH_RUNNER_CLASS:-}"
  printf 'RUNNER_OS=%s\n' "${RUNNER_OS:-}"
  printf 'GITHUB_RUN_ID=%s\n' "${GITHUB_RUN_ID:-}"
  printf 'GITHUB_SHA=%s\n' "${GITHUB_SHA:-}"
} >"$logs_dir/provenance.log"
cargo build --release -p html-to-markdown-bench --bin htmbench 2>&1 | tee "$logs_dir/build.log"
test -x "$bench_bin"
python3 scripts/benchmark_telemetry.py collect \
  --telemetry "$logs_dir/telemetry.jsonl" --runs-dir "$runs_dir" --binary "$bench_bin" --count 40
python3 scripts/benchmark_telemetry.py validate \
  --telemetry "$logs_dir/telemetry.jsonl" --runs-dir "$runs_dir" --count 40
