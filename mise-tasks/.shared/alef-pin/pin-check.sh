#!/usr/bin/env bash
#MISE description="Fail unless the alef on PATH is the clean release alef.toml pins"
set -euo pipefail

# ~keep The pin check runs FIRST, before anything writes. CI installs the
# ~keep `alef_version` from alef.toml, not whatever is on this machine, so regenerating
# ~keep with a different local alef commits output the pinned version cannot reproduce
# ~keep -- and every freshness gate then reads the tree as stale forever, with no diff
# ~keep to explain why. A binary built from a dirty tree is rejected for the same
# ~keep reason: its output is not attributable to any released version.
pinned=$(sed -n 's/^alef_version[[:space:]]*=[[:space:]]*"\([^"]*\)".*/\1/p' alef.toml | head -1)
[ -n "$pinned" ] || {
  echo "ERROR: alef.toml has no alef_version pin." >&2
  exit 1
}
identity=$(alef --version)
running=$(printf '%s\n' "$identity" | head -1 | awk '{print $2}')
if [ "$pinned" != "$running" ]; then
  echo "ERROR: alef.toml pins $pinned but the alef on PATH is $running." >&2
  echo "       Install the pinned version, or bump alef_version to the version that" >&2
  echo "       will actually generate this tree. CI installs the pin." >&2
  exit 1
fi
if printf '%s\n' "$identity" | grep -q 'tree:.*DIRTY'; then
  echo "ERROR: the alef on PATH was built from a dirty working tree, so its output" >&2
  echo "       is not reproducible from any released version." >&2
  exit 1
fi
