"""Prepare and verify exact Swift release bytes without rewriting release tags."""

from __future__ import annotations

import argparse
import hashlib
import json
import re
import struct
import subprocess
import sys
import tomllib
import zipfile
from pathlib import Path, PurePosixPath

REPOSITORY = "xberg-io/html-to-markdown"
ARTIFACT = "HtmlToMarkdown-rs.artifactbundle"
ARCHIVE = f"{ARTIFACT}.zip"
TRIPLES = {
    "arm64-apple-macosx",
    "x86_64-apple-macosx",
    "arm64-apple-ios",
    "arm64-apple-ios-simulator",
    "x86_64-apple-ios-simulator",
    "x86_64-unknown-linux-gnu",
    "aarch64-unknown-linux-gnu",
}
PLACEHOLDER = "__ALEF_SWIFT_CHECKSUM__"
CHECKSUM = re.compile(r'(checksum\s*:\s*")([^\"]*)(")')


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def git(*arguments: str) -> str:
    """Read Git output without normalizing committed manifest newline bytes."""
    result = subprocess.run(["git", *arguments], capture_output=True, check=False)
    require(result.returncode == 0, f"git {arguments[0]} failed with exit {result.returncode}")
    return result.stdout.decode("utf-8")


def digest(path: Path) -> str:
    with path.open("rb") as stream:
        return hashlib.file_digest(stream, "sha256").hexdigest()


def source_version(source: str) -> str:
    manifest = tomllib.loads(git("show", f"{source}:Cargo.toml"))
    return manifest["workspace"]["package"]["version"]


def manifest_checksum(manifest: str, version: str) -> str:
    matches = CHECKSUM.findall(manifest)
    require(
        len(matches) == 1 and re.fullmatch(r"[0-9a-f]{64}", matches[0][1]) is not None,
        "tagged Package.swift must contain one literal checksum",
    )
    url = f"https://github.com/{REPOSITORY}/releases/download/v{version}/{ARCHIVE}"
    urls = re.findall(r'url\s*:\s*"([^\"]+)"', manifest)
    require(urls.count(url) == 1, "tagged Package.swift must name the final versioned Swift asset URL")
    require(PLACEHOLDER not in manifest, "tagged Package.swift still contains a checksum placeholder")
    return matches[0][1]


def verify_library(archive: zipfile.ZipFile, member: str, triples: list[str]) -> None:
    with archive.open(member) as library:
        header = library.read(8)
        if header == b"!<arch>\n":
            require(len(triples) == 1, "multi-platform Swift library must contain all native slices")
            return
        formats = {
            b"\xca\xfe\xba\xbe": (">", "IIIII"),
            b"\xbe\xba\xfe\xca": ("<", "IIIII"),
            b"\xca\xfe\xba\xbf": (">", "IIQQII"),
            b"\xbf\xba\xfe\xca": ("<", "IIQQII"),
        }
        require(header[:4] in formats, "Swift variant is not a native archive")
        endian, fields = formats[header[:4]]
        count = struct.unpack(f"{endian}I", header[4:])[0]
        require(0 < count == len(triples) <= len(TRIPLES), "Swift universal archive slice count mismatch")
        rows = [struct.unpack(endian + fields, library.read(struct.calcsize(fields))) for _ in range(count)]
        for _, _, offset, size, *_ in rows:
            require(size > 8 and offset + size <= archive.getinfo(member).file_size, "invalid native slice bounds")
            library.seek(offset)
            require(library.read(8) == b"!<arch>\n", "Swift universal slice is not a native archive")


def bundle_metadata(bundle: Path) -> dict[str, object]:
    require(bundle.name == ARCHIVE, f"expected bundle filename {ARCHIVE}")
    with zipfile.ZipFile(bundle) as archive:
        names = archive.namelist()
        require(len(names) == len(set(names)), "Swift bundle contains duplicate entries")
        require(archive.testzip() is None, "Swift bundle CRC mismatch")
        for name in names:
            path = PurePosixPath(name)
            require(
                not path.is_absolute() and ".." not in path.parts and "\\" not in name,
                "Swift bundle contains an unsafe entry",
            )
            require(bool(path.parts) and path.parts[0] == ARTIFACT, "Swift bundle contains an unexpected root")
        info = json.loads(archive.read(f"{ARTIFACT}/info.json"))
        require(info["schemaVersion"] == "1.0", "unsupported Swift artifact schema")
        require(set(info["artifacts"]) == {"RustBridgeBinary"}, "unexpected Swift artifact identity")
        artifact = info["artifacts"]["RustBridgeBinary"]
        require(artifact["type"] == "staticLibrary", "Swift artifact must be a static library")
        observed: list[str] = []
        for variant in artifact["variants"]:
            member = f"{ARTIFACT}/{variant['path']}"
            require(member in names and archive.getinfo(member).file_size > 8, "missing Swift native library")
            verify_library(archive, member, variant["supportedTriples"])
            observed.extend(variant["supportedTriples"])
        require(set(observed) == TRIPLES and len(observed) == len(TRIPLES), "Swift platform census mismatch")
        return {"triples": sorted(observed), "artifact_version": artifact["version"]}


def prepare(arguments: argparse.Namespace) -> None:
    require(git("rev-parse", "HEAD").strip() == arguments.source, "preparation source is not the checked-out commit")
    require(not git("status", "--porcelain", "--untracked-files=normal"), "preparation source must be clean")
    require(source_version(arguments.source) == arguments.version, "prepared Cargo version mismatch")
    receipt = {
        "schema": 1,
        "repository": REPOSITORY,
        "source": arguments.source,
        "version": arguments.version,
        "archive": ARCHIVE,
        "sha256": digest(arguments.bundle),
        **bundle_metadata(arguments.bundle),
    }
    arguments.receipt.write_text(json.dumps(receipt, indent=2) + "\n")
    print(f"Prepared {ARCHIVE}: {receipt['sha256']} ({len(TRIPLES)} platform triples)")


