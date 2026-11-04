"""Verify the pub.dev guard checks the package that the release uploads."""

from __future__ import annotations

import tomllib
import unittest
from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parents[2]


def configured_alias() -> str:
    """Return the pub.dev name that alef writes into the Dart pubspec."""
    config = tomllib.loads((ROOT / "alef.toml").read_text(encoding="utf-8"))
    return config["crates"][0]["dart"]["pubspec_name"]


def uploaded_pubspec_name() -> str:
    """Return the name of the pubspec that assemble-dart-package uploads unchanged."""
    return yaml.safe_load((ROOT / "packages/dart/pubspec.yaml").read_text(encoding="utf-8"))["name"]


def guarded_package() -> str:
    """Return the package that check-pub asks pub.dev about."""
    workflow = yaml.safe_load((ROOT / ".github/workflows/publish.yaml").read_text(encoding="utf-8"))
    steps = workflow["jobs"]["check-pub"]["steps"]
    check = next(step for step in steps if step.get("uses", "").startswith("xberg-io/actions/check-registry@"))
    assert check["with"]["registry"] == "pub"
    return check["with"]["package"]


def identity_mismatches(alias: str, pubspec_name: str, registry_package: str) -> list[str]:
    """Name each source that disagrees with the configured pub.dev alias."""
    mismatches = []
    if pubspec_name != alias:
        mismatches.append(f"packages/dart/pubspec.yaml declares {pubspec_name!r}, alef.toml configures {alias!r}")
    if registry_package != alias:
        mismatches.append(f"check-pub queries {registry_package!r}, alef.toml configures {alias!r}")
    return mismatches


class DartPublishIdentityTests(unittest.TestCase):
    """Join the configured alias, the uploaded pubspec and the registry guard."""

    def test_guard_queries_the_uploaded_package(self) -> None:
        """The check-pub input, the uploaded pubspec and alef.toml must name one package."""
        assert identity_mismatches(configured_alias(), uploaded_pubspec_name(), guarded_package()) == []

    def test_contract_rejects_the_former_registry_input(self) -> None:
        """The pre-fix input html_to_markdown is an unrelated pub.dev package."""
        assert identity_mismatches("h2m", "h2m", "html_to_markdown") == [
            "check-pub queries 'html_to_markdown', alef.toml configures 'h2m'"
        ]

    def test_contract_accepts_the_corrected_registry_input(self) -> None:
        """The assembled package is h2m, so h2m is the identity to check."""
        assert identity_mismatches("h2m", "h2m", "h2m") == []

    def test_contract_rejects_a_wrong_alias(self) -> None:
        """A configured alias that the pubspec and guard do not share must fail both."""
        assert len(identity_mismatches("not_h2m", uploaded_pubspec_name(), guarded_package())) == 2


if __name__ == "__main__":
    unittest.main()
