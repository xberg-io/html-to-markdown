from typing import Any, Literal, TypedDict

class PreprocessingOptions:
    enabled: bool
    preset: Literal["minimal", "standard", "aggressive"]
    remove_navigation: bool
    remove_forms: bool

    def __init__(
        self,
        *,
        enabled: bool = False,
        preset: Literal["minimal", "standard", "aggressive"] = "standard",
        remove_navigation: bool = True,
        remove_forms: bool = True,
    ) -> None: ...

class ConversionOptions:
    heading_style: Literal["underlined", "atx", "atx_closed"]
    list_indent_type: Literal["spaces", "tabs"]
    list_indent_width: int
    bullets: str
    strong_em_symbol: str
    escape_asterisks: bool
    escape_underscores: bool
    escape_misc: bool
    escape_ascii: bool
    code_language: str
    autolinks: bool
    default_title: bool
    br_in_tables: bool
    highlight_style: Literal["double-equal", "html", "bold", "none"]
    extract_metadata: bool
    whitespace_mode: Literal["normalized", "strict"]
    strip_newlines: bool
    wrap: bool
    wrap_width: int
    convert_as_inline: bool
    sub_symbol: str
    sup_symbol: str
    newline_style: Literal["spaces", "backslash"]
    code_block_style: Literal["indented", "backticks", "tildes"]
    keep_inline_images_in: list[str]
    preprocessing: PreprocessingOptions
    encoding: str
    debug: bool
    strip_tags: list[str]
    preserve_tags: list[str]
    skip_images: bool
    output_format: Literal["markdown", "djot"]
    include_document_structure: bool
    extract_images: bool
    max_image_size: int
    capture_svg: bool
    infer_dimensions: bool
    max_depth: int | None

    def __init__(
        self,
        *,
        heading_style: Literal["underlined", "atx", "atx_closed"] = "underlined",
        list_indent_type: Literal["spaces", "tabs"] = "spaces",
        list_indent_width: int = 4,
        bullets: str = "*+-",
        strong_em_symbol: str = "*",
        escape_asterisks: bool = False,
        escape_underscores: bool = False,
        escape_misc: bool = False,
        escape_ascii: bool = False,
        code_language: str = "",
        autolinks: bool = True,
        default_title: bool = False,
        br_in_tables: bool = False,
        highlight_style: Literal["double-equal", "html", "bold", "none"] = "double-equal",
        extract_metadata: bool = True,
        whitespace_mode: Literal["normalized", "strict"] = "normalized",
        strip_newlines: bool = False,
        wrap: bool = False,
        wrap_width: int = 80,
        convert_as_inline: bool = False,
        sub_symbol: str = "",
        sup_symbol: str = "",
        newline_style: Literal["spaces", "backslash"] = "spaces",
        code_block_style: Literal["indented", "backticks", "tildes"] = "indented",
        keep_inline_images_in: list[str] = [],
        preprocessing: PreprocessingOptions | None = None,
        encoding: str = "utf-8",
        debug: bool = False,
        strip_tags: list[str] = [],
        preserve_tags: list[str] = [],
        skip_images: bool = False,
        output_format: Literal["markdown", "djot"] = "markdown",
        include_document_structure: bool = False,
        extract_images: bool = False,
        max_image_size: int = 5_242_880,
        capture_svg: bool = False,
        infer_dimensions: bool = True,
        max_depth: int | None = None,
    ) -> None: ...

class GridCell(TypedDict):
    content: str
    row: int
    col: int
    row_span: int
    col_span: int
    is_header: bool

class TableGrid(TypedDict):
    rows: int
    cols: int
    cells: list[GridCell]

class ExtractedTable(TypedDict):
    grid: TableGrid
    markdown: str

class ProcessingWarning(TypedDict):
    message: str
    kind: str

class ConversionResult(TypedDict):
    content: str | None
    document: None
    metadata: dict[str, Any] | None
    tables: list[ExtractedTable]
    images: list[Any]
    warnings: list[ProcessingWarning]

def convert(
    html: str,
    options: ConversionOptions | None = None,
    visitor: object | None = None,
) -> ConversionResult: ...
