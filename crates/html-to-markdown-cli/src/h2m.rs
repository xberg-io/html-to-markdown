//! `h2m`: the short name of the html-to-markdown CLI. It runs the same program, so the two
//! names cannot drift apart.

#[path = "main.rs"]
mod cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    cli::main()
}
