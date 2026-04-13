#![allow(clippy::let_unit_value, deprecated)]

use html_to_markdown_rs::metadata::{
    DocumentMetadata, HeaderMetadata, HtmlMetadata, ImageMetadata, LinkMetadata, StructuredData,
};

mod options;
mod types;

use options::{
    INVALID_OPTION_ERROR, decode_options_term,
    take_invalid_option_message,
};
use types::{
    ConversionResultTerm, DocumentMetadataTerm, ExtendedMetadataTerm, ExtractTableTerm, GridCellTerm,
    HeaderMetadataTerm, ImageMetadataTerm, InlineImageTerm, LinkMetadataTerm,
    StructuredDataTerm, WarningTerm,
};

use rustler::{Encoder, Env, Error, NifResult, Term};

rustler::init!(
    "Elixir.HtmlToMarkdown.Native",
    [
        convert
    ],
    load = on_load
);

fn on_load(_env: Env, _info: Term) -> bool {
    true
}

mod atoms {
    rustler::atoms! {
        ok,
        error,
        invalid_option,
        conversion_failed,
        atx,
        atx_closed,
        underlined,
        spaces,
        tabs,
        normalized,
        strict,
        minimal,
        standard,
        aggressive,
        backslash,
        indented,
        backticks,
        tildes,
        img_data_uri,
        svg_element,
    }
}

#[rustler::nif(schedule = "DirtyCpu")]
fn convert<'a>(env: Env<'a>, html: String, options_term: Term<'a>) -> NifResult<Term<'a>> {
    let options = match decode_options_term(options_term) {
        Ok(options) => options,
        Err(err) => return handle_invalid_option_error(env, err),
    };

    match html_to_markdown_rs::convert(&html, Some(options.clone())) {
        Ok(result) => {
            let tables: Vec<ExtractTableTerm> = result
                .tables
                .into_iter()
                .map(|t| {
                    let cells: Vec<GridCellTerm> = t
                        .grid
                        .cells
                        .into_iter()
                        .map(|c| GridCellTerm {
                            content: c.content,
                            row: c.row,
                            col: c.col,
                            row_span: c.row_span,
                            col_span: c.col_span,
                            is_header: c.is_header,
                        })
                        .collect();
                    ExtractTableTerm {
                        grid: types::TableGridTerm {
                            rows: t.grid.rows,
                            cols: t.grid.cols,
                            cells,
                        },
                        markdown: t.markdown,
                    }
                })
                .collect();

            let warnings: Vec<WarningTerm> = result
                .warnings
                .into_iter()
                .map(|w| {
                    let kind = match w.kind {
                        html_to_markdown_rs::WarningKind::ImageExtractionFailed => {
                            "image_extraction_failed"
                        }
                        html_to_markdown_rs::WarningKind::EncodingFallback => "encoding_fallback",
                        html_to_markdown_rs::WarningKind::TruncatedInput => "truncated_input",
                        html_to_markdown_rs::WarningKind::MalformedHtml => "malformed_html",
                        html_to_markdown_rs::WarningKind::SanitizationApplied => {
                            "sanitization_applied"
                        }
                    };
                    WarningTerm {
                        message: w.message,
                        kind: kind.to_string(),
                    }
                })
                .collect();

            let document = result.document.and_then(|doc| {
                serde_json::to_string(&doc).ok()
            });

            let images: Vec<InlineImageTerm> = result
                .images
                .into_iter()
                .map(|img| InlineImageTerm {
                    data: img.data,
                    format: img.format.to_string(),
                    filename: img.filename,
                    description: img.description,
                    dimensions: img.dimensions,
                    source: img.source.to_string(),
                    attributes: img.attributes.into_iter().collect(),
                })
                .collect();

            let term = ConversionResultTerm {
                content: result.content,
                document,
                metadata: build_metadata(result.metadata),
                tables,
                images,
                warnings,
            };

            Ok((atoms::ok(), term).encode(env))
        }
        Err(err) => Ok((atoms::error(), err.to_string()).encode(env)),
    }
}

fn build_metadata(metadata: HtmlMetadata) -> ExtendedMetadataTerm {
    ExtendedMetadataTerm {
        document: build_document_metadata(metadata.document),
        headers: metadata.headers.into_iter().map(build_header_metadata).collect(),
        links: metadata.links.into_iter().map(build_link_metadata).collect(),
        images: metadata.images.into_iter().map(build_image_metadata).collect(),
        structured_data: metadata
            .structured_data
            .into_iter()
            .map(build_structured_data)
            .collect(),
    }
}

fn build_document_metadata(metadata: DocumentMetadata) -> DocumentMetadataTerm {
    DocumentMetadataTerm {
        title: metadata.title,
        description: metadata.description,
        keywords: metadata.keywords,
        author: metadata.author,
        canonical_url: metadata.canonical_url,
        base_href: metadata.base_href,
        language: metadata.language,
        text_direction: metadata.text_direction.map(|td| td.to_string()),
        open_graph: metadata.open_graph.into_iter().collect(),
        twitter_card: metadata.twitter_card.into_iter().collect(),
        meta_tags: metadata.meta_tags.into_iter().collect(),
    }
}

fn build_header_metadata(metadata: HeaderMetadata) -> HeaderMetadataTerm {
    HeaderMetadataTerm {
        level: metadata.level,
        text: metadata.text,
        id: metadata.id,
        depth: metadata.depth as u64,
        html_offset: metadata.html_offset as u64,
    }
}

fn build_link_metadata(metadata: LinkMetadata) -> LinkMetadataTerm {
    LinkMetadataTerm {
        href: metadata.href,
        text: metadata.text,
        title: metadata.title,
        link_type: metadata.link_type.to_string(),
        rel: metadata.rel,
        attributes: metadata.attributes.into_iter().collect(),
    }
}

fn build_image_metadata(metadata: ImageMetadata) -> ImageMetadataTerm {
    ImageMetadataTerm {
        src: metadata.src,
        alt: metadata.alt,
        title: metadata.title,
        dimensions: metadata.dimensions,
        image_type: metadata.image_type.to_string(),
        attributes: metadata.attributes.into_iter().collect(),
    }
}

fn build_structured_data(metadata: StructuredData) -> StructuredDataTerm {
    StructuredDataTerm {
        data_type: metadata.data_type.to_string(),
        raw_json: metadata.raw_json,
        schema_type: metadata.schema_type,
    }
}

fn handle_invalid_option_error<'a>(env: Env<'a>, err: Error) -> NifResult<Term<'a>> {
    match err {
        Error::Atom(atom) if atom == INVALID_OPTION_ERROR => {
            let reason = take_invalid_option_message().unwrap_or_else(|| "invalid option".to_string());
            Ok((atoms::error(), reason).encode(env))
        }
        other => Err(other),
    }
}
