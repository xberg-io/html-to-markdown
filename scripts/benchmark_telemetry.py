"""Collect and independently validate quiet GitHub-hosted Linux benchmark evidence."""

import argparse
import hashlib
import json
import math
import os
import re
import signal
import socket
import subprocess
import time
from itertools import pairwise
from pathlib import Path

SAMPLE_SECONDS = 5
MAX_GAP_SECONDS = 10
QUIET_SECONDS = 300
LOAD_FRACTION = 0.25
BACKGROUND_CPU_FRACTION = 0.05
SETTLE_SECONDS = 1800
WORKLOAD = re.compile(
    r"^(cargo|rustc|rustdoc|cc|gcc|g\+\+|clang.*|cmake|ninja|make|pytest.*|htmbench|"
    r".*language.server.*|rust-analyzer|indexer|ctest|dotnet|javac|go|swift.*|ruby|php|beam.*)$"
)
RUNNER_KEYS = (
    "RUNNER_ENVIRONMENT",
    "RUNNER_OS",
    "RUNNER_ARCH",
    "RUNNER_NAME",
    "ImageOS",
    "ImageVersion",
    "GITHUB_RUN_ID",
    "GITHUB_RUN_ATTEMPT",
    "GITHUB_SHA",
)


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def parse_stat(raw: str) -> tuple[int, int, int]:
    fields = raw[raw.rindex(")") + 2 :].split()
    return int(fields[1]), int(fields[11]) + int(fields[12]), int(fields[19])


def processes(proc: Path = Path("/proc")) -> list[dict]:
    result = []
    for directory in proc.iterdir():
        if not directory.name.isdecimal():
            continue
        try:
            parent, ticks, start = parse_stat((directory / "stat").read_text())
            argv = (directory / "cmdline").read_bytes().rstrip(b"\0").decode().split("\0")
            if argv == [""]:
                continue
            try:
                executable = str((directory / "exe").resolve(strict=True))
            except PermissionError:
                executable = argv[0]
            result.append(
                {
                    "pid": int(directory.name),
                    "parent": parent,
                    "ticks": ticks,
                    "start": start,
                    "argv": argv,
                    "exe": executable,
                }
            )
        except (FileNotFoundError, ProcessLookupError):
            continue
    if not result:
        raise ValueError("missing process evidence")
    return result


def check_processes(observed: list[dict], allowances: list[dict]) -> None:
    if not isinstance(observed, list) or not observed:
        raise ValueError("malformed process evidence")
    for process in observed:
        for key in ("pid", "start", "ticks", "argv", "exe"):
            if key not in process:
                raise ValueError("malformed process evidence")
        if any(
            all(process[key] == permitted[key] for key in ("pid", "start", "argv", "exe")) for permitted in allowances
        ):
            continue
        argv = process["argv"]
        executable = Path(process["exe"]).name
        interpreted = " ".join(argv[1:]) if executable.startswith(("python", "node")) else ""
        if WORKLOAD.fullmatch(executable) or re.search(
            r"pytest|unittest|language.server|indexer|benchmark", interpreted
        ):
            raise ValueError(f"competing workload: pid={process['pid']} argv={argv}")


def identity() -> dict:
    cpuinfo = Path("/proc/cpuinfo").read_text()
    model = next(line.split(":", 1)[1].strip() for line in cpuinfo.splitlines() if line.startswith("model name"))
    return {
        "boot_id": Path("/proc/sys/kernel/random/boot_id").read_text().strip(),
        "cpu_count": os.cpu_count(),
        "cpu_model": model,
        "hostname": socket.gethostname(),
        "runner": {key: os.environ.get(key, "") for key in RUNNER_KEYS},
    }


def check_identity(host: dict) -> None:
    if not host.get("boot_id") or not host.get("cpu_model") or not host.get("hostname"):
        raise ValueError("missing guest identity")
    if not isinstance(host.get("cpu_count"), int) or host["cpu_count"] <= 0:
        raise ValueError("invalid CPU count")
    runner = host.get("runner", {})
    required = ("RUNNER_NAME", "RUNNER_ARCH", "ImageOS", "ImageVersion", "GITHUB_RUN_ID", "GITHUB_RUN_ATTEMPT")
    if any(not runner.get(key) for key in required) or not re.fullmatch(r"[0-9a-f]{40}", runner.get("GITHUB_SHA", "")):
        raise ValueError("missing pinned runner identity")
    if runner.get("RUNNER_ENVIRONMENT") != "github-hosted" or runner.get("RUNNER_OS") != "Linux":
        raise ValueError("policy requires GitHub-hosted Linux")


