"""Prove recovery packs only the exact commit the immutable release tag names."""

import os
import subprocess
import sys
from pathlib import Path

import pytest

ROOT = Path(__file__).resolve().parents[2]
TAG = "v3.12.3"


def git(repository: Path, *arguments: str) -> str:
    """Create and inspect isolated real Git histories for provenance checks."""
    return subprocess.run(
        ["git", "-C", str(repository), *arguments],
        check=True,
        capture_output=True,
        text=True,
        env={**os.environ, "GIT_CONFIG_GLOBAL": os.devnull, "GIT_CONFIG_NOSYSTEM": "1"},
    ).stdout.strip()


@pytest.mark.parametrize(
    ("change", "succeeds"),
    [
        ("unchanged", True),
        ("checksum-child", False),
        ("source-child", False),
        ("ancestor", False),
    ],
)
def test_recovery_requires_the_tag_to_name_the_original_release_commit(
    tmp_path: Path, change: str, succeeds: bool
) -> None:
    """Tags are final before publication, so no commit may follow the original release source."""
    git(tmp_path, "init", "--initial-branch=main")
    git(tmp_path, "config", "user.name", "Release Test")
    git(tmp_path, "config", "user.email", "release-test@example.com")
    manifest = tmp_path / "Package.swift"
    manifest.write_text(f'checksum: "{"a" * 64}"\n')
    (tmp_path / "source.rs").write_text("original source\n")
    git(tmp_path, "add", ".")
    git(tmp_path, "commit", "-m", "Original release")
    original_sha = git(tmp_path, "rev-parse", "HEAD")
    if change == "checksum-child":
        manifest.write_text(f'checksum: "{"b" * 64}"\n')
    if change == "source-child":
        (tmp_path / "source.rs").write_text("changed source\n")
    if change != "unchanged":
        git(tmp_path, "commit", "--allow-empty", "-am", "Post-release change")
    git(tmp_path, "tag", TAG)
    if change == "ancestor":
        original_sha = git(tmp_path, "rev-parse", "HEAD~1")
    result = subprocess.run(
        [sys.executable, str(ROOT / "scripts/ci/csharp/verify_recovery_source.py"), original_sha, TAG],
        cwd=tmp_path,
        capture_output=True,
        text=True,
        check=False,
    )
    assert (result.returncode == 0) is succeeds, result.stderr
    if not succeeds:
        assert "is not the commit the immutable tag" in result.stderr
        assert "Traceback" not in result.stderr
