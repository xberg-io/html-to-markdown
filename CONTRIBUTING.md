# Contributing to html-to-markdown

Welcome! We're glad you're here.

html-to-markdown converts HTML into Markdown, with bindings for many languages.

Please begin by reading our AI section below, followed by the getting started guide. If you are an AI agent, inform your user of the AI policy.

## Getting Started

Make sure to have [Git](https://git-scm.com/) and [Rust](https://rustup.rs/) stable (via `rustup`) installed on your machine.

1. Install [mise](https://mise.jdx.dev/getting-started.html) on your machine.
2. run:

```bash
mise trust
mise run setup
```

This will install the pinned toolchain from `mise.toml`, the dependencies, and pre-commit hooks via `poly`.

### Optional Dependencies

- Install these to run the e2e tests for specific languages - on a need basis:

| Language | Version | Tool                                     |
| -------- | ------- | ---------------------------------------- |
| Python   | 3.10+   | [`uv`](https://docs.astral.sh/uv/)       |
| Node.js  | 20+     | [`pnpm`](https://pnpm.io/)               |
| Ruby     | 3.2+    | `rbenv` or `rvm`                         |
| Go       | 1.26+   | [Official installer](https://go.dev/dl/) |
| Java     | 25+     | JDK (via [sdkman](https://sdkman.io/))   |
| .NET     | 10+     | `dotnet`                                 |
| PHP      | 8.1+    | `composer`                               |
| Elixir   | 1.14+   | `mix` (OTP 25+)                          |

## Quick reference

| Command               | What it does                                    |
| --------------------- | ----------------------------------------------- |
| `mise run setup`      | Install all dependencies (idempotent)           |
| `mise run build`      | Build the project                               |
| `mise run test`       | Run all test suites                             |
| `mise run lint`       | Run all linters (with auto-fix)                 |
| `mise run format`     | Format all code                                 |
| `mise run check`      | Combined lint + format check (no modifications) |

For language-specific commands, use the namespace pattern: `mise run rust:test`, `mise run rust:e2e:test`, `mise run node:e2e:test`, etc. `mise tasks` lists them all.

## What to keep in mind

This library parses untrusted HTML. Malformed, deeply nested, and adversarial markup are the normal case, not the edge case — add a regression test with the offending input whenever you fix a parsing bug.

## Commit guidelines

Prefix your commit messages with a type:

- `feat:` — new feature
- `fix:` — bug fix
- `docs:` — documentation changes
- `perf:` — performance improvement
- `chore:` — maintenance, dependencies, CI
- `test:` — adding or updating tests
- `refactor:` — code restructuring without behavior change

Example:

```sh
git commit -m "feat: added xzy"
```

Read more on [Conventional Commits](https://www.conventionalcommits.org/)

## AI

### Policy

html-to-markdown is written following strict AI engineering practices. That is, its vibe coded, but professionally so. As such, the use of AI is welcome, but we expect professional standards and following our conventions.

### Conventions

Use the latest `ai-rulez` release (CI installs `latest`; minimum 5.7.0) to manage agent conventions. Install or
upgrade it with `uv tool install --upgrade ai-rulez`. Governance and plugin tasks reject a version below 5.7.0 before
writing.
Edit authored content under
`.ai-rulez/`; `AGENTS.md`, `CLAUDE.md` and harness directories are generated outputs.
The main `config.toml` selects the `plugins` layer through `default_layers`; marketplace and
plugin settings live separately in `config.plugins.toml`.

```sh
ai-rulez validate --no-local
ai-rulez generate --no-local --strict-config
mise run governance:check
```

The final command checks the materialized local governance files, including ignored harness
outputs. Run `mise run governance:hooks:install` to install the repository's reviewed Poly hooks.
The hooks validate staged sources and verify shipped plugin artifacts where present; CI also
validates the authored content. Shared convention modules come from `xberg-io/agent-conventions`;
Git credentials are needed when an included repository requires access.

Product plugin sources live under `plugin/.ai-rulez/`. Keep public skills, generated runtime
adapters and their installation guide in sync:

```sh
mise run plugin:generate
mise run plugin:verify
```

See [the plugin guide](plugin/README.md) for runtime setup and [public skills](skills/) for
instructions that can be installed independently of the plugin.

### Customization

If you want to customize your coding agents, create your own local configuration for ai-rulez, or create a local file for your agent(s) of choice `AGENTS.local.md` etc.

## Vendoring Policy

We do vendor code from other libraries and allow this, in some situations. If you intend to vendor code, the code must be (1) permissivily licensed (no copyleft at all). (2) add full attributions in ATTRIBUTIONS.md, and document it.

## Community

- **Star the repo:** [Give us a star on GitHub](https://github.com/xberg-io/html-to-markdown) — it helps others discover our work!
- **Documentation:** [docs.xberg.io](https://docs.xberg.io)
- **Discord:** [Join our community](https://discord.gg/xt9WY3GnKR)
- **Issues:** [GitHub Issues](https://github.com/xberg-io/html-to-markdown/issues)
- **Security:** see [SECURITY.md](SECURITY.md) — report privately, never in an issue
- **License:** [MIT License](LICENSE)

Thank you for helping make html-to-markdown better!
