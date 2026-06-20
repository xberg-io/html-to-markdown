//! MCP error mapping for `ConversionError`.

use crate::error::ConversionError;
use rmcp::ErrorData as McpError;

/// Map `ConversionError` variants to MCP `ErrorData` with appropriate codes.
///
/// - `ParseError` → parse_error (-32700)
/// - `ConfigError` / `SanitizationError` / `InvalidInput` → invalid_params (-32602)
/// - `IoError` / `Panic` / `Other` → internal_error (-32603)
/// - `Visitor` (feature-gated) → internal_error (-32603)
pub fn map_conversion_error_to_mcp(error: ConversionError) -> McpError {
    match error {
        ConversionError::ParseError(msg) => McpError::parse_error(format!("HTML parse error: {msg}"), None),
        ConversionError::SanitizationError(msg) => McpError::invalid_params(format!("Sanitization error: {msg}"), None),
        ConversionError::ConfigError(msg) => McpError::invalid_params(format!("Configuration error: {msg}"), None),
        ConversionError::InvalidInput(msg) => McpError::invalid_params(format!("Invalid input: {msg}"), None),
        ConversionError::IoError(msg) => McpError::internal_error(format!("I/O error: {msg}"), None),
        ConversionError::Panic(msg) => McpError::internal_error(format!("Internal panic: {msg}"), None),
        #[cfg(feature = "visitor")]
        ConversionError::Visitor(msg) => McpError::internal_error(format!("Visitor error: {msg}"), None),
        ConversionError::Other(msg) => McpError::internal_error(msg, None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_error_maps_to_parse_error_code() {
        let err = ConversionError::ParseError("bad html".into());
        let mcp = map_conversion_error_to_mcp(err);
        assert_eq!(mcp.code.0, -32700);
        assert!(mcp.message.contains("parse error") || mcp.message.contains("HTML"));
    }

    #[test]
    fn test_sanitization_error_maps_to_invalid_params() {
        let err = ConversionError::SanitizationError("unsafe content".into());
        let mcp = map_conversion_error_to_mcp(err);
        assert_eq!(mcp.code.0, -32602);
    }

    #[test]
    fn test_config_error_maps_to_invalid_params() {
        let err = ConversionError::ConfigError("bad option".into());
        let mcp = map_conversion_error_to_mcp(err);
        assert_eq!(mcp.code.0, -32602);
    }

    #[test]
    fn test_invalid_input_maps_to_invalid_params() {
        let err = ConversionError::InvalidInput("binary data".into());
        let mcp = map_conversion_error_to_mcp(err);
        assert_eq!(mcp.code.0, -32602);
    }

    #[test]
    fn test_io_error_maps_to_internal_error() {
        let err = ConversionError::IoError("file not found".into());
        let mcp = map_conversion_error_to_mcp(err);
        assert_eq!(mcp.code.0, -32603);
    }

    #[test]
    fn test_panic_maps_to_internal_error() {
        let err = ConversionError::Panic("unexpected panic".into());
        let mcp = map_conversion_error_to_mcp(err);
        assert_eq!(mcp.code.0, -32603);
    }

    #[test]
    fn test_other_maps_to_internal_error() {
        let err = ConversionError::Other("something else".into());
        let mcp = map_conversion_error_to_mcp(err);
        assert_eq!(mcp.code.0, -32603);
    }

    #[test]
    fn test_error_type_differentiation() {
        let parse = ConversionError::ParseError("x".into());
        let config = ConversionError::ConfigError("x".into());
        let io = ConversionError::IoError("x".into());

        let p = map_conversion_error_to_mcp(parse);
        let c = map_conversion_error_to_mcp(config);
        let i = map_conversion_error_to_mcp(io);

        assert_ne!(p.code.0, c.code.0);
        assert_ne!(p.code.0, i.code.0);
        assert_ne!(c.code.0, i.code.0);
    }
}
