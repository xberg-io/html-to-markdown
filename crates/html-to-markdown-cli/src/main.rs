#![allow(clippy::all, clippy::pedantic, clippy::nursery, missing_docs)]

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
                .map(|s| s.as_str())
                .or_else(|| panic_payload.downcast_ref::<&str>().copied())
                .unwrap_or("unknown panic");
            return Err(format!("internal error during conversion (panic): {msg}").into());
        }
    };

    write_output(output_path, &output_content)?;

    Ok(())
}
