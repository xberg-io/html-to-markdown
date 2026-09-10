"""Exercise the release pack step at the external dotnet boundary."""

import json
import os
import shutil
import subprocess
import sys
from pathlib import Path

import pytest
import yaml

ROOT = Path(__file__).resolve().parents[2]
VERSION = "3.12.3"


def run_pack(tmp_path: Path, failing_rid: str = "") -> tuple[subprocess.CompletedProcess[str], list[list[str]]]:
    """Run the actual workflow shell with a recording dotnet executable."""
    workflow = yaml.safe_load((ROOT / ".github/workflows/publish.yaml").read_text())
    step = next(step for step in workflow["jobs"]["publish-nuget"]["steps"] if step.get("name") == "Pack NuGet package")
    script = step["run"].replace("${{ needs.prepare.outputs.version }}", VERSION)
    shutil.copytree(ROOT / "scripts/ci/csharp", tmp_path / "scripts/ci/csharp")
    shutil.copytree(
        ROOT / "packages/csharp", tmp_path / "packages/csharp", ignore=shutil.ignore_patterns("bin", "obj", "runtimes")
    )
    executable = tmp_path / "dotnet"
    executable.write_text(
        f"#!{sys.executable}\n"
        "import json, os, sys\n"
        "with open(os.environ['DOTNET_CALLS'], 'a') as output:\n"
        "    output.write(json.dumps(sys.argv[1:]) + '\\n')\n"
        "sys.exit(13 if os.environ['FAILING_RID'] and "
        "('-p:PublishedRID=' + os.environ['FAILING_RID']) in sys.argv else 0)\n"
    )
    executable.chmod(0o755)
    calls_file = tmp_path / "calls.jsonl"
    environment = {
        **os.environ,
        "PATH": f"{tmp_path}:{os.environ['PATH']}",
        "DOTNET_CALLS": str(calls_file),
        "FAILING_RID": failing_rid,
        "VERSION": VERSION,
    }
    result = subprocess.run(
        ["bash", "-euo", "pipefail", "-c", script],
        cwd=tmp_path,
        env=environment,
        capture_output=True,
        text=True,
        check=False,
    )
    return result, [json.loads(line) for line in calls_file.read_text().splitlines()]


def test_pack_includes_every_runtime_in_the_published_graph(tmp_path: Path) -> None:
    """A managed package must never be the only package sent to NuGet."""
    result, calls = run_pack(tmp_path)
    assert result.returncode == 0, result.stderr
    graph = json.loads((ROOT / "packages/csharp/HtmlToMarkdown/runtime.json.template").read_text())["runtimes"]
    expected_rids = sorted(rid for rid, dependencies in graph.items() if "XbergIo.HtmlToMarkdown" in dependencies)
    runtime_calls = [call for call in calls if any(arg.startswith("-p:PublishedRID=") for arg in call)]
    actual_rids = sorted(
        arg.split("=", 1)[1] for call in runtime_calls for arg in call if arg.startswith("-p:PublishedRID=")
    )
    assert actual_rids == expected_rids
    assert all(call[0] == "pack" and f"-p:Version={VERSION}" in call for call in runtime_calls)


@pytest.mark.parametrize("failing_rid", ["linux-arm64", "osx-arm64", "win-x64"])
def test_pack_failure_stops_before_publishing_partial_runtime_set(tmp_path: Path, failing_rid: str) -> None:
    """A failed runtime pack must fail the workflow step immediately."""
    result, calls = run_pack(tmp_path, failing_rid)
    assert result.returncode == 13
    assert f"-p:PublishedRID={failing_rid}" in calls[-1]


@pytest.mark.parametrize(
    ("run_id", "overrides", "succeeds"),
    [
        ("123", {}, True),
        ("123;echo invalid", {}, False),
        ("123", {"path": ".github/workflows/ci.yaml"}, False),
        ("123", {"event": "pull_request"}, False),
        ("123", {"head_branch": "v3.12.2"}, False),
    ],
)
def test_recovery_only_accepts_the_original_requested_release(
    tmp_path: Path, run_id: str, overrides: dict[str, str], succeeds: bool
) -> None:
    """Validate actual recovery shell inputs before checking out or publishing artifacts."""
    workflow = yaml.safe_load((ROOT / ".github/workflows/publish.yaml").read_text())
    script = workflow["jobs"]["recover-nuget-runtimes"]["steps"][0]["run"]
    metadata = {
        "path": ".github/workflows/publish.yaml",
        "event": "release",
        "head_branch": f"v{VERSION}",
        "head_sha": "a" * 40,
        **overrides,
    }
    executable = tmp_path / "gh"
    executable.write_text(f"#!{sys.executable}\nimport os\nprint(os.environ['RUN_METADATA'])\n")
    executable.chmod(0o755)
    output = tmp_path / "output"
    environment = {
        **os.environ,
        "PATH": f"{tmp_path}:{os.environ['PATH']}",
        "RUN_METADATA": json.dumps(metadata),
        "RELEASE_RUN_ID": run_id,
        "EXPECTED_TAG": f"v{VERSION}",
        "GITHUB_REPOSITORY": "xberg-io/html-to-markdown",
        "GITHUB_OUTPUT": str(output),
    }
    result = subprocess.run(
        ["bash", "-euo", "pipefail", "-c", script],
        cwd=tmp_path,
        env=environment,
        capture_output=True,
        text=True,
        check=False,
    )
    assert (result.returncode == 0) is succeeds, result.stderr
    if succeeds:
        assert output.read_text() == f"sha={'a' * 40}\n"
    else:
        assert not output.exists()
