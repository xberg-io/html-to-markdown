"""The wheel build hook repackages the released CLI archive for one target."""

from __future__ import annotations

import io
import tarfile
from pathlib import Path

import pytest
from hatch_build import CustomBuildHook

_TARGET = "x86_64-unknown-linux-gnu"


def _hook(root: Path) -> CustomBuildHook:
    return CustomBuildHook(
        str(root),
        config={},
        build_config={},
        metadata=None,
        directory=str(root / "dist"),
        target_name="wheel",
    )


def _stage(root: Path, members: dict[str, bytes]) -> None:
    dist = root / "dist"
    dist.mkdir(parents=True, exist_ok=True)
    with tarfile.open(dist / f"cli-{_TARGET}.tar.gz", "w:gz") as archive:
        for name, payload in members.items():
            info = tarfile.TarInfo(name)
            info.size = len(payload)
            archive.addfile(info, io.BytesIO(payload))


def test_target_wheel_force_includes_the_binary_and_tags_the_platform(tmp_path: Path, monkeypatch) -> None:
    _stage(tmp_path, {f"cli-{_TARGET}/html-to-markdown": b"\x7fELF", f"cli-{_TARGET}/LICENSE": b"MIT"})
    monkeypatch.setenv("HTML_TO_MARKDOWN_CLI_TARGET", _TARGET)
    build_data: dict = {}

    _hook(tmp_path).initialize("3.23.0", build_data)

    assert set(build_data["force_include"].values()) == {f"html_to_markdown_cli/bin/{_TARGET}/html-to-markdown"}
    assert build_data["tag"] == "py3-none-manylinux_2_28_x86_64"
    assert build_data["pure_python"] is False


def test_missing_archive_refuses_to_build_an_empty_platform_wheel(tmp_path: Path, monkeypatch) -> None:
    monkeypatch.setenv("HTML_TO_MARKDOWN_CLI_TARGET", _TARGET)

    with pytest.raises(RuntimeError, match="refusing to build an empty platform wheel"):
        _hook(tmp_path).initialize("3.23.0", {})


def test_without_a_target_the_hook_is_a_noop(tmp_path: Path, monkeypatch) -> None:
    monkeypatch.delenv("HTML_TO_MARKDOWN_CLI_TARGET", raising=False)
    build_data: dict = {}

    _hook(tmp_path).initialize("3.23.0", build_data)

    assert build_data == {}
