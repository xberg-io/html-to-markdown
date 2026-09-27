#!/usr/bin/env python3
"""Diff the native library's exported C-ABI symbols against the symbols the
bindings actually call.

Every other gate in this repo is *parse-based* (``gofmt -e``, ``cargo check``,
``swiftc -parse``, ``zig build``). A P/Invoke declaration or a ``dlsym`` lookup
naming a symbol that does not exist parses perfectly and only fails at the first
call, at runtime, in the consumer's process. This check is a different
instrument: build the exported set, build the called set, subtract.

Two independent comparisons are set up:

1. ``htm_*`` C-ABI exports (``crates/html-to-markdown-ffi``) vs. the symbols the
   vendored-header bindings name as strings or ``extern`` declarations.
2. PHP extension global functions (``#[php_function]`` in
   ``crates/html-to-markdown-php``) vs. the global functions the PHP extension
   smoke apps probe for. This one is diffed only once the extension exports a
   ``#[php_function]``; today it exports none and is reported as not diffed.

A detector that matches no call site fails the run. Each detector also names
the roots it must match in (the binding package, or for C the e2e and test-app
consumers), and a root with no call site fails the run too, so generated tests
and vendored header copies cannot stand in for a consumer that went silent.

Usage:
    python3 scripts/check_ffi_symbols.py               # strict: any gap fails
    python3 scripts/check_ffi_symbols.py --allow-known # known gaps warn, unknown fail
    python3 scripts/check_ffi_symbols.py --json        # machine-readable summary
"""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
from collections import defaultdict
from dataclasses import dataclass, field
from functools import cache
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent

FFI_HEADER = ROOT / "crates" / "html-to-markdown-ffi" / "include" / "html_to_markdown.h"
FFI_SRC = ROOT / "crates" / "html-to-markdown-ffi" / "src"
PHP_SRC = ROOT / "crates" / "html-to-markdown-php" / "src"

# Third-party sources that are checked in but are not ours to diff. Build output
# and fetch caches need no denylist: the file set comes from `git ls-files`, so
# anything gitignored (test_apps/zig/zig-pkg, target/, node_modules/) is already
# out. Dropping that rule reintroduces stale released copies of our own bindings
# as phantom call sites. ~keep
EXCLUDED_DIR_NAMES = frozenset({"vendor", "third_party"})

# The canonical header is the *source* of exports, not a consumer of them.
# Vendored copies elsewhere in the tree are deliberately still scanned: a stale
# copy that declares a removed symbol is exactly the drift this tool exists to
# catch. ~keep
EXPORT_SOURCE_PATHS = frozenset({FFI_HEADER})

# Symbols the bindings call that the native library does not export, which we
# have accepted as temporarily-broken rather than blocking on. Every entry must
# cite why it is here and what removes it. This map MUST shrink to empty; a
# stale entry (allowlisted symbol that is no longer missing) is itself a
# failure, so entries cannot outlive their fix. ~keep
KNOWN_MISSING_EXPORTS: dict[str, str] = {}

KNOWN_MISSING_PHP_FUNCTIONS: dict[str, str] = {}

BLOCK_COMMENT_RE = re.compile(r"/\*.*?\*/", re.DOTALL)
LINE_COMMENT_RE = re.compile(r"//[^\n]*")
PHP_LINE_COMMENT_RE = re.compile(r"(?://|#)[^\n]*")

# How many call sites to print per missing symbol before summarising the rest.
# One broken symbol can have dozens of sites; the cap keeps the report readable
# without hiding the total. ``--verbose`` prints all of them. ~keep
MAX_SITES_PER_SYMBOL = 6


@dataclass(frozen=True)
class Consumer:
    """One language's way of naming a native symbol."""

    language: str
    suffixes: tuple[str, ...]
    pattern: re.Pattern[str]
    how: str
    # ~keep The consumers this detector guards, as path prefixes relative to ROOT. Each
    # ~keep root must match at least one call site on its own: alef-generated e2e tests
    # ~keep match the same pattern (most of Zig's sites) and the vendored Go copy of the
    # ~keep C header matches the C pattern, so a per-language count stays nonzero when the
    # ~keep consumer itself stops matching. Files outside the roots are still diffed.
    required_roots: tuple[str, ...] = ()
    strip_comments: bool = True


