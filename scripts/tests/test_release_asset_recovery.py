"""Verify that release retries require the complete binary inventory."""

import json
import subprocess
import sys
from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parents[2]


def test_cli_cannot_undraft_release() -> None:
    jobs = yaml.safe_load((ROOT / ".github/workflows/publish.yaml").read_text())["jobs"]
    assert jobs["upload-cli-release"]["with"].get("undraft-release") is False


def test_complete_existing_archives_can_satisfy_release_jobs() -> None:
    jobs = yaml.safe_load((ROOT / ".github/workflows/publish.yaml").read_text())["jobs"]
    for kind in ("cli", "c-ffi"):
        checker = jobs[f"check-{kind}"]
        assert any("release_inventory.py" in step.get("run", "") for step in checker["steps"])
    for name, kinds in (("promote-release", ("cli", "c-ffi")), ("publish-homebrew-formula", ("cli", "c-ffi"))):
        for kind in kinds:
            assert f"check-{kind}" in jobs[name]["needs"]
            assert f"needs.check-{kind}.outputs.exists == 'true'" in jobs[name]["if"]
            assert f"needs.upload-{kind}-release.result == 'skipped'" in jobs[name]["if"]
    assert "needs.promote-release.outputs.promoted == 'true'" in jobs["verify-binstall"]["if"]
    assert "needs.promote-release.outputs.promoted == 'true'" in jobs["publish-scoop-manifest"]["if"]
    assert "release_inventory.py" in jobs["promote-release"]["steps"][-1]["run"]


def test_live_inventory_rejects_missing_duplicate_and_incomplete_assets() -> None:
    tag = "v3.20.0"
    inventories = {
        "cli": [
            "cli-SHA256SUMS",
            "cli-aarch64-apple-darwin.tar.gz",
            "cli-aarch64-unknown-linux-gnu.tar.gz",
            "cli-x86_64-pc-windows-msvc.zip",
            "cli-x86_64-unknown-linux-gnu.tar.gz",
            "cli-x86_64-unknown-linux-musl.tar.gz",
        ],
        "c-ffi": [
            f"html-to-markdown-rs-ffi-{tag}-{target}.tar.gz"
            for target in (
                "aarch64-apple-darwin",
                "aarch64-pc-windows-msvc",
                "aarch64-unknown-linux-gnu",
                "x86_64-apple-darwin",
                "x86_64-pc-windows-msvc",
                "x86_64-unknown-linux-gnu",
            )
        ],
    }
    for kind, names in inventories.items():
        command = [
            sys.executable,
            str(ROOT / "scripts/ci/release_inventory.py"),
            kind,
            "--release-id",
            "123",
            "--tag",
            tag,
        ]
        assets = [{"name": name, "state": "uploaded", "size": 5, "digest": "sha256:" + "a" * 64} for name in names]
        wrong_name = {**assets[0], "name": "similar-prefix-only.tar.gz"}
        for candidate, expected in (
            ([*assets, wrong_name], "true"),
            ([*assets[1:], wrong_name], "false"),
            ([*assets, assets[0]], "false"),
            ([{**assets[0], "state": "starter"}, *assets[1:]], "false"),
            ([{**assets[0], "size": 0}, *assets[1:]], "false"),
            ([{**assets[0], "digest": None}, *assets[1:]], "false"),
        ):
            result = subprocess.run(
                command,
                input=json.dumps({"id": 123, "tag_name": tag, "assets": candidate}),
                check=True,
                capture_output=True,
                text=True,
            )
            assert result.stdout.strip() == expected
        for release_id, release_tag in ((124, tag), (123, "v3.19.0")):
            wrong_identity = subprocess.run(
                command,
                input=json.dumps({"id": release_id, "tag_name": release_tag, "assets": assets}),
                check=False,
                capture_output=True,
                text=True,
            )
            assert wrong_identity.returncode != 0
            assert "does not match the seeded release" in wrong_identity.stderr
