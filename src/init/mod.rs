pub mod http;

use anyhow::Result;
use oauth2::basic::BasicClient;
use oauth2::reqwest::http_client;

use oauth2::{
    AuthType, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge,
    RedirectUrl, RefreshToken, Scope, TokenResponse, TokenUrl,
};

use chrono::{DateTime, Duration, Utc};

use crate::init::http::http_server_get_params;
use crate::settings::settings::OAuthTokenAuth;

pub static CLIENT_ID: &str = "5ce9d13f-e5d9-4cbb-bc81-e4e6fc3b99b2";
pub static CLIENT_SECRET: &str =
    "dIpW9RkXGZpjdb9u4RvKeLzGur6a3C7MBRUwTqgE6HbvVP9y32jEs14vBKa0YwDi6sWrsUBpAmkZxKq19c6rRg";
pub static AUTH_URL: &str = "https://id.sandbox.btgpactual.com/oauth2/authorize";
static TOKEN_URL: &str = "https://id.sandbox.btgpactual.com/oauth2/token";
static CALLBACK_URL: &str = "http://localhost:8000";
static SCOPES: [&str; 1] = ["openid"];

pub fn run() -> Result<()> {
    let auth_url = AuthUrl::new(AUTH_URL.to_string())?;
    let token_url = TokenUrl::new(TOKEN_URL.to_string())?;
    let redirect_url = RedirectUrl::new(CALLBACK_URL.to_string())?;

    // Create oauth2 client
    let client = BasicClient::new(
        ClientId::new(CLIENT_ID.to_string()),
        Some(ClientSecret::new(CLIENT_SECRET.to_string())),
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(redirect_url)
    .set_auth_type(AuthType::BasicAuth);

    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    // Create URL for user with the necessary scopes
    let mut client_state = client
        .authorize_url(CsrfToken::new_random)
        .set_pkce_challenge(pkce_challenge);

    // User did provide some scopes
    client_state = client_state.add_scope(Scope::new("openid".to_string()));

    client_state = client_state.add_scope(Scope::new("offline_access".to_string()));
    let (auth_url, csrf_state) = client_state.url();

    if webbrowser::open(&auth_url.to_string()).is_err() {
        // Try manually
        println!("Visit the following URL to authorize your app with BTG:");
        println!("{}\n", auth_url);
    }

    // Get authorization code and CSRF state from local HTTP server
    let runtime = tokio::runtime::Runtime::new()?;
    let params_response = runtime.block_on(http_server_get_params())?;
    let params_values: Vec<&str> = params_response.split_whitespace().collect();
    if params_values.is_empty() {
        anyhow::bail!(display_error_info(
            "Failed to receive authorization code from local HTTP server."
        ))
    }

    let response_status = params_values[0];
    if response_status == "denied" {
        anyhow::bail!("Consent denied. You must grant consent to btg-cli in order to login. If you don't want to do this consider using `btg-cli config`")
    } else if response_status == "error" {
        anyhow::bail!(display_error_info(
            "Failed to receive authorization code from local HTTP server."
        ))
    }

    // Get authorization code and CSRF state
    if params_values.len() != 3 {
        anyhow::bail!(display_error_info(
            "Failed to receive authorization code and/or csrf state from local HTTP server."
        ))
    }

    let auth_code = params_values[2];
    let recv_csrf_state = params_values[1];

    let recv_csrf_state = CsrfToken::new(recv_csrf_state.to_string());
    if recv_csrf_state.secret() != csrf_state.secret() {
        anyhow::bail!(display_error_info("Redirect URI CSRF state check failed."))
    }

    // Exchange authorization token for access token
    let token_response = client
        .exchange_code(AuthorizationCode::new(auth_code.to_string()))
        .set_pkce_verifier(pkce_verifier)
        .request(http_client)?;

    // Get access token expiration time
    let expires_in = match TokenResponse::expires_in(&token_response) {
        Some(time) => time,
        None => anyhow::bail!(display_error_info(
            "Failed to receive access_token expire time."
        )),
    };

    let expiration_time_value = match Utc::now().checked_add_signed(Duration::from_std(expires_in)?)
    {
        Some(time) => time,
        None => anyhow::bail!(display_error_info(
            "Failed to calculate access_token expiration time."
        )),
    };
    let expiration_time_value = expiration_time_value.to_rfc3339();

    let refresh_token_value = match token_response.refresh_token() {
        Some(token) => token,
        None => anyhow::bail!(display_error_info("Failed to receive refresh token.")),
    };

    // Configure settings with new token
    let settings = OAuthTokenAuth {
        oauth_token: TokenResponse::access_token(&token_response)
            .secret()
            .to_string(),
        refresh_token: refresh_token_value.secret().to_string(),
        expiration_time: expiration_time_value,
    };

    println!("{:?}", settings);

    Ok(())
}

// Adds additional info besides an error message
pub fn display_error_info(error_msg: &str) -> String {
    let error_info = format!("{} Please run `btg-cli login` again. If the error persists, consider reporting the issue through `btg-cli report`.", error_msg);
    error_info
}
