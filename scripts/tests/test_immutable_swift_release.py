"""Exercise immutable Swift publication against a disposable real Git remote."""

import hashlib
import importlib.util
import json
import os
import re
import shutil
import struct
import subprocess
import sys
import tempfile
import unittest
import zipfile
from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parents[2]


HELPER = ROOT / "scripts/ci/swift_release.py"
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
        if HELPER.exists():
            destination = self.work / "scripts/ci/swift_release.py"
            destination.parent.mkdir(parents=True)
            shutil.copyfile(HELPER, destination)
        self.git("add", "--", ".gitignore", "Cargo.toml", "Package.swift", "core.txt")
        if HELPER.exists():
            self.git("add", "--", "scripts/ci/swift_release.py")
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

    def committed_manifest(self, revision: str) -> bytes:
        """Read the real committed blob without universal-newline conversion."""
        return subprocess.check_output(["git", "show", f"{revision}:Package.swift"], cwd=self.work)

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

    def helper(self, operation: str, *, version: str = VERSION) -> subprocess.CompletedProcess[str]:
        """Invoke the shipping transaction helper against isolated source and assets."""
        arguments = [
            sys.executable,
            str(HELPER),
            operation,
            "--bundle",
            str(self.bundle),
            "--receipt",
            str(self.receipt),
            "--version",
            version,
            "--source",
            self.source if operation == "check" else self.commit,
        ]
        if operation != "check":
            arguments.extend(["--tag", TAG, "--tag-object", self.tag_object])
        return subprocess.run(
            arguments, cwd=self.work, env=self.environment(), capture_output=True, text=True, check=False
        )

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

    def release(self, *, change_core: bool = False, change_mode: bool = False) -> None:
        """Check the prepared source A and tag its checksum-only child B."""
        self.write_receipt()
        prepared = self.helper("check")
        assert prepared.returncode == 0, prepared.stderr
        manifest = self.work / "Package.swift"
        manifest.write_bytes(self.manifest(self.digest).encode())
        if change_core:
            (self.work / "core.txt").write_text("different native input\n")
        if change_mode:
            manifest.chmod(0o755)
        self.git("add", "--", "Package.swift", "core.txt")
        self.git("commit", "-m", "fixture: finalize manifest")
        self.tag_current()


