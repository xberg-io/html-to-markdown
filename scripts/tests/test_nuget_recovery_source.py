"""Prove recovery rejects source changes while accepting verified Swift metadata."""

import os
import subprocess
import sys
from pathlib import Path

import pytest

ROOT = Path(__file__).resolve().parents[2]
CHECKSUM = "a" * 64
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
        ("checksum", True),
        ("source", False),
        ("mode", False),
        ("line-endings", False),
        ("manifest", False),
        ("checksum-mismatch", False),
        ("ancestor", False),
    ],
)
def test_recovery_verifies_the_entire_tag_change(tmp_path: Path, change: str, succeeds: bool) -> None:
    """Only one direct checksum-substitution commit may follow the original source."""
    git(tmp_path, "init", "--initial-branch=main")
    git(tmp_path, "config", "user.name", "Release Test")
    git(tmp_path, "config", "user.email", "release-test@example.com")
    git(tmp_path, "config", "core.fileMode", "true")
    manifest = tmp_path / "Package.swift"
    manifest.write_text('checksum: "__ALEF_SWIFT_CHECKSUM__"\n')
    (tmp_path / "source.rs").write_text("original source\n")
    git(tmp_path, "add", ".")
    git(tmp_path, "commit", "-m", "Original release")
    original_sha = git(tmp_path, "rev-parse", "HEAD")
    if change == "ancestor":
        git(tmp_path, "commit", "--allow-empty", "-m", "Unrelated intermediate commit")
    if change != "unchanged":
        manifest.write_text(f'checksum: "{CHECKSUM}"\n')
        if change == "source":
            (tmp_path / "source.rs").write_text("changed source\n")
        if change == "mode":
            manifest.chmod(0o755)
        if change == "line-endings":
            manifest.write_bytes(f'checksum: "{CHECKSUM}"\r\n'.encode())
        if change == "manifest":
            manifest.write_text(f'changed package; checksum: "{CHECKSUM}"\n')
        git(tmp_path, "add", ".")
        git(tmp_path, "commit", "-m", "Post-release change")
    git(tmp_path, "tag", TAG)
    checksums = tmp_path / "checksums"
    checksums.mkdir()
    (checksums / "bundle.checksum").write_text(("b" * 64 if change == "checksum-mismatch" else CHECKSUM) + "\n")
    result = subprocess.run(
        [sys.executable, str(ROOT / "scripts/ci/csharp/verify_recovery_source.py"), original_sha, TAG, str(checksums)],
        cwd=tmp_path,
        capture_output=True,
        text=True,
        check=False,
    )
    assert (result.returncode == 0) is succeeds, result.stderr
