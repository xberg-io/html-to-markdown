#!/usr/bin/env bash
#MISE description="Run alef verify and fail on drifted frozen paths too"
set -euo pipefail

# ~keep The gate. A bump is not finished because the commands before it exited 0 -- each
# reports only on what it writes. `alef verify` is the only step that looks at the whole
# managed surface at once. Read its output: it exits 0 while still reporting frozen paths
# that drifted, so a zero exit code is necessary and not sufficient.
verify_log=$(mktemp)
trap 'rm -f "$verify_log"' EXIT
alef verify 2>&1 | tee "$verify_log"
# ~keep `alef verify` deliberately does NOT set a failure exit code for a drifted
# create-once seed: upstream reasons that a consumer cannot fix content the write
# guard refuses to overwrite, so failing there would only buy a blanket
# `user_owned` opt-out. We own these files, so here it IS fixable and IS fatal --
# a drifted version-bearing installer is exactly how one repository shipped a
# release-candidate artifact URL for three releases. `alef verify --help` names this
# grep as the supported way to make it fatal.
if grep -q 'frozen path(s) DRIFTED' "$verify_log"; then
  echo "ERROR: alef verify reported drifted frozen paths (see the DRIFTED block above)." >&2
  echo "       Fix the file, or declare it [workspace.ownership] user_owned in alef.toml once reviewed." >&2
  exit 1
fi
