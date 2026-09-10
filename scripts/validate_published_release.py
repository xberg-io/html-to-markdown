"""Validate the original 3.12.3 release after its reporting-only failure."""

from __future__ import annotations

import json
import sys
from pathlib import Path

REPOSITORY = "xberg-io/html-to-markdown"
RELEASE_RUN = 34390416762
RELEASE_TAG = "v3.12.3"
RELEASE_SHA = "d7b6acf1befc01a22b2d468e8f2cef3a94d66f93"
REQUIRED_JOBS = {
    "Prepare metadata",
    "Validate language manifest versions",
    "Crates.io publish gate",
    "Publish crates.io packages",
    "Publish WASM package",
    "Publish NuGet package",
    "Publish Node packages",
    "Publish Maven (Java) package",
    "Publish Kotlin Android to Maven Central",
    "Publish to PyPI",
    "Publish Ruby gems",
    "Publish Zig package metadata",
    "Publish Hex package",
    "Trigger pub.dev publish workflow",
    "Upload Go FFI archives to GitHub Release",
    "Upload C FFI archives to GitHub Release",
    "Upload Elixir NIF archives to GitHub Release",
    "Upload PHP PIE archives to GitHub Release",
    "Update Swift Package.swift manifest with artifact URL and checksum",
    "Merge Homebrew bottle DSL",
    "Verify release assets",
    "Finalize GitHub Release",
}


def validate(run: dict, pages: list[dict], release: dict) -> None:
    """Reject different source runs, incomplete publication, or nonpublic releases."""
    identity = (run["id"], run["repository"]["full_name"], run["event"], run["path"], run["head_sha"])
    expected = (RELEASE_RUN, REPOSITORY, "release", ".github/workflows/publish.yaml", RELEASE_SHA)
    if identity != expected or run["head_branch"] != RELEASE_TAG or run["status"] != "completed":
        raise ValueError("Expected the completed original 3.12.3 release run")
    jobs = [job for page in pages for job in page["jobs"]]
    failures = [job["name"] for job in jobs if job["conclusion"] not in {"success", "skipped"}]
    if failures != ["Report release outcome"]:
        raise ValueError(f"Unexpected release failures or unfinished jobs: {failures}")
    successful = {job["name"] for job in jobs if job["conclusion"] == "success"}
    if missing := REQUIRED_JOBS - successful:
        raise ValueError(f"Required publication jobs did not succeed: {sorted(missing)}")
    if release["tag_name"] != RELEASE_TAG or release["draft"] or not release["published_at"]:
        raise ValueError("Expected the public 3.12.3 release")


if __name__ == "__main__":
    validate(*(json.loads(Path(path).read_text()) for path in sys.argv[1:]))
    print(f"{RELEASE_TAG}: every required publication succeeded; original failure was reporting only.")
