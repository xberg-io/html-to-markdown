"""Hold the Publish Release workflow to the shared immutable Swift release flow."""

import hashlib
import json
import os
import re
import shutil
import subprocess
import sys
import tempfile
import unittest
import zipfile
from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parents[2]


VERSION = "3.19.2"
TAG = f"v{VERSION}"
ARCHIVE = "HtmlToMarkdown-rs.artifactbundle.zip"
RECEIPT = "HtmlToMarkdown-rs.artifactbundle.release.json"
TRIPLES = (
    "arm64-apple-macosx",
    "x86_64-apple-macosx",
    "arm64-apple-ios",
    "arm64-apple-ios-simulator",
    "x86_64-apple-ios-simulator",
    "x86_64-unknown-linux-gnu",
    "aarch64-unknown-linux-gnu",
)
PUBLISH_ACTION = "xberg-io/actions/publish-swift-release@v1"
PREPARE_WORKFLOW = "xberg-io/actions/.github/workflows/reusable-swift-release-prepare.yml@v1"
DIST = "dist/swift-artifactbundle"
TAG_MOVE = re.compile(r"git[ \t]+tag\b[^\n]*?[ \t](?:-f|--force|-d|--delete)\b")
REF_FORCE_PUSH = re.compile(r"git\s+push\b[^\n]*(?:\s-f\b|--force\b|--force-with-lease\b|--delete\b|\s\+\S|\s:refs/)")


def workflow_jobs() -> dict:
    """Load the real publish workflow."""
    return yaml.safe_load((ROOT / ".github/workflows/publish.yaml").read_text())["jobs"]


def run_scripts(jobs: dict) -> list[tuple[str, str]]:
    """Every inline shell script in the workflow, keyed by job."""
    return [(name, step["run"]) for name, job in jobs.items() for step in job.get("steps", []) if "run" in step]


