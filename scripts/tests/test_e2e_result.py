"""Verify the CI E2E aggregate fails on every leg outcome except success and a filtered skip."""

import copy
import json
from pathlib import Path
from typing import Any

import check_e2e_result
import pytest
import yaml
from check_e2e_result import AGGREGATE, check_workflow, verdict

WORKFLOW = Path(__file__).resolve().parents[2] / ".github" / "workflows" / "ci-e2e.yaml"


def _needs(results: dict[str, str], flags: dict[str, str], changes: str = "success") -> dict[str, Any]:
    needs: dict[str, Any] = {
        "changes": {"result": changes, "outputs": {f"run-{job}": flag for job, flag in flags.items()}},
    }
    for job, result in results.items():
        needs[job] = {"result": result, "outputs": {}}
    return needs


@pytest.fixture(scope="module")
def workflow() -> dict[str, Any]:
    return yaml.safe_load(WORKFLOW.read_text(encoding="utf-8"))


def test_should_pass_when_every_leg_succeeds() -> None:
    needs = _needs({"build-python": "success", "test-python": "success"}, {"build-python": "true"})

    assert verdict(needs) == []


def test_should_pass_a_leg_skipped_because_its_filters_did_not_match() -> None:
    needs = _needs({"test-r": "skipped", "test-go": "success"}, {"test-r": "false", "test-go": "true"})

    assert verdict(needs) == []


def test_should_fail_a_leg_skipped_although_its_filters_matched() -> None:
    needs = _needs({"test-r": "skipped", "test-go": "success"}, {"test-r": "true", "test-go": "true"})

    assert verdict(needs) == ["test-r: skipped although its path filters matched"]


def test_should_fail_a_skipped_leg_that_has_no_filter_output() -> None:
    needs = _needs({"e2e-freshness": "skipped"}, {})

    assert verdict(needs) == ["e2e-freshness: skipped, and no path filter output allows it to skip"]


@pytest.mark.parametrize("flag", ["true", "false"])
@pytest.mark.parametrize("result", ["failure", "cancelled"])
def test_should_fail_a_leg_that_did_not_succeed(result: str, flag: str) -> None:
    needs = _needs({"test-r": result, "test-go": "success"}, {"test-r": flag, "test-go": "true"})

    assert verdict(needs) == [f"test-r: {result}"]


@pytest.mark.parametrize("result", ["failure", "cancelled", "skipped"])
def test_should_fail_when_the_filter_job_did_not_succeed(result: str) -> None:
    needs = _needs({"test-r": "skipped"}, {}, changes=result)

    problems = verdict(needs)

    assert problems[0] == f"changes: {result}, so no leg's path filters are known"
    assert "test-r: skipped, and no path filter output allows it to skip" in problems


def test_should_fail_when_the_filter_job_is_missing() -> None:
    assert verdict({"test-r": {"result": "success", "outputs": {}}}) == [
        "changes: missing, so no leg's path filters are known",
    ]


def test_should_judge_every_gated_leg_of_the_real_workflow_by_its_own_output(workflow: dict[str, Any]) -> None:
    jobs = workflow["jobs"]
    gated = sorted(job for job in jobs if f"run-{job}" in jobs["changes"]["outputs"])
    assert len(gated) >= 27
    for job in gated:
        others = {other: "success" for other in jobs if other not in ("changes", AGGREGATE, job)}
        flags = dict.fromkeys(gated, "true")

        assert verdict(_needs({**others, job: "skipped"}, {**flags, job: "false"})) == []
        assert verdict(_needs({**others, job: "skipped"}, flags)) == [
            f"{job}: skipped although its path filters matched",
        ]


def test_should_find_the_real_workflow_wired_for_the_aggregate(workflow: dict[str, Any]) -> None:
    assert check_workflow(workflow) == []


def test_should_report_a_leg_missing_from_the_aggregate_needs(workflow: dict[str, Any]) -> None:
    changed = copy.deepcopy(workflow)
    changed["jobs"]["test-cobol"] = {"needs": ["changes"], "runs-on": "ubuntu-latest", "steps": []}

    assert check_workflow(changed) == [f"test-cobol: not listed in the needs of {AGGREGATE}"]


