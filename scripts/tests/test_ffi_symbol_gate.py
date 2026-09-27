"""Verify the FFI symbol gate cannot pass while examining nothing."""

import dataclasses
import json
import re
import sys
from pathlib import Path

import check_ffi_symbols
import pytest
from check_ffi_symbols import (
    C_ABI_CONSUMERS,
    ROOT,
    Comparison,
    compare_c_abi,
    compare_php_functions,
    report_comparison,
)

UNMATCHABLE = re.compile(r"(htm_this_pattern_matches_nothing_at_all)")


def _report(comparison: Comparison) -> tuple[int, list[str]]:
    """Run a comparison through the reporter, returning (blocking, silent languages)."""
    blocking, _allowed, _resolved, _orphaned, silent = report_comparison(comparison, {}, False, False)
    return blocking, silent


def test_should_report_a_detector_that_matched_no_call_site_as_silent() -> None:
    comparison = Comparison(
        title="probe",
        exported={"htm_convert"},
        call_sites=4,
        languages={"csharp", "java"},
        sites_by_language={"csharp": 0, "java": 4},
    )

    _blocking, silent = _report(comparison)

    assert silent == ["csharp"]


def test_should_not_report_a_detector_with_call_sites_as_silent() -> None:
    comparison = Comparison(
        title="probe",
        exported={"htm_convert"},
        call_sites=9,
        languages={"csharp", "java"},
        sites_by_language={"csharp": 5, "java": 4},
    )

    _blocking, silent = _report(comparison)

    assert silent == []


def test_should_report_a_registered_detector_as_silent_even_when_nothing_is_exported() -> None:
    """A C ABI with no exports and no calls is total breakage, not a declared gap."""
    comparison = Comparison(title="probe", exported=set(), call_sites=0, languages={"c"}, sites_by_language={"c": 0})

    _blocking, silent = _report(comparison)

    assert silent == ["c"]


def test_should_judge_a_language_with_a_binding_root_on_that_root_alone() -> None:
    """Generated tests outside the binding must not keep a silent binding alive."""
    comparison = Comparison(
        title="probe",
        exported={"htm_convert"},
        call_sites=486,
        languages={"zig"},
        sites_by_language={"zig": 486},
        sites_in_binding_root={"zig": 0},
    )

    _blocking, silent = _report(comparison)

    assert silent == ["zig"]


def test_should_report_every_silent_language_not_just_the_first() -> None:
    comparison = Comparison(
        title="probe",
        exported={"htm_convert"},
        call_sites=1,
        languages={"csharp", "go", "zig"},
        sites_by_language={"csharp": 0, "go": 1, "zig": 0},
    )

    _blocking, silent = _report(comparison)

    assert silent == ["csharp", "zig"]


def test_every_c_abi_detector_finds_call_sites_in_this_repository() -> None:
    """The regression guard: a restyled binding must not silently drop to zero."""
    comparison, _disagreements = compare_c_abi()

    languages = {consumer.language for consumer in C_ABI_CONSUMERS}
    rooted = {consumer.language for consumer in C_ABI_CONSUMERS if consumer.binding_root is not None}
    assert set(comparison.sites_by_language) == languages
    assert set(comparison.sites_in_binding_root) == rooted == {"csharp", "go", "java", "zig"}
    empty_roots = sorted(language for language, count in comparison.sites_in_binding_root.items() if count == 0)
    assert empty_roots == [], f"binding root(s) matched nothing: {empty_roots}"
    assert comparison.sites_by_language["c"] > 0


def _restyle_zig_binding(monkeypatch: pytest.MonkeyPatch) -> None:
    """Rename the Zig binding's @cImport alias from `c.` to `capi.`, leaving e2e/zig untouched."""
    binding_root = ROOT / "packages" / "zig"
    original = check_ffi_symbols.read_text

    def read_text(path: Path) -> str:
        text = original(path)
        if path.is_relative_to(binding_root):
            return re.sub(r"\bc\.htm_", "capi.htm_", text)
        return text

    monkeypatch.setattr("check_ffi_symbols.read_text", read_text)


def test_a_restyled_zig_binding_is_silent_while_generated_tests_still_match(
    monkeypatch: pytest.MonkeyPatch,
) -> None:
    _restyle_zig_binding(monkeypatch)

    comparison, _disagreements = compare_c_abi()
    _blocking, silent = _report(comparison)

    assert comparison.sites_in_binding_root["zig"] == 0
    assert comparison.sites_by_language["zig"] > 0, "the generated e2e/zig tests should still match"
    assert silent == ["zig"]


