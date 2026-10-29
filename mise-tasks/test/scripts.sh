#!/usr/bin/env bash
#MISE description="Run the Bats contract tests for the repo's own shell scripts"
set -euo pipefail

# Suites load the shared helpers with `bats_load_library xberg-bats`, which searches
# BATS_LIB_PATH and nothing else. CI sets the same directory for the bats job, so a local
# run and a CI run load identical bytes. ~keep
export BATS_LIB_PATH="$MISE_CONFIG_ROOT/scripts/tests/lib"
command -v bats >/dev/null || {
  echo "bats is required: mise install bats" >&2
  exit 1
}
test -f scripts/tests/lib/xberg-bats/load.bash || {
  echo "shared Bats helpers are missing; run \`mise run test:scripts:lib:sync\`" >&2
  exit 1
}
bats --recursive --print-output-on-failure scripts/tests/bats
