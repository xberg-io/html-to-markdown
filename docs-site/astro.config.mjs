// @ts-check
import starlight from "@astrojs/starlight";
import { xbergStarlightConfig } from "@xberg-io/docs-theme";
import { defineConfig } from "astro/config";
import starlightLlmsTxt from "starlight-llms-txt";

const API_LANGUAGES = [
  { label: "Rust", slug: "reference/api-rust" },
  { label: "Python", slug: "reference/api-python" },
  { label: "TypeScript", slug: "reference/api-typescript" },
  { label: "Go", slug: "reference/api-go" },
  { label: "Ruby", slug: "reference/api-ruby" },
  { label: "PHP", slug: "reference/api-php" },
  { label: "Java", slug: "reference/api-java" },
  { label: "C#", slug: "reference/api-csharp" },
  { label: "Elixir", slug: "reference/api-elixir" },
  { label: "R", slug: "reference/api-r" },
  { label: "Dart", slug: "reference/api-dart" },
  { label: "Kotlin (Android)", slug: "reference/api-kotlin-android" },
  { label: "Swift", slug: "reference/api-swift" },
  { label: "Zig", slug: "reference/api-zig" },
  { label: "C", slug: "reference/api-c" },
  { label: "WebAssembly", slug: "reference/api-wasm" },
];

export default defineConfig({
  site: "https://docs.html-to-markdown.xberg.io",
  integrations: [
    starlight(
      xbergStarlightConfig({
        title: "html-to-markdown",
        description:
          "High-performance HTML to Markdown conversion powered by Rust. One core plus 15 " +
          "generated packages, identical output on every runtime.",
        githubUrl: "https://github.com/xberg-io/html-to-markdown",
        editBaseUrl: "https://github.com/xberg-io/html-to-markdown/edit/main/docs-site/",
        plugins: [
          starlightLlmsTxt({
            customSets: [
              {
                label: "Get Started",
                description: "Installation, first conversion, and the CLI.",
                paths: ["installation", "usage", "cli"],
              },
              {
                label: "Guides",
                description: "Visitor pattern, table extraction, error handling, and AI coding assistants.",
                paths: ["visitor", "tables", "errors", "guides/ai-coding-assistants"],
              },
              {
                label: "Concepts",
                description: "Architecture, the conversion pipeline, and the plugin system.",
                paths: ["concepts/**"],
              },
              {
                label: "Reference",
                description: "Configuration, API reference, and per-language guides.",
                paths: ["configuration", "api-reference", "language-guides", "reference/**"],
              },
              {
                label: "More",
                description: "Migration, contributing, and the ecosystem.",
                paths: ["migration", "contributing", "ecosystem"],
              },
            ],
            optionalLinks: [
              {
                label: "GitHub",
                url: "https://github.com/xberg-io/html-to-markdown",
                description: "Source code and issues",
              },
            ],
          }),
        ],
        sidebar: [
          { label: "Home", link: "/" },
          {
            label: "Get Started",
            items: [
              { label: "Installation", slug: "installation" },
              { label: "Usage", slug: "usage" },
              { label: "CLI", slug: "cli" },
            ],
          },
          {
            label: "Guides",
            items: [
              { label: "Visitor pattern", slug: "visitor" },
              { label: "Table extraction", slug: "tables" },
              { label: "Error handling", slug: "errors" },
              {
                label: "AI Coding Assistants",
                slug: "guides/ai-coding-assistants",
              },
            ],
          },
          {
            label: "Concepts",
            items: [
              { label: "Architecture", slug: "concepts/architecture" },
              { label: "Conversion pipeline", slug: "concepts/pipeline" },
              { label: "Plugin system", slug: "concepts/plugin-system" },
            ],
          },
          {
            label: "Reference",
            items: [
              { label: "Configuration", slug: "configuration" },
              { label: "API reference", slug: "api-reference" },
              { label: "Language guides", slug: "language-guides" },
              { label: "Language APIs", items: API_LANGUAGES },
              { label: "Types", slug: "reference/types" },
              {
                label: "Configuration (generated)",
                slug: "reference/configuration",
              },
              { label: "Error types (generated)", slug: "reference/errors" },
              { label: "CLI (generated)", slug: "reference/cli" },
              { label: "MCP (generated)", slug: "reference/mcp" },
              { label: "Architecture decisions", slug: "reference/adrs" },
            ],
          },
          {
            label: "More",
            items: [
              { label: "Migration", slug: "migration" },
              { label: "Contributing", slug: "contributing" },
              { label: "Changelog", slug: "changelog" },
              { label: "Ecosystem", slug: "ecosystem" },
            ],
          },
        ],
      }),
    ),
  ],
});
