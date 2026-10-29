#!/usr/bin/env bash
#MISE description="Regenerate every alef output that carries the version"
set -euo pipefail

# ~keep `alef sync-versions` rewrites MANIFESTS ONLY. Everything generated from the
# version -- README install snippets, test-app installers, e2e download scripts, stubs,
# scaffolding -- is produced by separate subcommands, so a bump that stops at
# sync-versions leaves generated files contradicting the manifests beside them.
# Two releases have been tagged in exactly that state and reached no registry. Each
# command below writes a different part of that surface; none is redundant with another.
#
# `alef docs` is deliberately NOT here. It aborts on doc-snippet typechecking, and when
# it aborts it has already written the reference pages WITHOUT their `alef:hash:` lines,
# which flips `alef verify` from pass to fail. Run it separately and deliberately.
alef generate --skip-compile
alef stubs
alef scaffold
alef readme
alef test-apps generate
alef e2e generate