def _break_detector(monkeypatch: pytest.MonkeyPatch, language: str) -> None:
    monkeypatch.setattr(
        "check_ffi_symbols.C_ABI_CONSUMERS",
        tuple(
            dataclasses.replace(consumer, pattern=UNMATCHABLE) if consumer.language == language else consumer
            for consumer in C_ABI_CONSUMERS
        ),
    )


@pytest.mark.parametrize("language", sorted({consumer.language for consumer in C_ABI_CONSUMERS}))
def test_disabling_one_detector_makes_the_gate_fail(monkeypatch: pytest.MonkeyPatch, language: str) -> None:
    """Each detector must be load-bearing: break its pattern and the gate must fail."""
    _break_detector(monkeypatch, language)

    comparison, _disagreements = compare_c_abi()
    _blocking, silent = _report(comparison)

    assert silent == [language]


def test_php_comparison_claims_no_language_when_it_exports_nothing() -> None:
    """The extension is #[php_class]/#[php_impl] only, so this diff has no surface."""
    comparison = compare_php_functions()

    assert comparison.exported == set()
    assert comparison.languages == set()
    assert comparison.sites_by_language == {}


def test_php_probe_that_matches_nothing_is_silent_once_the_extension_exports_a_function(
    monkeypatch: pytest.MonkeyPatch,
) -> None:
    monkeypatch.setattr("check_ffi_symbols.php_exported_functions", lambda: {"html_to_markdown_convert"})

    comparison = compare_php_functions()
    _blocking, silent = _report(comparison)

    assert comparison.sites_by_language == {"php-ext": 0}
    assert silent == ["php-ext"]


def _run_main(monkeypatch: pytest.MonkeyPatch, capsys: pytest.CaptureFixture[str], *args: str) -> tuple[int, str]:
    monkeypatch.setattr(sys, "argv", ["check_ffi_symbols.py", "--allow-known", *args])
    status = check_ffi_symbols.main()
    return status, capsys.readouterr().out


def test_main_exits_zero_on_this_repository(
    monkeypatch: pytest.MonkeyPatch, capsys: pytest.CaptureFixture[str]
) -> None:
    status, out = _run_main(monkeypatch, capsys)

    assert status == 0
    assert "  per language: c " in out
    assert "  per binding root: csharp " in out
    assert "not diffed: nothing is exported, so no call site could disagree" in out
    assert "0 silent detectors" in out
    assert "SILENT" not in out
    assert out.rstrip().splitlines()[-2] == "OK"


@pytest.mark.parametrize("language", sorted({consumer.language for consumer in C_ABI_CONSUMERS}))
def test_main_exits_nonzero_and_names_the_language_when_one_detector_goes_silent(
    monkeypatch: pytest.MonkeyPatch, capsys: pytest.CaptureFixture[str], language: str
) -> None:
    _break_detector(monkeypatch, language)

    status, out = _run_main(monkeypatch, capsys)

    assert status == 1
    assert f"  SILENT  {language} is registered as diffed but matched no call site" in out
    assert "1 silent detectors" in out
    assert out.rstrip().splitlines()[-2] == "FAIL"


def test_main_exits_nonzero_when_the_zig_binding_is_restyled(
    monkeypatch: pytest.MonkeyPatch, capsys: pytest.CaptureFixture[str]
) -> None:
    _restyle_zig_binding(monkeypatch)

    status, out = _run_main(monkeypatch, capsys)

    assert status == 1
    assert "  SILENT  zig is registered as diffed but matched no call site in its binding root" in out
    assert "1 silent detectors" in out


def test_main_exits_nonzero_when_the_php_probe_goes_silent(
    monkeypatch: pytest.MonkeyPatch, capsys: pytest.CaptureFixture[str]
) -> None:
    monkeypatch.setattr("check_ffi_symbols.php_exported_functions", lambda: {"html_to_markdown_convert"})

    status, out = _run_main(monkeypatch, capsys, "--json")
    summary, _end = json.JSONDecoder().raw_decode(out)

    assert status == 1
    assert summary["silent_detectors"] == ["php-ext"]
    assert "  SILENT  php-ext is registered as diffed but matched no call site" in out
    assert "1 silent detectors" in out


def test_main_json_reports_per_root_counts_and_silent_detectors(
    monkeypatch: pytest.MonkeyPatch, capsys: pytest.CaptureFixture[str]
) -> None:
    _restyle_zig_binding(monkeypatch)

    status, out = _run_main(monkeypatch, capsys, "--json")
    summary, _end = json.JSONDecoder().raw_decode(out)

    assert status == 1
    assert summary["silent_detectors"] == ["zig"]
    assert summary["sites_in_binding_root_c_abi"]["zig"] == 0
    assert summary["sites_in_binding_root_c_abi"]["csharp"] > 0
    assert summary["sites_by_language_c_abi"]["zig"] > 0
