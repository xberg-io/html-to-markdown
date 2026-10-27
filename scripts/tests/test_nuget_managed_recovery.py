"""Restrict managed recovery to the two generated bool ABI adaptations."""

import importlib.util
import os
import subprocess
from pathlib import Path

import pytest
import yaml

ROOT = Path(__file__).resolve().parents[2]


def load_recovery():
    """Load the actual release guard rather than a test copy."""
    spec = importlib.util.spec_from_file_location(
        "managed_recovery", ROOT / "scripts/ci/csharp/stage_managed_recovery.py"
    )
    assert spec is not None and spec.loader is not None
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


@pytest.mark.parametrize("change", ["valid", "unchanged", "one-call", "extra", "wrong-policy", "missing-hash"])
def test_managed_recovery_allows_only_both_bool_adaptations(change: str) -> None:
    module = load_recovery()
    call = "NativeMethods.ConvertWithAccessibleNames(html, useAccessibleNames, optionsHandle)"
    fixed_call = call.replace("useAccessibleNames", "(useAccessibleNames ? 1 : 0)")
    original = "// alef:hash:" + "a" * 64 + "\n" + call + ";\n" + call + ";\n"
    fixed = original.replace(call, fixed_call).replace("a" * 64, "b" * 64)
    if change == "unchanged":
        fixed = original
    elif change == "one-call":
        fixed = original.replace(call, fixed_call, 1)
    elif change == "extra":
        fixed += "other();\n"
    elif change == "wrong-policy":
        fixed = fixed.replace("? 1 : 0", "? 0 : 1")
    elif change == "missing-hash":
        fixed = fixed.partition("\n")[2]
    if change == "valid":
        assert module.verify_adaptation(original, fixed) == 2
    else:
        with pytest.raises(ValueError, match="Managed recovery refused"):
            module.verify_adaptation(original, fixed)


@pytest.mark.parametrize("change", ["valid", "mode", "dirty", "floating", "wrong-sha", "extra"])
def test_only_validated_committed_wrapper_is_overlaid(tmp_path: Path, change: str) -> None:
    module = load_recovery()

    def git(*args: str) -> str:
        return subprocess.run(
            ["git", "-C", str(tmp_path), *args],
            check=True,
            capture_output=True,
            text=True,
            env={**os.environ, "GIT_CONFIG_GLOBAL": os.devnull, "GIT_CONFIG_NOSYSTEM": "1"},
        ).stdout.strip()

    git("init", "--initial-branch=main")
    git("config", "user.name", "Release Test")
    git("config", "user.email", "release-test@example.com")
    git("config", "core.fileMode", "true")
    wrapper = tmp_path / module.CONVERTER
    wrapper.parent.mkdir(parents=True)
    original = "// alef:hash:" + "a" * 64 + "\n" + (module.CALL + ";\n") * 2
    wrapper.write_text(original)
    git("add", module.CONVERTER)
    git("commit", "-m", "Original")
    original_sha = git("rev-parse", "HEAD")
    fixed = original.replace("useAccessibleNames", "(useAccessibleNames ? 1 : 0)")
    wrapper.write_text(fixed + ("other();\n" if change == "extra" else ""))
    if change == "mode":
        wrapper.chmod(0o755)
    git("add", module.CONVERTER)
    git("commit", "-m", "Generated fix")
    recovery_sha = git("rev-parse", "HEAD")
    git("switch", "--detach", original_sha)
    if change == "dirty":
        wrapper.write_text(original + "dirty();\n")
    recovery_sha = {"floating": "main", "wrong-sha": "0" * 40}.get(change, recovery_sha)
    before = wrapper.read_bytes()
    if change == "valid":
        receipt = module.stage(tmp_path, recovery_sha)
        assert receipt["adapted_calls"] == 2
        assert receipt["original_sha"] == original_sha
        assert wrapper.read_text() == fixed
        assert git("diff", "--name-only") == module.CONVERTER
    else:
        with pytest.raises((ValueError, subprocess.CalledProcessError)):
            module.stage(tmp_path, recovery_sha)
        assert wrapper.read_bytes() == before


def test_managed_recovery_verifies_before_packing_and_publishes_only_nuget() -> None:
    workflow = yaml.safe_load((ROOT / ".github/workflows/publish.yaml").read_text())
    job = workflow["jobs"]["recover-nuget-runtimes"]
    steps = job["steps"]
    names = [step.get("name") for step in steps]
    assert names.index("Verify release source and checksum-only metadata") < names.index(
        "Verify and stage managed ABI repair"
    )
    assert names.index("Verify and stage managed ABI repair") < names.index("Pack verified managed repair")
    managed = next(step for step in steps if step.get("name") == "Pack verified managed repair")
    assert "len(packages) != 7" in managed["run"]
    assert any(step.get("uses", "").startswith("xberg-io/actions/publish-nuget@") for step in steps)
    assert all("retag" not in step.get("uses", "") for step in steps)
    assert "!inputs.nuget_runtime_run_id" in workflow["jobs"]["prepare"]["if"]