class SwiftReleaseFixture(unittest.TestCase):
    """Create isolated Git sources, release assets and an external API boundary."""

    def setUp(self) -> None:
        """Create an isolated source, bare remote and seven-platform bundle."""
        scratch = Path(
            os.environ.get(
                "SWIFT_RELEASE_TEST_ROOT", os.environ.get("RUNNER_TEMP", ROOT / "target/swift-release-tests")
            )
        )
        scratch.mkdir(parents=True, exist_ok=True)
        self.temporary = tempfile.TemporaryDirectory(dir=scratch)
        self.addCleanup(self.temporary.cleanup)
        self.directory = Path(self.temporary.name)
        self.remote = self.directory / "origin.git"
        self.work = self.directory / "work"
        self.work.mkdir()
        self.git("init", "--bare", str(self.remote), cwd=self.directory)
        self.git("init", "-b", "main")
        for key, value in (
            ("user.name", "Release fixture"),
            ("user.email", "release@example.invalid"),
            ("commit.gpgsign", "false"),
            ("tag.gpgsign", "false"),
            ("core.hooksPath", "/dev/null"),
            ("core.autocrlf", "false"),
        ):
            self.git("config", key, value)
        self.git("remote", "add", "origin", str(self.remote))
        (self.work / ".gitignore").write_text("dist/\n")
        (self.work / "Cargo.toml").write_text(f'[workspace.package]\nversion = "{VERSION}"\n')
        (self.work / "Package.swift").write_bytes(self.manifest("__ALEF_SWIFT_CHECKSUM__").encode())
        (self.work / "core.txt").write_text("native source\n")
        self.git("add", "--", ".gitignore", "Cargo.toml", "Package.swift", "core.txt")
        self.git("commit", "-m", "fixture: prepare native inputs")
        self.source = self.git("rev-parse", "HEAD").strip()
        self.dist = self.work / "dist/swift-artifactbundle"
        self.dist.mkdir(parents=True)
        self.bundle = self.dist / ARCHIVE
        self.receipt = self.dist / RECEIPT
        self.write_bundle()
        self.digest = hashlib.sha256(self.bundle.read_bytes()).hexdigest()

    def git(self, *args: str, cwd: Path | None = None) -> str:
        """Run Git against the local fixture repository."""
        return subprocess.check_output(["git", *args], cwd=cwd or self.work, text=True, stderr=subprocess.PIPE)

    def manifest(self, checksum: str) -> str:
        """Name the root binary target and canonical versioned release asset."""
        url = f"https://github.com/xberg-io/html-to-markdown/releases/download/{TAG}/{ARCHIVE}"
        return f'.binaryTarget(name: "RustBridgeBinary", url: "{url}", checksum: "{checksum}")\n'

    def write_bundle(self, *, triples: tuple[str, ...] = TRIPLES) -> None:
        """Create seven native archive variants without compiling an SDK."""
        variants = [{"path": f"{index}/lib.a", "supportedTriples": [triple]} for index, triple in enumerate(triples)]
        info = {
            "schemaVersion": "1.0",
            "artifacts": {"RustBridgeBinary": {"type": "staticLibrary", "version": "1.0.0", "variants": variants}},
        }
        with zipfile.ZipFile(self.bundle, "w") as archive:
            archive.writestr("HtmlToMarkdown-rs.artifactbundle/info.json", json.dumps(info))
            for variant in variants:
                archive.writestr(f"HtmlToMarkdown-rs.artifactbundle/{variant['path']}", b"!<arch>\nfixture\n")

    def refs(self) -> str:
        """Read all remote refs for unchanged-ref assertions."""
        return self.git("show-ref", cwd=self.remote)

    def tag_current(self) -> None:
        """Record the annotated core tag object and its peeled source commit."""
        self.git("tag", "-a", TAG, "-m", TAG)
        self.git("push", "origin", "main", f"refs/tags/{TAG}")
        self.commit = self.git("rev-parse", "HEAD").strip()
        self.tag_object = self.git("rev-parse", f"refs/tags/{TAG}").strip()

    def environment(self) -> dict[str, str]:
        """Serve read-only release lookups and record any other GitHub call as a write."""
        binary = self.directory / "bin"
        binary.mkdir(exist_ok=True)
        gh = binary / "gh"
        gh.write_text(
            f"#!{sys.executable}\n"
            "import os,sys\n"
            "if sys.argv[1:3] == ['release','download']: sys.exit(0)\n"
            "if sys.argv[1:3] == ['release','view']: print(42); sys.exit(0)\n"
            "with open(os.environ['SWIFT_TEST_EVENTS'],'a') as events: events.write(' '.join(sys.argv[1:])+'\\n')\n"
            "sys.exit(1)\n"
        )
        gh.chmod(0o755)
        return {
            **os.environ,
            "PATH": f"{binary}{os.pathsep}{os.environ['PATH']}",
            "SWIFT_TEST_EVENTS": str(self.directory / "events"),
        }

    def write_receipt(self, *, source: str | None = None) -> None:
        """Write the receipt build-swift-artifactbundle emits for the bundle on disk."""
        with zipfile.ZipFile(self.bundle) as archive:
            info = json.loads(archive.read("HtmlToMarkdown-rs.artifactbundle/info.json"))
        artifact = next(iter(info["artifacts"].values()))
        receipt = {
            "schema": 1,
            "repository": "xberg-io/html-to-markdown",
            "source": source or self.source,
            "version": VERSION,
            "archive": ARCHIVE,
            "sha256": hashlib.sha256(self.bundle.read_bytes()).hexdigest(),
            "triples": sorted(triple for variant in artifact["variants"] for triple in variant["supportedTriples"]),
            "artifact_version": artifact["version"],
        }
        self.receipt.write_text(json.dumps(receipt, indent=2) + "\n", encoding="utf-8")

    def release(self) -> None:
        """Commit the pinned checksum on top of source A and tag it, as Prepare Swift release does."""
        self.write_receipt()
        (self.work / "Package.swift").write_bytes(self.manifest(self.digest).encode())
        self.git("add", "--", "Package.swift")
        self.git("commit", "-m", "fixture: pin the Swift checksum")
        self.tag_current()


