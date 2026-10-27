"""Exercise the real governance task entry points with an isolated generator executable."""

import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
import tempfile
import unittest


class AiRulezVersionTests(unittest.TestCase):
    """Run the repository Taskfiles in a disposable project before any real generation."""

    def setUp(self) -> None:
        """Copy the real task graph and provide an executable version boundary."""
        root = Path(__file__).resolve().parents[2]
        self.directory = tempfile.TemporaryDirectory(prefix="ai-rulez-version-")
        self.addCleanup(self.directory.cleanup)
        self.root = Path(self.directory.name)
        shutil.copyfile(root / "Taskfile.yaml", self.root / "Taskfile.yaml")
        shutil.copytree(root / ".task", self.root / ".task")
        (self.root / "Cargo.toml").write_text('[workspace.package]\nversion = "3.18.6"\n')
        self.binary = self.root / "ai-rulez"
        self.binary.write_text(
            f"#!{sys.executable}\n"
            "import os, pathlib, sys\n"
            "if sys.argv[1:] == ['--version']:\n"
            "    print('ai-rulez version ' + os.environ['AI_RULEZ_TEST_VERSION'])\n"
            "else:\n"
            "    pathlib.Path('generator-ran').write_text(' '.join(sys.argv[1:]))\n"
        )
        self.binary.chmod(0o755)
        self.task = shutil.which("task")
        assert self.task is not None

    def run_task(self, name: str, version: str) -> subprocess.CompletedProcess[str]:
        """Resolve root and included tasks against the same supplied executable."""
        return subprocess.run(
            [self.task, name, f"AI_RULEZ={self.binary}"],
            cwd=self.root,
            env=os.environ
            | {"AI_RULEZ_TEST_VERSION": version, "PATH": str(self.root) + os.pathsep + os.environ["PATH"]},
            capture_output=True,
            text=True,
            check=False,
            timeout=15,
        )

    def test_should_reject_old_generator_before_plugin_generation(self) -> None:
        """A generator below the minimum is rejected before it receives a write command."""
        result = self.run_task("plugin:generate", "5.3.0")
        assert result.returncode != 0
        assert not (self.root / "generator-ran").exists(), "Old generator reached generation before rejection"
        assert "requires ai-rulez 5.7.0 or newer, found 5.3.0" in result.stderr, result.stderr

    def test_should_accept_the_minimum_and_newer_generator_versions(self) -> None:
        """The minimum and newer releases, compared numerically, satisfy the guard without generating."""
        for version in ("5.7.0", "5.7.1", "5.10.0", "6.0.0"):
            with self.subTest(version=version):
                result = self.run_task("ai-rulez:version:check", version)
                assert result.returncode == 0, result.stderr
                assert not (self.root / "generator-ran").exists()

    def test_should_reject_versions_below_the_minimum_compared_numerically(self) -> None:
        """Older releases fail the guard with the minimum named in the message."""
        for version in ("5.6.10", "4.99.0"):
            with self.subTest(version=version):
                result = self.run_task("ai-rulez:version:check", version)
                assert result.returncode != 0
                assert f"requires ai-rulez 5.7.0 or newer, found {version}" in result.stderr, result.stderr

    def test_should_guard_every_governance_entry_point_before_writing(self) -> None:
        """Version bumps and hook installation must reject old generators before their first write."""
        tasks = [
            "plugin:verify",
            "governance:generate",
            "governance:check",
            "governance:hooks:install",
            "version:sync",
            "version:bump:major",
            "version:bump:minor",
            "version:bump:patch",
        ]
        assert len(tasks) == 8
        for task in tasks:
            with self.subTest(task=task):
                result = self.run_task(task, "5.3.0")
                assert result.returncode != 0
                assert "requires ai-rulez 5.7.0 or newer, found 5.3.0" in result.stderr, result.stderr
                assert not (self.root / "generator-ran").exists()
                assert not (self.root / "poly.local.toml").exists()

    def test_should_install_latest_in_all_ci_generator_installations(self) -> None:
        """Every explicit CI generator installation takes the latest release."""
        root = Path(__file__).resolve().parents[2]
        versions = []
        for path in (root / ".github/workflows").iterdir():
            versions.extend(re.findall(r'ai-rulez-version: "([^"]+)"', path.read_text()))
        assert len(versions) == 6, versions
        assert set(versions) == {"latest"}, versions


if __name__ == "__main__":
    unittest.main()