def check_sample(sample: dict, host: dict, permitted: list[dict]) -> None:
    if sample.get("identity") != host:
        raise ValueError("guest/runner identity changed")
    load = sample.get("load5")
    if isinstance(load, bool) or not isinstance(load, (int, float)) or not math.isfinite(load) or load < 0:
        raise ValueError("malformed five-minute load")
    if load >= host["cpu_count"] * LOAD_FRACTION:
        raise ValueError(f"five-minute load {load} exceeds quiet limit")
    if "processes" not in sample:
        raise ValueError("missing process evidence")
    check_processes(sample["processes"], permitted)


def read_records(path: Path) -> list[dict]:
    try:
        rows = [json.loads(line) for line in path.read_text().splitlines()]
    except (json.JSONDecodeError, OSError) as error:
        raise ValueError("missing or malformed telemetry") from error
    if not rows or rows[0].get("kind") != "header" or rows[-1].get("kind") != "complete":
        raise ValueError("campaign not complete")
    return rows


def check_observed_sample(row: dict, previous: float | None, header: dict, permitted: list[dict]) -> None:
    moment = row["monotonic"]
    if previous is not None and not 0 < moment - previous <= MAX_GAP_SECONDS:
        raise ValueError("telemetry sample gap or nonmonotonic clock")
    check_sample(row, header["identity"], permitted)
    for required in permitted:
        if not any(
            all(process[key] == required[key] for key in ("pid", "start", "argv", "exe"))
            for process in row["processes"]
        ):
            raise ValueError("missing collector/benchmark process evidence")


def check_benchmark_process(row: dict, header: dict) -> None:
    process = row["process"]
    executable = header.get("benchmark_binary", "")
    recorded_runs = header.get("runs_directory", "")
    expected_output = str(Path(recorded_runs) / f"{row['index']:04d}.json")
    if not Path(executable).is_absolute() or Path(executable).name != "htmbench":
        raise ValueError("invalid intended benchmark executable")
    if not Path(recorded_runs).is_absolute():
        raise ValueError("missing canonical capture directory")
    if process.get("exe") != executable or process.get("argv") != [executable, "run", "--output", expected_output]:
        raise ValueError("invalid intended benchmark arguments")
    if any(not isinstance(process.get(key), int) or process[key] <= 0 for key in ("pid", "start")):
        raise ValueError("invalid intended benchmark process identity")


def check_event_clock(moment: float, previous: float | None) -> None:
    if not isinstance(moment, (int, float)) or not math.isfinite(moment):
        raise ValueError("malformed telemetry clock")
    if previous is not None and moment < previous:
        raise ValueError("nonmonotonic telemetry event clock")


def validate_samples(rows: list[dict], header: dict) -> list[dict]:
    samples = [row for row in rows if row.get("kind") == "sample"]
    if not samples:
        raise ValueError("insufficient continuous samples")
    permitted = header.get("collector", [])
    if len(permitted) != 1 or not any(Path(arg).name == "benchmark_telemetry.py" for arg in permitted[0]["argv"]):
        raise ValueError("missing exact collector process identity")
    active = []
    previous = None
    previous_event = None
    active_index = None
    for row in rows[1:]:
        moment = row.get("monotonic")
        check_event_clock(moment, previous_event)
        previous_event = moment
        if row["kind"] == "capture_start":
            if active_index is not None:
                raise ValueError("overlapping captures")
            check_benchmark_process(row, header)
            active = [row["process"]]
            active_index = row["index"]
        elif row["kind"] == "capture_end":
            if row["index"] != active_index:
                raise ValueError("capture end without matching start")
            active = []
            active_index = None
        elif row["kind"] == "sample":
            check_observed_sample(row, previous, header, permitted + active)
            previous = moment
        elif row["kind"] == "settling":
            if previous is not None:
                raise ValueError("settling after quiet preflight started")
            if row.get("identity") != header["identity"] or not row.get("processes"):
                raise ValueError("missing settling process/identity evidence")
        elif row["kind"] != "complete":
            raise ValueError("unknown telemetry record")
    if active_index is not None:
        raise ValueError("unfinished capture")
    if rows[-1]["monotonic"] - samples[-1]["monotonic"] > MAX_GAP_SECONDS:
        raise ValueError("telemetry gap at campaign completion")
    if len(samples) < QUIET_SECONDS // SAMPLE_SECONDS + 1:
        raise ValueError("insufficient continuous samples")
    check_background_cpu(samples, header)
    return samples


