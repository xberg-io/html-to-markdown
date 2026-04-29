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
  def self.convert(html, options = {}, visitor = nil)
    # The Rust FFI expects options as a JSON string; serialise the hash here
    # rather than constructing a ConversionOptions object, which the generated
    # FFI layer cannot coerce back to String (see issue #334).
    opts_json = options.nil? || options.empty? ? nil : options.to_json
    result = HtmlToMarkdownRs.convert(html, opts_json, visitor)
    result.content || ''
  end
end
