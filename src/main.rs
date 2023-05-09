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
        Command::Accounts {
            account_id,
            csv,
            json,
            table,
        } => exec::accounts(account_id, csv, json, table),
        Command::Balances { account_id } => exec::balances(account_id),
    }
}
