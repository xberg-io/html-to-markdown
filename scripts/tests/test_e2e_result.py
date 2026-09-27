"""Verify the CI E2E aggregate fails on every leg outcome except success and a filtered skip."""

import copy
import json
import re
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


# The path filters each leg runs on, as the leg conditions on main stated them before they moved into
# the change detection job. A term dropped from an output skips its leg with a green result, so the
# outputs are pinned here rather than trusted.
LEG_FILTERS = {
    "build-ffi": {"core", "ffi"},
    "build-python": {"core", "python"},
    "build-node": {"core", "node"},
    "build-node-musl": {"core", "node"},
    "build-node-linux-arm64-gnu": {"core", "node"},
    "build-ruby": {"core", "ruby"},
    "build-php": {"core", "php"},
    "build-csharp": {"core", "csharp"},
    "build-java": {"core", "java"},
    "build-wasm": {"core", "wasm"},
    "build-kotlin-android": {"core", "ffi", "kotlin"},
    "test-python": {"core", "python"},
    "test-node": {"core", "node"},
    "test-ruby": {"core", "ruby"},
    "test-php": {"core", "php"},
    "test-csharp": {"core", "csharp"},
    "test-go": {"core", "ffi", "go"},
    "test-java": {"core", "java"},
    "test-elixir": {"core", "ffi", "elixir"},
    "test-r": {"core", "r"},
    "test-c-ffi": {"core", "ffi"},
    "test-c-ffi-windows": {"core", "ffi"},
    "test-wasm": {"core", "wasm"},
    "test-kotlin-android": {"core", "ffi", "kotlin"},
    "test-swift": {"core", "ffi", "swift"},
    "test-dart": {"core", "dart"},
    "test-zig": {"core", "ffi", "zig"},
}
DISPATCH = "github.event_name == 'workflow_dispatch'"
FILTER_TERM = re.compile(r"steps\.filter\.outputs\.([a-z]+) == 'true'")


def _output_filters(expression: str) -> set[str]:
    """Return the filters a `run-*` output ORs together, refusing any other shape."""
    body = expression.strip()
    assert body.startswith("${{") and body.endswith("}}"), expression
    dispatch, *terms = [term.strip() for term in body[3:-2].split("||")]
    assert dispatch == DISPATCH, expression
    filters = [FILTER_TERM.fullmatch(term) for term in terms]
    assert all(filters), expression
    return {match.group(1) for match in filters if match}


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


def test_should_name_cancellation_for_a_leg_the_cancelled_run_skipped() -> None:
    needs = _needs(
        {"build-python": "cancelled", "test-python": "skipped", "test-r": "skipped"},
        {"build-python": "true", "test-python": "true", "test-r": "false"},
    )

    assert verdict(needs) == ["build-python: cancelled", "test-python: skipped because the run was cancelled"]


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


GATE_PROBLEM = (
    "test-r: its `if:` must be `needs.changes.outputs.run-test-r == 'true'`, "
    "optionally after `always() && !cancelled() &&`"
)


@pytest.mark.parametrize(
    "condition",
    [
        pytest.param("needs.changes.outputs.run-test-go == 'true'", id="another-leg-output"),
        pytest.param("github.event_name == 'workflow_dispatch'", id="raw-condition"),
    ],
)
def test_should_report_a_leg_gated_on_something_other_than_its_own_output(
    workflow: dict[str, Any], condition: str
) -> None:
    changed = copy.deepcopy(workflow)
    changed["jobs"]["test-r"]["if"] = condition

    assert check_workflow(changed) == [GATE_PROBLEM, "changes: output run-test-r gates no job"]


@pytest.mark.parametrize(
    "condition",
    [
        pytest.param("always() && !cancelled() && needs.changes.outputs.run-test-r != 'true'", id="inverted"),
        pytest.param(
            "always() && !cancelled() && needs.changes.outputs.run-test-r == 'true' && "
            "github.event_name != 'pull_request'",
            id="extra-term",
        ),
        pytest.param("always() && needs.changes.outputs.run-test-r == 'true'", id="partial-prefix"),
    ],
)
def test_should_report_a_leg_that_reads_its_own_output_in_any_other_condition(
    workflow: dict[str, Any], condition: str
) -> None:
    changed = copy.deepcopy(workflow)
    changed["jobs"]["test-r"]["if"] = condition

    assert check_workflow(changed) == [GATE_PROBLEM]


