#!/usr/bin/env bash
#MISE description="Provision machine preferences and install reviewed AI governance hooks"
set -euo pipefail

command -v ai-rulez >/dev/null || {
  echo "ai-rulez is not on PATH; install it with npm install -g ai-rulez@latest (Node 22 or later)" >&2
  exit 1
}

# poly runs hooks only from the channels poly.local.toml (untracked, per machine) allows; the reviewed
# ai-rulez hooks come from the system channel. ~keep
preferences=poly.local.toml
if ! grep -q '^\[hook_preferences\]' "$preferences" 2>/dev/null; then
  if [ -s "$preferences" ]; then
    printf '\n' >>"$preferences"
  fi
  printf '[hook_preferences]\nchannels = ["system"]\n' >>"$preferences"
elif ! awk '/^\[/ { in_table = ($0 == "[hook_preferences]") }
  in_table && /^channels[[:space:]]*=/ && /"system"/ { found = 1 }
  END { exit !found }' "$preferences"; then
  echo "Add \"system\" to hook_preferences.channels in $preferences" >&2
  exit 1
fi

poly hooks install
