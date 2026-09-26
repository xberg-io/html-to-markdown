#!/usr/bin/env bash
# Verify the docs-site changelog mirror carries the same entries as CHANGELOG.md.
set -euo pipefail

ROOT="CHANGELOG.md"
MIRROR="docs-site/src/content/docs/changelog.md"

# ~keep Nothing syncs these two files, so they diverged unnoticed: the mirror's 3.15.0 section
# carried the wrong release date, was missing the alef re-pin and the R-binding entries, and
# summarised the Swift fix inaccurately. Everything from the first `## [` heading is compared,
# not just `## [Unreleased]` as the sibling check in crawlberg does: this repo releases straight
# out of `[Unreleased]`, leaving it empty on `main`, so an Unreleased-only comparison would
# compare zero lines and pass while a released section drifted. The mirror's frontmatter and
# preamble differ by design and sit above that heading, so they are excluded.
extract_entries() {
  awk '/^## \[/ { found = 1 } found' "$1"
}

workdir="$(mktemp -d)"
[ -n "$workdir" ] && [ -d "$workdir" ] || exit 90
trap 'rm -rf "$workdir"' EXIT

failures=0
for path in "$ROOT" "$MIRROR"; do
  if [ ! -f "$path" ]; then
    echo "::error::$path is missing"
    failures=$((failures + 1))
  elif ! grep -q '^## \[' "$path"; then
    echo "::error::$path has no '## [' version heading — this check would compare nothing"
    failures=$((failures + 1))
  fi
done
[ "$failures" -eq 0 ] || exit 1

extract_entries "$ROOT" >"$workdir/root.txt"
extract_entries "$MIRROR" >"$workdir/mirror.txt"
root_lines="$(wc -l <"$workdir/root.txt" | tr -d ' ')"
mirror_lines="$(wc -l <"$workdir/mirror.txt" | tr -d ' ')"
echo "changelog entries: $ROOT has $root_lines lines, $MIRROR has $mirror_lines lines"

if [ "$root_lines" -eq 0 ]; then
  echo "::error::no changelog entries were extracted from $ROOT — nothing was compared"
  exit 1
fi

if ! diff -q "$workdir/root.txt" "$workdir/mirror.txt" >/dev/null 2>&1; then
  diff -u "$workdir/root.txt" "$workdir/mirror.txt" | head -60
  echo "::error::$MIRROR is a hand-maintained mirror of $ROOT and has drifted. Copy the changed entries from $ROOT into $MIRROR."
  exit 1
fi

echo "ok: the changelog entries match ($root_lines lines)"