@dataclass(frozen=True)
class NotChecked:
    """A binding this tool deliberately does not diff, and why."""

    language: str
    status: str
    reason: str


@dataclass
class Finding:
    """One called-but-not-exported symbol at one call site."""

    symbol: str
    language: str
    path: str
    line: int


@dataclass
class Comparison:
    """One exported-vs-called diff."""

    title: str
    exported: set[str] = field(default_factory=set)
    call_sites: int = 0
    languages: set[str] = field(default_factory=set)
    findings: list[Finding] = field(default_factory=list)
    sites_by_language: dict[str, int] = field(default_factory=dict)
    sites_by_root: dict[str, dict[str, int]] = field(default_factory=dict)


C_ABI_CONSUMERS: tuple[Consumer, ...] = (
    Consumer(
        language="csharp",
        suffixes=(".cs",),
        pattern=re.compile(r'EntryPoint\s*=\s*"(htm_[A-Za-z0-9_]+)"'),
        how='[DllImport(..., EntryPoint = "htm_*")]',
        required_roots=("packages/csharp/",),
    ),
    Consumer(
        language="java",
        suffixes=(".java",),
        pattern=re.compile(r'\.find\(\s*"(htm_[A-Za-z0-9_]+)"\s*\)'),
        how='Panama SymbolLookup.find("htm_*")',
        required_roots=("packages/java/",),
    ),
    Consumer(
        language="go",
        suffixes=(".go",),
        pattern=re.compile(r"\bC\.(htm_[A-Za-z0-9_]+)"),
        how="cgo C.htm_*",
        required_roots=("packages/go/",),
    ),
    Consumer(
        language="zig",
        suffixes=(".zig",),
        pattern=re.compile(r"\bc\.(htm_[A-Za-z0-9_]+)"),
        how="@cImport c.htm_*",
        required_roots=("packages/zig/",),
    ),
    Consumer(
        language="c",
        suffixes=(".c", ".h"),
        pattern=re.compile(r"\b(htm_[A-Za-z0-9_]+)\s*\("),
        how="direct htm_*() call or declaration",
        # ~keep C has no binding package; its consumers are the e2e suite and the test
        # ~keep app. The vendored header in packages/go/include is scanned for drift but
        # ~keep holds declarations, not calls, so it is not a root.
        required_roots=("e2e/c/", "test_apps/c/"),
    ),
)

PHP_FUNCTION_CALL_RE = re.compile(
    r"""(?:function_exists\(\s*['"](html_to_markdown_[A-Za-z0-9_]+)['"]|"""
    r"""\b(html_to_markdown_[A-Za-z0-9_]+)\s*\()"""
)

# Where PHP global-function probes live. Scoped deliberately: a repo-wide sweep
# of *.php would match Composer vendor trees and the class-based binding's own
# method names. ~keep
PHP_PROBE_ROOTS = ("test_apps/php_ext/",)

