"""Verify the FFI symbol gate cannot pass while examining nothing."""

import dataclasses
import re

import pytest
from check_ffi_symbols import (
    C_ABI_CONSUMERS,
    Comparison,
    compare_c_abi,
    compare_php_functions,
    report_comparison,
)


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


def test_should_not_report_a_comparison_with_nothing_exported_as_silent() -> None:
    """An empty-vs-empty comparison is a declared gap, not a detector that broke."""
    comparison = Comparison(title="probe", exported=set(), call_sites=0)

    _blocking, silent = _report(comparison)

    assert silent == []


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
    assert set(comparison.sites_by_language) == languages
    silent = sorted(language for language, count in comparison.sites_by_language.items() if count == 0)
    assert silent == [], f"detector(s) matched nothing: {silent}"


@pytest.mark.parametrize("language", sorted({consumer.language for consumer in C_ABI_CONSUMERS}))
def test_disabling_one_detector_makes_the_gate_fail(monkeypatch: pytest.MonkeyPatch, language: str) -> None:
    """Each detector must be load-bearing: break its pattern and the gate must fail."""
    unmatchable = re.compile(r"(htm_this_pattern_matches_nothing_at_all)")
    monkeypatch.setattr(
        "check_ffi_symbols.C_ABI_CONSUMERS",
        tuple(
            dataclasses.replace(consumer, pattern=unmatchable) if consumer.language == language else consumer
            for consumer in C_ABI_CONSUMERS
        ),
    )

    comparison, _disagreements = compare_c_abi()
    _blocking, silent = _report(comparison)

    assert silent == [language]


def test_php_comparison_claims_no_language_when_it_exports_nothing() -> None:
    """The extension is #[php_class]/#[php_impl] only, so this diff has no surface."""
    comparison = compare_php_functions()

    assert comparison.exported == set()
    assert comparison.languages == set()
    assert comparison.sites_by_language == {}
