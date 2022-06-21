use crate::settings::settings::OAuthTokenAuth;
use anyhow::Result;

pub fn global_config(settings: &OAuthTokenAuth) -> Result<()> {
    settings.to_keyring()?;

    Ok(())
}
