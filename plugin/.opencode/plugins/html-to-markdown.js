// AI-RULEZ :: GENERATED FILE — DO NOT EDIT
// Content-Hash: blake3:764d1087e8dcd39db2f82f8f114e99aab9be9dddce1565d99b6b6b5015c8d6de
// Source-Hash: blake3:2a98a70a27241d0cbaa39fff18410711494d4759d9c0cdb3630d4ac0d2144c75
// Schema-Version: v1

import { tool } from "@opencode-ai/plugin";
import { spawn } from "node:child_process";

const schema = tool.schema;

const headingStyle = schema
  .enum(["atx", "underlined", "atx-closed"])
  .optional()
  .describe("Markdown heading style. Default: atx.");

const codeBlockStyle = schema
  .enum(["backticks", "indented", "tildes"])
  .optional()
  .describe("Code block fence style. Default: backticks.");

const outputFormat = schema.enum(["markdown", "djot"]).optional().describe("Output markup format. Default: markdown.");

const preset = schema
  .enum(["minimal", "standard", "aggressive"])
  .optional()
  .describe("Preprocessing aggressiveness. Requires `preprocess`. Default: standard.");

function hasValue(value) {
  return value !== undefined && value !== null && value !== "";
}

function pushOption(args, name, value) {
  if (hasValue(value)) {
    args.push(name, String(value));
  }
}

function pushFlag(args, name, value) {
  if (value === true) {
    args.push(name);
  }
}

function runCli(args, context, stdin) {
  const directory = context?.directory ?? context?.worktree ?? process.cwd();

  return new Promise((resolve, reject) => {
    const child = spawn("html-to-markdown", args, {
      cwd: directory,
      env: process.env,
      signal: context?.abort,
      stdio: [stdin === undefined ? "ignore" : "pipe", "pipe", "pipe"],
    });

    const stdout = [];
    const stderr = [];

    child.stdout.on("data", (chunk) => stdout.push(chunk));
    child.stderr.on("data", (chunk) => stderr.push(chunk));
    child.on("error", (error) => {
      if (error.code === "ENOENT") {
        resolve({
          title: "html-to-markdown CLI not found",
          output:
            "Install the html-to-markdown CLI with `brew install xberg-io/tap/html-to-markdown`, or run it via `npx -y @xberg-io/html-to-markdown-cli` / `uvx --from html-to-markdown-cli html-to-markdown`.",
          metadata: { exitCode: 127, command: "html-to-markdown" },
        });
        return;
      }
      reject(error);
    });
    child.on("close", (exitCode, signal) => {
      const stdoutText = Buffer.concat(stdout).toString("utf8").trim();
      const stderrText = Buffer.concat(stderr).toString("utf8").trim();
      const output = [stdoutText, stderrText && `stderr:\n${stderrText}`].filter(Boolean).join("\n\n");

      resolve({
        title: exitCode === 0 ? "html-to-markdown" : "html-to-markdown failed",
        output: output || "(no output)",
        metadata: { exitCode, signal, command: "html-to-markdown" },
      });
    });

    if (stdin !== undefined) {
      child.stdin.write(stdin);
      child.stdin.end();
    }
  });
}

function styleArgs(args, params) {
  pushOption(args, "--heading-style", params.heading_style);
  pushOption(args, "--code-block-style", params.code_block_style);
  pushOption(args, "--output-format", params.output_format);
  pushFlag(args, "--preprocess", params.preprocess);
  pushOption(args, "--preset", params.preset);
}

export const HtmlToMarkdownPlugin = async () => ({
  tool: {
    html_to_markdown_convert: tool({
      description:
        "Convert an HTML file or HTML string to Markdown (or Djot) with the html-to-markdown CLI. Provide either `path` or `html`.",
      args: {
        path: schema.string().min(1).optional().describe("Path to a local HTML file."),
        html: schema.string().min(1).optional().describe("Inline HTML to convert (used when `path` is omitted)."),
        heading_style: headingStyle,
        code_block_style: codeBlockStyle,
        output_format: outputFormat,
        preprocess: schema.boolean().optional().describe("Strip navigation, ads, and forms before converting."),
        preset,
      },
      async execute(args, context) {
        const cliArgs = [];
        styleArgs(cliArgs, args);

        if (hasValue(args.path)) {
          cliArgs.push(args.path);
          return runCli(cliArgs, context);
        }
        if (hasValue(args.html)) {
          return runCli(cliArgs, context, args.html);
        }
        throw new Error("Provide either `path` or `html`.");
      },
    }),
    html_to_markdown_fetch_url: tool({
      description: "Fetch a URL and convert its HTML to Markdown (or Djot) with the html-to-markdown CLI.",
      args: {
        url: schema.string().min(1).describe("URL to fetch and convert."),
        heading_style: headingStyle,
        code_block_style: codeBlockStyle,
        output_format: outputFormat,
        preprocess: schema.boolean().optional().describe("Strip navigation, ads, and forms before converting."),
        preset,
        user_agent: schema.string().min(1).optional().describe("Custom User-Agent header for the fetch."),
      },
      async execute(args, context) {
        const cliArgs = ["--url", args.url];
        pushOption(cliArgs, "--user-agent", args.user_agent);
        styleArgs(cliArgs, args);
        return runCli(cliArgs, context);
      },
    }),
    html_to_markdown_extract: tool({
      description:
        "Extract structured metadata, tables, and (optionally) document structure from HTML as JSON. Returns the full ConversionResult. Provide `path`, `html`, or `url`.",
      args: {
        path: schema.string().min(1).optional().describe("Path to a local HTML file."),
        html: schema
          .string()
          .min(1)
          .optional()
          .describe("Inline HTML to analyze (used when `path` and `url` are omitted)."),
        url: schema.string().min(1).optional().describe("URL to fetch and analyze."),
        include_structure: schema
          .boolean()
          .optional()
          .describe("Include the document structure tree in the JSON output."),
        no_content: schema
          .boolean()
          .optional()
          .describe("Suppress the Markdown content field — return metadata/tables/images only."),
      },
      async execute(args, context) {
        const cliArgs = ["--json"];
        pushFlag(cliArgs, "--include-structure", args.include_structure);
        pushFlag(cliArgs, "--no-content", args.no_content);

        if (hasValue(args.url)) {
          cliArgs.push("--url", args.url);
          return runCli(cliArgs, context);
        }
        if (hasValue(args.path)) {
          cliArgs.push(args.path);
          return runCli(cliArgs, context);
        }
        if (hasValue(args.html)) {
          return runCli(cliArgs, context, args.html);
        }
        throw new Error("Provide one of `path`, `html`, or `url`.");
      },
    }),
  },
});

export default HtmlToMarkdownPlugin;
