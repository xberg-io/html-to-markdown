"""Verify Swift release checksum recovery preserves a one-commit provenance chain."""

import os
import subprocess
import sys
from pathlib import Path

import pytest

ROOT = Path(__file__).resolve().parents[2]
SCRIPT = ROOT / "scripts/ci/swift/finalize_release_manifest.py"
OLD_CHECKSUM = "a" * 64
NEW_CHECKSUM = "b" * 64


def git(repository: Path, *arguments: str) -> str:
    """Run Git without inheriting user configuration."""
    return subprocess.run(
        ["git", "-C", str(repository), *arguments],
        check=True,
        capture_output=True,
        text=True,
        env={**os.environ, "GIT_CONFIG_GLOBAL": os.devnull, "GIT_CONFIG_NOSYSTEM": "1"},
    ).stdout.strip()


def initialize(repository: Path) -> Path:
    """Create an isolated release source commit."""
    git(repository, "init", "--initial-branch=main")
    git(repository, "config", "user.name", "Release Test")
    git(repository, "config", "user.email", "release-test@example.com")
    manifest = repository / "Package.swift"
    manifest.write_text('checksum: "__ALEF_SWIFT_CHECKSUM__"\n')
    git(repository, "add", "Package.swift")
    git(repository, "commit", "-m", "Release source")
    return manifest


def finalize(repository: Path, checksum: str) -> subprocess.CompletedProcess[str]:
    """Run the release helper like the workflow does."""
    return subprocess.run(
        [sys.executable, str(SCRIPT), checksum, "3.17.0"],
        cwd=repository,
        capture_output=True,
        text=True,
        check=False,
    )


@pytest.mark.parametrize("state", ["placeholder", "stale", "current"])
def test_finalizer_verifies_without_creating_or_amending_checksum_commits(tmp_path: Path, state: str) -> None:
    """Checksum preparation belongs before tagging, including recovery runs."""
    manifest = initialize(tmp_path)
    if state != "placeholder":
        manifest.write_text(f'checksum: "{OLD_CHECKSUM}"\n')
        git(tmp_path, "add", "Package.swift")
        git(tmp_path, "commit", "-m", "fixture: precomputed checksum")
    git(tmp_path, "-c", "tag.gpgsign=false", "tag", "-a", "v3.17.0", "-m", "fixture")
    before = git(tmp_path, "rev-parse", "HEAD")
    refs = git(tmp_path, "show-ref")
    source = manifest.read_bytes()
    result = finalize(tmp_path, OLD_CHECKSUM if state == "current" else NEW_CHECKSUM)
    assert (result.returncode == 0) is (state == "current"), result.stderr
    if state != "current":
        assert "before tagging" in result.stderr
    assert git(tmp_path, "rev-parse", "HEAD") == before
    assert git(tmp_path, "show-ref") == refs
    assert manifest.read_bytes() == source


def test_finalizer_rejects_unrelated_existing_checksum_commit(tmp_path: Path) -> None:
    """Recovery cannot amend or hide an unrelated native source change."""
    manifest = initialize(tmp_path)
    manifest.write_text(f'checksum: "{OLD_CHECKSUM}"\n')
    (tmp_path / "source.rs").write_text("changed\n")
    git(tmp_path, "add", "Package.swift", "source.rs")
    git(tmp_path, "commit", "-m", "Unrelated change")
    before = git(tmp_path, "rev-parse", "HEAD")
    source = manifest.read_bytes()
    result = finalize(tmp_path, NEW_CHECKSUM)
    assert result.returncode != 0
    assert git(tmp_path, "rev-parse", "HEAD") == before
    assert manifest.read_bytes() == source
