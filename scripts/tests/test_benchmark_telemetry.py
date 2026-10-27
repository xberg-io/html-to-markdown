"""Exercise actual telemetry parsing, campaign evidence and subprocess rejection."""

import importlib.util
import json
import sys
from pathlib import Path

import pytest

ROOT = Path(__file__).resolve().parents[2]
spec = importlib.util.spec_from_file_location("benchmark_telemetry", ROOT / "scripts/benchmark_telemetry.py")
assert spec is not None and spec.loader is not None
telemetry = importlib.util.module_from_spec(spec)
sys.modules[spec.name] = telemetry
spec.loader.exec_module(telemetry)


def test_proc_stat_handles_spaces_and_parentheses() -> None:
    fields = ["S", "1", *(["0"] * 9), "17", "19", *(["0"] * 6), "321"]
    assert telemetry.parse_stat("42 (space (worker)) " + " ".join(fields)) == (1, 36, 321)


@pytest.mark.parametrize(
    "command",
    [
        ["cargo", "build"],
        ["rustc", "a.rs"],
        ["python3", "-m", "pytest"],
        ["node", "typescript-language-server"],
        ["htmbench", "run"],
    ],
)
def test_competing_work_is_rejected(command: list[str]) -> None:
    with pytest.raises(ValueError, match="competing workload"):
        telemetry.check_processes([{"pid": 5, "start": 1, "argv": command, "exe": command[0], "ticks": 0}], [])


def test_allowance_requires_pid_start_executable_and_arguments() -> None:
    process = {"pid": 5, "start": 1, "argv": ["htmbench", "run"], "exe": "/bin/htmbench", "ticks": 0}
    telemetry.check_processes([process], [process])
    for key, value in [("pid", 6), ("start", 2), ("exe", "/tmp/htmbench"), ("argv", ["htmbench", "build"])]:
        with pytest.raises(ValueError, match="competing workload"):
            telemetry.check_processes([{**process, key: value}], [process])


def campaign(tmp_path: Path) -> Path:
    runs = tmp_path / "runs"
    runs.mkdir()
    identity = {
        "boot_id": "boot",
        "cpu_count": 4,
        "cpu_model": "CPU",
        "hostname": "host",
        "runner": {
            "RUNNER_ENVIRONMENT": "github-hosted",
            "RUNNER_OS": "Linux",
            "RUNNER_NAME": "runner",
            "RUNNER_ARCH": "X64",
            "ImageOS": "ubuntu24",
            "ImageVersion": "20261027",
            "GITHUB_RUN_ID": "123",
            "GITHUB_RUN_ATTEMPT": "1",
            "GITHUB_SHA": "a" * 40,
        },
    }
    collector = {
        "pid": 9,
        "start": 1,
        "argv": ["python3", "scripts/benchmark_telemetry.py"],
        "exe": "/bin/python3",
        "ticks": 0,
    }
    records = [
        {
            "kind": "header",
            "schema": 1,
            "identity": identity,
            "collector": [collector],
            "physical_policy": "github-managed",
            "benchmark_binary": "/bin/htmbench",
            "runs_directory": str(runs.resolve()),
            "expected_captures": 1,
        }
    ]
    process = {
        "pid": 5,
        "start": 1,
        "argv": ["/bin/htmbench", "run", "--output", str(runs.resolve() / "0001.json")],
        "exe": "/bin/htmbench",
        "ticks": 0,
    }
    for moment in range(0, 311, 5):
        records.append(
            {
                "kind": "sample",
                "monotonic": moment,
                "identity": identity,
                "load5": 0.1,
                "processes": [collector, *([process] if moment > 300 else [])],
            }
        )
        if moment == 300:
            records.append({"kind": "capture_start", "monotonic": moment, "index": 1, "process": process})
    path = runs / "0001.json"
    path.write_text('{"schema":2,"runs":[{}]}')
    records.extend(
        [
            {"kind": "capture_end", "monotonic": 310, "index": 1, "sha256": telemetry.digest(path)},
            {"kind": "complete", "monotonic": 310, "captures": 1},
        ]
    )
    evidence = tmp_path / "telemetry.jsonl"
    evidence.write_text("".join(json.dumps(record) + "\n" for record in records))
    return evidence


