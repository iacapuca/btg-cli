use std::collections::HashMap;
use std::fmt::Display;

use crate::client::Client;
use crate::{client::BASE_URL, settings::settings::OAuthTokenAuth};
use serde::{Deserialize, Serialize};
use tabled::{Style, Table, Tabled};

use anyhow::Result;

use std::io;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    account_id: String,
    available: BalanceAmount,
    blocked: BalanceAmount,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BalanceAmount {
    amount: f64,
    currency: String,
}

#[tokio::main]
pub async fn run(settings: OAuthTokenAuth, account_id: Option<String>) -> Result<()> {
    let client: Client = Client::new(&settings.oauth_token)?;
    match &account_id {
        Some(account_id) => {
            let balance: Balance = client.get_balance(account_id).await?;
            println!("{:?}", serde_json::to_string(&balance)?);
        }
        None => {
            let balances: Vec<Balance> = client.list_balances().await?;
            println!("{:?}", serde_json::to_string(&balances)?);
        }
    }

    Ok(())
}

pub async fn list(client: &reqwest::Client) -> Result<Vec<Balance>> {
    let url = format!("{base_url}/accounts/balances", base_url = BASE_URL);
    let builder = client.get(url.as_str());
    let res = builder.send().await?;
    Ok(res.json().await?)
}

pub async fn get(client: &reqwest::Client, account_id: &str) -> Result<Balance> {
    let url = format!(
        "{base_url}/accounts/{account_id}/balances",
        base_url = BASE_URL,
        account_id = account_id
    );
    let resp = client.get(url.as_str()).send().await?.json().await?;
    Ok(resp)
}
