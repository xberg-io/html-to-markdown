package dev.kreuzberg.htmltomarkdown;

import java.util.Collections;
import java.util.HashSet;
import java.util.Objects;
import java.util.Set;

/**
 * Configuration options for HTML to Markdown conversion.
 *
 * <p>This class groups all conversion-related options together for customizing the behavior of the
 * HTML to Markdown conversion process. Use the builder pattern to configure options.
 *
 * <p><b>Example usage:</b>
 *
 * <pre>{@code
 * ConversionOptions options = new ConversionOptions()
 *     .setHeadingStyle("atx")
 *     .setOutputFormat(OutputFormat.DJOT)
 *     .setListIndentWidth(2)
 *     .setEscapeAsterisks(true);
 * // Future: String markdown = HtmlToMarkdown.convert(html, options);
 * }</pre>
 *
 * <p><b>Default behavior:</b> When using {@link HtmlToMarkdown#convert(String)}, sensible
 * CommonMark-compliant defaults are applied automatically.
 *
 * @since 2.20.0
 */
public final class ConversionOptions {

  private String headingStyle = "atx";
  private String listIndentType = "spaces";
  private int listIndentWidth = 2;
  private String bullets = "-*+";
  private String strongEmSymbol = "*";
  private boolean escapeAsterisks = false;
  private boolean escapeUnderscores = false;
  private boolean escapeMisc = false;
  private boolean escapeAscii = false;
  private String codeLanguage = "";
  private String encoding = "utf-8";
  private boolean autolinks = true;
  private boolean defaultTitle = false;
  private Set<String> keepInlineImagesIn = Collections.emptySet();
  private boolean brInTables = false;
  private String highlightStyle = "double-equal";
  private boolean extractMetadata = true;
  private String whitespaceMode = "normalized";
  private boolean stripNewlines = false;
  private boolean wrap = false;
  private int wrapWidth = 80;
  private Set<String> stripTags = Collections.emptySet();
  private Set<String> preserveTags = Collections.emptySet();
  private boolean skipImages = false;
  private boolean convertAsInline = false;
  private String subSymbol = "";
  private String supSymbol = "";
  private String newlineStyle = "spaces";
  private String codeBlockStyle = "backticks";
  private OutputFormat outputFormat = OutputFormat.MARKDOWN;
  private Integer maxDepth = null;
  private boolean debug = false;

  /**
   * Create a new ConversionOptions with default settings.
   *
   * <p>All options are initialized to their default values, which correspond to CommonMark
   * compliance.
   */
  public ConversionOptions() {
    // Default values already set above
  }

  // Heading Style
  /**
   * Set the heading style.
   *
   * @param style the style: "atx" (default), "underlined", or "atx_closed"
   * @return this options object for method chaining
   */
  public ConversionOptions setHeadingStyle(final String style) {
    Objects.requireNonNull(style, "Heading style cannot be null");
    this.headingStyle = style;
    return this;
  }

  /** Get the heading style. */
  public String getHeadingStyle() {
    return headingStyle;
  }

  // List Indent Type
  /**
   * Set the list indentation type.
   *
   * @param type the type: "spaces" (default) or "tabs"
   * @return this options object for method chaining
   */
  public ConversionOptions setListIndentType(final String type) {
    Objects.requireNonNull(type, "List indent type cannot be null");
    this.listIndentType = type;
    return this;
  }

  /** Get the list indentation type. */
  public String getListIndentType() {
    return listIndentType;
  }

  // List Indent Width
  /**
   * Set the list indentation width in spaces.
   *
   * @param width the number of spaces (default 2)
   * @return this options object for method chaining
   */
  public ConversionOptions setListIndentWidth(final int width) {
    if (width < 0) {
      throw new IllegalArgumentException("List indent width cannot be negative");
    }
    this.listIndentWidth = width;
    return this;
  }

  /** Get the list indentation width. */
  public int getListIndentWidth() {
    return listIndentWidth;
  }

  // Bullets
  /**
   * Set the bullet characters for unordered lists.
   *
   * @param chars the characters to cycle through ("-*+" by default, CommonMark compliant)
   * @return this options object for method chaining
   */
  public ConversionOptions setBullets(final String chars) {
    Objects.requireNonNull(chars, "Bullets cannot be null");
    this.bullets = chars;
    return this;
  }

  /** Get the bullet characters. */
  public String getBullets() {
    return bullets;
  }

