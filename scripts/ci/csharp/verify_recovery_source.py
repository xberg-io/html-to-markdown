"""Verify that runtime recovery uses the exact commit the immutable release tag names."""

import argparse
import subprocess
import sys
from pathlib import Path


def git_output(repository: Path, *arguments: str) -> str:
    """Read Git metadata without changing the source checkout."""
    return subprocess.run(["git", "-C", str(repository), *arguments], check=True, capture_output=True).stdout.decode(
        "utf-8"
    )


def verify_source(repository: Path, original_sha: str, tag: str) -> None:
    """Swift checksums are pinned before tagging, so the tag must name the original run's commit."""
    tag_sha = git_output(repository, "rev-parse", f"{tag}^{{commit}}").strip()
    if tag_sha != original_sha:
        raise ValueError(f"Recovery source {original_sha} is not the commit the immutable tag {tag} names ({tag_sha})")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("original_sha")
    parser.add_argument("tag")
    arguments = parser.parse_args()
    try:
        verify_source(Path.cwd(), arguments.original_sha, arguments.tag)
    except (ValueError, subprocess.CalledProcessError) as error:
        print(f"Recovery refused: {error}", file=sys.stderr)
        sys.exit(1)
