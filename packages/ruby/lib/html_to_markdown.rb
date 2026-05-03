# frozen_string_literal: true

require_relative 'html_to_markdown/version'
require 'html_to_markdown_rb'
require 'json'

# High-performance HTML to Markdown conversion.
#
# @example Simple conversion
#   HtmlToMarkdown.convert('<h1>Hello</h1>') # => "# Hello\n\n"
#
# @example With options
#   HtmlToMarkdown.convert('<h1>Hello</h1>', heading_style: 'atx')
module HtmlToMarkdown
  # Convert HTML to Markdown.
  #
  # @param html [String] The HTML content to convert.
  # @param options [Hash] Optional conversion options.
  #   Supported keys (all optional):
  #   - :heading_style       - 'atx', 'atx_closed', 'setext', 'underlined'
  #   - :code_block_style    - 'backticks', 'tildes', 'indented'
  #   - :escape_asterisks    - Boolean
  #   - :escape_underscores  - Boolean
  #   - :escape_misc         - Boolean
  #   - :escape_ascii        - Boolean
  #   - :strip_newlines      - Boolean
  #   - :keep_inline_images_in - Array of tag names
  #   - :strip_tags          - Array of tag names to strip
  #   - :preserve_tags       - Array of tag names to preserve verbatim
  #   (and more, matching ConversionOptions fields)
  # @return [String] The converted Markdown content.
  def self.convert(html, options = {}, _visitor = nil)
    # Pass the options hash directly; the Magnus FFI layer will serialize it
    opts = options.nil? || options.empty? ? nil : options
    # NOTE: visitor support is not yet fully integrated in the Ruby binding
    result = HtmlToMarkdownRs.convert(html, opts)
    result.content || ''
  end

  # NOTE: The wrapper explicitly passes a hash, not a pre-serialized JSON string.
  # The FFI layer calls .to_json() on the hash to serialize nested options.
end
