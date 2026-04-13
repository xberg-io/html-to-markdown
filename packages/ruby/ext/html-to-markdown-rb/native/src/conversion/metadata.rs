//! Metadata configuration and conversion functions.

use html_to_markdown_rs::metadata::{
    DocumentMetadata as RustDocumentMetadata, HeaderMetadata as RustHeaderMetadata,
    HtmlMetadata as RustHtmlMetadata, ImageMetadata as RustImageMetadata, LinkMetadata as RustLinkMetadata,
    StructuredData as RustStructuredData, TextDirection as RustTextDirection,
};
use magnus::prelude::*;
use magnus::{Error, Ruby, Value};

fn opt_string_to_ruby(ruby: &Ruby, opt: Option<String>) -> Result<Value, Error> {
    match opt {
        Some(val) => Ok(ruby.str_from_slice(val.as_bytes()).as_value()),
        None => Ok(ruby.qnil().as_value()),
    }
}

fn btreemap_to_ruby_hash(ruby: &Ruby, map: std::collections::BTreeMap<String, String>) -> Result<Value, Error> {
    let hash = ruby.hash_new();
    for (k, v) in map {
        hash.aset(k, v)?;
    }
    Ok(hash.as_value())
}

fn text_direction_to_string(text_direction: Option<RustTextDirection>) -> Option<String> {
    text_direction.map(|direction| direction.to_string())
}

fn document_metadata_to_ruby(ruby: &Ruby, doc: RustDocumentMetadata) -> Result<Value, Error> {
    let hash = ruby.hash_new();

    hash.aset(ruby.intern("title"), opt_string_to_ruby(ruby, doc.title)?)?;
    hash.aset(ruby.intern("description"), opt_string_to_ruby(ruby, doc.description)?)?;

    let keywords = ruby.ary_new();
    for keyword in doc.keywords {
        keywords.push(keyword)?;
    }
    hash.aset(ruby.intern("keywords"), keywords)?;

    hash.aset(ruby.intern("author"), opt_string_to_ruby(ruby, doc.author)?)?;
    hash.aset(
        ruby.intern("canonical_url"),
        opt_string_to_ruby(ruby, doc.canonical_url)?,
    )?;
    hash.aset(ruby.intern("base_href"), opt_string_to_ruby(ruby, doc.base_href)?)?;
    hash.aset(ruby.intern("language"), opt_string_to_ruby(ruby, doc.language)?)?;

    match text_direction_to_string(doc.text_direction) {
        Some(dir) => hash.aset(ruby.intern("text_direction"), dir)?,
        None => hash.aset(ruby.intern("text_direction"), ruby.qnil())?,
    }

    hash.aset(ruby.intern("open_graph"), btreemap_to_ruby_hash(ruby, doc.open_graph)?)?;
    hash.aset(
        ruby.intern("twitter_card"),
        btreemap_to_ruby_hash(ruby, doc.twitter_card)?,
    )?;
    hash.aset(ruby.intern("meta_tags"), btreemap_to_ruby_hash(ruby, doc.meta_tags)?)?;

    Ok(hash.as_value())
}

fn headers_to_ruby(ruby: &Ruby, headers: Vec<RustHeaderMetadata>) -> Result<Value, Error> {
    let array = ruby.ary_new();
    for header in headers {
        let hash = ruby.hash_new();
        hash.aset(ruby.intern("level"), header.level)?;
        hash.aset(ruby.intern("text"), header.text)?;
        hash.aset(ruby.intern("id"), opt_string_to_ruby(ruby, header.id)?)?;
        hash.aset(ruby.intern("depth"), header.depth as i64)?;
        hash.aset(ruby.intern("html_offset"), header.html_offset as i64)?;
        array.push(hash)?;
    }
    Ok(array.as_value())
}

fn links_to_ruby(ruby: &Ruby, links: Vec<RustLinkMetadata>) -> Result<Value, Error> {
    let array = ruby.ary_new();
    for link in links {
        let hash = ruby.hash_new();
        hash.aset(ruby.intern("href"), link.href)?;
        hash.aset(ruby.intern("text"), link.text)?;
        hash.aset(ruby.intern("title"), opt_string_to_ruby(ruby, link.title)?)?;
        hash.aset(ruby.intern("link_type"), link.link_type.to_string())?;

        let rel_array = ruby.ary_new();
        for r in link.rel {
            rel_array.push(r)?;
        }
        hash.aset(ruby.intern("rel"), rel_array)?;

        hash.aset(ruby.intern("attributes"), btreemap_to_ruby_hash(ruby, link.attributes)?)?;
        array.push(hash)?;
    }
    Ok(array.as_value())
}

fn images_to_ruby(ruby: &Ruby, images: Vec<RustImageMetadata>) -> Result<Value, Error> {
    let array = ruby.ary_new();
    for image in images {
        let hash = ruby.hash_new();
        hash.aset(ruby.intern("src"), image.src)?;
        hash.aset(ruby.intern("alt"), opt_string_to_ruby(ruby, image.alt)?)?;
        hash.aset(ruby.intern("title"), opt_string_to_ruby(ruby, image.title)?)?;

        match image.dimensions {
            Some((width, height)) => {
                let dims = ruby.ary_new();
                dims.push(i64::from(width))?;
                dims.push(i64::from(height))?;
                hash.aset(ruby.intern("dimensions"), dims)?;
            }
            None => {
                hash.aset(ruby.intern("dimensions"), ruby.qnil())?;
            }
        }

        hash.aset(ruby.intern("image_type"), image.image_type.to_string())?;
        hash.aset(
            ruby.intern("attributes"),
            btreemap_to_ruby_hash(ruby, image.attributes)?,
        )?;
        array.push(hash)?;
    }
    Ok(array.as_value())
}

fn structured_data_to_ruby(ruby: &Ruby, data: Vec<RustStructuredData>) -> Result<Value, Error> {
    let array = ruby.ary_new();
    for item in data {
        let hash = ruby.hash_new();
        hash.aset(ruby.intern("data_type"), item.data_type.to_string())?;
        hash.aset(ruby.intern("raw_json"), item.raw_json)?;
        hash.aset(ruby.intern("schema_type"), opt_string_to_ruby(ruby, item.schema_type)?)?;
        array.push(hash)?;
    }
    Ok(array.as_value())
}

pub fn extended_metadata_to_ruby(ruby: &Ruby, metadata: RustHtmlMetadata) -> Result<Value, Error> {
    let hash = ruby.hash_new();

    hash.aset(
        ruby.intern("document"),
        document_metadata_to_ruby(ruby, metadata.document)?,
    )?;
    hash.aset(ruby.intern("headers"), headers_to_ruby(ruby, metadata.headers)?)?;
    hash.aset(ruby.intern("links"), links_to_ruby(ruby, metadata.links)?)?;
    hash.aset(ruby.intern("images"), images_to_ruby(ruby, metadata.images)?)?;
    hash.aset(
        ruby.intern("structured_data"),
        structured_data_to_ruby(ruby, metadata.structured_data)?,
    )?;

    Ok(hash.as_value())
}
