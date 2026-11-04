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

Prepare the seven-platform HtmlToMarkdown-rs artifact bundle from clean, versioned source A using the same official Alef producer as the final source. Keep the ZIP unchanged and record its SHA256, repository, source, version, canonical archive name and platform metadata in its receipt. Inner artifact metadata version 1.0.0 is independent of the product version. The authored workflow explicitly selects the root manifest's RustBridgeBinary identity.

Stage the downloaded ZIP and receipt under `dist/swift-artifactbundle/` at source A. Run `alef sync-versions` with the same package version to substitute only the root Package.swift checksum, preserving all other source bytes, newline bytes and file mode. Commit B, create the core version tag once at B, and seed a draft release with those exact two files before any actual core publisher runs. Publication verifies the receipt, exact A-to-B transition, tag object and peeled commit before reusing the seed. It never rebuilds, re-zips, amends or retags the Swift release.

The existing release/swift/<version> branch must be absent or already resolve to B. Create a missing branch without force; reject a divergent branch before publication. No separate Swift tag namespace is introduced. Real republish/force options fail before credentials are requested. The independent NuGet recovery entry point verifies the same immutable seed before packing or publishing. The separate CLI proxy version lineage and existing Go module finalization policy remain independent.

### Consequences

* Good, because the core semver tag, the Swift branch, manifest checksum and exact native ZIP remain bound to one verified release source.
* Good, because first publication and retries preserve the annotated tag object and reject conflicting metadata before ref or registry writes.
* Bad, because native preparation and receipt retention must complete before tagging; changing native inputs requires preparing a new source A.

### Confirmation

The executable local Git regressions in scripts/tests/test_immutable_swift_release.py exercise first publish/retry, corrupt or mismatched receipts, a divergent branch, tag replacement, source/mode/newline changes and actual workflow guard bodies. Existing manifest and release-notes tests verify read-only legacy tooling and bounded notes updates. The prepare and NuGet recovery steps in .github/workflows/publish.yaml verify the exact seed before actual core publishing, and the Swift job reuses the original ZIP without rebuilding after the tag.
