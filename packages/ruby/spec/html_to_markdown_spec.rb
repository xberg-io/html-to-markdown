# frozen_string_literal: true

require 'spec_helper'

RSpec.describe HtmlToMarkdown do
  describe '.convert' do
    it 'converts html without options' do
      result = described_class.convert('<h1>Hello</h1>')
      expect(result).to be_a(String)
      expect(result).to include('Hello')
    end

    it 'converts html with an empty options hash' do
      result = described_class.convert('<h1>Hello</h1>', {})
      expect(result).to be_a(String)
      expect(result).to include('Hello')
    end

    # Regression test for issue #334: alef regeneration reverts Rust FFI options fix.
    # Passing any options previously raised TypeError because the Ruby wrapper was
    # constructing a ConversionOptions object, which the Rust FFI (expecting a JSON
    # string) cannot coerce — resulting in a TypeError on every call with options.
    it 'does not raise TypeError when options are passed (regression #334)' do
      expect { described_class.convert('<h1>Hello</h1>', heading_style: 'atx') }.not_to raise_error
    end

    it 'applies heading_style option' do
      result = described_class.convert('<h1>Hello</h1>', heading_style: 'atx')
      expect(result).to match(/^# Hello/)
    end

    it 'applies escape_asterisks option' do
      result = described_class.convert('<p>1 * 2</p>', escape_asterisks: true)
      expect(result).to include('\*')
    end

    it 'applies strip_tags option' do
      result = described_class.convert('<p>Hello</p><script>alert(1)</script>', strip_tags: ['script'])
      expect(result).not_to include('alert')
    end

    it 'returns a String' do
      expect(described_class.convert('<p>test</p>')).to be_a(String)
      expect(described_class.convert('<p>test</p>', strip_newlines: true)).to be_a(String)
    end
  end
end
