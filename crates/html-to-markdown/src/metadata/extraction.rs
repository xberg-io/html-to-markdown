//! Metadata extraction utilities and helpers.

use super::types::{DocumentMetadata, StructuredData, StructuredDataType, TextDirection};
use std::collections::BTreeMap;

/// Extract document metadata from collected head metadata.
///
/// Parses head metadata into structured document metadata,
/// handling special cases like Open Graph, Twitter Card, keywords, etc.
pub(crate) fn extract_document_metadata(
    head_metadata: BTreeMap<String, String>,
    lang: Option<String>,
    dir: Option<String>,
) -> DocumentMetadata {
    let mut doc = DocumentMetadata::default();
    let has_title_element = head_metadata.contains_key("title");

    for (raw_key, value) in head_metadata {
        // ~keep Only the `<title>`, `<base>` and `<link rel="canonical">` elements write these
        // ~keep keys. A meta tag named `base` or `canonical` arrives as `meta-base` or
        // ~keep `meta-canonical` and is an ordinary meta tag; one named `title` gives the title
        // ~keep only on a page without a `<title>` element (#589).
        match raw_key.as_str() {
            "title" => {
                doc.title = Some(value);
                continue;
            }
            "base" => {
                doc.base_href = Some(value);
                continue;
            }
            "canonical" => {
                doc.canonical_url = Some(value);
                continue;
            }
            _ => {}
        }

        let mut key = raw_key.as_str();
        let mut replaced_key: Option<String> = None;

        if let Some(stripped) = key.strip_prefix("meta-") {
            key = stripped;
        }

        if key.as_bytes().contains(&b':') {
            replaced_key = Some(key.replace(':', "-"));
            key = replaced_key.as_deref().unwrap_or(key);
        }

        let lower_key = key.to_ascii_lowercase();

        match lower_key.as_str() {
            "title" if !has_title_element => doc.title = Some(value),
            "description" => doc.description = Some(value),
            "author" | "creator" | "publisher" => {
                if doc.author.is_none() {
                    doc.author = Some(value);
                }
            }
            k if k.starts_with("og-") => {
                let og_key = k.trim_start_matches("og-").replace('-', "_");
                doc.open_graph.insert(og_key, value);
            }
            k if k.starts_with("twitter-") => {
                let tw_key = k.trim_start_matches("twitter-").replace('-', "_");
                doc.twitter_card.insert(tw_key, value);
            }
            k if k.starts_with("dc.") || k.starts_with("dc-") => {
                let dc_field = k.trim_start_matches("dc.").trim_start_matches("dc-");
                match dc_field {
                    "title" => {
                        if doc.title.is_none() {
                            doc.title = Some(value);
                        }
                    }
                    "description" => {
                        if doc.description.is_none() {
                            doc.description = Some(value);
                        }
                    }
                    "creator" | "contributor" | "publisher" => {
                        if doc.author.is_none() {
                            doc.author = Some(value);
                        }
                    }
                    "subject" | "keywords" => {
                        if doc.keywords.is_empty() {
                            doc.keywords = split_keywords(&value);
                        }
                    }
                    _ => {
                        let meta_key = format!("dc_{}", dc_field.replace('-', "_"));
                        doc.meta_tags.insert(meta_key, value);
                    }
                }
            }
            k if k.starts_with("dcterms.") || k.starts_with("dcterms-") => {
                let dc_field = k.trim_start_matches("dcterms.").trim_start_matches("dcterms-");
                match dc_field {
                    "title" | "alternative" => {
                        if doc.title.is_none() {
                            doc.title = Some(value);
                        }
                    }
                    "description" | "abstract" => {
                        if doc.description.is_none() {
                            doc.description = Some(value);
                        }
                    }
                    "creator" | "contributor" | "publisher" => {
                        if doc.author.is_none() {
                            doc.author = Some(value);
                        }
                    }
                    "subject" | "keywords" => {
                        if doc.keywords.is_empty() {
                            doc.keywords = split_keywords(&value);
                        }
                    }
                    _ => {
                        let meta_key = format!("dcterms_{}", dc_field.replace('-', "_"));
                        doc.meta_tags.insert(meta_key, value);
                    }
                }
            }
            "keywords" | "news_keywords" | "citation_keywords" | "subject" | "topic" | "category"
            | "classification" => {
                if doc.keywords.is_empty() {
                    doc.keywords = split_keywords(&value);
                }
            }
            _ => {
                let meta_key = if key.as_ptr() == raw_key.as_ptr() && key.len() == raw_key.len() {
                    raw_key
                } else if let Some(replaced) = replaced_key {
                    replaced
                } else {
                    key.to_string()
                };
                doc.meta_tags.insert(meta_key, value);
            }
        }
    }

    if let Some(lang) = lang {
        doc.language = Some(lang);
    }

    if let Some(dir) = dir {
        if let Some(parsed_dir) = TextDirection::parse(&dir) {
            doc.text_direction = Some(parsed_dir);
        }
    }

    doc
}