NOT_CHECKED: tuple[NotChecked, ...] = (
    NotChecked(
        "swift",
        "not applicable",
        "swift-bridge generates the Swift shims and the Rust exports from the same "
        "#[swift_bridge::bridge] macro in the same build; the two cannot disagree",
    ),
    NotChecked(
        "node",
        "not applicable",
        "#[napi] generates the JS binding and the native registration from one macro "
        "expansion; exports are registered at module init, not looked up by name",
    ),
    NotChecked(
        "dart",
        "not applicable",
        "flutter_rust_bridge codegen emits the Dart side and the Rust side together from the same source of truth",
    ),
    NotChecked(
        "kotlin-android",
        "not applicable",
        "JNI symbols are derived from the Java class/method name by the jni crate's "
        "#[no_mangle] naming rule; a mismatch is a link/registration error, not a "
        "string lookup this tool could diff",
    ),
    NotChecked(
        "python",
        "unchecked",
        "PyO3 dynamic dispatch - attribute access on the extension module cannot be "
        "resolved statically without false positives",
    ),
    NotChecked(
        "ruby",
        "unchecked",
        "Magnus dynamic dispatch - method names are resolved at call time by the VM",
    ),
    NotChecked(
        "elixir",
        "unchecked",
        "Rustler NIF arity/name mismatches surface at module load, not via a symbol table this tool can read",
    ),
    NotChecked(
        "r",
        "unchecked",
        "extendr dynamic registration - .Call targets are registered at package load",
    ),
    NotChecked(
        "wasm",
        "unchecked",
        "wasm-bindgen emits the JS glue and the wasm exports from one macro expansion",
    ),
)


@cache
def tracked_files() -> tuple[Path, ...]:
    """Every git-tracked file, as absolute paths.

    Using git's index rather than a filesystem walk is what keeps fetch caches
    and build output (which contain stale *copies of our own bindings*) from
    being reported as call sites.
    """
    result = subprocess.run(
        ["git", "-C", str(ROOT), "ls-files", "-z"],
        capture_output=True,
        check=True,
        text=True,
    )
    return tuple(ROOT / name for name in result.stdout.split("\0") if name)


def iter_source_files(suffixes: tuple[str, ...]) -> list[Path]:
    """All first-party tracked files with any of ``suffixes``."""
    found: list[Path] = []
    for path in tracked_files():
        if path.suffix not in suffixes or path in EXPORT_SOURCE_PATHS:
            continue
        if EXCLUDED_DIR_NAMES.isdisjoint(path.relative_to(ROOT).parts[:-1]):
            found.append(path)
    return sorted(found)


def _blank(match: re.Match[str]) -> str:
    """Replace a match with spaces, preserving newlines so line numbers hold."""
    return re.sub(r"[^\n]", " ", match.group(0))


def strip_comments(text: str) -> str:
    """Blank out C-family comments, preserving line numbering.

    A doc comment that mentions ``htm_foo(ptr)`` is not a call site; without this
    the C scan reports every symbol named in cbindgen's own doc blocks.
    """
    return LINE_COMMENT_RE.sub(_blank, BLOCK_COMMENT_RE.sub(_blank, text))


def strip_php_comments(text: str) -> str:
    """Blank out PHP comments, including ``#`` line comments and docblocks."""
    return PHP_LINE_COMMENT_RE.sub(_blank, BLOCK_COMMENT_RE.sub(_blank, text))


def read_text(path: Path) -> str:
    return path.read_text(encoding="utf-8", errors="replace")


def exported_from_header() -> set[str]:
    """``htm_*`` declarations in the cbindgen-generated public header."""
    text = strip_comments(read_text(FFI_HEADER))
    return set(re.findall(r"\b(htm_[A-Za-z0-9_]+)\s*\(", text))


def exported_from_rust() -> set[str]:
    """``htm_*`` functions actually given C linkage in the FFI crate."""
    symbols: set[str] = set()
    for path in sorted(FFI_SRC.rglob("*.rs")):
        symbols.update(re.findall(r'extern\s+"C"\s+fn\s+(htm_[A-Za-z0-9_]+)', read_text(path)))
    return symbols


def php_exported_functions() -> set[str]:
    """Global functions the PHP extension registers via ``#[php_function]``."""
    names: set[str] = set()
    if not PHP_SRC.is_dir():
        return names
    for path in sorted(PHP_SRC.rglob("*.rs")):
        text = read_text(path)
        for match in re.finditer(r"#\[php_function[^\]]*\]\s*(?:pub\s+)?fn\s+([A-Za-z0-9_]+)", text):
            names.add(match.group(1))
        names.update(re.findall(r'#\[php_function\s*\(\s*name\s*=\s*"([^"]+)"', text))
    return names


