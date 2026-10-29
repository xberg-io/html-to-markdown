"""Verify workflow triggers and concurrency preserve required CI checks."""

from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parents[2]


def _workflow(name: str) -> dict[str, object]:
    source = (ROOT / ".github" / "workflows" / name).read_text(encoding="utf-8")
    return yaml.safe_load(source.replace("\non:\n", '\n"on":\n', 1))


def test_docs_pushes_use_commit_specific_concurrency_groups() -> None:
    workflow = _workflow("ci-docs.yaml")

    assert workflow["concurrency"] == {
        "group": "ci-docs-${{ github.event.pull_request.number || github.sha }}",
        "cancel-in-progress": False,
    }


def test_r_configure_changes_start_lint_only_for_main_pushes() -> None:
    workflow = _workflow("ci-lint.yaml")

    assert workflow["on"]["push"]["branches"] == ["main"]
    assert "packages/r/configure" in workflow["on"]["push"]["paths"]
    assert not any(event.startswith("pull_request") for event in workflow["on"])


def test_e2e_result_reports_for_every_main_push() -> None:
    workflow = _workflow("ci-e2e.yaml")

    assert workflow["on"]["push"] == {"branches": ["main"]}
    assert not any(event.startswith("pull_request") for event in workflow["on"])


def test_lint_runs_the_quality_warning_gate_through_mise() -> None:
    workflow = _workflow("ci-lint.yaml")
    job = workflow["jobs"]["quality-warnings"]

    assert [step["run"] for step in job["steps"] if "run" in step] == ["mise run check:quality-warnings"]
