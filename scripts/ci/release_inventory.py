"""Check the required CLI and C FFI files on the exact seeded GitHub release."""

import argparse
import json
import re
import sys
from collections import Counter

CLI = (
    "cli-SHA256SUMS",
    "cli-aarch64-apple-darwin.tar.gz",
    "cli-aarch64-unknown-linux-gnu.tar.gz",
    "cli-x86_64-pc-windows-msvc.zip",
    "cli-x86_64-unknown-linux-gnu.tar.gz",
    "cli-x86_64-unknown-linux-musl.tar.gz",
)
C_FFI_TARGETS = (
    "aarch64-apple-darwin",
    "aarch64-pc-windows-msvc",
    "aarch64-unknown-linux-gnu",
    "x86_64-apple-darwin",
    "x86_64-pc-windows-msvc",
    "x86_64-unknown-linux-gnu",
)
SHA256 = re.compile(r"sha256:[0-9a-f]{64}")


def missing_assets(release: dict, release_id: int, tag: str, kind: str) -> list[str]:
    """Return required names that are absent, ambiguous or incompletely uploaded."""
    if release["id"] != release_id or release["tag_name"] != tag:
        raise ValueError("release ID or tag does not match the seeded release")
    expected = (
        CLI if kind == "cli" else tuple(f"html-to-markdown-rs-ffi-{tag}-{target}.tar.gz" for target in C_FFI_TARGETS)
    )
    names = Counter(asset["name"] for asset in release["assets"])
    complete = {
        asset["name"]
        for asset in release["assets"]
        if asset["state"] == "uploaded" and asset["size"] > 0 and SHA256.fullmatch(asset.get("digest") or "")
    }
    return [name for name in expected if names[name] != 1 or name not in complete]


def main() -> None:
    """Report exact completeness for the selected release inventory."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("kind", choices=("cli", "c-ffi"))
    parser.add_argument("--release-id", required=True, type=int)
    parser.add_argument("--tag", required=True)
    args = parser.parse_args()
    missing = missing_assets(json.load(sys.stdin), args.release_id, args.tag, args.kind)
    if missing:
        print(f"Incomplete {args.kind} release inventory: {', '.join(missing)}", file=sys.stderr)
    print("false" if missing else "true")


if __name__ == "__main__":
    main()
