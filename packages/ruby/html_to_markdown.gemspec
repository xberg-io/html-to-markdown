# frozen_string_literal: true

Gem::Specification.new do |spec|
  spec.name = "html-to-markdown"
  spec.version = "3.6.2"
  spec.authors       = ["Na'aman Hirschfeld <naaman@kreuzberg.dev>"]
  spec.summary       = "High-performance HTML to Markdown converter"
  spec.description   = "High-performance HTML to Markdown converter"
  spec.homepage      = "https://github.com/kreuzberg-dev/html-to-markdown"

  spec.license       = "MIT"

  spec.required_ruby_version = [">= 3.2.0", "< 4.0"]
  spec.metadata["keywords"] = %w[converter html markdown].join(",")
  spec.metadata["rubygems_mfa_required"] = "true"

  candidate_files    = Dir.glob(%w[README* LICENSE* lib/**/* ext/**/* sig/**/* Steepfile]).select { |f| File.file?(f) }
  spec.files         = candidate_files.reject { |f| f.include?("/native/target/") || f.include?("/native/tmp/") }
  spec.require_paths = ["lib"]
  spec.extensions    = ["ext/html_to_markdown_rb/native/extconf.rb"]

  spec.add_dependency "rb_sys", ">= 0.9", "< 0.9.128"
  spec.add_dependency "sorbet-runtime", "~> 0.5"
end
