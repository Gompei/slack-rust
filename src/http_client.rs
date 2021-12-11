use async_trait::async_trait;
use mockall::automock;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait SlackWebAPIClient {
    async fn post_json(&self, url: String, body: String, token: String) -> surf::Result;
    async fn post(&self, url: String, token: String) -> surf::Result;
}

pub type Client = surf::Client;

/// Metadata.
#[derive(Deserialize, Serialize, Debug)]
pub struct ResponseMetadata {
    pub next_cursor: Option<String>,
    pub messages: Option<Vec<String>>,
    pub warnings: Option<Vec<String>>,
}

/// Returns the slack api url for each method.
pub fn get_slack_url(method: &str) -> String {
    format!("https://slack.com/api/{}", method)
}

/// Provides a default `surf` client to give to the API functions to send requests.
pub fn default_client() -> Client {
    surf::Client::new()
}

#[async_trait]
impl SlackWebAPIClient for Client {
    /// Send a post request to the slack api.
    async fn post_json(&self, url: String, body: String, token: String) -> surf::Result {
        let check_url = url::Url::parse(url.as_ref())?;

        self.post(check_url)
            .header("Authorization", format!("Bearer {}", token))
            .content_type(surf::http::mime::JSON)
            .body(body)
            .await
    }

    /// Send a post request to the slack api.
    async fn post(&self, url: String, token: String) -> surf::Result {
        let check_url = url::Url::parse(url.as_ref())?;

        self.post(check_url)
            .header("Authorization", format!("Bearer {}", token))
            .await
    }
}