  // Strong/Emphasis Symbol
  /**
   * Set the symbol for strong/emphasis formatting.
   *
   * @param symbol "*" (default) or "_"
   * @return this options object for method chaining
   */
  public ConversionOptions setStrongEmSymbol(final String symbol) {
    Objects.requireNonNull(symbol, "Strong/emphasis symbol cannot be null");
    this.strongEmSymbol = symbol;
    return this;
  }

  /** Get the strong/emphasis symbol. */
  public String getStrongEmSymbol() {
    return strongEmSymbol;
  }

  // Escape Asterisks
  /**
   * Set whether to escape asterisk characters.
   *
   * @param escape true to escape, false for minimal escaping (default)
   * @return this options object for method chaining
   */
  public ConversionOptions setEscapeAsterisks(final boolean escape) {
    this.escapeAsterisks = escape;
    return this;
  }

  /** Check if asterisks are escaped. */
  public boolean isEscapeAsterisks() {
    return escapeAsterisks;
  }

  // Escape Underscores
  /**
   * Set whether to escape underscore characters.
   *
   * @param escape true to escape, false for minimal escaping (default)
   * @return this options object for method chaining
   */
  public ConversionOptions setEscapeUnderscores(final boolean escape) {
    this.escapeUnderscores = escape;
    return this;
  }

  /** Check if underscores are escaped. */
  public boolean isEscapeUnderscores() {
    return escapeUnderscores;
  }

  // Escape Misc
  /**
   * Set whether to escape miscellaneous Markdown characters.
   *
   * @param escape true to escape, false for minimal escaping (default)
   * @return this options object for method chaining
   */
  public ConversionOptions setEscapeMisc(final boolean escape) {
    this.escapeMisc = escape;
    return this;
  }

  /** Check if miscellaneous characters are escaped. */
  public boolean isEscapeMisc() {
    return escapeMisc;
  }

  // Escape ASCII
  /**
   * Set whether to escape all ASCII punctuation.
   *
   * @param escape true to escape (for CommonMark spec compliance), false by default
   * @return this options object for method chaining
   */
  public ConversionOptions setEscapeAscii(final boolean escape) {
    this.escapeAscii = escape;
    return this;
  }

  /** Check if ASCII punctuation is escaped. */
  public boolean isEscapeAscii() {
    return escapeAscii;
  }

  // Code Language
  /**
   * Set the default language for code blocks.
   *
   * @param language the language identifier (e.g., "java", "python"), empty string by default
   * @return this options object for method chaining
   */
  public ConversionOptions setCodeLanguage(final String language) {
    Objects.requireNonNull(language, "Code language cannot be null");
    this.codeLanguage = language;
    return this;
  }

  /** Get the code language. */
  public String getCodeLanguage() {
    return codeLanguage;
  }

  // Encoding
  /**
   * Set the character encoding for HTML input.
   *
   * @param enc the encoding (e.g., "utf-8" by default)
   * @return this options object for method chaining
   */
  public ConversionOptions setEncoding(final String enc) {
    Objects.requireNonNull(enc, "Encoding cannot be null");
    this.encoding = enc;
    return this;
  }

  /** Get the encoding. */
  public String getEncoding() {
    return encoding;
  }

  // Autolinks
  /**
   * Set whether to convert bare URLs to automatic links.
   *
   * @param enable true to enable (default), false to disable
   * @return this options object for method chaining
   */
  public ConversionOptions setAutolinks(final boolean enable) {
    this.autolinks = enable;
    return this;
  }

  /** Check if autolinks are enabled. */
  public boolean isAutolinks() {
    return autolinks;
  }

  // Default Title
  /**
   * Set whether to add a default title if none exists.
   *
   * @param enable true to add default title, false by default
   * @return this options object for method chaining
   */
  public ConversionOptions setDefaultTitle(final boolean enable) {
    this.defaultTitle = enable;
    return this;
  }

  /** Check if default title is enabled. */
  public boolean isDefaultTitle() {
    return defaultTitle;
  }

  // Keep Inline Images In
  /**
   * Set parent tag names where images should remain inline.
   *
   * @param tagNames the set of tag names, empty by default
   * @return this options object for method chaining
   */
  public ConversionOptions setKeepInlineImagesIn(final Set<String> tagNames) {
    this.keepInlineImagesIn =
        tagNames != null
            ? Collections.unmodifiableSet(new HashSet<>(tagNames))
            : Collections.emptySet();
    return this;
  }

  /** Get the tag names where images remain inline. */
  public Set<String> getKeepInlineImagesIn() {
    return keepInlineImagesIn;
  }

