"""Verify original release archives are staged for the shared Zig packager."""

import io
import tarfile
from pathlib import Path

import pytest
from stage_zig_native_assets import TARGETS, stage


@pytest.mark.parametrize("different_header", [False, True])
def test_complete_archives_preserve_native_bytes_and_shared_header(tmp_path: Path, different_header: bool) -> None:
    """Stage all declared targets without changing their native payloads."""
    artifacts = tmp_path / "artifacts"
    artifacts.mkdir()
    for target, (rid, filename) in TARGETS.items():
        archive = artifacts / f"html-to-markdown-rs-ffi-v3.12.3-{target}.tar.gz"
        with tarfile.open(archive, "w:gz") as package:
            for name, payload in [
                (f"pkg/lib/{filename}", rid.encode()),
                (
                    "pkg/include/html_to_markdown.h",
                    (b"different\r\n" if different_header else b"header\r\n") if rid.startswith("win") else b"header\n",
                ),
            ]:
                member = tarfile.TarInfo(name)
                member.size = len(payload)
                package.addfile(member, io.BytesIO(payload))
    if different_header:
        with pytest.raises(ValueError, match="header differs"):
            stage(artifacts, tmp_path / "staged", "3.12.3")
        assert not (tmp_path / "staged").exists()
        return
    stage(artifacts, tmp_path / "staged", "3.12.3")
    for rid, filename in TARGETS.values():
        assert (tmp_path / "staged" / rid / filename).read_bytes() == rid.encode()
    assert (tmp_path / "staged/include/html_to_markdown.h").read_bytes() == b"header\n"


def test_missing_release_archive_fails_before_staging(tmp_path: Path) -> None:
    """Incomplete artifacts cannot produce a partial Zig package."""
    with pytest.raises(FileNotFoundError):
        stage(tmp_path, tmp_path / "staged", "3.12.3")
    assert not (tmp_path / "staged").exists()


def test_workflow_passes_staged_natives_to_shared_packager() -> None:
    """The actual publish job must consume every native built by its matrix."""
    import yaml

    root = Path(__file__).resolve().parents[2]
    jobs = yaml.safe_load((root / ".github/workflows/publish.yaml").read_text())["jobs"]
    assert set(TARGETS) == {entry["target"] for entry in jobs["c-ffi-libraries"]["strategy"]["matrix"]["include"]}
    publish = next(
        step for step in jobs["publish-zig"]["steps"] if step.get("uses") == "xberg-io/actions/publish-zig@v1"
    )
    assert publish["with"]["multi-platform-ffi-dir"] == "zig-ffi-artifacts"
    assert publish["with"]["module-name"] == "html_to_markdown_rs"
    assert publish["with"]["ffi-lib-name"] == "html_to_markdown_ffi"
