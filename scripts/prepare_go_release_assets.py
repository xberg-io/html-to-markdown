"""Add checksum-verified archive names consumed by the generated Go installer."""

import argparse
import hashlib
import shutil
from pathlib import Path


def digest(path: Path) -> str:
    """Hash an archive without loading the entire native payload into memory."""
    with path.open("rb") as source:
        return hashlib.file_digest(source, "sha256").hexdigest()


def prepare(directory: Path, prefix: str, version: str) -> None:
    """Preserve versioned archives and add installer aliases plus sidecars."""
    versioned_prefix = f"{prefix}-go-v{version}-"
    archives = sorted(directory.glob(f"{versioned_prefix}*.tar.gz"))
    if not archives:
        raise ValueError(f"No Go release archives for {prefix} {version} in {directory}")
    for source in archives:
        platform = source.name.removeprefix(versioned_prefix)
        alias = directory / f"{prefix}-go-{platform}"
        checksum = digest(source)
        if alias.exists() and digest(alias) != checksum:
            raise ValueError(f"Existing installer alias differs from release archive: {alias}")
        if not alias.exists():
            shutil.copyfile(source, alias)
        alias.with_name(alias.name + ".sha256").write_text(f"{checksum}  {alias.name}\n")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("directory", type=Path)
    parser.add_argument("prefix")
    parser.add_argument("version")
    arguments = parser.parse_args()
    prepare(arguments.directory, arguments.prefix, arguments.version)
