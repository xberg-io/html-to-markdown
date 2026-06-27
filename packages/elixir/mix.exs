defmodule HtmlToMarkdown.MixProject do
  use Mix.Project

  def project do
    [
      app: :html_to_markdown,
      version: "3.8.0-rc.2",
      elixir: "~> 1.14",
      elixirc_paths: ["lib", Path.expand("../../packages/elixir/native/html_to_markdown_nif/src", __DIR__)],
      rustler_crates: [
        html_to_markdown_nif: [
          mode: :release,
          targets: [
            "aarch64-apple-darwin",
            "aarch64-unknown-linux-gnu",
            "x86_64-unknown-linux-gnu",
            "x86_64-pc-windows-gnu"
          ]
        ]
      ],
      description: "High-performance HTML to Markdown converter",
      package: package(),
      deps: deps()
    ]
  end

  defp package do
    [
      licenses: ["MIT"],
      links: %{"GitHub" => "https://github.com/xberg-io/html-to-markdown"},
      files:
        ~w(lib .formatter.exs mix.exs README* checksum-*.exs native/html_to_markdown_nif/Cargo.toml native/html_to_markdown_nif/Cargo.lock native/html_to_markdown_nif/src)
    ]
  end

  defp deps do
    [
      {:jason, "~> 1.4"},
      {:rustler, "~> 0.37", runtime: false},
      {:rustler_precompiled, "~> 0.9"},
      {:credo, "~> 1.7", only: [:dev, :test], runtime: false},
      {:ex_doc, "~> 0.40", only: :dev, runtime: false}
    ]
  end
end
