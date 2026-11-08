// ~keep reason: CLI application modules do not expose docs to users; doc coverage not required
#![allow(missing_docs)]

// ~keep A `#[global_allocator]` is process-wide, so only this binary may set one — the
// ~keep library and the Python/Node/JNI/PHP extension crates load into a host process
// ~keep that already owns its allocator.
// ~keep `jemalloc` deliberately wins over `mimalloc` rather than the two raising
// ~keep `compile_error!`: cargo features are additive, so `--all-features` enables both,
// ~keep and an error there makes the crate unbuildable under `cargo llvm-cov
// ~keep --all-features` (four coverage tasks, one a CI gate) and `cargo doc
// ~keep --all-features`. Precedence keeps every build path working; selecting an
// ~keep allocator is a deployment choice, so the quieter resolution is the right one.
#[cfg(all(feature = "mimalloc", not(feature = "jemalloc")))]
#[global_allocator]
static ALLOCATOR: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[cfg(feature = "jemalloc")]
#[global_allocator]
static ALLOCATOR: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

mod args;
mod convert;
mod output;
mod utils;
mod validators;

use args::{Cli, Shell};
use clap::Parser;
use convert::{build_conversion_options, perform_conversion};
use output::write_output;
use std::fs;
use std::io::{self, Read, Write as IoWrite};
use std::panic;
use std::path::PathBuf;
use utils::{DEFAULT_USER_AGENT, decode_bytes, fetch_url};

/// Initialize the process-wide `tracing` subscriber for CLI diagnostics.
///
/// The core library (`html-to-markdown-rs`) never installs a subscriber — it only emits spans
/// and events; the CLI is a consumer and owns this responsibility. `RUST_LOG` always wins when
/// set; otherwise `--debug` raises the default level from `warn` to `debug` so pipeline-stage
/// diagnostics (parse/walk/render stage boundaries, tier routing) become visible.
fn init_tracing(debug: bool) {
    let default_level = if debug { "debug" } else { "warn" };
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(default_level));
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_writer(std::io::stderr)
        .init();
}

/// Run the MCP server (stdio or http transport).
///
/// Builds a blocking Tokio runtime and drives `start_mcp_server` / `start_mcp_server_http`.
/// Keeps `fn main()` synchronous in line with the Xberg pattern.
#[cfg(feature = "mcp")]
// ~keep host/port are only read in the mcp-http arm below; without that feature
// ~keep (now the CLI's default — see Cargo.toml) they are genuinely unused.
#[cfg_attr(not(feature = "mcp-http"), allow(unused_variables))]
fn run_mcp(transport: &str, host: &str, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let rt = tokio::runtime::Runtime::new()?;
    match transport.to_lowercase().as_str() {
        "stdio" => {
            rt.block_on(html_to_markdown_rs::mcp::start_mcp_server())
                .map_err(|e| format!("Failed to start MCP server: {e}"))?;
        }
        "http" => {
            #[cfg(not(feature = "mcp-http"))]
            {
                return Err("HTTP transport requires the 'mcp-http' feature. \
                            Rebuild with: cargo build --features mcp-http"
                    .into());
            }
            #[cfg(feature = "mcp-http")]
            {
                rt.block_on(html_to_markdown_rs::mcp::start_mcp_server_http(host, port))
                    .map_err(|e| format!("Failed to start MCP HTTP server on {host}:{port}: {e}"))?;
            }
        }
        other => {
            return Err(format!("Unknown transport '{other}'. Use 'stdio' or 'http'").into());
        }
    }
    Ok(())
}

fn generate_completions(shell: Shell) {
    use clap::CommandFactory;
    use clap_complete::{Shell as ClapShell, generate};

    let mut cmd = Cli::command();
    let shell = match shell {
        Shell::Bash => ClapShell::Bash,
        Shell::Zsh => ClapShell::Zsh,
        Shell::Fish => ClapShell::Fish,
        Shell::PowerShell => ClapShell::PowerShell,
        Shell::Elvish => ClapShell::Elvish,
    };

    generate(shell, &mut cmd, invoked_name(), &mut io::stdout());
}

/// The name this binary was invoked as (`html-to-markdown` or its `h2m` alias), so generated
/// completions bind to the command the user actually types.
fn invoked_name() -> String {
    std::env::args_os()
        .next()
        .as_deref()
        .map(std::path::Path::new)
        .and_then(std::path::Path::file_stem)
        .and_then(|stem| stem.to_str())
        .filter(|stem| *stem == "h2m")
        .unwrap_or("html-to-markdown")
        .to_owned()
}

fn generate_man_page() -> Result<(), String> {
    use clap::CommandFactory;

    let cmd = Cli::command();
    let man = clap_mangen::Man::new(cmd);
    let mut buffer = Vec::new();
    man.render(&mut buffer)
        .map_err(|e| format!("Failed to generate man page: {e}"))?;

    io::stdout()
        .write_all(&buffer)
        .map_err(|e| format!("Failed to write man page: {e}"))?;

    Ok(())
}

fn read_input(cli: &Cli) -> Result<String, Box<dyn std::error::Error>> {
    let html = match cli.input.as_deref() {
        _ if cli.url.is_some() => {
            let user_agent = cli.user_agent.as_deref().unwrap_or(DEFAULT_USER_AGENT);
            let fetched = fetch_url(
                cli.url.as_deref().expect("url already checked"),
                user_agent,
                &cli.encoding,
            )?;
            tracing::debug!(bytes = fetched.len(), "fetched HTML from URL");
            fetched
        }
        None | Some("-") => {
            let mut buffer = Vec::new();
            io::stdin()
                .read_to_end(&mut buffer)
                .map_err(|e| format!("Error reading from stdin: {e}"))?;
            let decoded = decode_bytes(&buffer, &cli.encoding)?;
            tracing::debug!(bytes = decoded.len(), "read HTML from stdin");
            decoded
        }
        Some(path) => {
            let path = PathBuf::from(path);
            let bytes = fs::read(&path).map_err(|e| format!("Error reading file '{}': {}", path.display(), e))?;
            let decoded = decode_bytes(&bytes, &cli.encoding)?;
            tracing::debug!(bytes = decoded.len(), path = %path.display(), "read HTML from file");
            decoded
        }
    };
    Ok(html)
}

/// Entry point shared by the `html-to-markdown` and `h2m` binaries.
///
/// # Errors
///
/// Returns an error when input cannot be read, conversion fails or output cannot be written.
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    init_tracing(cli.debug);

    #[cfg(feature = "mcp")]
    if let Some(args::Commands::Mcp { transport, host, port }) = &cli.command {
        return run_mcp(transport, host, *port);
    }

    if let Some(shell) = cli.generate_completion {
        generate_completions(shell);
        return Ok(());
    }

    if cli.generate_man {
        generate_man_page()?;
        return Ok(());
    }

    let html = read_input(&cli)?;
    let options = build_conversion_options(&cli);
    let output_path = cli.output.clone();

    let conversion_result = panic::catch_unwind(panic::AssertUnwindSafe(|| perform_conversion(&html, options, &cli)));

    let output_content = match conversion_result {
        Ok(result) => result?,
        Err(panic_payload) => {
            let msg = panic_payload
                .downcast_ref::<String>()
                .map(String::as_str)
                .or_else(|| panic_payload.downcast_ref::<&str>().copied())
                .unwrap_or("unknown panic");
            return Err(format!("internal error during conversion (panic): {msg}").into());
        }
    };

    write_output(output_path, &output_content)?;

    Ok(())
}
