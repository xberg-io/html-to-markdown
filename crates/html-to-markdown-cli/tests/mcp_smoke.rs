//! End-to-end smoke test for the `html-to-markdown mcp` stdio server.
//!
//! Drives the real binary over the MCP stdio transport (newline-delimited
//! JSON-RPC): initialize, then `tools/list`. Asserts both tools are advertised
//! with read-only annotations and that `convert_html` exposes its typed config
//! schema. The protocol mechanics themselves are rmcp's responsibility — this is
//! one happy-path wiring check.

#![cfg(feature = "mcp")]

use assert_cmd::Command;
use std::time::Duration;

#[test]
fn test_mcp_stdio_lists_both_tools_with_annotations() {
    // Newline-delimited JSON-RPC frames. Closing stdin (EOF) ends the session.
    let frames = concat!(
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"smoke","version":"0"}}}"#,
        "\n",
        r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
        "\n",
        r#"{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}"#,
        "\n",
    );

    let output = Command::new(env!("CARGO_BIN_EXE_html-to-markdown"))
        .args(["mcp", "--transport", "stdio"])
        .write_stdin(frames)
        .timeout(Duration::from_secs(15))
        .assert()
        .get_output()
        .clone();

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Both tools advertised.
    assert!(
        stdout.contains("convert_html"),
        "tools/list must include convert_html; got: {stdout}"
    );
    assert!(
        stdout.contains("extract_metadata"),
        "tools/list must include extract_metadata; got: {stdout}"
    );
    // Annotations present (MCP wire form is camelCase).
    assert!(
        stdout.contains("readOnlyHint"),
        "tools must carry annotations; got: {stdout}"
    );
    // Typed config schema is discoverable.
    assert!(
        stdout.contains("heading_style"),
        "convert_html input schema must expose typed config; got: {stdout}"
    );
}