class SwiftPublishWorkflowTests(unittest.TestCase):
    """The tag is final before publication starts; the Swift job only verifies it and adds a branch."""

    def test_ref_rewrite_patterns_fire_on_the_old_force_moving_job(self) -> None:
        """Negative control: the patterns below match the commands the old job ran."""
        old = (
            'git tag -f "v3.19.2" && git push origin -f refs/tags/v3.19.2\n'
            "git push --force origin HEAD:refs/heads/release/swift/3.19.2\n"
            "git push origin +HEAD:refs/tags/v3.19.2\n"
        )
        assert TAG_MOVE.search(old)
        assert len(REF_FORCE_PUSH.findall(old)) == 3

    def test_publish_workflow_never_moves_rewrites_or_deletes_release_refs(self) -> None:
        """No step re-tags, force-pushes, deletes refs or substitutes the checksum placeholder."""
        scripts = run_scripts(workflow_jobs())
        assert len(scripts) > 50
        for name, script in scripts:
            with self.subTest(job=name):
                assert not TAG_MOVE.search(script), script
                assert not REF_FORCE_PUSH.search(script), script
                assert "__ALEF_SWIFT_CHECKSUM__" not in script
                assert "finalize_release_manifest" not in script

    def test_swift_publication_uses_the_shared_immutable_action(self) -> None:
        """The prepared ZIP and receipt go to publish-swift-release on the tagged source."""
        steps = workflow_jobs()["update-swift-package-manifest"]["steps"]
        uses = [step.get("uses", "") for step in steps]
        publish = steps[uses.index(PUBLISH_ACTION)]
        assert uses.count(PUBLISH_ACTION) == 1
        assert publish["if"] == "${{ needs.prepare.outputs.dry_run != 'true' }}"
        assert publish["with"] == {
            "tag": "${{ needs.prepare.outputs.tag }}",
            "bundle-zip": "",
            "receipt": f"{DIST}/{RECEIPT}",
            "token": "${{ steps.app-token.outputs.token }}",
        }
        checkout = steps[uses.index("actions/checkout@v7")]
        assert checkout["with"]["ref"] == "${{ needs.prepare.outputs.source_sha }}"
        assert checkout["with"]["fetch-depth"] == 0
        assert checkout["with"]["token"] == "${{ steps.app-token.outputs.token }}"
        assert checkout["with"].get("persist-credentials", True) is not False
        download = steps[uses.index("actions/download-artifact@v8")]
        assert download["with"] == {"name": "swift-artifactbundle", "path": DIST}
        assert download["if"] == "${{ needs.prepare.outputs.dry_run != 'true' }}"
        guard = next(index for index, step in enumerate(steps) if step.get("id") == "prepared-tag")
        assert uses.index("actions/checkout@v7") < guard < uses.index(PUBLISH_ACTION)
        assert not any("swift_release.py publish" in step.get("run", "") for step in steps)

    def test_publish_never_builds_the_swift_bundle(self) -> None:
        """The bundle is not reproducible, so only Prepare Swift release builds it."""
        jobs = workflow_jobs()
        assert "swift-artifactbundle" not in jobs
        builders = [
            name
            for name, job in jobs.items()
            for step in job.get("steps", [])
            if step.get("uses") == "xberg-io/actions/build-swift-artifactbundle@v1"
        ]
        assert not builders, builders
        assert not any("swift_release.py" in script for _, script in run_scripts(jobs))

    def test_both_seed_entrypoints_verify_with_the_shared_action_without_writing(self) -> None:
        """Prepare and NuGet recovery verify the seeded tag, ZIP and receipt as a dry run."""
        jobs = workflow_jobs()
        cases = {
            "prepare": ("Verify immutable release seed", DIST, "${{ steps.meta.outputs.tag }}"),
            "recover-nuget-runtimes": (
                "Verify immutable release seed for recovery",
                f"release-source/{DIST}",
                "${{ inputs.tag }}",
            ),
        }
        for name, (step_name, directory, tag) in cases.items():
            with self.subTest(entrypoint=name):
                step = next(step for step in jobs[name]["steps"] if step.get("name") == step_name)
                assert step["uses"] == PUBLISH_ACTION
                assert step["with"]["tag"] == tag
                assert step["with"]["bundle-zip"] == f"{directory}/{ARCHIVE}"
                assert step["with"]["receipt"] == f"{directory}/{RECEIPT}"
                assert step["with"]["dry-run"] == "true"
        checkout = next(
            step
            for step in jobs["recover-nuget-runtimes"]["steps"]
            if step.get("name") == "Check out current recovery tooling"
        )
        assert checkout["with"]["fetch-depth"] == 0


