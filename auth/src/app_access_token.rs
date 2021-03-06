use reqwest::{Client, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppAccessToken {
    pub access_token: String,
    pub expires_in: u64,
    pub scope: Option<Vec<String>>,
    pub token_type: String,
}

pub async fn get_app_access_token(
    client_id: String,
    client_secret: String,
) -> Result<AppAccessToken> {
    let url = format!(
        "https://id.twitch.tv/oauth2/token?client_id={}&client_secret={}&grant_type=client_credentials",
        client_id, client_secret
    );
    Client::new()
        .post(&url)
        .send()
        .await?
        .json::<AppAccessToken>()
        .await
}

#[test]
fn it_gets_access_token() {
    let client_id = "";
    let client_secret = "";

    let mut rt = tokio::runtime::Runtime::new().unwrap();
    let _ = rt.block_on(get_app_access_token(client_id.into(), client_secret.into()));
}