class SwiftReleaseTransactionTests(SwiftReleaseFixture):
    """Reject inconsistent source, artifacts and refs before publishing the branch."""

    def test_preparation_check_rejects_wrong_source_version_and_dirty_inputs(self) -> None:
        """The check rejects a wrong source version and dirty inputs without rewriting the receipt."""
        self.write_receipt()
        receipt = self.receipt.read_bytes()
        result = self.helper("check", version="2.4.4")
        assert result.returncode != 0
        assert "prepared Cargo version mismatch" in result.stderr
        (self.work / "core.txt").write_text("uncommitted native input\n")
        result = self.helper("check")
        assert result.returncode != 0
        assert "preparation source must be clean" in result.stderr
        assert self.receipt.read_bytes() == receipt
        assert self.git("rev-parse", "HEAD").strip() == self.source

    def test_preparation_check_rejects_a_receipt_for_other_bytes_or_source(self) -> None:
        """The receipt the action wrote must bind this exact ZIP to the checked-out source."""
        self.write_receipt()
        original = json.loads(self.receipt.read_text())
        for field, value in (("sha256", "0" * 64), ("source", "0" * 40), ("triples", sorted(TRIPLES[:-1]))):
            with self.subTest(field=field):
                self.receipt.write_text(json.dumps({**original, field: value}))
                result = self.helper("check")
                assert result.returncode != 0, result.stdout
        self.receipt.unlink()
        result = self.helper("check")
        assert result.returncode != 0
        assert RECEIPT in result.stderr, result.stderr

    def test_verification_is_read_only_on_first_run_and_retry(self) -> None:
        """Verification preserves the tag object and peel and creates no ref."""
        self.release()
        before = self.refs()
        for _ in range(2):
            result = self.helper("verify")
            assert result.returncode == 0, result.stderr
            assert self.refs() == before
        assert not (self.directory / "events").exists()
        assert self.git("ls-remote", "origin", f"refs/heads/release/swift/{VERSION}") == ""

    def write_universal_simulator_archive(self, *, duplicate_cpu: bool = False, overlap: bool = False) -> None:
        """Write the simulator variant in the layout emitted by lipo."""
        with zipfile.ZipFile(self.bundle) as archive:
            members = {name: archive.read(name) for name in archive.namelist()}
        info_name = "HtmlToMarkdown-rs.artifactbundle/info.json"
        info = json.loads(members[info_name])
        variants = info["artifacts"]["RustBridgeBinary"]["variants"]
        variants[3]["supportedTriples"] += variants[4]["supportedTriples"]
        members.pop(f"HtmlToMarkdown-rs.artifactbundle/{variants[4]['path']}")
        del variants[4]
        library = b"!<arch>\nfixture\n"
        header_size = 8 + 2 * 20
        fat = struct.pack(">II", 0xCAFEBABE, 2)
        arm64 = (0x0100000C, 0)
        x86_64 = (0x01000007, 3)
        for index, (cpu, subtype) in enumerate((arm64, arm64 if duplicate_cpu else x86_64)):
            offset = header_size + (0 if overlap else index * len(library))
            fat += struct.pack(">IIIII", cpu, subtype, offset, len(library), 0)
        members[f"HtmlToMarkdown-rs.artifactbundle/{variants[3]['path']}"] = fat + library * 2
        members[info_name] = json.dumps(info).encode()
        with zipfile.ZipFile(self.bundle, "w") as archive:
            for name, data in members.items():
                archive.writestr(name, data)

    def test_preparation_accepts_the_actions_universal_simulator_archive(self) -> None:
        """Preparation accepts the actions universal simulator archive."""
        self.write_universal_simulator_archive()
        self.write_receipt()
        result = self.helper("check")
        assert result.returncode == 0, result.stderr
        receipt = json.loads(self.receipt.read_text())
        assert receipt["triples"] == sorted(TRIPLES)

    def test_preparation_rejects_duplicate_universal_slice_architectures(self) -> None:
        """Distinct slice ranges cannot advertise the same architecture twice."""
        self.write_universal_simulator_archive(duplicate_cpu=True)
        self.write_receipt()
        result = self.helper("check")
        assert result.returncode != 0
        assert "Swift universal archive architecture mismatch" in result.stderr

    def test_preparation_rejects_overlapping_universal_slices(self) -> None:
        """Distinct architecture labels cannot share archive bytes."""
        self.write_universal_simulator_archive(overlap=True)
        self.write_receipt()
        result = self.helper("check")
        assert result.returncode != 0
        assert "invalid native slice bounds" in result.stderr

    def test_invalid_receipt_fields_do_not_move_any_ref(self) -> None:
        """Invalid receipt fields do not move any ref."""
        self.release()
        original = json.loads(self.receipt.read_text())
        before = self.refs()
        cases = {
            "version": "2.4.4",
            "source": "0" * 40,
            "repository": "other/repository",
            "sha256": "0" * 64,
            "triples": list(TRIPLES[:-1]),
            "artifact_version": "2.0.0",
        }
        for field, value in cases.items():
            with self.subTest(field=field):
                self.receipt.write_text(json.dumps({**original, field: value}))
                result = self.helper("verify")
                assert result.returncode != 0
                assert self.refs() == before
                assert not (self.directory / "events").exists()

    def test_missing_or_changed_bundle_does_not_move_any_ref(self) -> None:
        """Missing or changed bundle does not move any ref."""
        self.release()
        before = self.refs()
        original = self.bundle.read_bytes()
        for data in (None, original + b"changed ZIP bytes"):
            with self.subTest(missing=data is None):
                if self.bundle.exists():
                    self.bundle.unlink()
                if data is not None:
                    self.bundle.write_bytes(data)
                result = self.helper("verify")
                assert result.returncode != 0
                assert self.refs() == before
                assert not (self.directory / "events").exists()

    def test_existing_divergent_branch_is_never_fast_forwarded(self) -> None:
        """Existing divergent branch is never fast forwarded."""
        self.release()
        self.git("push", "origin", f"{self.source}:refs/heads/release/swift/{VERSION}")
        before = self.refs()
        result = self.helper("verify")
        assert result.returncode != 0
        assert "Swift release ref already points elsewhere" in result.stderr
        assert self.refs() == before
        assert not (self.directory / "events").exists()

    def test_native_source_changes_after_preparation_prevent_publication(self) -> None:
        """Native source changes after preparation prevent publication."""
        self.release(change_core=True)
        before = self.refs()
        result = self.helper("verify")
        assert result.returncode != 0
        assert "only Package.swift may change after preparation" in result.stderr
        assert self.refs() == before

    def test_manifest_mode_changes_after_preparation_prevent_publication(self) -> None:
        """Manifest mode changes after preparation prevent publication."""
        self.release(change_mode=True)
        before = self.refs()
        result = self.helper("verify")
        assert result.returncode != 0
        assert "Package.swift mode changed" in result.stderr
        assert self.refs() == before

    def test_changed_tag_object_or_commit_is_rejected_without_ref_updates(self) -> None:
        """Changed tag object or commit is rejected without ref updates."""
        self.release()
        before = self.refs()
        tag_object, commit = self.tag_object, self.commit
        cases = (
            ("0" * 40, commit, "core release tag object changed"),
            (tag_object, self.source, "core release tag commit changed"),
        )
        for expected_object, expected_commit, message in cases:
            with self.subTest(message=message):
                self.tag_object, self.commit = expected_object, expected_commit
                result = self.helper("verify")
                assert result.returncode != 0
                assert message in result.stderr
                assert self.refs() == before
                assert not (self.directory / "events").exists()

    def test_missing_receipt_fails_before_creating_the_branch(self) -> None:
        """Missing receipt fails before creating the branch."""
        self.release()
        before = self.refs()
        self.receipt.unlink()
        result = self.helper("verify")
        assert result.returncode != 0
        assert self.refs() == before
        assert not (self.directory / "events").exists()

    def test_manifest_edits_beyond_checksum_fail_before_publishing(self) -> None:
        """Manifest edits beyond checksum fail before publishing."""
        self.write_receipt()
        assert self.helper("check").returncode == 0
        (self.work / "Package.swift").write_text(self.manifest(self.digest) + "// unrelated change\n")
        self.git("add", "--", "Package.swift")
        self.git("commit", "-m", "fixture: change manifest beyond checksum")
        self.tag_current()
        before = self.refs()
        result = self.helper("verify")
        assert result.returncode != 0
        assert "Package.swift changed beyond its checksum" in result.stderr
        assert self.refs() == before
        assert not (self.directory / "events").exists()

    def test_preparation_rejects_missing_platforms_and_accepts_independent_metadata_version(self) -> None:
        """Preparation rejects missing platforms and accepts an independent metadata version."""
        self.write_bundle(triples=TRIPLES[:-1])
        self.write_receipt()
        result = self.helper("check")
        assert result.returncode != 0
        assert "Swift platform census mismatch" in result.stderr
        self.write_bundle()
        self.write_receipt()
        result = self.helper("check")
        assert result.returncode == 0, result.stderr
        receipt = json.loads(self.receipt.read_text())
        assert receipt["artifact_version"] == "1.0.0"
        assert receipt["version"] == VERSION
        assert receipt["triples"] == sorted(TRIPLES)


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
            "bundle-zip": f"{DIST}/{ARCHIVE}",
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
        guard = next(index for index, step in enumerate(steps) if step.get("id") == "prepared-tag")
        assert uses.index("actions/checkout@v7") < guard < uses.index(PUBLISH_ACTION)
        assert not any("swift_release.py publish" in step.get("run", "") for step in steps)

    def test_swift_bundle_job_writes_the_actions_receipt_and_checks_it(self) -> None:
        """The build action writes the receipt; the repo check validates it instead of writing its own."""
        steps = workflow_jobs()["swift-artifactbundle"]["steps"]
        build = next(step for step in steps if step.get("uses") == "xberg-io/actions/build-swift-artifactbundle@v1")
        assert build["with"]["receipt-version"] == "${{ steps.swift-version.outputs.version }}"
        assert build["with"]["output-dir"] == DIST
        scripts = [step.get("run", "") for step in steps]
        assert not any("swift_release.py prepare" in script for script in scripts)
        check = next(index for index, script in enumerate(scripts) if "swift_release.py check" in script)
        upload = next(index for index, step in enumerate(steps) if step.get("uses") == "actions/upload-artifact@v7")
        assert steps.index(build) < check < upload

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