def scan_consumer(consumer: Consumer, exported: set[str]) -> tuple[int, dict[str, int], list[Finding]]:
    """Return (call sites seen, sites per required root, findings for symbols not in ``exported``)."""
    call_sites = 0
    sites_by_root = dict.fromkeys(consumer.required_roots, 0)
    findings: list[Finding] = []
    for path in iter_source_files(consumer.suffixes):
        text = read_text(path)
        if consumer.pattern.search(text) is None:
            continue
        if consumer.strip_comments:
            text = strip_comments(text)
        relative = path.relative_to(ROOT).as_posix()
        root = next((root for root in consumer.required_roots if relative.startswith(root)), None)
        for line_number, line in enumerate(text.splitlines(), start=1):
            for match in consumer.pattern.finditer(line):
                symbol = next(group for group in match.groups() if group)
                call_sites += 1
                if root is not None:
                    sites_by_root[root] += 1
                if symbol not in exported:
                    findings.append(Finding(symbol, consumer.language, str(path.relative_to(ROOT)), line_number))
    return call_sites, sites_by_root, findings


def compare_c_abi() -> tuple[Comparison, list[str]]:
    """Diff C-ABI exports against every statically analysable consumer."""
    header = exported_from_header()
    rust = exported_from_rust()
    disagreements: list[str] = [
        f'declared in html_to_markdown.h but no `extern "C" fn` in FFI src: {symbol}'
        for symbol in sorted(header - rust)
    ]
    disagreements.extend(
        f'`extern "C" fn` in FFI src but missing from html_to_markdown.h: {symbol}' for symbol in sorted(rust - header)
    )

    # Union, not intersection: a symbol present in only one source is already
    # reported above, and treating it as unexported here would double-report it
    # against every call site. ~keep
    comparison = Comparison(title="C ABI (htm_*)", exported=header | rust)
    for consumer in C_ABI_CONSUMERS:
        call_sites, sites_by_root, findings = scan_consumer(consumer, comparison.exported)
        comparison.call_sites += call_sites
        comparison.findings.extend(findings)
        comparison.languages.add(consumer.language)
        comparison.sites_by_language[consumer.language] = call_sites
        if consumer.required_roots:
            comparison.sites_by_root[consumer.language] = sites_by_root
    return comparison, disagreements


def compare_php_functions() -> Comparison:
    """Diff PHP extension global functions against the ext smoke apps' probes."""
    comparison = Comparison(title="PHP extension global functions", exported=php_exported_functions())
    # ~keep Registered as a diffed language only when something is exported to diff.
    # ~keep The extension is #[php_class]/#[php_impl] only and registers no
    # ~keep #[php_function], so this comparison is empty-vs-empty and must not be
    # ~keep counted as coverage it does not provide. Once something is exported, a
    # ~keep missing probe root counts as a root with no call site, not as a skipped diff.
    sites_by_root = dict.fromkeys(PHP_PROBE_ROOTS, 0)
    for root_name in PHP_PROBE_ROOTS:
        root = ROOT / root_name
        if not root.is_dir():
            continue
        for path in sorted(root.rglob("*.php")):
            for line_number, line in enumerate(strip_php_comments(read_text(path)).splitlines(), start=1):
                for match in PHP_FUNCTION_CALL_RE.finditer(line):
                    symbol = next(group for group in match.groups() if group)
                    sites_by_root[root_name] += 1
                    if symbol not in comparison.exported:
                        comparison.findings.append(Finding(symbol, "php-ext", str(path.relative_to(ROOT)), line_number))
    comparison.call_sites = sum(sites_by_root.values())
    if comparison.exported:
        comparison.languages.add("php-ext")
        comparison.sites_by_language["php-ext"] = comparison.call_sites
        comparison.sites_by_root["php-ext"] = sites_by_root
    return comparison


def group_by_symbol(findings: list[Finding]) -> dict[str, list[Finding]]:
    grouped: dict[str, list[Finding]] = defaultdict(list)
    for finding in findings:
        grouped[finding.symbol].append(finding)
    return dict(sorted(grouped.items()))


