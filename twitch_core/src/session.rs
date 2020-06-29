use auth::app_access_token::{get_app_access_token, AppAccessToken};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, IntoUrl, RequestBuilder,
};

pub struct Session {
    twitch_headers: HeaderMap,
    client: Client,
}

impl Session {
    // TODO: remove expect from access token
    pub async fn new(client_id: String, client_secret: String) -> Self {
        let access_token = get_app_access_token(client_id.clone(), client_secret)
            .await
            .expect("could not get access_token");
        let mut twitch_headers = HeaderMap::new();
        twitch_headers.insert(
            "Client-ID",
            HeaderValue::from_str(&client_id.clone()).unwrap(),
        );
        twitch_headers.insert(
            "Authorization",
            HeaderValue::from_str(&format!("Bearer {} ", access_token.access_token)).unwrap(),
        );
        Session {
            twitch_headers,
            client: Client::new(),
        }
    }

    pub fn get<U: IntoUrl>(self, url: U) -> RequestBuilder {
        self.client.get(url).headers(self.twitch_headers)
    }

    pub fn post<U: IntoUrl>(self, url: U) -> RequestBuilder {
        self.client.post(url).headers(self.twitch_headers)
    }

    pub fn put<U: IntoUrl>(self, url: U) -> RequestBuilder {
        self.client.put(url).headers(self.twitch_headers)
    }

    pub fn patch<U: IntoUrl>(self, url: U) -> RequestBuilder {
        self.client.patch(url).headers(self.twitch_headers)
    }

    pub fn delete<U: IntoUrl>(self, url: U) -> RequestBuilder {
        self.client.delete(url).headers(self.twitch_headers)
    }
}
