#!/usr/bin/env bash
#MISE description="Run release workflow and helper regression tests"
#USAGE arg "[path]..." var=#true help="pytest arguments (default: scripts/tests)"
set -euo pipefail

export PYTHONPATH="$MISE_CONFIG_ROOT/scripts"
[ "$#" -gt 0 ] || set -- scripts/tests
uv run --no-sync python -m pytest -q "$@"
