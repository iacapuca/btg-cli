use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct OAuthTokenAuth {
    pub oauth_token: String,
    pub refresh_token: String,
    pub expiration_time: String,
}

impl OAuthTokenAuth {
    pub fn as_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(&self).unwrap())
    }

    pub fn as_settings(json: &str) -> Result<OAuthTokenAuth> {
        Ok(serde_json::from_str(json)?)
    }
}
