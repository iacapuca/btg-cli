use anyhow::Result;
use oauth2::basic::BasicClient;
use oauth2::reqwest::http_client;

use oauth2::{
    AuthType, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge,
    RedirectUrl, RefreshToken, Scope, TokenResponse, TokenUrl,
};

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

    Ok(())
}