def test_should_report_every_existing_job_dropped_from_the_aggregate_needs(workflow: dict[str, Any]) -> None:
    for job in workflow["jobs"]:
        if job == AGGREGATE:
            continue
        changed = copy.deepcopy(workflow)
        changed["jobs"][AGGREGATE]["needs"].remove(job)

        assert check_workflow(changed) == [f"{job}: not listed in the needs of {AGGREGATE}"]


def test_should_report_a_leg_gated_on_a_condition_the_aggregate_cannot_read(workflow: dict[str, Any]) -> None:
    changed = copy.deepcopy(workflow)
    changed["jobs"]["test-r"]["if"] = "needs.changes.outputs.run-test-go == 'true'"

    assert check_workflow(changed) == [
        "test-r: its `if:` must read only needs.changes.outputs.run-test-r",
        "changes: output run-test-r gates no job",
    ]


def test_should_report_a_leg_gated_without_the_filter_outputs(workflow: dict[str, Any]) -> None:
    changed = copy.deepcopy(workflow)
    changed["jobs"]["test-r"]["if"] = "github.event_name == 'workflow_dispatch'"

    assert check_workflow(changed) == [
        "test-r: its `if:` must read only needs.changes.outputs.run-test-r",
        "changes: output run-test-r gates no job",
    ]


def test_should_report_a_leg_that_reads_an_output_the_filter_job_lacks(workflow: dict[str, Any]) -> None:
    changed = copy.deepcopy(workflow)
    del changed["jobs"]["changes"]["outputs"]["run-test-r"]

    assert check_workflow(changed) == ["test-r: changes has no run-test-r output"]


def test_should_report_a_leg_that_reads_the_filter_job_without_needing_it(workflow: dict[str, Any]) -> None:
    changed = copy.deepcopy(workflow)
    changed["jobs"]["test-python"]["needs"] = ["build-python"]

    assert check_workflow(changed) == ["test-python: reads changes outputs without needing changes"]


def test_should_report_an_aggregate_that_does_not_always_run(workflow: dict[str, Any]) -> None:
    changed = copy.deepcopy(workflow)
    changed["jobs"][AGGREGATE]["if"] = "success()"

    assert check_workflow(changed) == [f"{AGGREGATE}: must run with `if: always()`"]


def test_should_report_an_aggregate_that_never_runs_the_verdict(workflow: dict[str, Any]) -> None:
    changed = copy.deepcopy(workflow)
    changed["jobs"][AGGREGATE]["steps"] = changed["jobs"][AGGREGATE]["steps"][:1]

    assert check_workflow(changed) == [
        (
            f"{AGGREGATE}: no step runs `python3 scripts/check_e2e_result.py verdict` "
            "with NEEDS set to `${{ toJSON(needs) }}`"
        ),
    ]


def test_should_report_a_missing_aggregate(workflow: dict[str, Any]) -> None:
    changed = copy.deepcopy(workflow)
    del changed["jobs"][AGGREGATE]

    assert check_workflow(changed) == [f"{AGGREGATE}: job is missing"]


def test_should_exit_nonzero_and_annotate_from_the_needs_environment(
    monkeypatch: pytest.MonkeyPatch, capsys: pytest.CaptureFixture[str]
) -> None:
    needs = _needs({"test-r": "skipped", "test-go": "success"}, {"test-r": "true", "test-go": "true"})
    monkeypatch.setenv("NEEDS", json.dumps(needs))

    assert check_e2e_result.main(["check_e2e_result.py", "verdict"]) == 1
    assert "::error::test-r: skipped although its path filters matched" in capsys.readouterr().out


def test_should_exit_zero_when_every_leg_ran_or_was_filtered(monkeypatch: pytest.MonkeyPatch) -> None:
    needs = _needs({"test-r": "skipped", "test-go": "success"}, {"test-r": "false", "test-go": "true"})
    monkeypatch.setenv("NEEDS", json.dumps(needs))

    assert check_e2e_result.main(["check_e2e_result.py", "verdict"]) == 0


def test_should_refuse_an_empty_needs_environment(monkeypatch: pytest.MonkeyPatch) -> None:
    monkeypatch.setenv("NEEDS", "")

    assert check_e2e_result.main(["check_e2e_result.py", "verdict"]) == 2


def test_should_refuse_an_unknown_command(monkeypatch: pytest.MonkeyPatch) -> None:
    monkeypatch.setenv("NEEDS", json.dumps(_needs({}, {})))

    assert check_e2e_result.main(["check_e2e_result.py", "workflow"]) == 2