class SwiftPrepareCallerTests(unittest.TestCase):
    """Step 1 of a release calls the shared preparation workflow with this repository's bundle identity."""

    def test_prepare_caller_uses_the_shared_workflow(self) -> None:
        """The caller pins @v1 and passes the bundle identity Package.swift names."""
        workflow = yaml.safe_load((ROOT / ".github/workflows/prepare-swift-release.yaml").read_text())
        assert workflow["name"] == "Prepare Swift release"
        trigger = workflow[True] if True in workflow else workflow["on"]
        assert trigger == {"workflow_dispatch": {"inputs": {"dry-run": {"type": "boolean", "default": False}}}}
        assert workflow["concurrency"] == {"group": "prepare-swift-release", "cancel-in-progress": False}
        assert workflow["permissions"] == {"contents": "read"}
        assert list(workflow["jobs"]) == ["swift"]
        job = workflow["jobs"]["swift"]
        assert job["uses"] == PREPARE_WORKFLOW
        assert job["secrets"] == {
            "app-id": "${{ secrets.BOT_APP_ID }}",
            "app-private-key": "${{ secrets.BOT_APP_PRIVATE_KEY }}",
        }
        assert job["with"] == {
            "dry-run": "${{ inputs.dry-run }}",
            "crate-name": "html-to-markdown-rs-swift",
            "artifact-name": ARCHIVE.removesuffix(".artifactbundle.zip"),
            "binary-target-name": "RustBridgeBinary",
            "header-path": "target/release/build/html-to-markdown-rs-swift-*/out",
            "submodules": "recursive",
            "expected-triples": ",".join(TRIPLES),
        }
        manifest = (ROOT / "Package.swift").read_text()
        assert re.findall(r'\.binaryTarget\(\s*name:\s*"([^"]+)"', manifest) == ["RustBridgeBinary"]
        assert f"/{ARCHIVE}" in manifest

    def test_downstream_jobs_still_wait_for_swift_publication(self) -> None:
        """The needs graph around the Swift job is unchanged."""
        jobs = workflow_jobs()
        assert jobs["update-swift-package-manifest"]["needs"] == ["prepare", "upload-swift-release", "publish-zig"]
        for name in ("release-finalize", "release-report"):
            assert "update-swift-package-manifest" in jobs[name]["needs"], name
        report = next(
            step for step in jobs["release-report"]["steps"] if "RESULT_SWIFT_MANIFEST" in step.get("env", {})
        )
        assert report["env"]["RESULT_SWIFT_MANIFEST"] == "${{ needs.update-swift-package-manifest.result }}"


class SwiftPreparedTagGuardTests(SwiftReleaseFixture):
    """Run the real pre-publication tag guard against a disposable remote."""

    def guard(self, tag_object: str, source: str) -> subprocess.CompletedProcess[str]:
        """Execute the guard step with the identities prepare recorded."""
        steps = workflow_jobs()["update-swift-package-manifest"]["steps"]
        step = next(step for step in steps if step.get("id") == "prepared-tag")
        return subprocess.run(
            ["bash", "-euo", "pipefail", "-c", step["run"]],
            cwd=self.work,
            env={**os.environ, "TAG": TAG, "TAG_OBJECT": tag_object, "SOURCE_SHA": source},
            capture_output=True,
            text=True,
            check=False,
        )

    def test_guard_admits_only_the_tag_prepare_verified(self) -> None:
        """A replaced tag object or a different peeled commit stops publication."""
        self.release()
        before = self.refs()
        assert self.guard(self.tag_object, self.commit).returncode == 0
        for tag_object, source in (("0" * 40, self.commit), (self.tag_object, self.source), ("", "")):
            with self.subTest(tag_object=tag_object, source=source):
                result = self.guard(tag_object, source)
                assert result.returncode != 0, result.stdout
                assert "changed since prepare verified it" in result.stderr
        assert self.refs() == before


