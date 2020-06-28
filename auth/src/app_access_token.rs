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

pub async fn get_app_access_token(
    client_id: String,
    client_secret: String,
) -> Result<AppAccessTokenResponse> {
    let url = format!(
        "https://id.twitch.tv/oauth2/token
    ?client_id={}
    &client_secret={}
    &grant_type=client_credentials",
        client_id, client_secret
    );
    Client::new()
        .post(&url)
        .send()
        .await?
        .json::<AppAccessTokenResponse>()
        .await
}
