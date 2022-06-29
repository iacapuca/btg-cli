use crate::client::Client;
use crate::{client::BASE_URL, settings::settings::OAuthTokenAuth};

use serde::{Deserialize, Serialize};
use tabled::{Table, Tabled};

use anyhow::Result;

use std::io;

#[derive(Serialize, Deserialize, Debug, Tabled)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    account_id: String,
    tax_id: String,
    bank_code: String,
    r#type: String,
    currency: String,
    branch_code: String,
    number: String,
    status: String,
}

#[tokio::main]
pub async fn run(
    settings: OAuthTokenAuth,
    account_id: Option<String>,
    csv: bool,
    json: bool,
    table: bool,
) -> Result<()> {
    let client: Client = Client::new(&settings.oauth_token)?;

    match &account_id {
        Some(account_id) => {
            let account: Account = client.get_account(account_id).await?;
            let accounts: Vec<Account> = vec![account];
            if csv {
                display_as_csv(&accounts)?;
            }
            if json {
                display_as_json(&accounts)?;
            }
            if table {
                display_as_table(&accounts)?;
            }
        }
        None => {
            let accounts: Vec<Account> = client.list_accounts().await?;

            if csv {
                display_as_csv(&accounts)?;
            }
            if json {
                display_as_json(&accounts)?;
            }

            if table {
                display_as_table(&accounts)?;
            }
        }
    }

    Ok(())
}

fn display_as_csv(accounts: &Vec<Account>) -> Result<()> {
    let mut wtr = csv::Writer::from_writer(io::stdout());
    wtr.serialize(accounts)?;
    wtr.flush()?;
    Ok(())
}

fn display_as_json(accounts: &Vec<Account>) -> Result<()> {
    println!("{:?}", serde_json::to_string(&accounts)?);
    Ok(())
}

fn display_as_table(accounts: &Vec<Account>) -> Result<()> {
    let table = Table::new(accounts);
    println!("{}", table);
    Ok(())
}

pub async fn list(client: &reqwest::Client) -> Result<Vec<Account>> {
    let url = format!("{base_url}/accounts", base_url = BASE_URL);
    let builder = client.get(url.as_str());
    let res = builder.send().await?;
    Ok(res.json().await?)
}

pub async fn get(client: &reqwest::Client, account_id: &str) -> Result<Account> {
    let url = format!(
        "{base_url}/accounts/{account_id}",
        base_url = BASE_URL,
        account_id = account_id
    );
    let resp = client.get(url.as_str()).send().await?.json().await?;
    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_as_csv() {
        let accounts: Vec<Account> = vec![
            Account {
                account_id: "1".to_string(),
                tax_id: "11".to_string(),
                bank_code: "208".to_string(),
                r#type: "TRANSACTION".to_string(),
                currency: "BRL".to_string(),
                branch_code: "50".to_string(),
                number: "1".to_string(),
                status: "ACTIVE".to_string(),
            },
            Account {
                account_id: "2".to_string(),
                tax_id: "11".to_string(),
                bank_code: "208".to_string(),
                r#type: "TRANSACTION".to_string(),
                currency: "BRL".to_string(),
                branch_code: "50".to_string(),
                number: "2".to_string(),
                status: "INACTIVE".to_string(),
            },
        ];
        display_as_csv(&accounts).unwrap();
    }
}
