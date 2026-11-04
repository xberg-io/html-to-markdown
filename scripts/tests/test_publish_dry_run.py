"""Verify source-built release packages remain buildable before registry publication."""

from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parents[2]
DRY_RUN_EXPRESSION = "${{ needs.prepare.outputs.dry_run }}"


def test_source_package_builds_receive_dry_run_mode() -> None:
    """Dry runs must not resolve the unpublished workspace crate from crates.io."""
    workflow = yaml.safe_load((ROOT / ".github/workflows/publish.yaml").read_text())
    jobs = workflow["jobs"]
    build_actions = {
        "python-sdist": "xberg-io/actions/build-python-sdist@v1",
        "ruby-gem": "xberg-io/actions/build-ruby-gem@v1",
        "php-extension": "xberg-io/actions/build-php-extension@v1",
    }

    for job_name, action in build_actions.items():
        step = next(step for step in jobs[job_name]["steps"] if step.get("uses") == action)
        assert step["with"]["dry-run"] == DRY_RUN_EXPRESSION


def test_elixir_dry_run_skips_registry_dependency_rewrite() -> None:
    """Elixir dry runs must retain the workspace dependency path."""
    workflow = yaml.safe_load((ROOT / ".github/workflows/publish.yaml").read_text())
    steps = workflow["jobs"]["elixir-natives"]["steps"]
    rewrite = next(step for step in steps if step.get("uses") == "xberg-io/actions/rewrite-native-deps@v1")

    assert rewrite["if"] == "needs.prepare.outputs.dry_run != 'true'"


def test_node_release_builds_allow_napi_platform_regeneration() -> None:
    """The NAPI action deletes and recreates workspace platform packages."""
    workflow = yaml.safe_load((ROOT / ".github/workflows/publish.yaml").read_text())

    assert workflow["jobs"]["node-bindings"]["env"] == {"PNPM_CONFIG_FROZEN_LOCKFILE": "false"}


def test_swift_release_finalizer_only_verifies_prepared_source_and_assets() -> None:
    """Recovery uses exact tagged source and cannot finalize a checksum child."""
    workflow = yaml.safe_load((ROOT / ".github/workflows/publish.yaml").read_text())
    steps = workflow["jobs"]["update-swift-package-manifest"]["steps"]
    checkout = next(step for step in steps if step.get("uses") == "actions/checkout@v7")
    update = next(step for step in steps if step.get("name") == "Publish immutable Swift references")
    assert checkout["with"]["ref"] == "${{ needs.prepare.outputs.source_sha }}"
    assert "scripts/ci/swift_release.py publish" in update["run"]
    assert "finalize_release_manifest.py" not in update["run"]
    assert "git tag" not in update["run"]


def test_release_report_accepts_already_finalized_release_and_scoop_manifest() -> None:
    """Recovery reporting must recognize external state created by an earlier run."""
    workflow = yaml.safe_load((ROOT / ".github/workflows/publish.yaml").read_text())
    steps = workflow["jobs"]["release-report"]["steps"]
    report = next(step for step in steps if step.get("name") == "Verify every enabled publish target succeeded")
    script = report["run"]

    assert report["env"]["VERSION"] == "${{ needs.prepare.outputs.version }}"
    assert 'release_is_draft="$(gh release view' in script
    assert 'if [[ "${RESULT_FINALIZE}" != "success" && "${release_is_draft}" != "false" ]]' in script
    assert "repos/xberg-io/scoop-bucket/contents/bucket/html-to-markdown.json" in script
    assert '"${scoop_version}" == "${VERSION}"' in script
