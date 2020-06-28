use serde::{Deserialize, Serialize};
use twitch_core::reqwest::{Client, Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppAccessTokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: String,
    pub scope: Vec<String>,
    pub token_type: String,
}

pub async fn get_app_access_token() -> Result<()> {
    let url = format!("");
    Client::new().post(&url).send().await?;
    Ok(())
}
