defmodule Html_to_markdown.MixProject do
  use Mix.Project

  @version "3.2.6"
  @source_url "https://github.com/kreuzberg-dev/html-to-markdown"

  def project do
    [
      app: :html_to_markdown,
      version: @version,
      elixir: "~> 1.14",
      description: "High-performance HTML to Markdown converter",
      package: package(),
      deps: deps(),
      source_url: @source_url
    ]
  end

  defp package do
    [
      licenses: ["MIT"],
      links: %{"GitHub" => @source_url},
      files: ~w(
        lib
        mix.exs
        README.md
        .formatter.exs
        checksum-Elixir.HtmlToMarkdown.Native.exs
      )
    ]
  end

  defp deps do
    [
      {:rustler, "~> 0.34", optional: true, runtime: false},
      {:rustler_precompiled, "~> 0.9"},
      {:credo, "~> 1.7", only: [:dev, :test], runtime: false},
      {:ex_doc, "~> 0.40", only: :dev, runtime: false}
    ]
  end
end
