#!/usr/bin/env python3
"""Decide the CI E2E result from every leg's outcome, and check the workflow stays wired for it.

A leg passes when it succeeded, or when it was skipped and the ``changes`` job's
``run-<job id>`` output for it is ``false``. Everything else fails the run.

Usage (the ``e2e-result`` job in ci-e2e.yaml):
    NEEDS='${{ toJSON(needs) }}' python3 scripts/check_e2e_result.py verdict
"""

from __future__ import annotations

import json
import os
import re
import sys
from typing import Any

AGGREGATE = "e2e-result"
CHANGES = "changes"
VERDICT_COMMAND = "python3 scripts/check_e2e_result.py verdict"
NEEDS_ENV = "${{ toJSON(needs) }}"
# ~keep Allow-lists, not deny-lists: `continue-on-error`, a step `if:` or a `shell:` override each
# ~keep let the result job pass without the verdict failing it.
AGGREGATE_KEYS = ("name", "needs", "if", "runs-on", "timeout-minutes", "steps")
VERDICT_STEP_KEYS = ("name", "env", "run", "shell")
CHANGES_OUTPUT = re.compile(r"needs\.changes\.outputs\.([A-Za-z0-9_-]+)")


def verdict(needs: dict[str, Any]) -> list[str]:
    """Return one problem per leg that neither succeeded nor was skipped by its path filters."""
    problems = []
    changes = needs.get(CHANGES)
    if changes is None:
        problems.append(f"{CHANGES}: missing, so no leg's path filters are known")
        flags: dict[str, str] = {}
    else:
        if changes.get("result") != "success":
            problems.append(f"{CHANGES}: {changes.get('result')}, so no leg's path filters are known")
        flags = changes.get("outputs") or {}
    # ~keep `needs` records a cancelled job, and the verdict reads that to explain a leg whose
    # ~keep filters matched but whose `!cancelled()` guard skipped it. A leg that hits its
    # ~keep `timeout-minutes` is also reported as cancelled, so one timeout labels an unrelated
    # ~keep skipped leg the same way. The result stays red; only the message is imprecise.
    run_cancelled = any(info.get("result") == "cancelled" for info in needs.values())

    for job, info in sorted(needs.items()):
        if job == CHANGES:
            continue
        result = info.get("result")
        if result == "success":
            continue
        if result == "skipped":
            flag = flags.get(f"run-{job}")
            if flag == "false":
                continue
            if flag == "true" and run_cancelled:
                problems.append(f"{job}: skipped because the run was cancelled")
            elif flag == "true":
                problems.append(f"{job}: skipped although its path filters matched")
            else:
                problems.append(f"{job}: skipped, and no path filter output allows it to skip")
            continue
        problems.append(f"{job}: {result}")
    return problems


def _needs_list(job: dict[str, Any]) -> list[str]:
    needs = job.get("needs") or []
    return [needs] if isinstance(needs, str) else list(needs)


def check_workflow(workflow: dict[str, Any]) -> list[str]:
    """Return one problem per way the workflow can hide a leg from the aggregate job."""
    jobs: dict[str, Any] = workflow.get("jobs") or {}
    problems = []

    aggregate = jobs.get(AGGREGATE)
    if aggregate is None:
        return [f"{AGGREGATE}: job is missing"]
    if str(aggregate.get("if", "")).strip() != "always()":
        problems.append(f"{AGGREGATE}: must run with `if: always()`")
    aggregate_needs = set(_needs_list(aggregate))
    problems.extend(
        f"{job}: not listed in the needs of {AGGREGATE}" for job in sorted(set(jobs) - {AGGREGATE} - aggregate_needs)
    )
    problems.extend(
        f"{AGGREGATE}: sets `{key}`; only {', '.join(AGGREGATE_KEYS)} are allowed"
        for key in aggregate
        if key not in AGGREGATE_KEYS
    )
    steps = aggregate.get("steps") or []
    verdict_steps = [step for step in steps if str(step.get("run", "")).strip() == VERDICT_COMMAND]
    if not verdict_steps:
        problems.append(f"{AGGREGATE}: no step runs `{VERDICT_COMMAND}`")
    for step in verdict_steps:
        if not any(
            str(earlier.get("uses", "")).startswith("actions/checkout@") for earlier in steps[: steps.index(step)]
        ):
            problems.append(f"{AGGREGATE}: no checkout step runs before the verdict step")
        problems.extend(
            f"{AGGREGATE}: the verdict step sets `{key}`; only {', '.join(VERDICT_STEP_KEYS)} are allowed"
            for key in step
            if key not in VERDICT_STEP_KEYS
        )
        if step.get("env") != {"NEEDS": NEEDS_ENV}:
            problems.append(f"{AGGREGATE}: the verdict step's env must be exactly NEEDS: `{NEEDS_ENV}`")
        if step.get("shell") != "bash":
            problems.append(f"{AGGREGATE}: the verdict step must set `shell: bash`")

    outputs = set((jobs.get(CHANGES) or {}).get("outputs") or {})
    read_outputs: set[str] = set()
    for job_id, job in sorted(jobs.items()):
        if job_id in (CHANGES, AGGREGATE):
            continue
        read = set(CHANGES_OUTPUT.findall(str(job.get("if", ""))))
        read_outputs |= read
        own = f"run-{job_id}"
        gate = f"needs.changes.outputs.{own} == 'true'"
        if "if" in job and " ".join(str(job["if"]).split()) not in (gate, f"always() && !cancelled() && {gate}"):
            problems.append(f"{job_id}: its `if:` must be `{gate}`, optionally after `always() && !cancelled() &&`")
        if read and own not in outputs:
            problems.append(f"{job_id}: {CHANGES} has no {own} output")
        if read and CHANGES not in _needs_list(job):
            problems.append(f"{job_id}: reads {CHANGES} outputs without needing {CHANGES}")
    problems.extend(f"{CHANGES}: output {output} gates no job" for output in sorted(outputs - read_outputs))
    return problems


def main(argv: list[str]) -> int:
    if argv[1:] != ["verdict"]:
        print(__doc__, file=sys.stderr)
        return 2
    raw = os.environ.get("NEEDS")
    if not raw:
        print("::error::NEEDS is empty; set it to the needs context as JSON")
        return 2
    needs = json.loads(raw)
    for job, info in sorted(needs.items()):
        print(f"{job}: {info.get('result')}")
    problems = verdict(needs)
    for problem in problems:
        print(f"::error::{problem}")
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
