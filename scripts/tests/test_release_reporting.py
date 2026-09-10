"""Exercise release status queries without a checked-out repository."""

from __future__ import annotations

import os
import re
import subprocess
import tempfile
import unittest
from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parents[2]


class ReleaseReportingTests(unittest.TestCase):
    """Keep checkout-free reporting explicit about its GitHub repository."""

    def test_python_setup_uses_the_published_action(self) -> None:
        """Use the existing environment action with its supported inputs."""
        workflow = yaml.safe_load((ROOT / ".github/workflows/ci-lint.yaml").read_text())
        setup = workflow["jobs"]["release-scripts"]["steps"][1]
        assert setup["uses"] == "xberg-io/actions/setup-python-env@v1"
        assert setup["with"]["python-version"] == "3.13"

    def test_both_queries_address_the_repository_outside_a_checkout(self) -> None:
        """Reject implicit repository lookup while retaining draft-state results."""
        workflow = yaml.safe_load((ROOT / ".github/workflows/publish.yaml").read_text())
        scripts = "\n".join(step.get("run", "") for step in workflow["jobs"]["release-report"]["steps"])
        queries = re.findall(r"gh release view [^\n]+?(?= 2>/dev/null)", scripts)
        assert len(queries) == 2
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            executable = root / "gh"
            executable.write_text(
                '#!/bin/bash\n[[ "$*" == *"--repo xberg-io/html-to-markdown"* ]] || exit 1\n'
                'printf "%s\\n" "$DRAFT_STATE"\n'
            )
            executable.chmod(0o755)
            for query in queries:
                for state in ("true", "false"):
                    environment = {
                        **os.environ,
                        "PATH": f"{root}:{os.environ['PATH']}",
                        "GITHUB_REPOSITORY": "xberg-io/html-to-markdown",
                        "TAG": "v3.12.3",
                        "DRAFT_STATE": state,
                    }
                    result = subprocess.run(
                        ["bash", "-c", query], cwd=root, env=environment, capture_output=True, text=True, check=False
                    )
                    assert result.returncode == 0, result.stderr
                    assert result.stdout == f"{state}\n"
