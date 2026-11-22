"""Pinned-release and checksum behaviour of the runtime downloader."""

from __future__ import annotations

import hashlib
import json
from pathlib import Path

import pytest
from html_to_markdown_cli import downloader


@pytest.mark.parametrize(
    ("version", "tag"),
    [
        ("3.23.0", "v3.23.0"),
        ("3.24.0rc1", "v3.24.0-rc.1"),
        ("3.24.0a2", "v3.24.0-alpha.2"),
        ("3.24.0b3", "v3.24.0-beta.3"),
    ],
)
def test_release_tag_is_derived_from_the_package_version(version: str, tag: str) -> None:
    assert downloader.release_tag(version) == tag


def test_release_tag_rejects_a_version_it_cannot_map() -> None:
    with pytest.raises(RuntimeError, match="cannot derive a release tag"):
        downloader.release_tag("3.24.0.dev1")


def _release(monkeypatch: pytest.MonkeyPatch, requested: list[str]) -> None:
    assets = [
        {"name": "cli-x86_64-unknown-linux-gnu.tar.gz", "browser_download_url": "https://example.invalid/a"},
        {"name": "cli-SHA256SUMS", "browser_download_url": "https://example.invalid/sums"},
    ]

    def fake_get(url: str, accept: str | None = None) -> bytes:  # noqa: ARG001
        requested.append(url)
        return b'{"tag_name": "v3.23.0", "assets": %s}' % json.dumps(assets).encode()

    monkeypatch.setattr(downloader, "_http_get", fake_get)
    monkeypatch.setattr(downloader, "_target_triple", lambda: "x86_64-unknown-linux-gnu")


def test_resolve_release_requests_the_pinned_tag_not_latest(monkeypatch: pytest.MonkeyPatch) -> None:
    requested: list[str] = []
    _release(monkeypatch, requested)
    monkeypatch.delenv(downloader.VERSION_ENV, raising=False)
    monkeypatch.setattr(downloader, "release_tag", lambda version=None: "v3.23.0")  # noqa: ARG005

    tag, archive, checksums = downloader._resolve_release()

    assert requested == ["https://api.github.com/repos/xberg-io/html-to-markdown/releases/tags/v3.23.0"]
    assert tag == "v3.23.0"
    assert archive["name"] == "cli-x86_64-unknown-linux-gnu.tar.gz"
    assert checksums is not None
    assert checksums["name"] == "cli-SHA256SUMS"


def test_verify_checksum_accepts_the_matching_digest(tmp_path: Path) -> None:
    archive = tmp_path / "cli-x86_64-unknown-linux-gnu.tar.gz"
    archive.write_bytes(b"archive-bytes")
    digest = hashlib.sha256(b"archive-bytes").hexdigest()
    sums = f"{'0' * 64}  cli-other.tar.gz\n{digest}  {archive.name}\n"

    downloader._verify_checksum(archive, archive.name, sums)


def test_verify_checksum_rejects_a_mismatch(tmp_path: Path) -> None:
    archive = tmp_path / "cli-x86_64-unknown-linux-gnu.tar.gz"
    archive.write_bytes(b"tampered")

    with pytest.raises(RuntimeError, match="checksum mismatch"):
        downloader._verify_checksum(archive, archive.name, f"{'0' * 64}  {archive.name}\n")


def test_verify_checksum_rejects_a_missing_entry(tmp_path: Path) -> None:
    archive = tmp_path / "cli-x86_64-unknown-linux-gnu.tar.gz"
    archive.write_bytes(b"x")

    with pytest.raises(RuntimeError, match="no checksum entry"):
        downloader._verify_checksum(archive, archive.name, f"{'0' * 64}  cli-other.tar.gz\n")


def test_download_without_a_sums_asset_is_refused(tmp_path: Path) -> None:
    archive = tmp_path / "cli-x86_64-unknown-linux-gnu.tar.gz"
    archive.write_bytes(b"x")

    with pytest.raises(RuntimeError, match="refusing to install unverified binary"):
        downloader._verify_download(archive, archive.name, None)
