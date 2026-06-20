// reason: CLI application modules do not expose docs to users; doc coverage not required
#![allow(missing_docs)]

mod args;
mod convert;
mod output;
mod utils;
mod validators;

use args::{Cli, Shell};
use clap::Parser;
use convert::{build_conversion_options, perform_conversion};
use output::{output_debug_info, write_output};
use std::fs;
use std::io::{self, Read, Write as IoWrite};
use std::panic;
use std::path::PathBuf;
use utils::{DEFAULT_USER_AGENT, decode_bytes, fetch_url};

/// Run the MCP server (stdio or http transport).
///
/// Builds a blocking Tokio runtime and drives `start_mcp_server` / `start_mcp_server_http`.
/// Keeps `fn main()` synchronous in line with the kreuzberg pattern.
#[cfg(feature = "mcp")]
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

    generate(shell, &mut cmd, "html-to-markdown", &mut io::stdout());
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
            output_debug_info(cli, &format!("Fetched {} bytes from URL", fetched.len()));
            fetched
        }
        None | Some("-") => {
            let mut buffer = Vec::new();
            io::stdin()
                .read_to_end(&mut buffer)
                .map_err(|e| format!("Error reading from stdin: {e}"))?;
            let decoded = decode_bytes(&buffer, &cli.encoding)?;
            output_debug_info(cli, &format!("Read {} bytes from stdin", decoded.len()));
            decoded
        }
        Some(path) => {
            let path = PathBuf::from(path);
            let bytes = fs::read(&path).map_err(|e| format!("Error reading file '{}': {}", path.display(), e))?;
            let decoded = decode_bytes(&bytes, &cli.encoding)?;
            output_debug_info(
                cli,
                &format!("Read {} bytes from file '{}'", decoded.len(), path.display()),
            );
            decoded
        }
    };
    Ok(html)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Dispatch MCP subcommand before any convert-from-file logic.
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

    // Wrap conversion in catch_unwind so an unexpected panic produces a clean error
    // message rather than a Rust backtrace, and no partial output is written.
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
