defmodule RustlerPrecompiled.MixProject do
  use Mix.Project

  @version "0.9.0"
  @repo "https://github.com/philss/rustler_precompiled"

  def project do
    [
      app: :rustler_precompiled,
      version: @version,
      elixir: "~> 1.15",
      start_permanent: Mix.env() == :prod,
      description: "Make the usage of precompiled NIFs easier for projects using Rustler",
      package: package(),
      test_ignore_filters: [&String.starts_with?(&1, "test/fixtures")],
      docs: docs(),
      deps: deps()
    ]
  end

  def application do
    [
      extra_applications: [:logger, :inets, :ssl]
    ]
  end

  defp docs do
    [
      main: "RustlerPrecompiled",
      extras: ["PRECOMPILATION_GUIDE.md", "CHANGELOG.md", "TROUBLESHOOTING.md"],
      source_url: @repo,
      source_ref: "v#{@version}"
    ]
  end

  defp deps do
    [
      {:rustler, "~> 0.23", optional: true},
      {:ex_doc, "~> 0.27", only: :dev},
      {:bypass, "~> 2.1", only: :test}
    ]
  end

  defp package do
    %{
      licenses: ["Apache-2.0"],
      maintainers: ["Philip Sampaio"],
      files: ~w(lib mix.exs README.md CHANGELOG.md PRECOMPILATION_GUIDE.md TROUBLESHOOTING.md),
      links: %{"GitHub" => @repo}
    }
  end
end
