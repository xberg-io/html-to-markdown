"""Verify source-built release packages remain buildable before registry publication."""

from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parents[2]
DRY_RUN_EXPRESSION = "${{ needs.prepare.outputs.dry_run }}"


def test_publish_recovery_uses_the_tag_concurrency_group() -> None:
    """A tag-only repository dispatch must queue behind a manual publish of that tag."""
    workflow = yaml.safe_load((ROOT / ".github/workflows/publish.yaml").read_text())

    assert workflow["concurrency"] == {
        "group": "${{ github.workflow }}-${{ inputs.nuget_runtime_run_id && "
        "format('nuget-runtimes-{0}', inputs.nuget_runtime_run_id) || "
        "(github.event_name == 'workflow_dispatch' && (github.event.inputs.ref || github.event.inputs.tag)) || "
        "(github.event_name == 'repository_dispatch' && !github.event.client_payload.ref && "
        "github.event.client_payload.dry_run != true && github.event.client_payload.dry_run != 'true' && "
        "github.event.client_payload.tag) || github.ref || github.run_id }}",
        "cancel-in-progress": False,
    }


def test_publish_rust_setup_omits_coverage_tool() -> None:
    """Publish builders do not need cargo-llvm-cov, which has no locked Windows ARM artifact."""
    workflow = yaml.safe_load((ROOT / ".github/workflows/publish.yaml").read_text())
    rust_setup = {
        job: [step for step in body.get("steps", []) if step.get("uses") == "xberg-io/actions/setup-rust@v1"]
        for job, body in workflow["jobs"].items()
    }
    assert sum(map(len, rust_setup.values())) == 18
    unexpected = {
        job
        for job, steps in rust_setup.items()
        if any(step.get("with", {}).get("install-llvm-cov") != "false" for step in steps)
    }
    assert not unexpected, f"publish jobs install the coverage tool: {sorted(unexpected)}"


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


def test_swift_release_job_only_verifies_the_tag_and_adds_the_branch() -> None:
    """Publication checks out the tagged source and hands it to the shared immutable action."""
    workflow = yaml.safe_load((ROOT / ".github/workflows/publish.yaml").read_text())
    steps = workflow["jobs"]["update-swift-package-manifest"]["steps"]
    checkout = next(step for step in steps if step.get("uses") == "actions/checkout@v7")
    assert checkout["with"]["ref"] == "${{ needs.prepare.outputs.source_sha }}"
    assert [step for step in steps if step.get("uses") == "xberg-io/actions/publish-swift-release@v1"]
    for step in steps:
        assert "git tag" not in step.get("run", "")
        assert "swift_release.py publish" not in step.get("run", "")


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


def test_rubygems_installs_locked_ruby_from_verified_release_source() -> None:
    """The artifact-only publisher needs the tagged mise configuration before resolving Ruby."""
    workflow = yaml.safe_load((ROOT / ".github/workflows/publish.yaml").read_text())
    steps = workflow["jobs"]["publish-rubygems"]["steps"]
    checkout = next((step for step in steps if step.get("uses") == "actions/checkout@v7"), None)
    download = next(step for step in steps if step.get("uses") == "actions/download-artifact@v8")
    setup = next(step for step in steps if step.get("uses") == "xberg-io/actions/setup-mise@v1")

    assert checkout is not None
    assert checkout["with"]["ref"] == "${{ needs.prepare.outputs.source_sha }}"
    assert checkout["with"]["persist-credentials"] is False
    assert steps.index(checkout) < steps.index(download) < steps.index(setup)
    assert setup["with"]["tools"] == "ruby"
    assert download["with"]["pattern"] == "rubygems-*"
