"""Package original Dart native artifacts for the generated release downloader."""

from __future__ import annotations

import argparse
import gzip
import hashlib
import re
import tarfile
from pathlib import Path

PLATFORMS = {
    "linux-arm64": ("linux-aarch64", "libhtml_to_markdown_rs_dart.so"),
    "windows-arm64": ("windows-aarch64", "html_to_markdown_rs_dart.dll"),
    "linux-x64": ("linux-x86_64", "libhtml_to_markdown_rs_dart.so"),
    "macos-arm64": ("macos-aarch64", "libhtml_to_markdown_rs_dart.dylib"),
    "macos-x64": ("macos-x86_64", "libhtml_to_markdown_rs_dart.dylib"),
    "windows-x64": ("windows-x86_64", "html_to_markdown_rs_dart.dll"),
}


def package_native_assets(artifacts: Path, destination: Path, version: str) -> None:
    """Require the complete published platform set before writing archives or sidecars."""
    if not re.fullmatch(r"[0-9]+\.[0-9]+\.[0-9]+(?:-[0-9A-Za-z.-]+)?", version):
        raise ValueError(f"Invalid release version: {version}")
    natives = []
    for rid, (blob, filename) in PLATFORMS.items():
        native = artifacts / rid / filename
        if not native.is_file():
            raise FileNotFoundError(f"Missing original Dart native artifact: {native}")
        natives.append((blob, native))
    destination.mkdir(parents=True, exist_ok=True)
    for blob, native in natives:
        archive = destination / f"html-to-markdown-rs-dart-v{version}-{blob}.tar.gz"
        write_archive(native, archive)
        digest = hashlib.sha256(archive.read_bytes()).hexdigest()
        Path(f"{archive}.sha256").write_text(f"{digest}  {archive.name}\n")


def write_archive(native: Path, archive: Path) -> None:
    """Write reproducible archives containing only the platform library at the root."""
    with (
        archive.open("wb") as output,
        gzip.GzipFile(filename="", fileobj=output, mode="wb", mtime=0) as compressed,
        tarfile.open(fileobj=compressed, mode="w") as package,
        native.open("rb") as source,
    ):
        metadata = tarfile.TarInfo(native.name)
        metadata.size = native.stat().st_size
        metadata.mode = 0o644
        package.addfile(metadata, source)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("artifacts", type=Path)
    parser.add_argument("destination", type=Path)
    parser.add_argument("version")
    arguments = parser.parse_args()
    package_native_assets(arguments.artifacts, arguments.destination, arguments.version)
