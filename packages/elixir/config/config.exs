import Config

config :html_to_markdown, HtmlToMarkdown.Native,
  crate: "html_to_markdown_nif",
  path: "native/html_to_markdown_nif",
  mode: if(Mix.env() == :prod, do: :release, else: :debug)
