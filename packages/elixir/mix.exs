defmodule HtmlToMarkdown.MixProject do
  use Mix.Project

  def project do
    [
      app: :html_to_markdown,
      version: "3.4.0-rc.31",
      elixir: "~> 1.14",
      elixirc_paths: ["lib", "../..//packages/elixir/native/html_to_markdown_nif/src"],
      rustler_crates: [html_to_markdown_nif: [mode: :release]],
      description: "High-performance HTML to Markdown converter",
      package: package(),
      deps: deps()
    ]
  end

  defp package do
    [
      licenses: ["MIT"],
      links: %{"GitHub" => "https://github.com/kreuzberg-dev/html-to-markdown"},
      files: ~w(lib native .formatter.exs mix.exs README* checksum-*.exs)
    ]
  end

  defp deps do
    [
      {:jason, "~> 1.4"},
      {:rustler, "~> 0.37.0", runtime: false},
      {:rustler_precompiled, "~> 0.9"},
      {:credo, "~> 1.7", only: [:dev, :test], runtime: false},
      {:ex_doc, "~> 0.40", only: :dev, runtime: false}
    ]
  end
end
