use anyhow::Result;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};

use crate::accounts::{self, Account};

pub static BASE_URL: &str = "https://api.sandbox.empresas.btgpactual.com/v1";

#[derive(Clone)]
pub struct Client {
    http_client: reqwest::Client,
}

impl Client {
    pub fn new(access_token: &str) -> Result<Self> {
        let token_value = "Bearer ".to_owned() + &access_token;
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&token_value)?);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        Ok(Client {
            http_client: reqwest::Client::builder()
                .default_headers(headers)
                .build()?,
        })
    }

    /// Returns all accounts
    pub async fn list_accounts(&self) -> Result<Vec<Account>> {
        accounts::list(&self.http_client).await
    }

    /// Returns a account according to the provided account_id
    pub async fn get_account(&self, account_id: &str) -> Result<Account> {
        accounts::get(&self.http_client, account_id).await
    }
}
