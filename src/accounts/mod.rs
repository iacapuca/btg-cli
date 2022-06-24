use crate::settings::settings::OAuthTokenAuth;
use crate::terminal::message::{Message, StdOut};
use crate::terminal::styles;

use serde::{Deserialize, Serialize};
use tabled::{object::Columns, Format, Modify, Style, Table, Tabled};

use anyhow::Result;

use std::io;

#[derive(Serialize, Deserialize, Debug, Tabled)]
#[serde(rename_all = "camelCase")]
struct Account {
    account_id: String,
    tax_id: String,
    bank_code: String,
    r#type: String,
    currency: String,
    branch_code: String,
    number: String,
    status: String,
}

#[derive(Serialize)]
enum AccountType {
    Accounts(Vec<Account>),
    Account(Account),
}

static ACCOUNTS_URL: &str = "https://api.sandbox.empresas.btgpactual.com/v1/accounts";

#[tokio::main]
pub async fn run(
    settings: OAuthTokenAuth,
    account_id: Option<String>,
    csv: bool,
    json: bool,
    table: bool,
) -> Result<()> {
    if account_id.is_none() {
        let accounts = fetch_all_accounts(settings).await?;
        if csv {
            display_as_csv(AccountType::Accounts(accounts))?;
        } else if json && !csv {
            display_as_json(AccountType::Accounts(accounts))?;
        }
    } else {
        let account: Account = fetch_account_by_id(settings, account_id.unwrap()).await?;
        if csv {
            display_as_csv(AccountType::Account(account))?;
        } else if json && !csv {
            display_as_json(AccountType::Account(account))?;
        }
    }
    Ok(())
}

async fn fetch_account_by_id(settings: OAuthTokenAuth, account_id: String) -> Result<Account> {
    let account: Account = reqwest::Client::new()
        .get(format!("{}/{}", ACCOUNTS_URL, account_id))
        .header("Accept", "application/json")
        .header(
            "Authorization",
            "Bearer ".to_owned() + &settings.oauth_token,
        )
        .send()
        .await?
        .json()
        .await?;
    Ok(account)
}

async fn fetch_all_accounts(settings: OAuthTokenAuth) -> Result<Vec<Account>> {
    let accounts: Vec<Account> = reqwest::Client::new()
        .get(ACCOUNTS_URL)
        .header("Accept", "application/json")
        .header(
            "Authorization",
            "Bearer ".to_owned() + &settings.oauth_token,
        )
        .send()
        .await?
        .json()
        .await?;
    Ok(accounts)
}

fn display_as_csv(accounts: AccountType) -> Result<()> {
    let mut wtr = csv::Writer::from_writer(io::stdout());
    wtr.serialize(accounts)?;
    wtr.flush()?;
    Ok(())
}

fn display_as_json(accounts: AccountType) -> Result<()> {
    println!("{:?}", serde_json::to_string(&accounts)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::accounts::{display_as_csv, Account};

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
        display_as_csv(crate::accounts::AccountType::Accounts(accounts)).unwrap();
    }
}
