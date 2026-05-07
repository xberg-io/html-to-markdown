#!/usr/bin/env bash
set -euo pipefail

cd packages/typescript

# Build only the TypeScript part (native bindings are already published)
# The native bindings should already be available from npm at this point
# since we depend on publish-node completing first
pnpm exec tsc --project tsconfig.json

# Replace workspace:* with the actual version so the published package is
# installable outside this monorepo. pnpm publish would do this automatically
# but `npm publish` (used by kreuzberg-dev/actions/publish-npm) does not.
version="$(node -p "require('./package.json').version")"
node -e '
  const fs = require("node:fs");
  const path = "package.json";
  const pkg = JSON.parse(fs.readFileSync(path, "utf8"));
  for (const field of ["dependencies", "peerDependencies", "optionalDependencies"]) {
    if (pkg[field]) {
      for (const [name, spec] of Object.entries(pkg[field])) {
        if (typeof spec === "string" && spec.startsWith("workspace:")) {
          pkg[field][name] = process.env.PKG_VERSION;
        }
      }
    }
  }
  fs.writeFileSync(path, JSON.stringify(pkg, null, 2) + "\n");
' PKG_VERSION="$version"

echo "TypeScript wrapper package built; workspace:* rewritten to ${version}"
