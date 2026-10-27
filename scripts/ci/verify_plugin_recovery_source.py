"""Prove a plugin packaging commit differs from its immutable tag only in served lock digests."""

import argparse
import copy
import hashlib
import json
import re
import subprocess
from pathlib import Path

import tomllib

LOCK_PATH = ".ai-rulez/ai-rulez.lock"


def git(repository: Path, *arguments: str) -> str:
    """Inspect the committed source without modifying Git state."""
    return subprocess.run(
        ["git", "-C", str(repository), *arguments], check=True, capture_output=True, text=True
    ).stdout.strip()


def without_served_digests(lock: dict) -> dict:
    """Retain every authored pin and served identity for exact comparison."""
    result = copy.deepcopy(lock)
    result.pop("tree")
    for served in result.get("served", []):
        served.pop("digest")
    return result


def verify_source(repository: Path, tag: str, source_sha: str = "") -> dict:
    """Accept the tag itself or a clean commit changing only served lock digests."""
    if not re.fullmatch(r"v[0-9]+\.[0-9]+\.[0-9]+", tag):
        raise ValueError("an explicit stable release tag is required")
    if source_sha and not re.fullmatch(r"[0-9a-f]{40}", source_sha):
        raise ValueError("source_sha must be a full immutable commit SHA")
    tag_sha = git(repository, "rev-parse", f"{tag}^{{commit}}")
    head_sha = git(repository, "rev-parse", "HEAD")
    if head_sha != (source_sha or tag_sha) or git(repository, "status", "--porcelain"):
        raise ValueError("checkout does not match the requested clean source commit")
    changed = git(repository, "diff", "--name-only", tag_sha, head_sha).splitlines()
    if changed not in ([], [LOCK_PATH]):
        raise ValueError("source or runtime files differ from the immutable release tag")
    if (
        git(repository, "ls-tree", tag_sha, "--", LOCK_PATH).split()[:2]
        != git(repository, "ls-tree", head_sha, "--", LOCK_PATH).split()[:2]
    ):
        raise ValueError("lock file mode or type changed")
    original = tomllib.loads(git(repository, "show", f"{tag_sha}:{LOCK_PATH}"))
    current_bytes = (repository / LOCK_PATH).read_bytes()
    current = tomllib.loads(current_bytes.decode())
    if without_served_digests(original) != without_served_digests(current):
        raise ValueError("lock changes extend beyond served digests and their aggregate tree")
    count = sum(before != after for before, after in zip(original["served"], current["served"], strict=True))
    return {
        "tag": tag,
        "tag_commit": tag_sha,
        "packaging_commit": head_sha,
        "source_and_runtime_bytes_identical": True,
        "changed_served_digests": count,
        "lock_sha256": hashlib.sha256(current_bytes).hexdigest(),
    }


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("tag")
    parser.add_argument("source_sha", nargs="?", default="")
    arguments = parser.parse_args()
    try:
        print(json.dumps(verify_source(Path.cwd(), arguments.tag, arguments.source_sha), indent=2))
    except ValueError as error:
        raise SystemExit(f"Plugin recovery refused: {error}") from error
