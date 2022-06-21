use crate::init::{AUTH_URL, CLIENT_ID};
use crate::settings::settings::OAuthTokenAuth;

use anyhow::Result;

use oauth2::basic::BasicClient;
use oauth2::reqwest::http_client;
use oauth2::{AuthType, AuthUrl, ClientId, RefreshToken, RevocationUrl, StandardRevocableToken};

static REVOKE_URL: &str = "https://id.sandbox.btgpactual.com/oauth2/revoke";

// Revoke refresh token, which also invalidates the current access token
pub fn revoke_token(settings: &OAuthTokenAuth) -> Result<()> {
    let auth_url = AuthUrl::new(AUTH_URL.to_string())?;
    let revoke_url = RevocationUrl::new(REVOKE_URL.to_string())?;

    let client = BasicClient::new(ClientId::new(CLIENT_ID.to_string()), None, auth_url, None)
        .set_revocation_uri(revoke_url)
        .set_auth_type(AuthType::RequestBody);

    let token_to_revoke =
        StandardRevocableToken::RefreshToken(RefreshToken::new(settings.refresh_token.to_string()));

    if let Err(err) = client.revoke_token(token_to_revoke)?.request(http_client) {
        anyhow::bail!(err)
    }

    Ok(())
}

// Invalidatess previous OAuth token if present
pub fn invalidate_oauth_token(command: String) {
    if let Ok(settings) = OAuthTokenAuth::new() {
        // Try to invalidate previous token
        let result = revoke_token(&settings);
        if result.is_err() {
            // A failure to invalidate a previous token should not block the user from being able to login with a new OAuth token
            log::debug!("Failed to invalidate OAuth token before {}", command);
        }
    }
}
