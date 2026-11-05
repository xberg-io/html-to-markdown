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


def test_package_minimum_is_macos_15() -> None:
    """The equality rule below is anchored on the manifest: macOS 15, the oldest macOS Apple supports."""
    assert package_minimum() == "15.0"


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


# ~keep A called workflow does not inherit the caller's `env:`, so each one that builds a macOS binary must be
# ~keep handed the target as an input. Every remote reusable workflow a shipping workflow calls is classified here;
# ~keep a new one fails the test until someone decides which kind it is.
SHIPPING_WORKFLOWS = ("publish.yaml", "prepare-swift-release.yaml")
MACOS_BUILDERS = {
    "xberg-io/actions/.github/workflows/reusable-cli-release.yml@v1": "required",
    # Empty derives the target from Package.swift's .macOS minimum, and a set value must equal it.
    "xberg-io/actions/.github/workflows/reusable-swift-release-prepare.yml@v1": "derived",
}
NOT_MACOS_BUILDERS = {"xberg-io/actions/.github/workflows/reusable-binstall-verify.yml@v1"}


def reusable_jobs() -> list[tuple[str, str, dict]]:
    """(workflow job, called workflow, job) for every remote reusable call in a shipping workflow."""
    found = []
    for name in SHIPPING_WORKFLOWS:
        workflow = yaml.safe_load((ROOT / ".github/workflows" / name).read_text())
        for job_name, job in workflow["jobs"].items():
            uses = job.get("uses", "")
            if uses and not uses.startswith("./"):
                found.append((f"{name} {job_name}", uses, job))
    return found


def test_every_reusable_macos_build_receives_the_package_minimum() -> None:
    """A reusable job that builds on macOS gets macos-deployment-target equal to Package.swift's minimum."""
    minimum = package_minimum()
    calls = reusable_jobs()
    assert {uses for _, uses, _ in calls} >= set(MACOS_BUILDERS), calls
    unclassified = [(job, uses) for job, uses, _ in calls if uses not in MACOS_BUILDERS.keys() | NOT_MACOS_BUILDERS]
    assert not unclassified, f"classify these reusable workflows as macOS builders or not: {unclassified}"
    wrong = []
    for location, uses, job in calls:
        mode = MACOS_BUILDERS.get(uses)
        value = str((job.get("with") or {}).get("macos-deployment-target", ""))
        if (mode == "required" and value != minimum) or (mode == "derived" and value not in ("", minimum)):
            wrong.append((location, value or "<unset>"))
    assert not wrong, f"reusable macOS builds without macos-deployment-target={minimum}: {wrong}"
