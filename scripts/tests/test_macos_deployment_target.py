"""Every shipped macOS binary targets the macOS minimum the root Package.swift declares."""

import re
from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parents[2]
WORKFLOWS = sorted((ROOT / ".github/workflows").glob("*.y*ml"))
VARIABLE = "MACOSX_DEPLOYMENT_TARGET"
ASSIGNMENT = re.compile(rf"{VARIABLE}\s*[=:]\s*[\"']?([0-9][0-9.]*)")


def package_minimum() -> str:
    """Read `.macOS(.vNN)` from the root manifest SwiftPM consumers resolve."""
    platforms = re.findall(r"\.macOS\(\.v(\d+)(?:_(\d+))?\)", (ROOT / "Package.swift").read_text())
    assert len(platforms) == 1, platforms
    major, minor = platforms[0]
    return f"{major}.{minor or 0}"


def environments(workflow: dict):
    """Yield (location, env mapping) for the workflow, every job and every step."""
    yield "workflow", workflow.get("env") or {}
    for name, job in workflow.get("jobs", {}).items():
        yield name, job.get("env") or {}
        for index, step in enumerate(job.get("steps", [])):
            yield f"{name} step {index}", step.get("env") or {}


def declared_targets() -> list[tuple[str, str]]:
    """Every deployment target a workflow, action input or build script sets."""
    found = []
    for path in WORKFLOWS:
        workflow = yaml.safe_load(path.read_text())
        for location, env in environments(workflow):
            if VARIABLE in env:
                found.append((f"{path.name} {location}", str(env[VARIABLE])))
        for name, job in workflow.get("jobs", {}).items():
            for step in job.get("steps", []):
                for key, value in (step.get("with") or {}).items():
                    if "deployment-target" in key:
                        found.append((f"{path.name} {name} {key}", str(value)))
                    found += [(f"{path.name} {name} {key}", match) for match in ASSIGNMENT.findall(str(value))]
                found += [(f"{path.name} {name} run", match) for match in ASSIGNMENT.findall(step.get("run", ""))]
    for directory in ("scripts", "mise-tasks"):
        for path in sorted((ROOT / directory).rglob("*")):
            if path.is_file() and path.suffix in {".sh", ".py", ".toml", ".bash"} and "tests" not in path.parts:
                found += [(str(path.relative_to(ROOT)), match) for match in ASSIGNMENT.findall(path.read_text())]
    found += [("mise.toml", match) for match in ASSIGNMENT.findall((ROOT / "mise.toml").read_text())]
    return found


def test_package_minimum_is_macos_26() -> None:
    """The equality rule below is anchored on the manifest, which declares macOS 26."""
    assert package_minimum() == "26.0"


def test_every_declared_macos_deployment_target_equals_the_package_minimum() -> None:
    """No workflow or script builds a macOS binary for an older system than SwiftPM allows."""
    minimum = package_minimum()
    targets = declared_targets()
    assert targets, "no deployment target declared anywhere"
    wrong = [(location, value) for location, value in targets if value != minimum]
    assert not wrong, f"deployment targets differ from Package.swift's macOS {minimum}: {wrong}"


def test_every_macos_publish_job_builds_with_the_package_minimum() -> None:
    """A macOS job without the variable falls back to the toolchain default (10.12 or 11.0)."""
    minimum = package_minimum()
    workflow = yaml.safe_load((ROOT / ".github/workflows/publish.yaml").read_text())
    inherited = (workflow.get("env") or {}).get(VARIABLE)
    macos_jobs = {
        name: job
        for name, job in workflow["jobs"].items()
        if "macos" in str(job.get("runs-on", "")) or "macos" in str(job.get("strategy", {}).get("matrix", ""))
    }
    assert len(macos_jobs) >= 10, sorted(macos_jobs)
    missing = sorted(
        name for name, job in macos_jobs.items() if (job.get("env") or {}).get(VARIABLE, inherited) != minimum
    )
    assert not missing, f"macOS publish jobs without MACOSX_DEPLOYMENT_TARGET={minimum}: {missing}"
