use crate::balances;
use crate::settings::settings::OAuthTokenAuth;
use anyhow::Result;

pub fn run(settings: OAuthTokenAuth, account_id: Option<String>) -> Result<()> {
    balances::run(settings, account_id)
}