class SwiftWorkflowGuardTests(unittest.TestCase):
    """Gate every core publisher and both credential-bearing entry points."""

    def test_core_publishers_require_successful_verified_preparation(self) -> None:
        """Core publishers require successful verified preparation."""
        jobs = yaml.safe_load((ROOT / ".github/workflows/publish.yaml").read_text())["jobs"]
        publishers = {
            name
            for name, job in jobs.items()
            if name not in {"prepare", "recover-nuget-runtimes", "cli-proxy"}
            and (
                name
                in {"upload-swift-release", "update-swift-package-manifest", "upload-cli-release", "trigger-pubdev"}
                or any(
                    re.search(
                        r"/(?:publish-|upload-release-assets|finalize-release|update-release)|gh (?:release (?:edit|upload|create)|api .*(?:POST|PATCH))|git push",
                        step.get("uses", "") + "\n" + step.get("run", ""),
                    )
                    for step in job.get("steps", [])
                )
            )
        }
        # ~keep Lower bounds prove the matcher still finds the publishers without pinning the job count.
        assert len(publishers) >= 29, sorted(publishers)
        assert len(jobs) > len(publishers)
        missing = {name for name in publishers if "needs.prepare.result == 'success'" not in jobs[name].get("if", "")}
        assert not missing, f"publishers bypass successful immutable preparation: {sorted(missing)}"

    def test_options_are_refused_before_credentials_on_both_core_entrypoints(self) -> None:
        """Options are refused before credentials on both core entrypoints."""
        jobs = yaml.safe_load((ROOT / ".github/workflows/publish.yaml").read_text())["jobs"]
        for name in ("prepare", "recover-nuget-runtimes"):
            step = jobs[name]["steps"][0]
            assert step.get("name") == "Reject destructive release options", name
            for flag in ("REPUBLISH", "FORCE_REPUBLISH", "PAYLOAD_REPUBLISH", "PAYLOAD_FORCE_REPUBLISH"):
                for dry in ("true", "false"):
                    with self.subTest(entrypoint=name, flag=flag, dry_run=dry):
                        values = dict.fromkeys(
                            (
                                "REPUBLISH",
                                "FORCE_REPUBLISH",
                                "PAYLOAD_REPUBLISH",
                                "PAYLOAD_FORCE_REPUBLISH",
                                "DRY_RUN",
                                "PAYLOAD_DRY_RUN",
                            ),
                            "false",
                        )
                        values.update({flag: "true", "DRY_RUN": dry})
                        result = subprocess.run(
                            ["bash", "-euo", "pipefail", "-c", step["run"] + "\necho admitted"],
                            env={**os.environ, **values},
                            capture_output=True,
                            text=True,
                            check=False,
                        )
                        assert (result.returncode == 0) is (dry == "true"), result.stderr
                        assert ("admitted" in result.stdout) is (dry == "true")
            assert not any("retag-for-republish" in item.get("uses", "") for item in jobs[name]["steps"])

    def test_false_options_and_payload_dry_run_are_admitted_without_retagging(self) -> None:
        """Literal false is not a destructive request, and payload dry-run remains read-only."""
        jobs = yaml.safe_load((ROOT / ".github/workflows/publish.yaml").read_text())["jobs"]
        for name in ("prepare", "recover-nuget-runtimes"):
            for payload_dry in ("false", "true"):
                with self.subTest(entrypoint=name, payload_dry_run=payload_dry):
                    values = dict.fromkeys(
                        (
                            "REPUBLISH",
                            "FORCE_REPUBLISH",
                            "PAYLOAD_REPUBLISH",
                            "PAYLOAD_FORCE_REPUBLISH",
                            "DRY_RUN",
                            "PAYLOAD_DRY_RUN",
                        ),
                        "false",
                    )
                    if payload_dry == "true":
                        values.update(PAYLOAD_DRY_RUN="true", PAYLOAD_REPUBLISH="true")
                    result = subprocess.run(
                        ["bash", "-euo", "pipefail", "-c", jobs[name]["steps"][0]["run"] + "\necho admitted"],
                        env={**os.environ, **values},
                        capture_output=True,
                        text=True,
                        check=False,
                    )
                    assert result.returncode == 0, result.stderr
                    assert "admitted" in result.stdout

    def test_recovery_verifies_seed_before_packing_or_publishing(self) -> None:
        """Recovery verifies seed before packing or publishing."""
        jobs = yaml.safe_load((ROOT / ".github/workflows/publish.yaml").read_text())["jobs"]
        steps = jobs["recover-nuget-runtimes"]["steps"]
        names = [step.get("name") for step in steps]
        assert names.index("Download immutable release seed for recovery") < names.index(
            "Verify immutable release seed for recovery"
        )
        assert names.index("Verify immutable release seed for recovery") < names.index(
            "Pack only missing runtime package candidates"
        )
        prepare = jobs["prepare"]["steps"]
        names = [step.get("name") for step in prepare]
        assert names.index("Download immutable release seed") < names.index("Verify immutable release seed")
        assert names.index("Verify immutable release seed") < names.index("Ensure GitHub Release exists for tag")


