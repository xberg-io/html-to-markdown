"""High-level Python API backed by the Rust core."""

from __future__ import annotations

from typing import Any, TypedDict, cast

import html_to_markdown._html_to_markdown as _rust
from html_to_markdown.options import ConversionOptions, PreprocessingOptions


class GridCell(TypedDict):
    """A single cell in a structured table grid."""

    content: str
    row: int
    col: int
    row_span: int
    col_span: int
    is_header: bool


class TableGrid(TypedDict):
    """Structured table grid with cell-level data."""

    rows: int
    cols: int
    cells: list[GridCell]


class ExtractedTable(TypedDict):
    """A table extracted via the ConversionResult API."""

    grid: TableGrid
    markdown: str


class ProcessingWarning(TypedDict):
    """A non-fatal warning emitted during conversion."""

    message: str
    kind: str


class ConversionResult(TypedDict):
    """Full result of the convert() API."""

    content: str | None
    document: dict[str, Any] | str | None
    metadata: dict[str, Any] | None
    tables: list[ExtractedTable]
    images: list[Any]
    warnings: list[ProcessingWarning]


def _as_list(value: set[str] | None) -> list[str]:
    return sorted(value) if value else []


def _rust_preprocessing(preprocessing: PreprocessingOptions) -> _rust.PreprocessingOptions:
    return _rust.PreprocessingOptions(
        enabled=preprocessing.enabled,
        preset=preprocessing.preset,
        remove_navigation=preprocessing.remove_navigation,
        remove_forms=preprocessing.remove_forms,
    )


def _rust_options(
    options: ConversionOptions | None,
    preprocessing: PreprocessingOptions | None,
) -> _rust.ConversionOptions | None:
    if options is None and preprocessing is None:
        return None

    if options is None:
        options = ConversionOptions()
    if preprocessing is None:
        preprocessing = PreprocessingOptions()

    rust_opts = _rust.ConversionOptions(
        heading_style=options.heading_style,
        list_indent_type=options.list_indent_type,
        list_indent_width=options.list_indent_width,
        bullets=options.bullets,
        strong_em_symbol=options.strong_em_symbol,
        escape_asterisks=options.escape_asterisks,
        escape_underscores=options.escape_underscores,
        escape_misc=options.escape_misc,
        escape_ascii=options.escape_ascii,
        code_language=options.code_language,
        autolinks=options.autolinks,
        default_title=options.default_title,
        br_in_tables=options.br_in_tables,
        highlight_style=options.highlight_style,
        extract_metadata=options.extract_metadata,
        whitespace_mode=options.whitespace_mode,
        strip_newlines=options.strip_newlines,
        wrap=options.wrap,
        wrap_width=options.wrap_width,
        convert_as_inline=options.convert_as_inline,
        sub_symbol=options.sub_symbol,
        sup_symbol=options.sup_symbol,
        newline_style=options.newline_style,
        code_block_style=options.code_block_style,
        keep_inline_images_in=_as_list(options.keep_inline_images_in),
        preprocessing=_rust_preprocessing(preprocessing),
        debug=options.debug,
        strip_tags=_as_list(options.strip_tags),
        preserve_tags=_as_list(options.preserve_tags),
        encoding=options.encoding,
        skip_images=options.skip_images,
        output_format=options.output_format,
    )
    if options.include_document_structure:
        rust_opts.include_document_structure = options.include_document_structure
    if options.extract_images:
        rust_opts.extract_images = options.extract_images
    if options.max_image_size != 5_242_880:
        rust_opts.max_image_size = options.max_image_size
    if options.capture_svg:
        rust_opts.capture_svg = options.capture_svg
    if not options.infer_dimensions:
        rust_opts.infer_dimensions = options.infer_dimensions
    return rust_opts


def convert(
    html: str,
    options: ConversionOptions | None = None,
    preprocessing: PreprocessingOptions | None = None,
    visitor: object | None = None,
) -> ConversionResult:
    """Convert HTML to Markdown.

    Returns a typed dict containing the converted content alongside all extracted
    metadata, tables, images, and processing warnings in a single pass.

    Args:
        html: HTML string to convert
        options: Optional conversion configuration
        preprocessing: Optional preprocessing configuration
        visitor: Optional visitor object with callback methods for custom element handling

    Returns:
        ConversionResult dict with keys:
            - content (str | None): Converted markdown, or None in extraction-only mode
            - document (None): Document structure (not yet exposed in bindings)
            - metadata (dict | None): Extracted HTML metadata (when metadata feature is enabled)
            - tables (list[dict]): Extracted tables with grid and markdown fields
            - images (list): Extracted inline images (when inline-images feature is enabled)
            - warnings (list[dict]): Non-fatal processing warnings
    """
    rust_options = _rust_options(options, preprocessing)
    return cast("ConversionResult", _rust.convert(html, rust_options, visitor))


__all__ = [
    "ConversionResult",
    "ExtractedTable",
    "GridCell",
    "ProcessingWarning",
    "TableGrid",
    "convert",
]
