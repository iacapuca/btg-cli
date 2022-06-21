use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::init::check_update_oauth_token;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct OAuthTokenAuth {
    pub oauth_token: String,
    pub refresh_token: String,
    pub expiration_time: String,
}

impl OAuthTokenAuth {
    pub fn new() -> Result<Self> {
        let mut new_settings = OAuthTokenAuth::build();

        // Check if oauth token is expired
        if let Ok(ref mut settings) = new_settings {
            // Let caller handle possible errors
            check_update_oauth_token(settings)?;
        }
        new_settings
    }

    fn build() -> Result<Self> {
        Self::from_keyring()
    }

    fn from_keyring() -> Result<Self> {
        let entry = keyring::Entry::new("btg-cli", "OAuthTokenAuth");
        Self::as_settings(&entry.get_password()?)
    }

    pub fn as_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(&self).unwrap())
    }

    pub fn as_settings(json: &str) -> Result<OAuthTokenAuth> {
        Ok(serde_json::from_str(json)?)
    }

    pub fn to_keyring(&self) -> Result<()> {
        println!("{:?}", &self);

        let json = self.as_json();
        let entry = keyring::Entry::new("btg-cli", "OAuthTokenAuth");
        entry.set_password(&json?)?;
        Ok(())
    }
}
