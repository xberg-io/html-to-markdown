# frozen_string_literal: true

Gem::Specification.new do |spec|
  spec.name = 'html-to-markdown'
  spec.version = '3.4.0.pre.rc.9'
  spec.authors       = ['Kreuzberg Team']
  spec.summary       = 'High-performance HTML to Markdown converter'
  spec.description   = 'High-performance HTML to Markdown converter'
  spec.homepage      = 'https://github.com/kreuzberg-dev/html-to-markdown'
  spec.license       = 'MIT'
  spec.required_ruby_version = '>= 3.2.0'
  spec.metadata['keywords'] = %w[html markdown converter].join(',')
  spec.metadata['rubygems_mfa_required'] = 'true'

  spec.files         = Dir.glob(%w[lib/**/* ext/**/* sig/**/* Steepfile])
  spec.require_paths = ['lib']
  spec.extensions    = ['ext/html_to_markdown_rb/extconf.rb']

  spec.add_dependency 'rb_sys', '~> 0.9'
end