  // BR in Tables
  /**
   * Set whether to use &lt;br&gt; tags for line breaks in table cells.
   *
   * @param enable true to use &lt;br&gt;, false for spaces (default)
   * @return this options object for method chaining
   */
  public ConversionOptions setBrInTables(final boolean enable) {
    this.brInTables = enable;
    return this;
  }

  /** Check if &lt;br&gt; is used in tables. */
  public boolean isBrInTables() {
    return brInTables;
  }

  // Highlight Style
  /**
   * Set the style for highlighting &lt;mark&gt; elements.
   *
   * @param style the style: "double-equal" (default), "html", or "bold"
   * @return this options object for method chaining
   */
  public ConversionOptions setHighlightStyle(final String style) {
    Objects.requireNonNull(style, "Highlight style cannot be null");
    this.highlightStyle = style;
    return this;
  }

  /** Get the highlight style. */
  public String getHighlightStyle() {
    return highlightStyle;
  }

  // Extract Metadata
  /**
   * Set whether to extract metadata from HTML head.
   *
   * @param enable true to extract (default), false to skip
   * @return this options object for method chaining
   */
  public ConversionOptions setExtractMetadata(final boolean enable) {
    this.extractMetadata = enable;
    return this;
  }

  /** Check if metadata extraction is enabled. */
  public boolean isExtractMetadata() {
    return extractMetadata;
  }

  // Whitespace Mode
  /**
   * Set how to handle whitespace.
   *
   * @param mode the mode: "normalized" (default, collapses whitespace) or "strict"
   * @return this options object for method chaining
   */
  public ConversionOptions setWhitespaceMode(final String mode) {
    Objects.requireNonNull(mode, "Whitespace mode cannot be null");
    this.whitespaceMode = mode;
    return this;
  }

  /** Get the whitespace mode. */
  public String getWhitespaceMode() {
    return whitespaceMode;
  }

  // Strip Newlines
  /**
   * Set whether to remove newlines from HTML before processing.
   *
   * @param enable true to strip, false by default
   * @return this options object for method chaining
   */
  public ConversionOptions setStripNewlines(final boolean enable) {
    this.stripNewlines = enable;
    return this;
  }

  /** Check if newline stripping is enabled. */
  public boolean isStripNewlines() {
    return stripNewlines;
  }

  // Wrap
  /**
   * Set whether to enable text wrapping.
   *
   * @param enable true to enable, false by default
   * @return this options object for method chaining
   */
  public ConversionOptions setWrap(final boolean enable) {
    this.wrap = enable;
    return this;
  }

  /** Check if text wrapping is enabled. */
  public boolean isWrap() {
    return wrap;
  }

  // Wrap Width
  /**
   * Set the column width for text wrapping.
   *
   * @param width the width in columns (default 80)
   * @return this options object for method chaining
   */
  public ConversionOptions setWrapWidth(final int width) {
    if (width <= 0) {
      throw new IllegalArgumentException("Wrap width must be positive");
    }
    this.wrapWidth = width;
    return this;
  }

  /** Get the wrap width. */
  public int getWrapWidth() {
    return wrapWidth;
  }

  // Strip Tags
  /**
   * Set HTML tags to strip from output (output only text content, no markdown conversion).
   *
   * @param tags the set of tag names, empty by default
   * @return this options object for method chaining
   */
  public ConversionOptions setStripTags(final Set<String> tags) {
    this.stripTags =
        tags != null ? Collections.unmodifiableSet(new HashSet<>(tags)) : Collections.emptySet();
    return this;
  }

  /** Get the tags to strip. */
  public Set<String> getStripTags() {
    return stripTags;
  }

  // Preserve Tags
  /**
   * Set HTML tags to preserve as-is in the output (keep original HTML).
   *
   * @param tags the set of tag names, empty by default
   * @return this options object for method chaining
   */
  public ConversionOptions setPreserveTags(final Set<String> tags) {
    this.preserveTags =
        tags != null ? Collections.unmodifiableSet(new HashSet<>(tags)) : Collections.emptySet();
    return this;
  }

  /** Get the tags to preserve. */
  public Set<String> getPreserveTags() {
    return preserveTags;
  }

  // Skip Images
  /**
   * Set whether to skip all images during conversion.
   *
   * <p>When enabled, all &lt;img&gt; elements are completely omitted from output. Useful for
   * text-only extraction or filtering out visual content.
   *
   * @param skip true to skip images, false by default
   * @return this options object for method chaining
   */
  public ConversionOptions setSkipImages(final boolean skip) {
    this.skipImages = skip;
    return this;
  }

