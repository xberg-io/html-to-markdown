#!/usr/bin/env bash
# Reconstruct the module layout alef's Swift snippet validator expects.
#
# alef resolves the snippet search path with `swift build --show-bin-path` and then passes
# `-I <bin-path>/Modules`, plus `-I <dir>` for any immediate subdirectory of the bin path
# holding a `module.modulemap` (alef `src/snippets/validators/swift.rs`,
# `swift_module_directories_in`). That is the layout of SwiftPM's *native* build system.
#
# Swift 6.3 made `swiftbuild` the default build system, and it emits a different tree: the
# bin path becomes `.build/out/Products/Debug[-<platform>]`, the `.swiftmodule` files sit
# directly in it, and no `Modules/` directory or generated modulemap is produced anywhere
# under `.build`. So `-I <bin-path>/Modules` names a path that does not exist and every
# snippet fails with `no such module 'HtmlToMarkdown'` -- reported as Unavailable, which
# `alef snippets check --strict` fails the run on. That is a change of toolchain, not of
# this repo: CI Lint went red across a commit range whose tree was byte-identical, because
# the runner's preinstalled Swift moved to 6.4. Reproduced in `swift:6.4` against this
# package: `swift build` exits 0, `--show-bin-path` reports the Products dir, and it has
# no `Modules/`. Upstream: xberg-io/alef#435.
#
# `--build-system native` is not a fix: alef runs its own `swift build --show-bin-path`
# without that flag, so it would still resolve the swiftbuild path. SwiftPM honours no
# environment variable for the build-system choice, so the session `env` cannot set it
# either. Populating the layout after the build is what both sides can agree on.
#
# Idempotent, and a no-op on a toolchain whose native layout alef already understands.
# Delete this script and restore `before = ["swift build"]` once alef reads the swiftbuild
# layout. ~keep
set -euo pipefail

cd "$(dirname "$0")/../../../packages/swift"

bin_path="$(swift build --show-bin-path)"
[ -n "$bin_path" ] && [ -d "$bin_path" ] || {
  echo "swift build --show-bin-path gave no usable directory: '$bin_path'" >&2
  exit 1
}

# The native layout already satisfies alef; leave it untouched.
if compgen -G "$bin_path/Modules/*.swiftmodule" >/dev/null; then
  echo "native module layout already present at $bin_path/Modules; nothing to do"
  exit 0
fi

shopt -s nullglob
modules=("$bin_path"/*.swiftmodule)
shopt -u nullglob
# A zero-length match here means the build produced nothing to point alef at, which would
# otherwise surface as the same `no such module` this script exists to prevent. ~keep
if [ "${#modules[@]}" -eq 0 ]; then
  echo "no .swiftmodule in $bin_path -- did 'swift build' run first?" >&2
  exit 1
fi

mkdir -p "$bin_path/Modules"
for module in "${modules[@]}"; do
  ln -sfn "$module" "$bin_path/Modules/$(basename "$module")"
done

# RustBridgeC is a C target; the swiftbuild layout emits no modulemap for it, so the Swift
# modules above cannot load their own dependency. alef adds `-I <subdir>` for a subdirectory
# holding `module.modulemap` at its top level, so the map and the header go in the same dir.
header="Sources/RustBridgeC/RustBridgeC.h"
[ -f "$header" ] || {
  echo "missing $header" >&2
  exit 1
}
mkdir -p "$bin_path/RustBridgeC"
cp "$header" "$bin_path/RustBridgeC/RustBridgeC.h"
cat >"$bin_path/RustBridgeC/module.modulemap" <<'MODULEMAP'
module RustBridgeC {
    header "RustBridgeC.h"
    export *
}
MODULEMAP

echo "populated ${#modules[@]} module(s) and the RustBridgeC modulemap under $bin_path"
