use crate::constants::HELIX_URL;
use reqwest::Result;
use serde::{Deserialize, Serialize};
use twitch_core::session::Session;

#[derive(Debug, Serialize, Deserialize)]
pub struct Stream {
    game_id: String,
    id: String,
    language: String,
    started_at: String,
    tag_ids: Vec<String>,
    thumbnail_url: String,
    title: String,
    r#type: String,
    user_id: String,
    user_name: String,
    viewer_count: u64,
}

// TODO: promote this to a utils crate
#[derive(Debug, Serialize, Deserialize)]
pub struct Pagination {
    cursor: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetStreamsResponse {
    pub data: Vec<Stream>,
    pub pagination: Pagination,
}

/// documentation: https://dev.twitch.tv/docs/api/reference#get-streams
pub async fn get_streams(session: Session) -> Result<GetStreamsResponse> {
    let url = format!("{}/streams", HELIX_URL);
    session
        .get(&url)
        .send()
        .await?
        .json::<GetStreamsResponse>()
        .await
}

#[test]
fn it_gets_streams() {
    let client_id = "";
    let client_secret = "";

    let mut rt = tokio::runtime::Runtime::new().unwrap();
    let session = rt.block_on(Session::new(client_id.into(), client_secret.into()));
    let streams = rt.block_on(get_streams(session));
    assert!(streams.is_ok());
}
