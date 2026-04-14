# frozen_string_literal: true

require_relative 'lib/html_to_markdown/version'

repo_root = File.expand_path('../..', __dir__)
crate_prefix = 'packages/ruby/'
git_cmd = %(git -C "#{repo_root}" ls-files -z #{crate_prefix})
ruby_files =
  `#{git_cmd}`.split("\x0")
              .select { |path| path.start_with?(crate_prefix) }
              .map { |path| path.delete_prefix(crate_prefix) }

fallback_files = Dir.chdir(__dir__) do
  Dir.glob(
    %w[
      README.md
      ext/**/*
      exe/*
      lib/**/*.rb
      lib/bin/*
      src/**/*.rs
      spec/**/*.rb
      sig/**/*.rbs
    ]
  )
end

# Vendor files: include vendored crates and workspace Cargo.toml
vendor_files = Dir.chdir(__dir__) do
  Dir.glob('vendor/html-to-markdown-rs/**/*', File::FNM_DOTMATCH)
     .select { |f| File.file?(f) }
     .grep_v(%r{/target/})
     .grep_v(/\.(swp|bak|tmp)$/)
end

# Include vendor/Cargo.toml (workspace definition) if it exists
workspace_toml = if File.exist?(File.join(__dir__, 'vendor/Cargo.toml'))
                   ['vendor/Cargo.toml']
                 else
                   []
                 end

# When vendor exists, use ext/ files from filesystem (modified by vendor script)
# instead of git (which has the unmodified Cargo.toml with workspace paths)
ext_files_from_fs = Dir.chdir(__dir__) do
  Dir.glob('ext/**/*', File::FNM_DOTMATCH)
     .reject { |f| File.directory?(f) }
     .reject { |f| f.include?('/target/') }
end

# Include native artifacts (.so, .bundle, .dylib) if present (for platform gems)
native_files = Dir.chdir(__dir__) do
  Dir.glob('lib/**/*.{so,bundle,dylib}')
end

files = if vendor_files.any?
          # Vendor exists: use ext/ from filesystem (has modified Cargo.toml)
          non_ext_ruby_files = (ruby_files.empty? ? fallback_files : ruby_files)
                               .reject { |f| f.start_with?('ext/') }
          non_ext_ruby_files + ext_files_from_fs + vendor_files + workspace_toml + native_files
        else
          ruby_files.empty? ? fallback_files : ruby_files
        end

files = files.uniq

Gem::Specification.new do |spec|
  spec.name          = 'html-to-markdown'
  spec.version       = HtmlToMarkdown::VERSION
  spec.authors       = ["Na'aman Hirschfeld"]
  spec.email         = ['nhirschfeld@gmail.com']

  spec.summary       = 'Blazing-fast HTML to Markdown conversion for Ruby, powered by Rust.'
  spec.description   = <<~DESC.strip
    html-to-markdown is a native Ruby extension built on the shared Rust engine that powers the html-to-markdown project.
    It delivers identical HTML-to-Markdown output across languages, exposes inline image extraction, and ships with a CLI for automation workflows.
  DESC
  spec.homepage      = 'https://github.com/kreuzberg-dev/html-to-markdown'
  spec.license       = 'MIT'

  spec.required_ruby_version = Gem::Requirement.new('>= 3.2')

  spec.bindir = 'exe'
  spec.executables = ['html-to-markdown']
  spec.require_paths = ['lib']

  spec.files = files
  spec.extra_rdoc_files = ['README.md']

  spec.extensions = ['ext/html_to_markdown_rb/extconf.rb']

  spec.add_dependency 'rb_sys', '>= 0.9', '< 1.0'
  spec.metadata['rubygems_mfa_required'] = 'true'
  spec.metadata['homepage_uri'] = 'https://github.com/kreuzberg-dev/html-to-markdown'
  spec.metadata['source_code_uri'] = 'https://github.com/kreuzberg-dev/html-to-markdown'
  spec.metadata['bug_tracker_uri'] = 'https://github.com/kreuzberg-dev/html-to-markdown/issues'
  spec.metadata['changelog_uri'] = 'https://github.com/kreuzberg-dev/html-to-markdown/releases'
  spec.metadata['documentation_uri'] = 'https://github.com/kreuzberg-dev/html-to-markdown/blob/main/packages/ruby/README.md'
end
