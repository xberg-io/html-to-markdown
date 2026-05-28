# frozen_string_literal: true

Gem::Specification.new do |spec|
  spec.name = "html-to-markdown"
  spec.version = "3.5.4"
  spec.authors       = ["Kreuzberg Team"]
  spec.summary       = "High-performance HTML to Markdown converter"
  spec.description   = "High-performance HTML to Markdown converter"
  spec.homepage      = "https://github.com/kreuzberg-dev/html-to-markdown"
  spec.license       = "MIT"
  spec.required_ruby_version = ">= 3.2.0"
  spec.metadata["keywords"] = %w[converter html markdown].join(",")
  spec.metadata["rubygems_mfa_required"] = "true"

  spec.files         = Dir.glob(%w[README* LICENSE* lib/**/* ext/**/* sig/**/* Steepfile]).reject { |f| f.include?("/native/target/") || f.include?("/native/tmp/") }
  spec.require_paths = ["lib"]
  spec.extensions    = ["ext/html_to_markdown_rb/extconf.rb"]

  spec.add_dependency "rb_sys", "~> 0.9"
  spec.add_dependency "sorbet-runtime", "~> 0.5"
end
