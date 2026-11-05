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


def test_ffi_build_skips_the_coverage_tool() -> None:
    workflow = _workflow("ci-e2e-delegated.yaml")
    jobs = workflow["jobs"]
    assert isinstance(jobs, dict)
    setup_steps = [
        step
        for job in jobs.values()
        for step in job.get("steps", [])
        if step.get("uses") == "xberg-io/actions/setup-rust@v1"
        and str(step.get("with", {}).get("cache-key-prefix", "")).startswith("ffi-")
    ]
    assert len(setup_steps) == 1
    assert setup_steps[0]["with"]["install-llvm-cov"] == "false"
