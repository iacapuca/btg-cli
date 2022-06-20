use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum Settings {
    OAuthTokenAuth {
        oauth_token: String,
        refresh_token: String,
        expiration_time: String,
    },
}

impl Settings {
    pub fn as_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(&self).unwrap())
    }

    pub fn as_settings(json: &str) -> Result<Settings> {
        Ok(serde_json::from_str(json)?)
    }

    pub fn get_expiration_time(&self) -> &String {
        match self {
            Settings::OAuthTokenAuth {
                oauth_token: _,
                refresh_token: _,
                expiration_time,
            } => expiration_time,
        }
    }

    pub fn get_refresh_token(&self) -> &String {
        match self {
            Settings::OAuthTokenAuth {
                oauth_token: _,
                refresh_token,
                expiration_time: _,
            } => refresh_token,
        }
    }

    pub fn set_oauth_token(&mut self, new_oauth_token: String) {
        match self {
            Settings::OAuthTokenAuth {
                ref mut oauth_token,
                refresh_token: _,
                expiration_time: _,
            } => *oauth_token = new_oauth_token,
        }
    }

    pub fn set_refresh_token(&mut self, new_refresh_token: String) {
        match self {
            Settings::OAuthTokenAuth {
                oauth_token: _,
                ref mut refresh_token,
                expiration_time: _,
            } => *refresh_token = new_refresh_token,
        }
    }

    pub fn set_expiration_time(&mut self, new_expiration_time: String) {
        match self {
            Settings::OAuthTokenAuth {
                oauth_token: _,
                refresh_token: _,
                expiration_time,
            } => *expiration_time = new_expiration_time,
        }
    }
}