/// Split a comma-separated keywords string into a `Vec<String>`.
fn split_keywords(value: &str) -> Vec<String> {
    value
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

/// Extract structured data blocks into `StructuredData` items.
pub(crate) fn extract_structured_data(json_ld: Vec<String>) -> Vec<StructuredData> {
    let mut result = Vec::with_capacity(json_ld.len());

    for json_str in json_ld {
        let schema_type = scan_schema_type(&json_str)
            .or_else(|| {
                if json_str.contains("\"@type\"") {
                    serde_json::from_str::<serde_json::Value>(&json_str).ok().and_then(|v| {
                        v.get("@type")
                            .and_then(|t| t.as_str().map(std::string::ToString::to_string))
                    })
                } else {
                    None
                }
            })
            .or_else(|| {
                if !json_str.contains("\"@graph\"") {
                    return None;
                }

                let value = serde_json::from_str::<serde_json::Value>(&json_str).ok()?;
                let graph = value.get("@graph")?;
                let items = graph.as_array()?;
                items.iter().find_map(|item| {
                    item.get("@type")
                        .and_then(|t| t.as_str().map(std::string::ToString::to_string))
                })
            });

        result.push(StructuredData {
            data_type: StructuredDataType::JsonLd,
            raw_json: json_str,
            schema_type,
        });
    }

    result
}

/// Scan for @type in JSON string without full parsing.
fn scan_schema_type(json_str: &str) -> Option<String> {
    let needle = "\"@type\"";
    let start = json_str.find(needle)? + needle.len();
    let bytes = json_str.as_bytes();
    let mut i = start;

    while i < bytes.len() && bytes[i].is_ascii_whitespace() {
        i += 1;
    }
    if i >= bytes.len() || bytes[i] != b':' {
        return None;
    }
    i += 1;
    while i < bytes.len() && bytes[i].is_ascii_whitespace() {
        i += 1;
    }
    if i >= bytes.len() {
        return None;
    }

    if bytes[i] == b'[' {
        i += 1;
        while i < bytes.len() && bytes[i].is_ascii_whitespace() {
            i += 1;
        }
        if i >= bytes.len() || bytes[i] != b'"' {
            return None;
        }
    } else if bytes[i] != b'"' {
        return None;
    }

    let start_quote = i;
    i += 1;
    let mut escaped = false;
    while i < bytes.len() {
        let byte = bytes[i];
        if escaped {
            escaped = false;
            i += 1;
            continue;
        }
        if byte == b'\\' {
            escaped = true;
            i += 1;
            continue;
        }
        if byte == b'"' {
            let end_quote = i;
            let slice = &json_str[start_quote..=end_quote];
            return serde_json::from_str::<String>(slice).ok();
        }
        i += 1;
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_document_metadata() {
        let mut head_metadata = BTreeMap::new();
        head_metadata.insert("title".to_string(), "Test Title".to_string());
        head_metadata.insert("description".to_string(), "Test Description".to_string());
        head_metadata.insert("keywords".to_string(), "rust, testing".to_string());

        let doc = extract_document_metadata(head_metadata, Some("en".to_string()), Some("ltr".to_string()));

        assert_eq!(doc.title, Some("Test Title".to_string()));
        assert_eq!(doc.description, Some("Test Description".to_string()));
        assert_eq!(doc.keywords, vec!["rust", "testing"]);
        assert_eq!(doc.language, Some("en".to_string()));
    }

    #[test]
    fn test_extract_document_metadata_with_og() {
        let mut head_metadata = BTreeMap::new();
        head_metadata.insert("og-title".to_string(), "OG Title".to_string());
        head_metadata.insert("og-description".to_string(), "OG Description".to_string());

        let doc = extract_document_metadata(head_metadata, None, None);

        assert_eq!(doc.open_graph.get("title"), Some(&"OG Title".to_string()));
        assert_eq!(doc.open_graph.get("description"), Some(&"OG Description".to_string()));
    }

    #[test]
    fn test_keywords_case_insensitive() {
        let mut head_metadata = BTreeMap::new();
        head_metadata.insert("meta-Keywords".to_string(), "Rust, HTML, Markdown".to_string());

        let doc = extract_document_metadata(head_metadata, None, None);
        assert_eq!(doc.keywords, vec!["Rust", "HTML", "Markdown"]);
    }

    #[test]
    fn test_keywords_dc_subject() {
        let mut head_metadata = BTreeMap::new();
        head_metadata.insert("meta-DC.subject".to_string(), "weather, forecast".to_string());

        let doc = extract_document_metadata(head_metadata, None, None);
        assert_eq!(doc.keywords, vec!["weather", "forecast"]);
    }

    #[test]
    fn test_keywords_dc_keywords() {
        let mut head_metadata = BTreeMap::new();
        head_metadata.insert("meta-DC.keywords".to_string(), "climate, data".to_string());

        let doc = extract_document_metadata(head_metadata, None, None);
        assert_eq!(doc.keywords, vec!["climate", "data"]);
    }

    #[test]
    fn test_keywords_dcterms_subject() {
        let mut head_metadata = BTreeMap::new();
        head_metadata.insert("meta-DCTERMS.subject".to_string(), "science, research".to_string());

        let doc = extract_document_metadata(head_metadata, None, None);
        assert_eq!(doc.keywords, vec!["science", "research"]);
    }

    #[test]
    fn test_keywords_news_keywords() {
        let mut head_metadata = BTreeMap::new();
        head_metadata.insert("meta-news_keywords".to_string(), "breaking, world".to_string());

        let doc = extract_document_metadata(head_metadata, None, None);
        assert_eq!(doc.keywords, vec!["breaking", "world"]);
    }

    #[test]
    fn test_keywords_citation_keywords() {
        let mut head_metadata = BTreeMap::new();
        head_metadata.insert("meta-citation_keywords".to_string(), "biology, genetics".to_string());

        let doc = extract_document_metadata(head_metadata, None, None);
        assert_eq!(doc.keywords, vec!["biology", "genetics"]);
    }

    #[test]
    fn test_dc_title_and_description_fallback() {
        let mut head_metadata = BTreeMap::new();
        head_metadata.insert("meta-DC.title".to_string(), "DC Title".to_string());
        head_metadata.insert("meta-DC.description".to_string(), "DC Description".to_string());
        head_metadata.insert("meta-DC.creator".to_string(), "DC Author".to_string());

        let doc = extract_document_metadata(head_metadata, None, None);
        assert_eq!(doc.title, Some("DC Title".to_string()));
        assert_eq!(doc.description, Some("DC Description".to_string()));
        assert_eq!(doc.author, Some("DC Author".to_string()));
    }

    #[test]
    fn test_dc_does_not_override_standard_fields() {
        let mut head_metadata = BTreeMap::new();
        head_metadata.insert("description".to_string(), "Standard Description".to_string());
        head_metadata.insert("meta-DC.description".to_string(), "DC Description".to_string());
        head_metadata.insert("title".to_string(), "Standard Title".to_string());
        head_metadata.insert("meta-DC.title".to_string(), "DC Title".to_string());

        let doc = extract_document_metadata(head_metadata, None, None);
        assert_eq!(doc.title, Some("Standard Title".to_string()));
        assert_eq!(doc.description, Some("Standard Description".to_string()));
    }

    #[test]
    fn test_case_insensitive_title_description_author() {
        let mut head_metadata = BTreeMap::new();
        head_metadata.insert("meta-Title".to_string(), "My Title".to_string());
        head_metadata.insert("meta-Description".to_string(), "My Desc".to_string());
        head_metadata.insert("meta-Author".to_string(), "My Author".to_string());

        let doc = extract_document_metadata(head_metadata, None, None);
        assert_eq!(doc.title, Some("My Title".to_string()));
        assert_eq!(doc.description, Some("My Desc".to_string()));
        assert_eq!(doc.author, Some("My Author".to_string()));
    }

    #[test]
    fn test_dcterms_remaining_go_to_meta_tags() {
        let mut head_metadata = BTreeMap::new();
        head_metadata.insert("meta-DCTERMS.license".to_string(), "MIT".to_string());

        let doc = extract_document_metadata(head_metadata, None, None);
        assert_eq!(doc.meta_tags.get("dcterms_license"), Some(&"MIT".to_string()));
    }

    #[test]
    fn test_scan_schema_type() {
        let json = r#"{"@type":"Article","title":"Test"}"#;
        assert_eq!(scan_schema_type(json), Some("Article".to_string()));

        let json_array = r#"{"@type":["Article","NewsArticle"]}"#;
        assert_eq!(scan_schema_type(json_array), Some("Article".to_string()));

        let no_type = r#"{"title":"Test"}"#;
        assert_eq!(scan_schema_type(no_type), None);
    }
}
