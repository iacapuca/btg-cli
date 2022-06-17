pub mod init;

pub mod exec {
    pub use super::init::init;
}

use clap::{AppSettings, Parser, Subcommand};

#[derive(clap::Parser, Debug)]
#[clap(name = "btg-cli")]
#[clap(about = "A CLI to access the BTG Empresas API")]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(clap::Subcommand, Debug)]
pub enum Command {
    /// Authenticate btg-cli with your BTG Empresas account
    Init,
}
