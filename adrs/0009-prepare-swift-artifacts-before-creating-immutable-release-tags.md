---
status: proposed
date: 2026-11-04
decision-makers:
  - Naaman Hirschfeld
---

# Prepare Swift artifacts before creating immutable release tags

## Context and Problem Statement

SwiftPM resolves the root Package.swift from the core semver tag. The previous release job built its binary ZIP after publication began, committed a checksum child and force-moved both the core tag and a versioned Swift branch. That invalidated the original tag object and let divergent consumers resolve different native source identities under one version.

## Considered Options

* Continue writing checksum children and moving published refs.
* Publish a separate Swift-only source tree under a different semver lineage.
* Prepare exact native bytes before tagging and allow only a checksum substitution in the final source commit.

## Decision Outcome

Chosen option: "Prepare exact native bytes before tagging and allow only a checksum substitution in the final source commit", because ordinary Swift semver consumers and the legacy Swift branch must resolve the same immutable native source.

A release takes two steps, the shared xberg-io procedure described in the [prepare-swift-release README](https://github.com/xberg-io/actions/blob/v1/prepare-swift-release/README.md):

1. After the version bump lands on main, dispatch **Prepare Swift release** (`.github/workflows/prepare-swift-release.yaml`), which calls `xberg-io/actions/.github/workflows/reusable-swift-release-prepare.yml@v1`. It builds the seven-platform HtmlToMarkdown-rs bundle (binary target RustBridgeBinary) once on macOS from clean source A, with a receipt that records the ZIP's SHA256, repository, source, version, archive name and triples. It pins that checksum into the root Package.swift and verifies the pin commit B changes nothing but that checksum, byte for byte. It pushes B and the core `vX.Y.Z` tag at B together, without force, and attaches the exact ZIP and receipt to a draft release. The ZIP is not reproducible, so a failed run is recovered with "Re-run failed jobs" from the same uploaded bytes, never a rebuild. A dry run stops after the build and prints the ref plan. Inner artifact metadata version 1.0.0 is independent of the product version.
2. Dispatch **Publish Release** on the tag, for example `gh workflow run publish.yaml --ref "$tag" -f tag="$tag"`. Before any core publisher runs, its prepare job downloads the seeded ZIP and receipt and verifies them against the tagged manifest with `xberg-io/actions/publish-swift-release@v1` as a dry run. The Swift job requires the tag object prepare recorded and runs the same action against the release asset, which creates release/swift/<version> at B create-only and rejects a divergent branch. Before draft promotion the workflow checks the live asset digests, the tag object and the verified release ID, then promotes by ID. Publish Release never builds, re-zips, amends or retags the Swift release.

No separate Swift tag namespace is introduced. Real republish/force options fail before credentials are requested. NuGet recovery admits a source run only when its head SHA is the immutable tag commit and verifies the same seed with the shared action before packing or publishing. The separate CLI proxy version lineage and existing Go module finalization policy remain independent.

### Consequences

* Good, because the core semver tag, the Swift branch, manifest checksum and exact native ZIP remain bound to one verified release source.
* Good, because first publication and retries preserve the annotated tag object and reject conflicting metadata before ref or registry writes.
* Bad, because native preparation and receipt retention must complete before tagging; changing native inputs requires preparing a new source A.

### Confirmation

`scripts/tests/test_immutable_swift_release.py` asserts that the Prepare Swift release caller passes this repository's bundle identity to the shared workflow at `@v1`, that Publish Release never builds the bundle, and that no workflow step re-tags, force-pushes or deletes a ref. It also checks that both seed entry points verify with `publish-swift-release` as a dry run before publishing. It runs the tag-object guard, seed download, NuGet recovery source check and promotion checks against a disposable Git remote. The shared actions carry their own regressions for pinning, verification, first publish and retry.