def check_background_cpu(samples: list[dict], header: dict) -> None:
    ticks_per_second = header.get("clock_ticks", 100)
    if not isinstance(ticks_per_second, int) or not 30 <= ticks_per_second <= 10000:
        raise ValueError("invalid process CPU clock rate")
    for before, after in pairwise(samples):
        previous = {(process["pid"], process["start"]): process for process in before["processes"]}
        elapsed = after["monotonic"] - before["monotonic"]
        background_ticks = 0
        for process in after["processes"]:
            old = previous.get((process["pid"], process["start"]))
            if old is None:
                continue
            delta = process["ticks"] - old["ticks"]
            if delta < 0:
                raise ValueError("process CPU counters moved backwards")
            if process["pid"] in [entry["pid"] for entry in header.get("collector", [])]:
                continue
            if Path(process["exe"]).name == "htmbench":
                continue
            background_ticks += delta
        limit = elapsed * ticks_per_second * header["identity"]["cpu_count"] * BACKGROUND_CPU_FRACTION
        if background_ticks > limit:
            raise ValueError("competing background CPU workload")


def validate_captures(rows: list[dict], samples: list[dict], runs: Path, expected: int) -> None:
    starts = [row for row in rows if row.get("kind") == "capture_start"]
    ends = [row for row in rows if row.get("kind") == "capture_end"]
    indices = list(range(1, expected + 1))
    if [row["index"] for row in starts] != indices or [row["index"] for row in ends] != indices:
        raise ValueError("capture count/order mismatch")
    if starts[0]["monotonic"] - samples[0]["monotonic"] < QUIET_SECONDS:
        raise ValueError("missing five-minute quiet preflight")
    actual = sorted(path.name for path in runs.glob("*.json"))
    if actual != [f"{index:04d}.json" for index in indices]:
        raise ValueError("capture file inventory mismatch")
    for start, end in zip(starts, ends, strict=True):
        before = [sample for sample in samples if sample["monotonic"] <= start["monotonic"]]
        after = [sample for sample in samples if sample["monotonic"] >= end["monotonic"]]
        if not before or not after or end["monotonic"] <= start["monotonic"]:
            raise ValueError("capture lacks continuous telemetry coverage")
        path = runs / f"{start['index']:04d}.json"
        if digest(path) != end["sha256"]:
            raise ValueError("capture hash mismatch")
        capture = json.loads(path.read_text())
        if capture.get("schema") != 2 or not capture.get("runs"):
            raise ValueError("empty/non-v2 benchmark capture")


def validate(path: Path, runs: Path, expected: int) -> dict:
    rows = read_records(path)
    header = rows[0]
    if header.get("schema") != 1 or header.get("physical_policy") != "github-managed":
        raise ValueError("unsupported telemetry schema/physical policy")
    if header.get("expected_captures") != expected or rows[-1].get("captures") != expected or expected <= 0:
        raise ValueError("capture count mismatch")
    check_identity(header["identity"])
    samples = validate_samples(rows, header)
    validate_captures(rows, samples, runs, expected)
    return {"captures": expected, "samples": len(samples)}


