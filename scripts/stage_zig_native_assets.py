"""Stage published C FFI tarballs for the shared multi-platform Zig packager."""

import argparse
import tarfile
from pathlib import Path

TARGETS = {
    "x86_64-unknown-linux-gnu": ("linux-x64", "libhtml_to_markdown_ffi.so"),
    "aarch64-unknown-linux-gnu": ("linux-arm64", "libhtml_to_markdown_ffi.so"),
    "x86_64-apple-darwin": ("osx-x64", "libhtml_to_markdown_ffi.dylib"),
    "aarch64-apple-darwin": ("osx-arm64", "libhtml_to_markdown_ffi.dylib"),
    "x86_64-pc-windows-msvc": ("win-x64", "html_to_markdown_ffi.dll"),
    "aarch64-pc-windows-msvc": ("win-arm64", "html_to_markdown_ffi.dll"),
}


def read_member(package: tarfile.TarFile, suffix: str) -> bytes:
    """Read one regular file by its package-relative suffix without extracting paths."""
    matches = [member for member in package.getmembers() if member.name.endswith(suffix) and member.isfile()]
    if len(matches) != 1:
        raise ValueError(f"Expected exactly one {suffix} in {package.name}")
    source = package.extractfile(matches[0])
    if source is None:
        raise ValueError(f"Could not read {suffix} in {package.name}")
    return source.read()


def stage(artifacts: Path, destination: Path, version: str) -> None:
    """Require every native and matching header before writing staged artifacts."""
    payloads = {}
    header = None
    for target, (rid, filename) in TARGETS.items():
        archive = artifacts / f"html-to-markdown-rs-ffi-v{version}-{target}.tar.gz"
        with tarfile.open(archive) as package:
            payloads[Path(rid) / filename] = read_member(package, f"/lib/{filename}")
            current_header = read_member(package, "/include/html_to_markdown.h")
            if header is not None and header.replace(b"\r\n", b"\n") != current_header.replace(b"\r\n", b"\n"):
                raise ValueError(f"FFI header differs across targets: {archive}")
            if header is None:
                header = current_header
    payloads[Path("include/html_to_markdown.h")] = header
    for relative, payload in payloads.items():
        output = destination / relative
        output.parent.mkdir(parents=True, exist_ok=True)
        output.write_bytes(payload)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("artifacts", type=Path)
    parser.add_argument("destination", type=Path)
    parser.add_argument("version")
    arguments = parser.parse_args()
    stage(arguments.artifacts, arguments.destination, arguments.version)
