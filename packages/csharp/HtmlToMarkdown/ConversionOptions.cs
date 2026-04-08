using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace HtmlToMarkdown;

/// <summary>
/// Configuration options for HTML to Markdown conversion.
///
/// This class groups all conversion-related options together, providing fine-grained
/// control over the conversion process and output formatting.
/// </summary>
public class ConversionOptions
{
    /// <summary>
    /// Heading style for Markdown output.
    ///
    /// Controls how headings (h1-h6) are rendered. Valid values: "underlined", "atx", "atx_closed".
    /// Default: "atx"
    /// </summary>
    [JsonPropertyName("headingStyle")]
    public string HeadingStyle { get; set; } = "atx";

    /// <summary>
    /// List indentation type.
    ///
    /// Controls whether list items are indented with spaces or tabs.
    /// Valid values: "spaces", "tabs".
    /// Default: "spaces"
    /// </summary>
    [JsonPropertyName("listIndentType")]
    public string ListIndentType { get; set; } = "spaces";

    /// <summary>
    /// List indentation width in spaces.
    ///
    /// Number of spaces for list indentation (applied if using spaces indentation).
    /// Ignored if list_indent_type='tabs'.
    /// Default: 2
    /// </summary>
    [JsonPropertyName("listIndentWidth")]
    public int ListIndentWidth { get; set; } = 2;

    /// <summary>
    /// Bullet characters for unordered lists.
    ///
    /// Characters to use for unordered list bullets (e.g., "-", "*", "+").
    /// Cycles through characters for nested levels.
    /// Default: "-"
    /// </summary>
    [JsonPropertyName("bullets")]
    public string Bullets { get; set; } = "-";

    /// <summary>
    /// Symbol for strong/emphasis emphasis rendering.
    ///
    /// Character to use for emphasis (* or _).
    /// Default: "*"
    /// </summary>
    [JsonPropertyName("strongEmSymbol")]
    public char StrongEmSymbol { get; set; } = '*';

    /// <summary>
    /// Escape asterisks (*) in text to prevent accidental formatting.
    ///
    /// Default: false
    /// </summary>
    [JsonPropertyName("escapeAsterisks")]
    public bool EscapeAsterisks { get; set; } = false;

    /// <summary>
    /// Escape underscores (_) in text to prevent accidental formatting.
    ///
    /// Default: false
    /// </summary>
    [JsonPropertyName("escapeUnderscores")]
    public bool EscapeUnderscores { get; set; } = false;

    /// <summary>
    /// Escape miscellaneous markdown characters (\ & < ` [ > ~ # = + | -).
    ///
    /// Default: false
    /// </summary>
    [JsonPropertyName("escapeMisc")]
    public bool EscapeMisc { get; set; } = false;

    /// <summary>
    /// Escape all ASCII punctuation characters.
    ///
    /// For CommonMark spec compliance tests only.
    /// Default: false
    /// </summary>
    [JsonPropertyName("escapeAscii")]
    public bool EscapeAscii { get; set; } = false;

    /// <summary>
    /// Default code language for fenced code blocks when not specified.
    ///
    /// Default: ""
    /// </summary>
    [JsonPropertyName("codeLanguage")]
    public string CodeLanguage { get; set; } = "";

    /// <summary>
    /// Use autolinks syntax for bare URLs (&lt;http://example.com&gt;).
    ///
    /// Default: true
    /// </summary>
    [JsonPropertyName("autolinks")]
    public bool Autolinks { get; set; } = true;

    /// <summary>
    /// Add default title element to HTML if none exists before conversion.
    ///
    /// Default: false
    /// </summary>
    [JsonPropertyName("defaultTitle")]
    public bool DefaultTitle { get; set; } = false;

    /// <summary>
    /// Use HTML &lt;br&gt; elements in tables instead of spaces for line breaks.
    ///
    /// Default: false
    /// </summary>
    [JsonPropertyName("brInTables")]
    public bool BrInTables { get; set; } = false;

    /// <summary>
    /// Highlight style for &lt;mark&gt; elements.
    ///
    /// Controls how highlighted text is rendered. Valid values: "double_equal", "html", "bold", "none".
    /// Default: "double_equal"
    /// </summary>
    [JsonPropertyName("highlightStyle")]
    public string HighlightStyle { get; set; } = "double_equal";

    /// <summary>
    /// Extract metadata from HTML during conversion.
    ///
    /// Extracts title, description, images, links, etc.
    /// Default: true
    /// </summary>
    [JsonPropertyName("extractMetadata")]
    public bool ExtractMetadata { get; set; } = true;

    /// <summary>
    /// Whitespace handling mode during conversion.
    ///
    /// Valid values: "normalized" (collapse whitespace), "strict" (preserve whitespace).
    /// Default: "normalized"
    /// </summary>
    [JsonPropertyName("whitespaceMode")]
    public string WhitespaceMode { get; set; } = "normalized";

    /// <summary>
    /// Strip newline characters from HTML before processing.
    ///
    /// Default: false
    /// </summary>
    [JsonPropertyName("stripNewlines")]
    public bool StripNewlines { get; set; } = false;

    /// <summary>
    /// Enable automatic text wrapping.
    ///
    /// Default: false
    /// </summary>
    [JsonPropertyName("wrap")]
    public bool Wrap { get; set; } = false;

