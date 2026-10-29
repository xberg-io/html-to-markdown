#!/usr/bin/env bash
#MISE description="Refresh the root pnpm and uv lockfiles after a version sync"
#MISE hide=true
set -euo pipefail

# ~keep `alef sync-versions` writes the version INTO manifests; a lockfile is not a
# ~keep manifest, so `alef sync-versions` touches none. In the 3.12.4 release that cost four
# ~keep separate CI failures, each surfacing in a different workflow, discovered one
# ~keep push at a time: the root pnpm lockfile still carried the previous version for
# ~keep `crates/html-to-markdown-node`'s eight platform `optionalDependencies` (they
# ~keep resolve as `link:npm/<target>`, so no unpublished package is fetched), and
# ~keep `uv.lock` for the editable `html-to-markdown` package. Use the PINNED pnpm from
# ~keep `package.json`'s `packageManager`: a different major rewrites the lockfile
# ~keep format. `test_apps/*` lockfiles legitimately lag until publish and are NOT
# ~keep refreshed here -- `alef validate versions` reports those as BLOCKED by design.
pin=$(python3 -c 'import json; print(json.load(open("package.json"))["packageManager"])')
case "$pin" in
pnpm@*) ;;
*)
  echo "ERROR: package.json packageManager is $pin, expected a pnpm@ pin." >&2
  exit 1
  ;;
esac
pnpm dlx "$pin" install --lockfile-only
uv lock
