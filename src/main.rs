use btg_cli::cli::{exec, Cli, Command};

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    run()?;
    Ok(())
}

fn run() -> Result<()> {
    let args: Cli = Cli::parse();

    match args.command {
        Command::Init {} => exec::init(),
    }
}
