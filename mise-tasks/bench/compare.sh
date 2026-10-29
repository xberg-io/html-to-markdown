#!/usr/bin/env bash
#MISE description="Compare latest benchmark results against the baseline"
#USAGE flag "--results-file <path>" env="RESULTS_FILE" default="tools/benchmark-harness/results/latest.json"
#USAGE flag "--allow-host-mismatch" env="ALLOW_HOST_MISMATCH" help="Let an unscored run on other hardware pass with a warning"
# shellcheck disable=SC2154 # usage_* are set by mise from the #USAGE lines
set -euo pipefail

# Timings are scored only on the CPU the baseline was calibrated on; on any other CPU the run
# is reported as not scored and fails. --allow-host-mismatch is for heterogeneous CI runner
# pools only: it never relaxes the provenance contract or the fixture inventory, and it changes
# nothing when the hardware matches the baseline. ~keep
extra=()
if [ "${usage_allow_host_mismatch:-false}" = true ]; then
  extra=(--allow-host-mismatch)
fi
cargo run --release -p html-to-markdown-bench -- compare --results "$usage_results_file" "${extra[@]}"
