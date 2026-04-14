# Contributing

Thanks for considering a contribution to html-to-markdown. Every fix, feature, and documentation improvement helps.

## Ways to contribute

- **Report a bug** — [open an issue](https://github.com/kreuzberg-dev/html-to-markdown/issues/new?labels=bug) with a minimal reproduction
- **Fix a bug** — look for issues tagged [`good first issue`](https://github.com/kreuzberg-dev/html-to-markdown/issues?q=label%3A%22good+first+issue%22) or [`help wanted`](https://github.com/kreuzberg-dev/html-to-markdown/issues?q=label%3A%22help+wanted%22)
- **Improve the docs** — edit any page directly on GitHub using the pencil icon, or clone and run the site locally
- **Add a feature** — open an issue first to discuss scope before writing code; large changes without prior discussion may not be accepted

## Getting started

```bash
git clone https://github.com/kreuzberg-dev/html-to-markdown.git
cd html-to-markdown
task setup          # installs deps, builds Rust extension, wires commit hooks
task test           # should pass before you make any changes
```

Full prerequisites and per-language build instructions are in [`CONTRIBUTING.md`](https://github.com/kreuzberg-dev/html-to-markdown/blob/main/CONTRIBUTING.md) at the repo root.

## Workflow

1. Fork the repo and create a branch: `git checkout -b fix/your-change`
2. Make your change, add tests if applicable
3. Run `task test` (Rust + Python) and `pnpm test` (JS/TS) as needed
4. Commit using [Conventional Commits](https://www.conventionalcommits.org/) — prek enforces this automatically
5. Push and open a pull request against `main`

## Documentation changes

The docs site lives in `docs/` and builds with Zensical:

```bash
uv sync --group doc
uv run --no-sync zensical serve         # live preview at localhost:8000
scripts/ci/docs/build.sh --strict       # what CI runs — fix all warnings before pushing
```

New pages go in `docs/`, must be added to `nav:` in `mkdocs.yaml`, and should follow the style already on the page you're editing: terse, table-driven.

## Getting help

- **Have Any Questions** — [Join the GitHub Discussions](https://github.com/kreuzberg-dev/html-to-markdown/discussions)
- **Join Our community** — [Discord](https://discord.gg/pXxagNK2zN)
- **Report a Bugs** — [GitHub Issues](https://github.com/kreuzberg-dev/html-to-markdown/issues)

--8<-- "snippets/feedback.md"
