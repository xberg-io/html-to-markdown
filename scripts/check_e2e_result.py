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
            if flag == "true":
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
    if not any(
        step.get("run", "").strip() == VERDICT_COMMAND and (step.get("env") or {}).get("NEEDS") == NEEDS_ENV
        for step in aggregate.get("steps") or []
    ):
        problems.append(f"{AGGREGATE}: no step runs `{VERDICT_COMMAND}` with NEEDS set to `{NEEDS_ENV}`")

    outputs = set((jobs.get(CHANGES) or {}).get("outputs") or {})
    read_outputs: set[str] = set()
    for job_id, job in sorted(jobs.items()):
        if job_id in (CHANGES, AGGREGATE):
            continue
        read = set(CHANGES_OUTPUT.findall(str(job.get("if", ""))))
        read_outputs |= read
        own = f"run-{job_id}"
        if "if" in job and read != {own}:
            problems.append(f"{job_id}: its `if:` must read only needs.changes.outputs.{own}")
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