class Recorder:
    """Flush raw observations before enforcing gates so failures retain diagnosis."""

    def __init__(self, path: Path, host: dict, collector: dict) -> None:
        self.output = path.open("x")
        self.host = host
        self.collector = collector
        self.last_sample = None
        self.last_row = None

    def write(self, kind: str, **values) -> None:
        """Retain one raw campaign event immediately."""
        self.output.write(json.dumps({"kind": kind, "monotonic": time.monotonic(), **values}) + "\n")
        self.output.flush()

    def sample(self, active: list[dict]) -> None:
        """Observe the full guest and reject unsafe conditions."""
        observed = processes()
        sample = {
            "identity": identity(),
            "load5": float(Path("/proc/loadavg").read_text().split()[1]),
            "processes": observed,
        }
        sample["monotonic"] = time.monotonic()
        self.write("sample", **sample)
        check_sample(sample, self.host, [self.collector, *active])
        moment = time.monotonic()
        if self.last_sample is not None and moment - self.last_sample > MAX_GAP_SECONDS:
            raise ValueError("live telemetry sample gap")
        if self.last_row is not None:
            check_background_cpu(
                [self.last_row, sample],
                {"identity": self.host, "collector": [self.collector], "clock_ticks": os.sysconf("SC_CLK_TCK")},
            )
        self.last_row = sample
        self.last_sample = moment

    def settle(self) -> None:
        """Wait for build load to decay before starting the uninterrupted quiet window."""
        deadline = time.monotonic() + SETTLE_SECONDS
        while True:
            sample = {
                "identity": identity(),
                "load5": float(Path("/proc/loadavg").read_text().split()[1]),
                "processes": processes(),
            }
            try:
                check_sample(sample, self.host, [self.collector])
                return
            except ValueError:
                self.write("settling", **sample)
                if time.monotonic() >= deadline:
                    raise ValueError("runner did not settle within 30 minutes") from None
                time.sleep(SAMPLE_SECONDS)


def process_identity(pid: int) -> dict:
    return next(process for process in processes() if process["pid"] == pid)


def run_capture(recorder: Recorder, binary: Path, runs: Path, index: int) -> None:
    path = runs / f"{index:04d}.json"
    command = [str(binary), "run", "--output", str(path)]
    with subprocess.Popen(command, start_new_session=True) as child:
        try:
            process = process_identity(child.pid)
            if process["exe"] != str(binary) or process["argv"] != command:
                raise ValueError("benchmark process identity mismatch")
            recorder.write("capture_start", index=index, process=process)
            while child.poll() is None:
                recorder.sample([process])
                time.sleep(SAMPLE_SECONDS)
            if child.returncode != 0:
                raise ValueError(f"benchmark capture failed: {child.returncode}")
            recorder.write("capture_end", index=index, sha256=digest(path))
            recorder.sample([])
        except BaseException:
            if child.poll() is None:
                os.killpg(child.pid, signal.SIGTERM)
                try:
                    child.wait(timeout=5)
                except subprocess.TimeoutExpired:
                    os.killpg(child.pid, signal.SIGKILL)
            raise


def collect(path: Path, runs: Path, binary: Path, count: int) -> None:
    host = identity()
    check_identity(host)
    if count not in (1, 40):
        raise ValueError("collection requires one guardrail capture or exactly 40 calibration captures")
    runs = runs.resolve()
    binary = binary.resolve(strict=True)
    runs.mkdir(parents=True, exist_ok=True)
    path.parent.mkdir(parents=True, exist_ok=True)
    if any(runs.iterdir()):
        raise ValueError("capture directory must be empty")
    recorder = Recorder(path, host, process_identity(os.getpid()))
    try:
        recorder.write(
            "header",
            schema=1,
            identity=host,
            physical_policy="github-managed",
            expected_captures=count,
            benchmark_binary=str(binary),
            runs_directory=str(runs),
            collector=[recorder.collector],
            clock_ticks=os.sysconf("SC_CLK_TCK"),
        )
        recorder.settle()
        start = time.monotonic()
        while True:
            recorder.sample([])
            if time.monotonic() - start >= QUIET_SECONDS:
                break
            time.sleep(SAMPLE_SECONDS)
        for index in range(1, count + 1):
            run_capture(recorder, binary.resolve(strict=True), runs, index)
        recorder.write("complete", captures=count)
    except (ValueError, OSError, StopIteration) as error:
        recorder.write("invalid", error=str(error))
        raise
    finally:
        recorder.output.close()
    print(json.dumps(validate(path, runs, count)))


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("mode", choices=("collect", "validate"))
    parser.add_argument("--telemetry", type=Path, required=True)
    parser.add_argument("--runs-dir", type=Path, required=True)
    parser.add_argument("--count", type=int, choices=(1, 40), default=40)
    parser.add_argument("--binary", type=Path)
    args = parser.parse_args()
    if args.mode == "collect":
        if args.binary is None:
            parser.error("collect requires --binary")
        collect(args.telemetry, args.runs_dir, args.binary, args.count)
    else:
        print(json.dumps(validate(args.telemetry, args.runs_dir, args.count)))


if __name__ == "__main__":
    main()
