"""Sync release pins in maintained consumer manifests without replacing their contents."""

from __future__ import annotations

import argparse
import re
import tomllib
from pathlib import Path

CONSUMER_PINS = (
    ("test_apps/dart/pubspec.yaml", r"(h2m: \^)[^\s]+", 1),
    ("test_apps/elixir/mix.exs", r'(html_to_markdown, "~> )[^\"]+', 1),
    ("test_apps/kotlin_android/build.gradle.kts", r'(io.xberg:html-to-markdown-android:)[^"\s]+', 2),
    ("e2e/go/go.mod", r"(github\.com/xberg-io/html-to-markdown/packages/go/v3[ \t]+v)[^\s]+", 1),
    # ~keep The CLI proxy packages and the MCP registry entry ship in lockstep with the product:
    # the proxy downloads the release asset of exactly its own version, and server.json names it.
    ("cli-proxy/npm/package.json", r'(?m)^(  "version": ")[^"]+', 1),
    ("cli-proxy/pypi/pyproject.toml", r'(?m)^(version = ")[^"]+', 1),
    ("server.json", r'(?m)^(  "version": ")[^"]+', 1),
    ("server.json", r'(?m)^(      "version": ")[^"]+', 1),
)

PEP440_PATHS = frozenset({"cli-proxy/pypi/pyproject.toml"})
_PRERELEASE = re.compile(r"^(?P<base>[^-]+)-(?P<kind>alpha|beta|rc)\.?(?P<num>\d+)$")
_PEP440_KIND = {"alpha": "a", "beta": "b", "rc": "rc"}


def pep440(version: str) -> str:
    """Respell a semver prerelease (3.24.0-rc.1) the way PyPI expects it (3.24.0rc1)."""
    match = _PRERELEASE.match(version)
    return version if match is None else f"{match['base']}{_PEP440_KIND[match['kind']]}{match['num']}"


def synchronize(root: Path, *, check: bool = False) -> bool:
    """Update all pins after validating every match; return whether pins already matched."""
    version = tomllib.loads((root / "Cargo.toml").read_text())["workspace"]["package"]["version"]
    texts: dict[str, str] = {}
    originals: dict[str, str] = {}
    for relative_path, pattern, expected_count in CONSUMER_PINS:
        pinned = pep440(version) if relative_path in PEP440_PATHS else version
        text = texts.get(relative_path)
        if text is None:
            text = originals[relative_path] = (root / relative_path).read_text()
        updated, count = re.subn(pattern, lambda match, pinned=pinned: match[1] + pinned, text)
        if count != expected_count:
            raise ValueError(f"syncing {relative_path}: expected {expected_count} release pins, found {count}")
        texts[relative_path] = updated
    changes = [(root / rel, text) for rel, text in texts.items() if text != originals[rel]]
    for path, updated in changes:
        if check:
            print(f"stale consumer version: {path.relative_to(root)}")
        else:
            path.write_text(updated)
    return not changes


def main() -> int:
    """Apply the canonical version, or report drift for the release gate."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true")
    arguments = parser.parse_args()
    current = synchronize(Path(__file__).resolve().parent.parent, check=arguments.check)
    return int(arguments.check and not current)


if __name__ == "__main__":
    raise SystemExit(main())