class SwiftSeedEntrypointTests(SwiftReleaseFixture):
    """Execute actual preparation and recovery guards before publisher sentinels."""

    def entrypoint(self, job: str) -> str:
        """Read the actual pre-publication seed verifier from the workflow."""
        jobs = yaml.safe_load((ROOT / ".github/workflows/publish.yaml").read_text())["jobs"]
        names = {"Download immutable release seed", "Download immutable release seed for recovery"}
        return next(step["run"] for step in jobs[job]["steps"] if step.get("name") in names)

    def run_entrypoint(
        self,
        script: str,
        *,
        source_event: str = "release",
        expected_sha: str | None = None,
        workflow_ref: str | None = None,
    ) -> subprocess.CompletedProcess[str]:
        """Expose a publisher sentinel only after the actual verifier succeeds."""
        return subprocess.run(
            ["bash", "-euo", "pipefail", "-c", script + "\necho publisher-sentinel"],
            cwd=self.work,
            env={
                **self.environment(),
                "TAG": TAG,
                "VERSION": VERSION,
                "SOURCE_SHA": self.commit,
                "DRY_RUN": "false",
                "IS_TAG": "true",
                "GITHUB_REPOSITORY": "xberg-io/html-to-markdown",
                "GITHUB_EVENT_NAME": "workflow_dispatch" if workflow_ref else "release",
                "GITHUB_REF": workflow_ref or f"refs/tags/{TAG}",
                "GITHUB_SHA": self.commit,
                "GITHUB_WORKSPACE": str(self.work),
                "EXPECTED_SHA": expected_sha or self.commit,
                "EXPECTED_TAG": TAG,
                "SOURCE_EVENT": source_event,
                "RELEASE_RUN_ID": "1",
                "GITHUB_OUTPUT": str(self.directory / "output"),
            },
            capture_output=True,
            text=True,
            check=False,
        )

    def test_prepare_records_the_seeded_release_identity(self) -> None:
        """Promotion later compares the live release with the identities recorded here."""
        self.release()
        before = self.refs()
        valid = self.run_entrypoint(self.entrypoint("prepare"))
        assert valid.returncode == 0, valid.stderr
        assert "publisher-sentinel" in valid.stdout
        outputs = dict(line.split("=", 1) for line in (self.directory / "output").read_text().splitlines())
        assert outputs["release_id"] == "42"
        assert outputs["tag_object"] == self.tag_object
        assert outputs["swift_bundle_sha256"] == self.digest
        assert outputs["swift_receipt_sha256"] == hashlib.sha256(self.receipt.read_bytes()).hexdigest()
        assert self.refs() == before
        assert not (self.directory / "events").exists()

    def test_manual_publish_run_requires_its_workflow_ref_to_match_the_seeded_tag(self) -> None:
        """A manual run may not build a different tag than Actions records in provenance."""
        self.release()
        script = self.entrypoint("prepare")
        rejected = self.run_entrypoint(script, workflow_ref="refs/tags/v9.9.9")
        assert rejected.returncode != 0, rejected.stdout
        assert "Manual publish must run on the exact version tag" in rejected.stderr
        assert "publisher-sentinel" not in rejected.stdout
        valid = self.run_entrypoint(script, workflow_ref=f"refs/tags/{TAG}")
        assert valid.returncode == 0, valid.stderr
        assert "publisher-sentinel" in valid.stdout

    def test_recovery_predecessor_preserves_ancestry_for_the_valid_seed(self) -> None:
        """Execute source verification and seed verification in their actual workflow order."""
        self.release()
        tooling = self.work / "scripts/ci/csharp/verify_recovery_source.py"
        tooling.parent.mkdir(parents=True)
        shutil.copyfile(ROOT / "scripts/ci/csharp/verify_recovery_source.py", tooling)
        jobs = yaml.safe_load((ROOT / ".github/workflows/publish.yaml").read_text())["jobs"]
        predecessor = next(
            step["run"] for step in jobs["recover-nuget-runtimes"]["steps"] if step.get("id") == "version"
        )
        before = {"refs": self.refs(), "shallow": self.git("rev-parse", "--is-shallow-repository").strip()}
        assert before["shallow"] == "false"
        script = predecessor + "\n" + self.entrypoint("recover-nuget-runtimes")
        result = self.run_entrypoint(script, source_event="workflow_dispatch")
        witness = {
            "before": before,
            "after": {"refs": self.refs(), "shallow": self.git("rev-parse", "--is-shallow-repository").strip()},
            "source_A": self.source,
            "source_B": self.commit,
            "tag_object": self.tag_object,
            "metadata_output": (self.directory / "output").read_text(),
            "exit": result.returncode,
            "stdout": result.stdout,
            "stderr": result.stderr,
        }
        (self.directory.parent / "recovery-sequence-witness.json").write_text(json.dumps(witness, indent=2) + "\n")
        assert f"version={VERSION}" in witness["metadata_output"], "the actual predecessor must run successfully"
        assert result.returncode == 0, result.stderr
        assert "publisher-sentinel" in result.stdout
        assert witness["after"] == before
        assert not (self.directory / "events").exists()
        mismatch = self.run_entrypoint(script, source_event="workflow_dispatch", expected_sha=self.source)
        assert mismatch.returncode != 0
        assert "is not the commit the immutable tag" in mismatch.stderr
        assert "publisher-sentinel" not in mismatch.stdout