class SwiftByteClosureTests(SwiftReleaseFixture):
    """Preserve the exact source bytes and configured binary identity."""

    def test_crlf_changes_beyond_checksum_are_rejected_before_alias_creation(self) -> None:
        """Crlf changes beyond checksum are rejected before alias creation."""
        original = self.committed_manifest(self.source)
        assert self.git("config", "--get", "core.autocrlf").strip() == "false"
        assert original == self.manifest("__ALEF_SWIFT_CHECKSUM__").encode()
        assert b"\r\n" not in original and original.endswith(b"\n")
        self.write_receipt()
        prepared = self.helper("check")
        assert prepared.returncode == 0, prepared.stderr
        (self.work / "Package.swift").write_bytes(self.manifest(self.digest).encode().replace(b"\n", b"\r\n"))
        self.git("add", "Package.swift")
        self.git("commit", "-m", "fixture: checksum plus forbidden newline mutation")
        self.tag_current()
        changed = self.committed_manifest(self.commit)
        assert changed == original.replace(b"__ALEF_SWIFT_CHECKSUM__", self.digest.encode()).replace(b"\n", b"\r\n")
        assert b"\r\n" in changed and b"\n" not in changed.replace(b"\r\n", b"")
        before = self.refs()
        result = self.helper("verify")
        assert result.returncode != 0, "checksum-only closure must preserve CRLF bytes"
        assert "changed beyond its checksum" in result.stderr, result.stderr
        assert self.refs() == before
        assert not (self.directory / "events").exists()

    def test_existing_crlf_is_preserved_during_exact_checksum_only_preparation(self) -> None:
        """Existing crlf is preserved during exact checksum only preparation."""
        manifest = self.work / "Package.swift"
        manifest.write_bytes(manifest.read_bytes().replace(b"\n", b"\r\n"))
        self.git("add", "Package.swift")
        self.git("commit", "-m", "fixture: original CRLF source")
        self.source = self.git("rev-parse", "HEAD").strip()
        original = self.committed_manifest(self.source)
        assert original == self.manifest("__ALEF_SWIFT_CHECKSUM__").encode().replace(b"\n", b"\r\n")
        assert b"\r\n" in original and b"\n" not in original.replace(b"\r\n", b"")
        self.write_receipt()
        assert self.helper("check").returncode == 0
        manifest.write_bytes(manifest.read_bytes().replace(b"__ALEF_SWIFT_CHECKSUM__", self.digest.encode()))
        self.git("add", "Package.swift")
        self.git("commit", "-m", "fixture: preserve source newline bytes")
        self.tag_current()
        committed = self.committed_manifest(self.commit)
        assert committed == original.replace(b"__ALEF_SWIFT_CHECKSUM__", self.digest.encode())
        assert b"\r\n" in committed and b"\n" not in committed.replace(b"\r\n", b"")
        result = self.helper("verify")
        assert result.returncode == 0, result.stderr
        assert b"\r\n" in manifest.read_bytes()
        assert self.git("rev-parse", f"refs/tags/{TAG}", cwd=self.remote).strip() == self.tag_object

    def test_workflow_artifact_identity_matches_manifest_and_verifier(self) -> None:
        """Workflow artifact identity matches manifest and verifier."""
        jobs = yaml.safe_load((ROOT / ".github/workflows/publish.yaml").read_text())["jobs"]
        steps = [
            step
            for step in jobs["swift-artifactbundle"]["steps"]
            if step.get("uses") == "xberg-io/actions/build-swift-artifactbundle@v1"
        ]
        assert len(steps) == 1
        identity = steps[0]["with"].get("binary-target-name", steps[0]["with"]["artifact-name"])
        with zipfile.ZipFile(self.bundle) as archive:
            members = {name: archive.read(name) for name in archive.namelist()}
        path = "HtmlToMarkdown-rs.artifactbundle/info.json"
        info = json.loads(members[path])
        info["artifacts"] = {identity: next(iter(info["artifacts"].values()))}
        members[path] = json.dumps(info).encode()
        with zipfile.ZipFile(self.bundle, "w") as archive:
            for name, data in members.items():
                archive.writestr(name, data)
        self.write_receipt()
        prepared = self.helper("check")
        assert prepared.returncode == 0, prepared.stderr
        assert re.findall(r'\.binaryTarget\(\s*name:\s*"([^"]+)"', (ROOT / "Package.swift").read_text()) == [identity]


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
        assert len(jobs) == 76
        assert len(publishers) == 29
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
        assert names.index("Verify immutable release seed for recovery") < names.index(
            "Pack only missing runtime package candidates"
        )
        prepare = jobs["prepare"]["steps"]
        names = [step.get("name") for step in prepare]
        assert names.index("Verify immutable release seed") < names.index("Ensure GitHub Release exists for tag")