def test_should_accept_both_gate_forms_across_line_breaks(workflow: dict[str, Any]) -> None:
    changed = copy.deepcopy(workflow)
    changed["jobs"]["test-r"]["if"] = "always() &&\n  !cancelled() &&\n  needs.changes.outputs.run-test-r == 'true'\n"
    changed["jobs"]["test-go"]["if"] = "needs.changes.outputs.run-test-go == 'true'"

    assert check_workflow(changed) == []


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


def _verdict_step(workflow: dict[str, Any]) -> dict[str, Any]:
    (step,) = [
        step for step in workflow["jobs"][AGGREGATE]["steps"] if "check_e2e_result.py verdict" in step.get("run", "")
    ]
    return step


def test_should_report_an_aggregate_that_never_runs_the_verdict(workflow: dict[str, Any]) -> None:
    changed = copy.deepcopy(workflow)
    changed["jobs"][AGGREGATE]["steps"].remove(_verdict_step(changed))

    assert check_workflow(changed) == [
        (
            f"{AGGREGATE}: no step runs `python3 scripts/check_e2e_result.py verdict` "
            "with NEEDS set to `${{ toJSON(needs) }}`"
        ),
    ]


def test_should_report_an_aggregate_that_never_checks_out_the_script(workflow: dict[str, Any]) -> None:
    changed = copy.deepcopy(workflow)
    steps = changed["jobs"][AGGREGATE]["steps"]
    (checkout,) = [step for step in steps if str(step.get("uses", "")).startswith("actions/checkout@")]
    steps.remove(checkout)

    assert check_workflow(changed) == [f"{AGGREGATE}: no checkout step runs before the verdict step"]


def test_should_report_a_checkout_that_runs_after_the_verdict(workflow: dict[str, Any]) -> None:
    changed = copy.deepcopy(workflow)
    steps = changed["jobs"][AGGREGATE]["steps"]
    verdict_step = _verdict_step(changed)
    steps.remove(verdict_step)
    steps.insert(0, verdict_step)

    assert check_workflow(changed) == [f"{AGGREGATE}: no checkout step runs before the verdict step"]


@pytest.mark.parametrize(
    ("key", "value"),
    [
        pytest.param("if", "github.event_name == 'push'", id="if"),
        pytest.param("continue-on-error", True, id="continue-on-error"),
    ],
)
def test_should_report_a_verdict_step_that_can_stop_enforcing(
    workflow: dict[str, Any], key: str, value: object
) -> None:
    changed = copy.deepcopy(workflow)
    _verdict_step(changed)[key] = value

    assert check_workflow(changed) == [f"{AGGREGATE}: the verdict step must not set `{key}`"]


def test_should_report_an_aggregate_job_that_may_fail_without_failing_the_run(workflow: dict[str, Any]) -> None:
    changed = copy.deepcopy(workflow)
    changed["jobs"][AGGREGATE]["continue-on-error"] = True

    assert check_workflow(changed) == [f"{AGGREGATE}: must not set `continue-on-error`"]


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


def _pinned_output_problems(workflow: dict[str, Any]) -> list[str]:
    outputs = workflow["jobs"]["changes"]["outputs"]
    problems = [f"run-{job}: missing" for job in LEG_FILTERS if f"run-{job}" not in outputs]
    problems += [f"{name}: not pinned" for name in outputs if name.removeprefix("run-") not in LEG_FILTERS]
    problems += [
        f"run-{job}: runs on {sorted(_output_filters(outputs[f'run-{job}']))}, expected {sorted(filters)}"
        for job, filters in LEG_FILTERS.items()
        if f"run-{job}" in outputs and _output_filters(outputs[f"run-{job}"]) != filters
    ]
    return problems


def test_should_pin_each_leg_output_to_the_filters_its_leg_runs_on(workflow: dict[str, Any]) -> None:
    assert _pinned_output_problems(workflow) == []


def test_should_name_only_filters_the_change_detection_step_defines(workflow: dict[str, Any]) -> None:
    (step,) = [step for step in workflow["jobs"]["changes"]["steps"] if step.get("id") == "filter"]
    defined = set(yaml.safe_load(step["with"]["filters"]))

    assert set().union(*LEG_FILTERS.values()) <= defined


def test_should_report_a_filter_term_dropped_from_an_output(workflow: dict[str, Any]) -> None:
    changed = copy.deepcopy(workflow)
    outputs = changed["jobs"]["changes"]["outputs"]
    outputs["run-test-go"] = outputs["run-test-go"].replace(" || steps.filter.outputs.core == 'true'", "")

    assert _pinned_output_problems(changed) == [
        "run-test-go: runs on ['ffi', 'go'], expected ['core', 'ffi', 'go']",
    ]
