#[cfg(all(test, feature = "js-bindings"))]
mod wasm_tests {
    use crate::enums::WasmHeadingStyle;
    use crate::options::WasmConversionOptions;
    use wasm_bindgen::JsValue;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_convert_basic() {
        let html = "<h1>Hello World</h1>".to_string();
        let result = crate::convert::convert(html, JsValue::UNDEFINED);
        assert!(result.is_ok());
        let obj = result.unwrap();
        let content = js_sys::Reflect::get(&obj, &JsValue::from_str("content")).unwrap();
        let content_str = content.as_string().unwrap_or_default();
        assert!(content_str.contains("Hello World"));
    }

    #[wasm_bindgen_test]
    fn test_convert_with_options() {
        let html = "<h1>Hello</h1>".to_string();
        let options = WasmConversionOptions {
            heading_style: Some(WasmHeadingStyle::Atx),
            list_indent_type: None,
            list_indent_width: None,
            bullets: None,
            strong_em_symbol: None,
            escape_asterisks: None,
            escape_underscores: None,
            escape_misc: None,
            escape_ascii: None,
            code_language: None,
            autolinks: None,
            default_title: None,
            br_in_tables: None,
            highlight_style: None,
            extract_metadata: None,
            whitespace_mode: None,
            strip_newlines: None,
            wrap: None,
            wrap_width: None,
            convert_as_inline: None,
            sub_symbol: None,
            sup_symbol: None,
            newline_style: None,
            code_block_style: None,
            keep_inline_images_in: None,
            skip_images: None,
            link_style: None,
            preprocessing: None,
            encoding: None,
            debug: None,
            strip_tags: None,
            preserve_tags: None,
            output_format: None,
            include_document_structure: None,
            extract_images: None,
            max_image_size: None,
            capture_svg: None,
            infer_dimensions: None,
            max_depth: None,
        };

        let js_options = serde_wasm_bindgen::to_value(&options).unwrap();
        let result = crate::convert::convert(html, js_options);
        assert!(result.is_ok());
    }

    #[wasm_bindgen_test]
    fn test_convert_returns_v3_object() {
        let html = "<h1>Hello World</h1>".to_string();

        let result = crate::convert::convert(html, JsValue::UNDEFINED);
        assert!(result.is_ok());

        let obj = result.unwrap();
        assert!(js_sys::Reflect::has(&obj, &JsValue::from_str("content")).unwrap());
        assert!(js_sys::Reflect::has(&obj, &JsValue::from_str("metadata")).unwrap());
        assert!(js_sys::Reflect::has(&obj, &JsValue::from_str("tables")).unwrap());
        assert!(js_sys::Reflect::has(&obj, &JsValue::from_str("warnings")).unwrap());
        assert!(js_sys::Reflect::has(&obj, &JsValue::from_str("document")).unwrap());
    }

    #[wasm_bindgen_test]
    fn test_convert_content_includes_html_text() {
        let html = r#"<html><head><title>Test</title></head><body><h1 id="main">Main Title</h1><h2>Subsection</h2></body></html>"#
            .to_string();

        let result = crate::convert::convert(html, JsValue::UNDEFINED);
        assert!(result.is_ok());

        let obj = result.unwrap();
        let content = js_sys::Reflect::get(&obj, &JsValue::from_str("content")).unwrap();
        let content_str = content.as_string().unwrap();
        assert!(content_str.contains("Main Title"));
    }

    #[wasm_bindgen_test]
    fn test_convert_tables_is_array() {
        let html = "<p>no tables</p>".to_string();

        let result = crate::convert::convert(html, JsValue::UNDEFINED);
        assert!(result.is_ok());

        let obj = result.unwrap();
        let tables = js_sys::Reflect::get(&obj, &JsValue::from_str("tables")).unwrap();
        assert!(js_sys::Array::is_array(&tables));
    }

    #[wasm_bindgen_test]
    fn test_convert_warnings_is_array() {
        let html = "<p>simple</p>".to_string();

        let result = crate::convert::convert(html, JsValue::UNDEFINED);
        assert!(result.is_ok());

        let obj = result.unwrap();
        let warnings = js_sys::Reflect::get(&obj, &JsValue::from_str("warnings")).unwrap();
        assert!(js_sys::Array::is_array(&warnings));
    }
}