  /** Check if image skipping is enabled. */
  public boolean isSkipImages() {
    return skipImages;
  }

  // Convert As Inline
  /**
   * Set whether to treat block elements as inline during conversion.
   *
   * @param enable true to convert as inline, false by default
   * @return this options object for method chaining
   */
  public ConversionOptions setConvertAsInline(final boolean enable) {
    this.convertAsInline = enable;
    return this;
  }

  /** Check if conversion as inline is enabled. */
  public boolean isConvertAsInline() {
    return convertAsInline;
  }

  // Sub Symbol
  /**
   * Set the symbol for subscript text.
   *
   * @param symbol the symbol (empty string by default)
   * @return this options object for method chaining
   */
  public ConversionOptions setSubSymbol(final String symbol) {
    Objects.requireNonNull(symbol, "Sub symbol cannot be null");
    this.subSymbol = symbol;
    return this;
  }

  /** Get the subscript symbol. */
  public String getSubSymbol() {
    return subSymbol;
  }

  // Sup Symbol
  /**
   * Set the symbol for superscript text.
   *
   * @param symbol the symbol (empty string by default)
   * @return this options object for method chaining
   */
  public ConversionOptions setSupSymbol(final String symbol) {
    Objects.requireNonNull(symbol, "Sup symbol cannot be null");
    this.supSymbol = symbol;
    return this;
  }

  /** Get the superscript symbol. */
  public String getSupSymbol() {
    return supSymbol;
  }

  // Newline Style
  /**
   * Set the style for newlines in Markdown.
   *
   * @param style the style: "spaces" (two trailing spaces, CommonMark default) or "backslash" (\\)
   * @return this options object for method chaining
   */
  public ConversionOptions setNewlineStyle(final String style) {
    Objects.requireNonNull(style, "Newline style cannot be null");
    this.newlineStyle = style;
    return this;
  }

  /** Get the newline style. */
  public String getNewlineStyle() {
    return newlineStyle;
  }

  // Code Block Style
  /**
   * Set the style for code blocks in Markdown.
   *
   * @param style the style: "backticks" (default, better whitespace preservation), "indented" (4
   *     spaces), or "tildes" (~~~)
   * @return this options object for method chaining
   */
  public ConversionOptions setCodeBlockStyle(final String style) {
    Objects.requireNonNull(style, "Code block style cannot be null");
    this.codeBlockStyle = style;
    return this;
  }

  /** Get the code block style. */
  public String getCodeBlockStyle() {
    return codeBlockStyle;
  }

  // Output Format
  /**
   * Set the output format for conversion.
   *
   * @param format the format: {@link OutputFormat#MARKDOWN} (default) or {@link OutputFormat#DJOT}
   * @return this options object for method chaining
   */
  public ConversionOptions setOutputFormat(final OutputFormat format) {
    Objects.requireNonNull(format, "Output format cannot be null");
    this.outputFormat = format;
    return this;
  }

  /** Get the output format. */
  public OutputFormat getOutputFormat() {
    return outputFormat;
  }

  // Max Depth
  /**
   * Set the maximum allowed DOM tree depth.
   *
   * <p>When set, the converter returns an error if the HTML DOM tree exceeds this depth. Useful for
   * preventing excessive recursion on deeply nested input.
   *
   * @param depth the maximum depth, or {@code null} for unlimited (default)
   * @return this options object for method chaining
   */
  public ConversionOptions setMaxDepth(final Integer depth) {
    if (depth != null && depth <= 0) {
      throw new IllegalArgumentException("Max depth must be positive");
    }
    this.maxDepth = depth;
    return this;
  }

  /** Get the maximum DOM tree depth, or {@code null} if unlimited. */
  public Integer getMaxDepth() {
    return maxDepth;
  }

  // Debug
  /**
   * Set whether to enable debug mode.
   *
   * <p>When enabled, produces diagnostic warnings about unhandled elements and hOCR processing.
   *
   * @param enable true to enable debug mode, false by default
   * @return this options object for method chaining
   */
  public ConversionOptions setDebug(final boolean enable) {
    this.debug = enable;
    return this;
  }

  /** Check if debug mode is enabled. */
  public boolean isDebug() {
    return debug;
  }

  @Override
  public String toString() {
    return "ConversionOptions{"
        + "headingStyle='"
        + headingStyle
        + '\''
        + ", outputFormat="
        + outputFormat
        + ", listIndentWidth="
        + listIndentWidth
        + ", skipImages="
        + skipImages
        + ", wrapWidth="
        + wrapWidth
        + '}';
  }
}
