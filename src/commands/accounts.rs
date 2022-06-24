use crate::accounts;
use crate::settings::settings::OAuthTokenAuth;
use anyhow::Result;

pub fn run(
    settings: OAuthTokenAuth,
    account_id: Option<String>,
    csv: bool,
    json: bool,
    table: bool,
) -> Result<()> {
    accounts::run(settings, account_id, csv, json, table)
}
