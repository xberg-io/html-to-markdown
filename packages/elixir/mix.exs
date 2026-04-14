defmodule Html_to_markdown.MixProject do
  use Mix.Project

  def project do
    [
      app: :html_to_markdown,
      version: "3.1.0",
      elixir: "~> 1.14",
      description: "High-performance HTML to Markdown converter",
      package: package(),
      deps: deps()
    ]
  end

  defp package do
    [
      licenses: ["MIT"],
      links: %{"GitHub" => "https://github.com/kreuzberg-dev/html-to-markdown"}
    ]
  end

  defp deps do
    [
      {:rustler, "~> 0.34"},
      {:credo, "~> 1.7", only: [:dev, :test], runtime: false}
    ]
  end
end