def test_valid_campaign_is_bound_to_nonempty_capture(tmp_path: Path) -> None:
    evidence = campaign(tmp_path)
    assert telemetry.validate(evidence, tmp_path / "runs", 1) == {"captures": 1, "samples": 63}


@pytest.mark.parametrize(
    ("fault", "reason"),
    [
        ("busy", "load"),
        ("gap", "gap"),
        ("zero", "samples"),
        ("identity", "identity"),
        ("missing_process", "process"),
        ("changed_capture", "hash"),
        ("missing_end", "complete"),
    ],
)
def test_invalid_campaign_is_rejected(tmp_path: Path, fault: str, reason: str) -> None:
    evidence = campaign(tmp_path)
    rows = [json.loads(line) for line in evidence.read_text().splitlines()]
    samples = [row for row in rows if row["kind"] == "sample"]
    if fault == "busy":
        samples[10]["load5"] = 1.0
    elif fault == "gap":
        rows = [row for row in rows if row not in samples[10:14]]
    elif fault == "zero":
        rows = [row for row in rows if row["kind"] != "sample"]
    elif fault == "identity":
        samples[10]["identity"]["boot_id"] = "changed"
    elif fault == "missing_process":
        del samples[10]["processes"]
    elif fault == "changed_capture":
        (tmp_path / "runs/0001.json").write_text("changed")
    elif fault == "missing_end":
        rows.pop()
    evidence.write_text("".join(json.dumps(row) + "\n" for row in rows))
    with pytest.raises(ValueError, match=reason):
        telemetry.validate(evidence, tmp_path / "runs", 1)


def test_empty_process_inventory_is_not_quiet() -> None:
    with pytest.raises(ValueError, match="process"):
        telemetry.check_processes([], [])


def test_real_proc_files_are_parsed_and_disappeared_process_is_ignored(tmp_path: Path) -> None:
    directory = tmp_path / "42"
    directory.mkdir()
    fields = ["S", "1", *(["0"] * 9), "17", "19", *(["0"] * 6), "321"]
    (directory / "stat").write_text("42 (space (worker)) " + " ".join(fields))
    (directory / "cmdline").write_bytes(b"cargo\0build\0")
    (directory / "exe").symlink_to(sys.executable)
    (tmp_path / "43").mkdir()
    observed = telemetry.processes(tmp_path)
    assert len(observed) == 1
    assert observed[0]["ticks"] == 36
    assert observed[0]["argv"] == ["cargo", "build"]


def test_actual_failed_subprocess_does_not_create_capture_end(tmp_path: Path, monkeypatch) -> None:
    binary = tmp_path / "benchmark"
    binary.write_text("#!/bin/sh\nsleep 0.05\nexit 7\n")
    binary.chmod(0o755)
    recorder = telemetry.Recorder(tmp_path / "events.jsonl", {}, {})
    monkeypatch.setattr(telemetry, "SAMPLE_SECONDS", 0.01)
    monkeypatch.setattr(
        telemetry,
        "process_identity",
        lambda pid: {
            "pid": pid,
            "exe": str(binary),
            "argv": [str(binary), "run", "--output", str(tmp_path / "0001.json")],
        },
    )
    monkeypatch.setattr(recorder, "sample", lambda _active: None)
    try:
        with pytest.raises(ValueError, match="capture failed: 7"):
            telemetry.run_capture(recorder, binary, tmp_path, 1)
    finally:
        recorder.output.close()
    assert [json.loads(line)["kind"] for line in (tmp_path / "events.jsonl").read_text().splitlines()] == [
        "capture_start"
    ]


def test_telemetry_failure_terminates_actual_benchmark_process(tmp_path: Path, monkeypatch) -> None:
    binary = tmp_path / "benchmark"
    binary.write_text("#!/bin/sh\nsleep 30\n")
    binary.chmod(0o755)
    recorder = telemetry.Recorder(tmp_path / "events.jsonl", {}, {})
    child_ids = []

    def identify(pid):
        child_ids.append(pid)
        return {"pid": pid, "exe": str(binary), "argv": [str(binary), "run", "--output", str(tmp_path / "0001.json")]}

    def reject(active):
        raise ValueError("planted busy telemetry")

    monkeypatch.setattr(telemetry, "process_identity", identify)
    monkeypatch.setattr(recorder, "sample", reject)
    try:
        with pytest.raises(ValueError, match="planted busy"):
            telemetry.run_capture(recorder, binary, tmp_path, 1)
    finally:
        recorder.output.close()
    import os

    with pytest.raises(ProcessLookupError):
        os.kill(child_ids[0], 0)


