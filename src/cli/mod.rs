pub mod accounts;
pub mod balances;
pub mod init;

pub mod exec {
    pub use super::accounts::accounts;
    pub use super::balances::balances;
    pub use super::init::init;
}

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(name = "btg-cli")]
#[clap(about = "A CLI to access the BTG Empresas API")]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Authenticate btg-cli with your BTG Empresas account
    Init,
    Accounts {
        #[clap(short, long)]
        /// Optional BTG Empresas account_id
        account_id: Option<String>,

        #[clap(long)]
        /// Output as a csv
        csv: bool,

        #[clap(long)]
        /// Output as a json
        json: bool,

        #[clap(long)]
        /// Output as a table
        table: bool,
    },
    Balances {
        #[clap(short, long)]
        /// Optional BTG Empresas account_id
        account_id: Option<String>,
    },
}