def remote_refs(*references: str) -> dict[str, str]:
    rows = git("ls-remote", "origin", *references).splitlines()
    return {reference: sha for sha, reference in (row.split() for row in rows)}


def verify_tag(arguments: argparse.Namespace) -> None:
    reference = f"refs/tags/{arguments.tag}"
    refs = remote_refs(reference, f"{reference}^{{}}")
    require(refs.get(reference) == arguments.tag_object, "core release tag object changed")
    require(refs.get(f"{reference}^{{}}", refs.get(reference)) == arguments.source, "core release tag commit changed")


def verify_source(source: str, release: str, checksum: str) -> None:
    require(re.fullmatch(r"[0-9a-f]{40}", source) is not None, "invalid prepared source identity")
    git("merge-base", "--is-ancestor", source, release)
    changed = git("diff", "--name-only", source, release).splitlines()
    require(changed == ["Package.swift"], "only Package.swift may change after preparation")
    previous = git("show", f"{source}:Package.swift")
    matches = CHECKSUM.findall(previous)
    require(
        len(matches) == 1
        and (matches[0][1] == PLACEHOLDER or re.fullmatch(r"[0-9a-f]{64}", matches[0][1]) is not None),
        "prepared Package.swift has an unrecognized checksum token",
    )
    expected = CHECKSUM.sub(lambda match: f"{match[1]}{checksum}{match[3]}", previous, count=1)
    require(expected == git("show", f"{release}:Package.swift"), "Package.swift changed beyond its checksum")
    previous_mode = git("ls-tree", source, "--", "Package.swift").split()[0]
    release_mode = git("ls-tree", release, "--", "Package.swift").split()[0]
    require(previous_mode == release_mode, "Package.swift mode changed after preparation")


def legacy_references(version: str) -> tuple[str]:
    return (f"refs/heads/release/swift/{version}",)


def verify_legacy_refs(arguments: argparse.Namespace) -> None:
    references = legacy_references(arguments.version)
    refs = remote_refs(*references)
    for reference in references:
        existing = refs.get(reference)
        require(existing in (None, arguments.source), f"Swift release ref already points elsewhere: {reference}")


def verify(arguments: argparse.Namespace) -> None:
    verify_tag(arguments)
    checksum = manifest_checksum(git("show", f"{arguments.source}:Package.swift"), arguments.version)
    require(digest(arguments.bundle) == checksum, "Swift bundle digest differs from tagged Package.swift")
    receipt = json.loads(arguments.receipt.read_text())
    expected = {
        "schema": 1,
        "repository": REPOSITORY,
        "version": arguments.version,
        "archive": ARCHIVE,
        "sha256": checksum,
        **bundle_metadata(arguments.bundle),
    }
    require(set(receipt) == {*expected, "source"}, "unexpected Swift preparation receipt fields")
    for field, value in expected.items():
        require(receipt[field] == value, f"Swift preparation receipt {field} mismatch")
    source = receipt["source"]
    require(
        isinstance(source, str) and re.fullmatch(r"[0-9a-f]{40}", source) is not None,
        "invalid prepared source identity",
    )
    require(source_version(source) == arguments.version, "prepared Cargo version mismatch")
    require(source_version(arguments.source) == arguments.version, "release Cargo version mismatch")
    verify_source(source, arguments.source, checksum)
    verify_legacy_refs(arguments)
    verify_tag(arguments)
    print(f"Verified immutable {arguments.tag}: {checksum}")


def create_reference(reference: str, source: str) -> None:
    refs = remote_refs(reference)
    existing = refs.get(reference)
    require(existing in (None, source), f"Swift release ref already points elsewhere: {reference}")
    if existing is None:
        result = subprocess.run(
            ["gh", "api", "--method", "POST", f"repos/{REPOSITORY}/git/refs", "--input", "-"],
            input=json.dumps({"ref": reference, "sha": source}),
            text=True,
            capture_output=True,
            check=False,
        )
        refs = remote_refs(reference)
        require(
            result.returncode == 0 or refs.get(reference) == source,
            f"could not create immutable Swift release ref: {reference}",
        )
    refs = remote_refs(reference)
    require(refs.get(reference) == source, f"Swift release ref commit mismatch: {reference}")


def publish(arguments: argparse.Namespace) -> None:
    verify(arguments)
    for reference in legacy_references(arguments.version):
        create_reference(reference, arguments.source)
    verify_legacy_refs(arguments)
    verify_tag(arguments)
    print(f"Verified Swift branch at {arguments.source}; core tag preserved")


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("operation", choices=("prepare", "verify", "publish"))
    parser.add_argument("--bundle", type=Path, required=True)
    parser.add_argument("--receipt", type=Path, required=True)
    parser.add_argument("--version", required=True)
    parser.add_argument("--source", required=True)
    parser.add_argument("--tag")
    parser.add_argument("--tag-object")
    arguments = parser.parse_args()
    require(re.fullmatch(r"[0-9a-f]{40}", arguments.source) is not None, "expected a full source commit SHA")
    if arguments.operation != "prepare":
        require(arguments.tag == f"v{arguments.version}", "release tag/version mismatch")
        require(arguments.tag_object is not None, "missing original release tag object")
    {"prepare": prepare, "verify": verify, "publish": publish}[arguments.operation](arguments)


if __name__ == "__main__":
    try:
        main()
    except (ValueError, OSError, KeyError, zipfile.BadZipFile, struct.error) as error:
        print(f"Swift release refused: {error}", file=sys.stderr)
        sys.exit(1)