class SwiftSeedEntrypointTests(SwiftReleaseFixture):
    """Execute actual preparation and recovery guards before publisher sentinels."""

    def entrypoint(self, job: str) -> str:
        """Read the actual pre-publication seed verifier from the workflow."""
        jobs = yaml.safe_load((ROOT / ".github/workflows/publish.yaml").read_text())["jobs"]
        names = {"Verify immutable release seed", "Verify immutable release seed for recovery"}
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

    def test_both_real_entrypoints_admit_valid_seed_and_refuse_missing_receipt(self) -> None:
        """Both real entrypoints admit valid seed and refuse missing receipt."""
        self.release()
        before = self.refs()
        source = self.receipt.read_bytes()
        for name in ("prepare", "recover-nuget-runtimes"):
            with self.subTest(entrypoint=name):
                script = self.entrypoint(name)
                valid = self.run_entrypoint(script)
                assert valid.returncode == 0, valid.stderr
                assert "publisher-sentinel" in valid.stdout
                if name == "prepare":
                    outputs = dict(line.split("=", 1) for line in (self.directory / "output").read_text().splitlines())
                    assert outputs["release_id"] == "42"
                    assert outputs["swift_bundle_sha256"] == self.digest
                    assert outputs["swift_receipt_sha256"] == hashlib.sha256(source).hexdigest()
                self.receipt.unlink()
                rejected = self.run_entrypoint(script)
                assert rejected.returncode != 0
                assert RECEIPT in rejected.stderr, rejected.stderr
                assert "publisher-sentinel" not in rejected.stdout
                self.receipt.write_bytes(source)
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

    def test_both_real_entrypoints_reject_divergent_branch_before_publication(self) -> None:
        """Both real entrypoints reject the divergent branch before publication."""
        self.release()
        self.git("push", "origin", f"{self.source}:refs/heads/release/swift/{VERSION}")
        before = self.refs()
        for name in ("prepare", "recover-nuget-runtimes"):
            with self.subTest(entrypoint=name):
                rejected = self.run_entrypoint(self.entrypoint(name))
                assert rejected.returncode != 0
                assert "already points elsewhere" in rejected.stderr, rejected.stderr
                assert "publisher-sentinel" not in rejected.stdout
                assert self.refs() == before
                assert not (self.directory / "events").exists()

    def test_seed_guard_negative_control_proves_the_publisher_sentinel_is_reachable(self) -> None:
        """Seed guard negative control proves the publisher sentinel is reachable."""
        self.release()
        self.receipt.unlink()
        script = self.entrypoint("prepare")
        weakened = script[: script.index("python3 scripts/ci/swift_release.py verify")]
        result = self.run_entrypoint(weakened)
        assert result.returncode == 0, result.stderr
        assert "publisher-sentinel" in result.stdout

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