@pytest.mark.parametrize("value", [float("nan"), float("inf"), -1, None, True])
def test_malformed_five_minute_load_is_rejected(tmp_path: Path, value) -> None:
    evidence = campaign(tmp_path)
    rows = [json.loads(line) for line in evidence.read_text().splitlines()]
    next(row for row in rows if row["kind"] == "sample")["load5"] = value
    evidence.write_text("".join(json.dumps(row) + "\n" for row in rows))
    with pytest.raises(ValueError, match="malformed five-minute load"):
        telemetry.validate(evidence, tmp_path / "runs", 1)


def test_unknown_cpu_workload_cannot_hide_behind_quiet_load(tmp_path: Path) -> None:
    evidence = campaign(tmp_path)
    rows = [json.loads(line) for line in evidence.read_text().splitlines()]
    samples = [row for row in rows if row["kind"] == "sample"]
    for index, sample in enumerate(samples):
        sample["processes"].append(
            {"pid": 23, "start": 4, "argv": ["innocent"], "exe": "/bin/innocent", "ticks": index * 1000}
        )
    evidence.write_text("".join(json.dumps(row) + "\n" for row in rows))
    with pytest.raises(ValueError, match="competing background CPU workload"):
        telemetry.validate(evidence, tmp_path / "runs", 1)


def test_malformed_json_is_not_a_success_marker(tmp_path: Path) -> None:
    evidence = tmp_path / "telemetry.jsonl"
    evidence.write_text("{not-json}\n")
    with pytest.raises(ValueError, match="malformed telemetry"):
        telemetry.validate(evidence, tmp_path / "runs", 40)


@pytest.mark.parametrize(
    "field",
    ["RUNNER_NAME", "RUNNER_ARCH", "ImageOS", "ImageVersion", "GITHUB_RUN_ID", "GITHUB_RUN_ATTEMPT", "GITHUB_SHA"],
)
def test_missing_pinned_runner_identity_fails(tmp_path: Path, field: str) -> None:
    evidence = campaign(tmp_path)
    rows = [json.loads(line) for line in evidence.read_text().splitlines()]
    rows[0]["identity"]["runner"][field] = ""
    evidence.write_text("".join(json.dumps(row) + "\n" for row in rows))
    with pytest.raises(ValueError, match="missing pinned runner identity"):
        telemetry.validate(evidence, tmp_path / "runs", 1)


@pytest.mark.parametrize("executable", ["cargo", "rustc"])
def test_fake_capture_allowance_cannot_exempt_compiler(tmp_path: Path, executable: str) -> None:
    evidence = campaign(tmp_path)
    rows = [json.loads(line) for line in evidence.read_text().splitlines()]
    start = next(row for row in rows if row["kind"] == "capture_start")
    start["process"]["exe"] = "/bin/" + executable
    start["process"]["argv"] = ["/bin/" + executable, "build"]
    rows[0]["benchmark_binary"] = "/bin/" + executable
    evidence.write_text("".join(json.dumps(row) + "\n" for row in rows))
    with pytest.raises(ValueError, match="invalid intended benchmark executable"):
        telemetry.validate(evidence, tmp_path / "runs", 1)


def test_wrong_capture_output_path_is_not_an_intended_benchmark(tmp_path: Path) -> None:
    evidence = campaign(tmp_path)
    rows = [json.loads(line) for line in evidence.read_text().splitlines()]
    start = next(row for row in rows if row["kind"] == "capture_start")
    start["process"]["argv"][-1] = "/elsewhere/0001.json"
    evidence.write_text("".join(json.dumps(row) + "\n" for row in rows))
    with pytest.raises(ValueError, match="invalid intended benchmark arguments"):
        telemetry.validate(evidence, tmp_path / "runs", 1)
