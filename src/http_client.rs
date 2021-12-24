use crate::error::Error;
use async_trait::async_trait;
use mockall::*;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[automock]
#[async_trait]
pub trait SlackWebAPIClient {
    async fn post_multipart_forms(
        &self,
        url: &str,
        body: &str,
        token: &str,
    ) -> Result<String, Error>;
    async fn post_json(&self, url: &str, body: &str, token: &str) -> Result<String, Error>;
    async fn post(&self, url: &str, token: &str) -> Result<String, Error>;
}

pub type Client = surf::Client;

/// Slack default response.
#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct DefaultResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub response_metadata: Option<ResponseMetadata>,
}

/// Metadata.
#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, PartialEq)]
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
    // TODO
    /// Send a post request to the slack api.
    async fn post_multipart_forms(
        &self,
        url: &str,
        body: &str,
        token: &str,
    ) -> Result<String, Error> {
        let check_url = url::Url::parse(url)?;

        Ok(self
            .post(check_url)
            .header("Authorization", format!("Bearer {}", token))
            .content_type(surf::http::mime::MULTIPART_FORM)
            .body(body)
            .await?
            .body_string()
            .await?)
    }

    /// Send a post request to the slack api.
    async fn post_json(&self, url: &str, body: &str, token: &str) -> Result<String, Error> {
        let check_url = url::Url::parse(url)?;

        Ok(self
            .post(check_url)
            .header("Authorization", format!("Bearer {}", token))
            // TODO
            .header("Content-type", "application/json; charset=utf-8")
            //.content_type(surf::http::mime::JSON)
            .body(body)
            .await?
            .body_string()
            .await?)
    }

    /// Send a post request to the slack api.
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
