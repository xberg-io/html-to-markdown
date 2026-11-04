"""Verify a prepared Swift checksum without changing source or release refs."""

import argparse
import re
import subprocess
import sys
from pathlib import Path

CHECKSUM_PATTERN = re.compile(r'checksum: "([0-9a-f]{64})"')


def finalize(repository: Path, manifest: Path, checksum: str, version: str) -> None:
    """Retain the legacy entry point as a read-only checksum verifier."""
    if re.fullmatch(r"[0-9a-f]{64}", checksum) is None:
        raise ValueError("Expected a single lowercase SHA256 checksum")
    if re.fullmatch(r"\d+\.\d+\.\d+(?:-[\w.-]+)?", version) is None:
        raise ValueError("Expected a release version")
    source = manifest.read_bytes()
    if CHECKSUM_PATTERN.findall(source.decode("utf-8")) != [checksum]:
        raise ValueError("Prepare the exact Swift checksum before tagging; post-tag checksum writes are forbidden")
    relative = manifest.relative_to(repository).as_posix()
    committed = subprocess.run(
        ["git", "-C", str(repository), "show", f"HEAD:{relative}"], capture_output=True, check=True
    ).stdout
    if committed != source:
        raise ValueError("Swift release manifest differs from the committed source")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("checksum")
    parser.add_argument("version")
    parser.add_argument("manifest", type=Path, nargs="?", default=Path("Package.swift"))
    arguments = parser.parse_args()
    try:
        finalize(Path.cwd().resolve(), arguments.manifest.resolve(), arguments.checksum, arguments.version)
    except (ValueError, OSError, subprocess.CalledProcessError) as error:
        print(f"Swift release refused: {error}", file=sys.stderr)
        sys.exit(1)
