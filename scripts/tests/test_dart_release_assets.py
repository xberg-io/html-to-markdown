"""Verify Dart release archives match the generated downloader contract."""

from __future__ import annotations

import hashlib
import subprocess
import sys
import tarfile
import tempfile
import unittest
from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parents[2]
SCRIPT = ROOT / "scripts/publish/dart/package_native_assets.py"
VERSION = "3.12.3"
PLATFORMS = {
    "linux-arm64": ("linux-aarch64", "libhtml_to_markdown_rs_dart.so"),
    "windows-arm64": ("windows-aarch64", "html_to_markdown_rs_dart.dll"),
    "linux-x64": ("linux-x86_64", "libhtml_to_markdown_rs_dart.so"),
    "macos-arm64": ("macos-aarch64", "libhtml_to_markdown_rs_dart.dylib"),
    "macos-x64": ("macos-x86_64", "libhtml_to_markdown_rs_dart.dylib"),
    "windows-x64": ("windows-x86_64", "html_to_markdown_rs_dart.dll"),
}


class DartReleaseAssetTests(unittest.TestCase):
    """Exercise archive payloads and the release workflow contract."""

    def test_published_archive_set_covers_the_build_matrix(self) -> None:
        """Ensure all built platforms are uploaded with downloader-compatible names."""
        workflow = yaml.safe_load((ROOT / ".github/workflows/publish.yaml").read_text())
        jobs = workflow["jobs"]
        matrix = jobs["dart-natives"]["strategy"]["matrix"]["include"]
        assert set(PLATFORMS) == {entry["rid"] for entry in matrix}
        assembly = jobs["assemble-dart-package"]
        assert assembly["permissions"]["contents"] == "write"
        upload = next(step for step in assembly["steps"] if step.get("name") == "Upload Dart native download assets")
        assert upload["with"]["working-directory"] == "dist/dart"
        assert upload["with"]["assets"].split() == ["*.tar.gz", "*.sha256"]

    def test_archives_include_exact_native_payload_and_checksum(self) -> None:
        """Verify archive contents and sidecars using actual tar files."""
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            for rid, (_, filename) in PLATFORMS.items():
                native = root / "artifacts" / f"{rid}" / filename
                native.parent.mkdir(parents=True)
                native.write_bytes(rid.encode())
            result = subprocess.run(
                [sys.executable, str(SCRIPT), str(root / "artifacts"), str(root / "dist"), VERSION],
                capture_output=True,
                text=True,
                check=False,
            )
            assert result.returncode == 0, result.stderr
            assert len(list((root / "dist").iterdir())) == len(PLATFORMS) * 2
            for rid, (blob, filename) in PLATFORMS.items():
                archive = root / "dist" / f"html-to-markdown-rs-dart-v{VERSION}-{blob}.tar.gz"
                digest = hashlib.sha256(archive.read_bytes()).hexdigest()
                assert Path(f"{archive}.sha256").read_text() == f"{digest}  {archive.name}\n"
                with tarfile.open(archive) as package:
                    assert package.getnames() == [filename]
                    native = package.extractfile(filename)
                    assert native is not None
                    assert native.read() == rid.encode()

    def test_missing_platform_fails_before_creating_any_archive(self) -> None:
        """Reject incomplete artifact sets before writing release assets."""
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            result = subprocess.run(
                [sys.executable, str(SCRIPT), str(root / "artifacts"), str(root / "dist"), VERSION],
                capture_output=True,
                text=True,
                check=False,
            )
            assert result.returncode != 0
            assert not (root / "dist").exists()


if __name__ == "__main__":
    unittest.main()