    /// <summary>
    /// Text wrapping width in characters.
    ///
    /// Applied if wrap=true.
    /// Default: 80
    /// </summary>
    [JsonPropertyName("wrapWidth")]
    public int WrapWidth { get; set; } = 80;

    /// <summary>
    /// Treat block-level elements as inline during conversion.
    ///
    /// Default: false
    /// </summary>
    [JsonPropertyName("convertAsInline")]
    public bool ConvertAsInline { get; set; } = false;

    /// <summary>
    /// Custom symbol for subscript content.
    ///
    /// Default: ""
    /// </summary>
    [JsonPropertyName("subSymbol")]
    public string SubSymbol { get; set; } = "";

    /// <summary>
    /// Custom symbol for superscript content.
    ///
    /// Default: ""
    /// </summary>
    [JsonPropertyName("supSymbol")]
    public string SupSymbol { get; set; } = "";

    /// <summary>
    /// Newline style in markdown output.
    ///
    /// Valid values: "spaces" (two trailing spaces), "backslash" (backslash at end).
    /// Default: "spaces"
    /// </summary>
    [JsonPropertyName("newlineStyle")]
    public string NewlineStyle { get; set; } = "spaces";

    /// <summary>
    /// Code block fence style.
    ///
    /// Valid values: "indented" (4 spaces), "backticks" (```), "tildes" (~~~).
    /// Default: "indented"
    /// </summary>
    [JsonPropertyName("codeBlockStyle")]
    public string CodeBlockStyle { get; set; } = "indented";

    /// <summary>
    /// HTML elements where images should remain as markdown links.
    ///
    /// Images in these elements won't be converted to alt text.
    /// Default: empty list
    /// </summary>
    [JsonPropertyName("keepInlineImagesIn")]
    public List<string> KeepInlineImagesIn { get; set; } = new List<string>();

    /// <summary>
    /// Source document encoding.
    ///
    /// Informational; typically "utf-8".
    /// Default: "utf-8"
    /// </summary>
    [JsonPropertyName("encoding")]
    public string Encoding { get; set; } = "utf-8";

    /// <summary>
    /// Enable debug mode with diagnostic warnings.
    ///
    /// Provides diagnostic warnings on conversion issues.
    /// Default: false
    /// </summary>
    [JsonPropertyName("debug")]
    public bool Debug { get; set; } = false;

    /// <summary>
    /// HTML tags to strip.
    ///
    /// Extract text content without markdown conversion for specified tags.
    /// Default: empty list
    /// </summary>
    [JsonPropertyName("stripTags")]
    public List<string> StripTags { get; set; } = new List<string>();

    /// <summary>
    /// HTML tags to preserve as-is in output.
    ///
    /// Useful for complex tables and special content.
    /// Default: empty list
    /// </summary>
    [JsonPropertyName("preserveTags")]
    public List<string> PreserveTags { get; set; } = new List<string>();

    /// <summary>
    /// Skip all images during conversion.
    ///
    /// When enabled, all &lt;img&gt; elements are completely omitted from output.
    /// Useful for text-only extraction or filtering out visual content.
    /// Default: false
    /// </summary>
    [JsonPropertyName("skipImages")]
    public bool SkipImages { get; set; } = false;

    /// <summary>
    /// Maximum allowed DOM tree depth.
    ///
    /// When set, the converter returns an error if the HTML DOM tree exceeds
    /// this depth. Useful for preventing excessive recursion on deeply nested input.
    /// Default: null (unlimited)
    /// </summary>
    [JsonPropertyName("maxDepth")]
    public int? MaxDepth { get; set; } = null;

    /// <summary>
    /// Output format for conversion.
    ///
    /// Specifies target markup language: "markdown" (CommonMark compatible) or "djot".
    /// Default: "markdown"
    /// </summary>
    [JsonPropertyName("outputFormat")]
    public string OutputFormat { get; set; } = "markdown";

    /// <summary>
    /// HTML preprocessing options for document cleanup before conversion.
    ///
    /// Default: PreprocessingOptions with defaults
    /// </summary>
    [JsonPropertyName("preprocessing")]
    public PreprocessingOptions Preprocessing { get; set; } = new PreprocessingOptions();
}

/// <summary>
/// HTML preprocessing options for document cleanup before conversion.
/// </summary>
public class PreprocessingOptions
{
    /// <summary>
    /// Enable HTML preprocessing globally.
    ///
    /// Default: false
    /// </summary>
    [JsonPropertyName("enabled")]
    public bool Enabled { get; set; } = false;

    /// <summary>
    /// Preprocessing preset level.
    ///
    /// Valid values: "minimal" (remove scripts/styles), "standard" (remove nav/forms),
    /// "aggressive" (remove extensive non-content).
    /// Default: "standard"
    /// </summary>
    [JsonPropertyName("preset")]
    public string Preset { get; set; } = "standard";

    /// <summary>
    /// Remove navigation elements.
    ///
    /// Removes nav, breadcrumbs, menus, sidebars.
    /// Default: true
    /// </summary>
    [JsonPropertyName("removeNavigation")]
    public bool RemoveNavigation { get; set; } = true;

    /// <summary>
    /// Remove form elements.
    ///
    /// Removes forms, inputs, buttons, etc.
    /// Default: true
    /// </summary>
    [JsonPropertyName("removeForms")]
    public bool RemoveForms { get; set; } = true;
}
