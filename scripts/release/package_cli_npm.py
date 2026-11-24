#!/usr/bin/env python3
"""Wrap the released CLI archives into per-platform npm packages plus the root manifest.

Reads `cli-<target>.tar.gz|zip` and `cli-SHA256SUMS` from --archives, verifies every archive it
uses against the checksum file, and writes under --out:

    <out>/<os>-<cpu>[-<libc>]/   one package per target that has an archive
    <out>/root/package.json      the root manifest with optionalDependencies pinned to --version

Targets without an archive are skipped and reported; the root launcher falls back to its
download path on those platforms.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import re
import shutil
import sys
import tarfile
import zipfile
from dataclasses import dataclass
from pathlib import Path, PurePosixPath

REPO_ROOT = Path(__file__).resolve().parents[2]
ROOT_MANIFEST = REPO_ROOT / "cli-proxy" / "npm" / "package.json"
ROOT_PACKAGE = "@xberg-io/html-to-markdown-cli"
BIN_NAME = "html-to-markdown"
SUMS_NAME = "cli-SHA256SUMS"
VERSION_RE = re.compile(r"^\d+\.\d+\.\d+(-[0-9A-Za-z.]+)?$")
SUMS_LINE_RE = re.compile(r"([0-9a-fA-F]{64}) [ *]?(\S.*)")
MAX_BINARY_BYTES = 512 * 1024 * 1024


class PackagingError(Exception):
    """A release archive is missing, unsafe or fails verification."""


@dataclass(frozen=True)
class Target:
    """One release target and the npm platform it maps to."""

    triple: str
    os: str
    cpu: str
    libc: str | None = None

    @property
    def suffix(self) -> str:
        """Directory and package-name suffix, e.g. linux-x64-musl."""
        return "-".join(part for part in (self.os, self.cpu, self.libc) if part)

    @property
    def package(self) -> str:
        """Scoped npm package name."""
        return f"{ROOT_PACKAGE}-{self.suffix}"

    @property
    def archive(self) -> str:
        """Release asset file name."""
        return f"cli-{self.triple}.{'zip' if self.os == 'win32' else 'tar.gz'}"

    @property
    def binary(self) -> str:
        """Executable file name inside the archive and the package."""
        return f"{BIN_NAME}.exe" if self.os == "win32" else BIN_NAME


# libc names follow the napi convention: glibc packages end in `-gnu`, musl in `-musl`, so the two
# Linux variants of one cpu have distinct names and the `libc` field makes npm install only one.
TARGETS = (
    Target("x86_64-unknown-linux-gnu", "linux", "x64", "gnu"),
    Target("aarch64-unknown-linux-gnu", "linux", "arm64", "gnu"),
    Target("x86_64-unknown-linux-musl", "linux", "x64", "musl"),
    Target("aarch64-unknown-linux-musl", "linux", "arm64", "musl"),
    Target("x86_64-apple-darwin", "darwin", "x64"),
    Target("aarch64-apple-darwin", "darwin", "arm64"),
    Target("x86_64-pc-windows-msvc", "win32", "x64"),
    Target("aarch64-pc-windows-msvc", "win32", "arm64"),
)
LIBC_FIELD = {"gnu": "glibc", "musl": "musl"}


def parse_checksums(path: Path) -> dict[str, str]:
    if not path.is_file():
        raise PackagingError(f"{SUMS_NAME} not found in {path.parent}")
    sums: dict[str, str] = {}
    for number, line in enumerate(path.read_text(encoding="utf-8").splitlines(), 1):
        if not line.strip():
            continue
        match = SUMS_LINE_RE.fullmatch(line)
        if not match:
            raise PackagingError(f"{SUMS_NAME} line {number} is not in sha256sum format")
        digest = match.group(1).lower()
        if sums.setdefault(match.group(2), digest) != digest:
            raise PackagingError(f"{SUMS_NAME} lists {match.group(2)} twice with different digests")
    return sums


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1 << 20), b""):
            digest.update(chunk)
    return digest.hexdigest()


def _is_member(name: str, wanted: str) -> bool:
    posix = PurePosixPath(name.replace("\\", "/"))
    if posix.is_absolute() or ".." in posix.parts:
        raise PackagingError(f"unsafe archive member {name!r}")
    return posix.name == wanted and len(posix.parts) <= 2


def read_member(archive: Path, wanted: str) -> bytes | None:
    """Return the bytes of the regular file `wanted` at the archive root or one directory down."""
    if archive.name.endswith(".zip"):
        with zipfile.ZipFile(archive) as zf:
            for info in zf.infolist():
                if not info.is_dir() and _is_member(info.filename, wanted):
                    if info.file_size > MAX_BINARY_BYTES:
                        raise PackagingError(f"{archive.name}: {wanted} is implausibly large")
                    return zf.read(info)
        return None
    with tarfile.open(archive, "r:gz") as tf:
        for member in tf:
            if member.isfile() and _is_member(member.name, wanted):
                if member.size > MAX_BINARY_BYTES:
                    raise PackagingError(f"{archive.name}: {wanted} is implausibly large")
                handle = tf.extractfile(member)
                return handle.read() if handle else None
    return None


def write_json(path: Path, data: dict) -> None:
    path.write_text(json.dumps(data, indent=2) + "\n", encoding="utf-8")


def package_one(target: Target, archive: Path, version: str, base: dict, out: Path) -> str:
    binary = read_member(archive, target.binary)
    if not binary:
        raise PackagingError(f"{archive.name} does not contain {target.binary}")
    pkg_dir = out / target.suffix
    bin_dir = pkg_dir / "bin"
    bin_dir.mkdir(parents=True)
    exe = bin_dir / target.binary
    exe.write_bytes(binary)
    exe.chmod(0o644 if target.os == "win32" else 0o755)
    files = ["bin", "README.md"]
    license_bytes = read_member(archive, "LICENSE")
    if license_bytes:
        (pkg_dir / "LICENSE").write_bytes(license_bytes)
        files.append("LICENSE")
    (pkg_dir / "README.md").write_text(
        f"# {target.package}\n\nPrebuilt {BIN_NAME} binary for {target.os} {target.cpu}"
        f"{' (' + target.libc + ')' if target.libc else ''}. "
        f"It is installed automatically by `{ROOT_PACKAGE}`; install that instead.\n",
        encoding="utf-8",
    )
    manifest = {
        "name": target.package,
        "version": version,
        "description": f"Prebuilt {BIN_NAME} binary for {target.os} {target.cpu}",
        "license": base["license"],
        "author": base["author"],
        "repository": base["repository"],
        "os": [target.os],
        "cpu": [target.cpu],
        "files": files,
        "publishConfig": {"access": "public"},
    }
    if target.libc:
        manifest["libc"] = [LIBC_FIELD[target.libc]]
    write_json(pkg_dir / "package.json", manifest)
    return target.package


def package_all(
    archives: Path, version: str, out: Path, root_manifest: Path = ROOT_MANIFEST
) -> tuple[list[str], list[str]]:
    if not VERSION_RE.fullmatch(version):
        raise PackagingError(f"invalid version {version!r}: expected X.Y.Z or X.Y.Z-prerelease")
    sums = parse_checksums(archives / SUMS_NAME)
    base = json.loads(root_manifest.read_text(encoding="utf-8"))
    if out.exists():
        shutil.rmtree(out)
    out.mkdir(parents=True)

    built: list[str] = []
    skipped: list[str] = []
    for target in TARGETS:
        archive = archives / target.archive
        if not archive.is_file():
            skipped.append(target.package)
            continue
        expected = sums.get(target.archive)
        if expected is None:
            raise PackagingError(f"{target.archive} has no entry in {SUMS_NAME}; refusing to package it")
        actual = sha256_file(archive)
        if actual != expected:
            raise PackagingError(f"checksum mismatch for {target.archive} (expected {expected}, got {actual})")
        built.append(package_one(target, archive, version, base, out))
    if not built:
        raise PackagingError(f"no CLI archives found in {archives}; nothing to package")

    root = dict(base, version=version)
    root["optionalDependencies"] = dict.fromkeys(sorted(built), version)
    root_dir = out / "root"
    root_dir.mkdir()
    write_json(root_dir / "package.json", root)
    return built, skipped


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--archives", type=Path, required=True)
    parser.add_argument("--version", required=True)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args(argv)
    try:
        built, skipped = package_all(args.archives, args.version, args.out)
    except (PackagingError, tarfile.TarError, zipfile.BadZipFile) as error:
        print(f"package_cli_npm: {error}", file=sys.stderr)
        return 1
    for name in built:
        print(f"built   {name}@{args.version}")
    for name in skipped:
        print(f"skipped {name} (no release archive)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
