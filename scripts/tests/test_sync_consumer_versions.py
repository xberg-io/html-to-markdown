"""Regression coverage for maintained consumer release pins."""

from pathlib import Path

import pytest
from sync_consumer_versions import pep440, synchronize


@pytest.fixture
def consumer_files(tmp_path: Path) -> dict[str, str]:
    """Create consumer manifests with custom settings that must survive synchronization."""
    (tmp_path / "Cargo.toml").write_text('[workspace.package]\nversion = "3.12.3"\n')
    files = {
        "e2e/go/go.mod": (
            "module github.com/xberg-io/html-to-markdown/packages/go/v3/e2e\n\n"
            "go 1.25.0\n\n"
            "require (\n"
            "\tgithub.com/xberg-io/html-to-markdown/packages/go/v3 v3.12.2\n"
            "\tgithub.com/stretchr/testify v1.12.1\n"
            ")\n\n"
            "replace github.com/xberg-io/html-to-markdown/packages/go/v3 => ../../packages/go\n"
        ),
        "test_apps/dart/pubspec.yaml": "dependencies:\n  h2m: ^3.12.2\n  ffi: ^2.2.0\n",
        "test_apps/elixir/mix.exs": '{:html_to_markdown, "~> 3.12.2"}, {:rustler, "~> 0.38"}\n',
        "test_apps/kotlin_android/build.gradle.kts": (
            'implementation("io.xberg:html-to-markdown-android:3.12.2")\n'
            'val aarCoord = "io.xberg:html-to-markdown-android:3.12.2"\n'
            'tasks.register("buildHostJni") { custom() }\n'
        ),
        "cli-proxy/npm/package.json": '{\n  "name": "x",\n  "version": "3.12.2",\n  "engines": {"node": ">=22"}\n}\n',
        "cli-proxy/pypi/pyproject.toml": '[project]\nname = "x"\nversion = "3.12.2"\nrequires-python = ">=3.11"\n',
        "server.json": (
            '{\n  "name": "io.github.x/y",\n  "version": "3.12.2",\n  "packages": [\n    {\n'
            '      "identifier": "@x/y",\n      "version": "3.12.2"\n    }\n  ]\n}\n'
        ),
    }
    for relative_path, text in files.items():
        path = tmp_path / relative_path
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(text)
    return files


def test_sync_preserves_custom_content_and_is_idempotent(tmp_path: Path, consumer_files: dict[str, str]) -> None:
    """Update every pin while leaving custom consumer configuration untouched."""
    assert synchronize(tmp_path) is False
    for relative_path, text in consumer_files.items():
        assert (tmp_path / relative_path).read_text() == text.replace("3.12.2", "3.12.3")
    assert synchronize(tmp_path) is True
    assert synchronize(tmp_path, check=True) is True


def test_check_detects_drift_without_modifying_manifests(tmp_path: Path, consumer_files: dict[str, str]) -> None:
    """Reject stale versions without rewriting any file in check mode."""
    assert synchronize(tmp_path, check=True) is False
    for relative_path, text in consumer_files.items():
        assert (tmp_path / relative_path).read_text() == text


@pytest.mark.parametrize(
    "relative_path", ["test_apps/elixir/mix.exs", "e2e/go/go.mod", "server.json", "cli-proxy/pypi/pyproject.toml"]
)
def test_missing_pin_fails_before_any_manifest_is_written(
    tmp_path: Path, consumer_files: dict[str, str], relative_path: str
) -> None:
    """Reject renamed dependencies before changing even earlier valid manifests."""
    (tmp_path / relative_path).write_text("renamed_dependency()\n")
    with pytest.raises(ValueError, match="expected 1 release pins, found 0"):
        synchronize(tmp_path)
    assert (tmp_path / "test_apps/dart/pubspec.yaml").read_text() == consumer_files["test_apps/dart/pubspec.yaml"]


def test_duplicate_go_release_pins_fail_before_any_manifest_is_written(
    tmp_path: Path, consumer_files: dict[str, str]
) -> None:
    """Reject ambiguous Go release pins instead of synchronizing multiple dependencies."""
    path = tmp_path / "e2e/go/go.mod"
    path.write_text(
        consumer_files["e2e/go/go.mod"] + "require github.com/xberg-io/html-to-markdown/packages/go/v3 v3.12.2\n"
    )
    with pytest.raises(ValueError, match="expected 1 release pins, found 2"):
        synchronize(tmp_path)
    assert (tmp_path / "test_apps/dart/pubspec.yaml").read_text() == consumer_files["test_apps/dart/pubspec.yaml"]


def test_server_json_pins_both_the_server_and_the_package_version(
    tmp_path: Path, consumer_files: dict[str, str]
) -> None:
    """The registry entry's server version and its npm package version move together."""
    synchronize(tmp_path)
    text = (tmp_path / "server.json").read_text()
    assert text.count('"version": "3.12.3"') == 2
    assert "3.12.2" not in text


def test_deliberate_proxy_drift_fails_the_check(tmp_path: Path, consumer_files: dict[str, str]) -> None:
    """A proxy package left behind the product version is reported, not ignored."""
    synchronize(tmp_path)
    assert synchronize(tmp_path, check=True) is True
    path = tmp_path / "cli-proxy/npm/package.json"
    path.write_text(path.read_text().replace("3.12.3", "3.12.2"))
    assert synchronize(tmp_path, check=True) is False


def test_pypi_proxy_uses_the_pep440_spelling_of_a_prerelease(tmp_path: Path, consumer_files: dict[str, str]) -> None:
    """PyPI rejects 3.13.0-rc.1, so the proxy wheel version is respelled while npm keeps semver."""
    (tmp_path / "Cargo.toml").write_text('[workspace.package]\nversion = "3.13.0-rc.1"\n')
    synchronize(tmp_path)
    assert 'version = "3.13.0rc1"' in (tmp_path / "cli-proxy/pypi/pyproject.toml").read_text()
    assert '"version": "3.13.0-rc.1"' in (tmp_path / "cli-proxy/npm/package.json").read_text()
    assert synchronize(tmp_path, check=True) is True


def test_pep440_leaves_a_final_version_alone() -> None:
    assert pep440("3.13.0") == "3.13.0"
    assert pep440("3.13.0-beta.2") == "3.13.0b2"
