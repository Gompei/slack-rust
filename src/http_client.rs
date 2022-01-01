//! HTTP client for Slack WEB API.

use crate::error::Error;
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Slack WEB API Client.
#[cfg_attr(test, automock)]
#[async_trait]
pub trait SlackWebAPIClient: Sync + Send {
    async fn post_json(&self, url: &str, body: &str, token: &str) -> Result<String, Error>;
    async fn post(&self, url: &str, token: &str) -> Result<String, Error>;
}

/// HTTP Client(surf::Client).
pub type Client = surf::Client;

#[async_trait]
impl SlackWebAPIClient for Client {
    /// Send a post request including the body to the slack web api.
    async fn post_json(&self, url: &str, body: &str, token: &str) -> Result<String, Error> {
        let check_url = url::Url::parse(url)?;

        Ok(self
            .post(check_url)
            .header("Authorization", format!("Bearer {}", token))
            .header("Content-type", "application/json; charset=utf-8")
            .body(body)
            .await?
            .body_string()
            .await?)
    }
    /// Send a post request to the slack web api.
    async fn post(&self, url: &str, token: &str) -> Result<String, Error> {
        let check_url = url::Url::parse(url)?;

        Ok(self
            .post(check_url)
            .header("Authorization", format!("Bearer {}", token))
            .await?
            .body_string()
            .await?)
    }
}

/// Returns the slack api url for each method.
pub fn get_slack_url(method: &str) -> String {
    format!("https://slack.com/api/{}", method)
}

/// Provides a default `surf` client to give to the API functions to send requests.
pub fn default_client() -> Client {
    surf::Client::new()
}

/// Slack WEB API default response.
#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct DefaultResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub response_metadata: Option<ResponseMetadata>,
}

/// Slack WEB API response metadata.
#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct ResponseMetadata {
    pub next_cursor: Option<String>,
    pub messages: Option<Vec<String>>,
    pub warnings: Option<Vec<String>>,
}