def empty_roots(comparison: Comparison, language: str) -> list[str]:
    """The required roots of ``language`` in which its detector matched no call site."""
    return sorted(root for root, count in comparison.sites_by_root.get(language, {}).items() if count == 0)


def silent_languages(comparison: Comparison) -> list[str]:
    """Languages registered as diffed with no call site at all, or none in one of their required roots."""
    return sorted(
        language
        for language, count in comparison.sites_by_language.items()
        if count == 0 or empty_roots(comparison, language)
    )


def report_comparison(
    comparison: Comparison,
    allowlist: dict[str, str],
    allow_known: bool,
    verbose: bool,
) -> tuple[int, int, list[str], list[str], list[str]]:
    """Print one comparison. Returns (blocking, allowed, resolved, orphaned, silent)."""
    print(f"\n{comparison.title}")
    print(
        f"  {len(comparison.exported)} symbols exported, "
        f"{comparison.call_sites} call sites checked across "
        f"{len(comparison.languages)} languages ({', '.join(sorted(comparison.languages)) or 'none'})"
    )
    if comparison.sites_by_language:
        breakdown = ", ".join(f"{lang} {count}" for lang, count in sorted(comparison.sites_by_language.items()))
        print(f"  per language: {breakdown}")
    if comparison.sites_by_root:
        breakdown = ", ".join(
            f"{lang} {root} {count}"
            for lang, roots in sorted(comparison.sites_by_root.items())
            for root, count in sorted(roots.items())
        )
        print(f"  per required root: {breakdown}")

    # ~keep A detector whose pattern stops matching reports zero call sites and finds
    # ~keep nothing, which renders identically to a clean pass. Treated as a failure so
    # ~keep the gate cannot succeed having examined nothing. A comparison with nothing
    # ~keep exported registers no language (see the PHP comparison), so it is reported
    # ~keep as not diffed; one that still registers a language with zero sites is silent.
    if not comparison.languages and comparison.call_sites == 0:
        print("  not diffed: nothing is exported, so no call site could disagree")
    silent = silent_languages(comparison)

    grouped = group_by_symbol(comparison.findings)
    blocking = 0
    allowed = 0
    for symbol, findings in grouped.items():
        known = allowlist.get(symbol)
        sites = f"{len(findings)} call site{'' if len(findings) == 1 else 's'}"
        if known is not None and allow_known:
            allowed += 1
            print(f"  ALLOWED  {symbol}  ({sites}; {known})")
        else:
            blocking += 1
            suffix = f"  [allowlisted: {known}]" if known else ""
            print(f"  MISSING  {symbol}  ({sites}){suffix}")
        shown = findings if verbose else findings[:MAX_SITES_PER_SYMBOL]
        for finding in shown:
            print(f"             {finding.path}:{finding.line} ({finding.language})")
        if len(shown) < len(findings):
            print(f"             ... and {len(findings) - len(shown)} more (--verbose to list)")

    # ~keep An allowlisted symbol drops out of `grouped` for two opposite reasons, and
    # ~keep conflating them reports a live bug as fixed: either the export finally appeared
    # ~keep (resolved), or the CALLER disappeared while the export never existed (orphaned).
    # ~keep This is not hypothetical - deleting C#'s htm_register_html_visitor DllImport
    # ~keep removed the very string the C# detector matches on, so the symbol went "stale"
    # ~keep while its public caller stayed put and the package stopped compiling.
    resolved = [s for s in allowlist if s not in grouped and s in comparison.exported]
    orphaned = [s for s in allowlist if s not in grouped and s not in comparison.exported]
    if not grouped:
        print("  no called-but-not-exported symbols")
    for language in silent:
        roots = empty_roots(comparison, language)
        where = f" under {', '.join(roots)}" if roots else ""
        print(f"  SILENT  {language} is registered as diffed but matched no call site{where}")
    return blocking, allowed, resolved, orphaned, silent


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument(
        "--allow-known",
        action="store_true",
        help="downgrade the symbols in the known-missing allowlists to warnings",
    )
    parser.add_argument(
        "-v",
        "--verbose",
        action="store_true",
        help="list every call site instead of the first few per symbol",
    )
    parser.add_argument("--json", action="store_true", help="emit a machine-readable summary on stdout")
    args = parser.parse_args()

    if not FFI_HEADER.is_file():
        print(f"FATAL: canonical header not found: {FFI_HEADER}", file=sys.stderr)
        return 2

    c_abi, disagreements = compare_c_abi()
    php = compare_php_functions()

    if args.json:
        print(
            json.dumps(
                {
                    "exported_c_abi": len(c_abi.exported),
                    "call_sites_c_abi": c_abi.call_sites,
                    "languages_c_abi": sorted(c_abi.languages),
                    "sites_by_language_c_abi": c_abi.sites_by_language,
                    "sites_by_root_c_abi": c_abi.sites_by_root,
                    "silent_detectors": silent_languages(c_abi) + silent_languages(php),
                    "export_source_disagreements": disagreements,
                    "missing": [
                        {"symbol": f.symbol, "language": f.language, "path": f.path, "line": f.line}
                        for f in c_abi.findings + php.findings
                    ],
                },
                indent=2,
            )
        )

    print("=" * 78)
    print("FFI symbol export/caller diff")
    print("=" * 78)

    print("\nExport source cross-check")
    print(f"  html_to_markdown.h declares {len(exported_from_header())} htm_* symbols")
    print(f'  FFI crate defines {len(exported_from_rust())} htm_* `extern "C" fn`')
    for line in disagreements:
        print(f"  DISAGREEMENT  {line}")
    if not disagreements:
        print("  header and Rust sources agree")

    c_blocking, c_allowed, c_resolved, c_orphaned, c_silent = report_comparison(
        c_abi, KNOWN_MISSING_EXPORTS, args.allow_known, args.verbose
    )
    php_blocking, php_allowed, php_resolved, php_orphaned, php_silent = report_comparison(
        php, KNOWN_MISSING_PHP_FUNCTIONS, args.allow_known, args.verbose
    )
    silent = c_silent + php_silent

    print("\nNot diffed")
    for entry in NOT_CHECKED:
        print(f"  {entry.language:<16} {entry.status}: {entry.reason}")

    resolved = c_resolved + php_resolved
    if resolved:
        print("\nResolved allowlist entries (the export now exists - delete the entry)")
        for symbol in resolved:
            print(f"  RESOLVED  {symbol}")

    orphaned = c_orphaned + php_orphaned
    if orphaned:
        print("\nOrphaned allowlist entries (NO export and NO caller - do NOT just delete)")
        print("  The caller vanished rather than the export appearing. That is a fix only if")
        print("  the feature was removed on purpose. If a public API still reaches this symbol,")
        print("  its call site was deleted out from under it and the binding is now broken.")
        for symbol in orphaned:
            print(f"  ORPHANED  {symbol}")

    stale = resolved + orphaned

    blocking = c_blocking + php_blocking
    allowed = c_allowed + php_allowed
    total_call_sites = c_abi.call_sites + php.call_sites
    total_languages = len(c_abi.languages | php.languages)

    print("\n" + "=" * 78)
    print(
        f"{len(c_abi.exported) + len(php.exported)} symbols exported, "
        f"{total_call_sites} call sites checked across {total_languages} languages, "
        f"{len(NOT_CHECKED)} bindings not diffed"
    )
    print(
        f"{blocking} blocking, {allowed} allowlisted, "
        f"{len(disagreements)} export-source disagreements, "
        f"{len(resolved)} resolved allowlist entries, {len(orphaned)} orphaned allowlist entries, "
        f"{len(silent)} silent detectors"
    )

    failed = blocking + len(disagreements) + len(stale) + len(silent)
    print("FAIL" if failed else "OK")
    print("=" * 78)
    return 1 if failed else 0


if __name__ == "__main__":
    raise SystemExit(main())