class SwiftPromotionTests(SwiftReleaseFixture):
    """Verify live release bytes before flipping the verified release object."""

    def promote(self, *, field: str = "", draft: bool = True) -> subprocess.CompletedProcess[str]:
        """Run the promotion shell with a recording GitHub API boundary."""
        jobs = yaml.safe_load((ROOT / ".github/workflows/publish.yaml").read_text())["jobs"]
        step = next(step for step in jobs["promote-release"]["steps"] if step.get("id") == "flip")
        script = step["run"].replace("${{ needs.prepare.outputs.tag }}", TAG)
        checksum = "a" * 64
        cli = (
            "cli-SHA256SUMS",
            "cli-aarch64-apple-darwin.tar.gz",
            "cli-aarch64-unknown-linux-gnu.tar.gz",
            "cli-x86_64-pc-windows-msvc.zip",
            "cli-x86_64-unknown-linux-gnu.tar.gz",
            "cli-x86_64-unknown-linux-musl.tar.gz",
        )
        ffi = tuple(
            f"html-to-markdown-rs-ffi-{TAG}-{row['target']}.tar.gz"
            for row in jobs["c-ffi-libraries"]["strategy"]["matrix"]["include"]
        )
        tooling = self.work / "scripts/ci/release_inventory.py"
        tooling.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(ROOT / "scripts/ci/release_inventory.py", tooling)
        release = {
            "id": 42,
            "tag_name": TAG,
            "draft": draft,
            "assets": [
                {"name": name, "state": "uploaded", "size": 5, "digest": f"sha256:{checksum}"}
                for name in (ARCHIVE, RECEIPT, *cli, *ffi)
            ],
        }
        if field == "id":
            release["id"] = 43
        elif field == "missing-cli":
            release["assets"] = [asset for asset in release["assets"] if asset["name"] != cli[0]]
        elif field == "missing-c-ffi":
            release["assets"] = [asset for asset in release["assets"] if asset["name"] != ffi[0]]
        elif field and field != "tag":
            next(asset for asset in release["assets"] if asset["name"] == field)["digest"] = f"sha256:{'b' * 64}"
        executable = self.directory / "gh"
        executable.write_text(
            f"#!{sys.executable}\n"
            "import os,sys\n"
            "if sys.argv[1:4] == ['api','--method','PATCH']:\n"
            "    assert sys.argv[4] == 'repos/xberg-io/html-to-markdown/releases/42'\n"
            "    with open(os.environ['PROMOTION_EVENTS'],'a') as output: output.write('promoted\\n')\n"
            "    print('false')\n"
            "elif '/git/ref/tags/' in sys.argv[2]:\n"
            "    print(os.environ['LIVE_TAG_OBJECT'])\n"
            "else:\n"
            "    print(os.environ['LIVE_RELEASE'])\n"
        )
        executable.chmod(0o755)
        environment = {
            **os.environ,
            "PATH": f"{self.directory}{os.pathsep}{os.environ['PATH']}",
            "GITHUB_REPOSITORY": "xberg-io/html-to-markdown",
            "GITHUB_OUTPUT": str(self.directory / "output"),
            "PROMOTION_EVENTS": str(self.directory / "promotion-events"),
            "LIVE_RELEASE": json.dumps(release),
            "LIVE_TAG_OBJECT": "b" * 40 if field == "tag" else "a" * 40,
            "TAG_OBJECT": "a" * 40,
            "RELEASE_ID": "42",
            "SWIFT_BUNDLE_SHA256": checksum,
            "SWIFT_RECEIPT_SHA256": checksum,
            "TAG": TAG,
        }
        return subprocess.run(
            ["bash", "-euo", "pipefail", "-c", script],
            cwd=self.work,
            env=environment,
            capture_output=True,
            text=True,
            check=False,
        )

    def test_promotion_checks_live_bundle_receipt_and_release_identity(self) -> None:
        """A replaced release or changed asset cannot reach the publication operation."""
        events = self.directory / "promotion-events"
        for field in ("id", "tag", ARCHIVE, RECEIPT, "missing-cli", "missing-c-ffi"):
            with self.subTest(field=field):
                rejected = self.promote(field=field)
                assert rejected.returncode != 0, rejected.stdout
                assert not events.exists()
        valid = self.promote()
        assert valid.returncode == 0, valid.stderr
        assert events.read_text() == "promoted\n"
        assert (self.directory / "output").read_text() == "promoted=true\n"

    def test_already_public_complete_release_does_not_patch_the_release(self) -> None:
        """An exact public recovery permits downstream work without rewriting the release."""
        result = self.promote(draft=False)
        assert result.returncode == 0, result.stderr
        assert not (self.directory / "promotion-events").exists()
        assert (self.directory / "output").read_text() == "promoted=true\n"
