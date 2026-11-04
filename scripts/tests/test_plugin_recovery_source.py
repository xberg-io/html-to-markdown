"""Reject plugin recovery commits that alter anything beyond served lock digests."""

import json
import os
import shutil
import subprocess
import sys
from pathlib import Path

import pytest
import yaml

ROOT = Path(__file__).resolve().parents[2]


def git(repository: Path, *arguments: str) -> str:
    """Build isolated histories without user signing or hooks configuration."""
    return subprocess.run(
        ["git", "-C", str(repository), *arguments],
        check=True,
        capture_output=True,
        text=True,
        env={**os.environ, "GIT_CONFIG_GLOBAL": os.devnull, "GIT_CONFIG_NOSYSTEM": "1"},
    ).stdout.strip()


@pytest.mark.parametrize(
    "change", ["unchanged", "served", "content", "pin", "name", "mode", "source", "dirty", "wrong-sha", "floating-ref"]
)
def test_recovery_accepts_only_served_digest_refresh(tmp_path: Path, change: str) -> None:
    git(tmp_path, "init", "--initial-branch=main")
    git(tmp_path, "config", "user.name", "Release Test")
    git(tmp_path, "config", "user.email", "release-test@example.com")
    git(tmp_path, "config", "core.fileMode", "true")
    lock = tmp_path / ".ai-rulez/ai-rulez.lock"
    lock.parent.mkdir()
    original = (
        'version = 1\ntree = "old"\n[[include]]\ncommit = "pinned"\n'
        '[[served]]\nname = "skill"\ndigest = "old"\n[[item]]\ndigest = "authored"\n'
    )
    lock.write_text(original)
    (tmp_path / "source.txt").write_text("original\n")
    git(tmp_path, "add", ".ai-rulez/ai-rulez.lock", "source.txt")
    git(tmp_path, "commit", "-m", "Release source")
    git(tmp_path, "tag", "v3.18.7")
    if change != "unchanged":
        updated = original.replace('"old"', '"new"')
        replacements = {"content": ("authored", "changed"), "pin": ("pinned", "moved"), "name": ("skill", "other")}
        if change in replacements:
            updated = updated.replace(*replacements[change])
        lock.write_text(updated)
        if change == "mode":
            lock.chmod(0o755)
        if change == "source":
            (tmp_path / "source.txt").write_text("changed\n")
        git(tmp_path, "add", ".ai-rulez/ai-rulez.lock", "source.txt")
        git(tmp_path, "commit", "-m", "Refresh metadata")
        if change == "dirty":
            (tmp_path / "source.txt").write_text("uncommitted\n")
    source_sha = {"wrong-sha": "0" * 40, "floating-ref": "main", "unchanged": ""}.get(
        change, git(tmp_path, "rev-parse", "HEAD")
    )
    result = subprocess.run(
        [sys.executable, str(ROOT / "scripts/ci/verify_plugin_recovery_source.py"), "v3.18.7", source_sha],
        cwd=tmp_path,
        capture_output=True,
        text=True,
        check=False,
    )
    assert (result.returncode == 0) is (change in {"unchanged", "served"}), result.stderr
    if change not in {"unchanged", "served"}:
        assert "Plugin recovery refused:" in result.stderr
    else:
        provenance = json.loads(result.stdout)
        assert provenance["changed_served_digests"] == (1 if change == "served" else 0)
        assert provenance["tag_commit"] == git(tmp_path, "rev-parse", "v3.18.7^{commit}")
        assert provenance["packaging_commit"] == git(tmp_path, "rev-parse", "HEAD")


def test_plugin_workflow_verifies_source_before_packaging() -> None:
    workflow = yaml.safe_load((ROOT / ".github/workflows/plugin-artifacts.yaml").read_text())
    steps = workflow["jobs"]["package"]["steps"]
    verifier = next(index for index, step in enumerate(steps) if step.get("name") == "Verify immutable release source")
    packaging = next(
        index for index, step in enumerate(steps) if step.get("name") == "Package all supported plugin runtimes"
    )
    assert verifier < packaging
    assert steps[packaging]["with"]["working-directory"] == "release-source"
    assert "verify_plugin_recovery_source.py" in steps[verifier]["run"]
    assert steps[-1]["with"]["clobber"] == "false"


@pytest.mark.parametrize(
    ("workflow_name", "job"), [("publish.yaml", "publish-plugin-artifacts"), ("plugin-artifacts.yaml", "package")]
)
def test_release_verifies_fresh_skills_without_dirtying_tag_checkout(
    tmp_path: Path, workflow_name: str, job: str
) -> None:
    workflow = yaml.safe_load((ROOT / ".github/workflows" / workflow_name).read_text())
    step = next(step for step in workflow["jobs"][job]["steps"] if step.get("name") == "Verify public skill exports")
    checkout = tmp_path / "checkout"
    checkout.mkdir()
    git(checkout, "init", "--initial-branch=main")
    git(checkout, "config", "user.name", "Release Test")
    git(checkout, "config", "user.email", "release-test@example.com")

    scripts = checkout / "scripts"
    scripts.mkdir()
    exporter = scripts / "sync_public_skills.py"
    shutil.copyfile(ROOT / "scripts" / exporter.name, exporter)
    source = checkout / "plugin/skills/example/SKILL.md"
    source.parent.mkdir(parents=True)
    (checkout / "plugin/plugin.json").write_text('{"name":"example"}')
    skill = "---\nname: example\ndescription: Example skill.\n---\n"
    source.write_text(skill + "Old body\n")
    subprocess.run([sys.executable, str(exporter)], check=True, capture_output=True)
    source.write_text(skill + "Tagged body\n")
    git(checkout, "add", "plugin", "scripts", "skills")
    git(checkout, "commit", "-m", "Release source")
    git(checkout, "tag", "v3.20.0")
    commit = git(checkout, "rev-parse", "HEAD")

    temp = tmp_path / "runner"
    temp.mkdir()
    verified = subprocess.run(
        ["bash", "-c", step["run"]],
        cwd=checkout,
        env={**os.environ, "RUNNER_TEMP": str(temp)},
        check=False,
        capture_output=True,
        text=True,
    )
    assert verified.returncode == 0, verified.stderr
    assert "Public skills: 1 skills, 1 files (verified)" in verified.stdout
    assert git(checkout, "status", "--porcelain") == ""
    assert git(checkout, "rev-parse", "HEAD") == git(checkout, "rev-parse", "v3.20.0") == commit
    assert (checkout / "skills/example/SKILL.md").read_text() == skill + "Old body\n"
    exports = list(temp.glob("public-skills.*/skills/example/SKILL.md"))
    assert len(exports) == 1
    assert exports[0].read_text() == skill + "Tagged body\n"
