use crate::commands;
use crate::terminal::message::{Message, StdOut};
use crate::terminal::styles;

use crate::settings::settings::OAuthTokenAuth;

use anyhow::Result;

pub fn balances(account_id: Option<String>) -> Result<()> {
    log::info!("Getting balances");
    let settings = OAuthTokenAuth::new();

    let settings = match settings {
        Ok(settings) => settings,
        Err(_) => anyhow::bail!(display_auth_error_info()),
    };

    return commands::balances::run(settings, account_id);
}

// Adds additional info besides an error message
fn display_auth_error_info() -> String {
    let btg_cli_init_msg = styles::highlight("`btg-cli init`");
    StdOut::billboard(&format!(
        "You have not provided your BTG Empresas credentials.\n\nPlease run {}.",
        btg_cli_init_msg
    ));
    let error_info = format!("btg-cli key was not found on the systems keyring");
    error_info
}
