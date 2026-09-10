"""Verify that runtime recovery uses the original release's source tree."""

import argparse
import re
import subprocess
from pathlib import Path


def git_output(repository: Path, *arguments: str) -> str:
    """Read Git metadata without changing the source checkout."""
    return subprocess.run(["git", "-C", str(repository), *arguments], check=True, capture_output=True).stdout.decode(
        "utf-8"
    )


def verify_source(repository: Path, original_sha: str, tag: str, checksum_directory: Path) -> None:
    """Accept unchanged source or exactly the release's verified Swift checksum substitution."""
    tag_sha = git_output(repository, "rev-parse", f"{tag}^{{commit}}").strip()
    if tag_sha == original_sha:
        return
    # Read raw commit headers: the tag checkout may be shallow, hiding parents from rev-list.
    headers = git_output(repository, "cat-file", "-p", tag_sha).partition("\n\n")[0].splitlines()
    parents = [line.removeprefix("parent ") for line in headers if line.startswith("parent ")]
    changed = git_output(repository, "diff", "--name-only", original_sha, tag_sha).splitlines()
    if parents != [original_sha] or changed != ["Package.swift"]:
        raise ValueError("Release tag contains changes beyond one Swift checksum commit")
    original_mode = git_output(repository, "ls-tree", original_sha, "--", "Package.swift").split()[:2]
    current_mode = git_output(repository, "ls-tree", tag_sha, "--", "Package.swift").split()[:2]
    if original_mode != current_mode:
        raise ValueError("Swift manifest file mode or type changed after the release")
    files = list(checksum_directory.glob("*.checksum"))
    if len(files) != 1:
        raise ValueError("Expected exactly one Swift checksum artifact from the original release run")
    checksum = files[0].read_text().strip()
    if not re.fullmatch(r"[0-9a-f]{64}", checksum):
        raise ValueError("Original release artifact does not contain a SHA256 checksum")
    original = git_output(repository, "show", f"{original_sha}:Package.swift")
    current = git_output(repository, "show", f"{tag_sha}:Package.swift")
    placeholder = "__ALEF_SWIFT_CHECKSUM__"
    if original.count(placeholder) != 1 or current != original.replace(placeholder, checksum):
        raise ValueError("Swift manifest differs from the original artifact's exact checksum substitution")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("original_sha")
    parser.add_argument("tag")
    parser.add_argument("checksum_directory", type=Path)
    arguments = parser.parse_args()
    verify_source(Path.cwd(), arguments.original_sha, arguments.tag, arguments.checksum_directory)
