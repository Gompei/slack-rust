use crate::error::Error;
use crate::http_client::{get_slack_url, SlackWebAPIClient};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct GetPermalinkRequest {
    pub channel: String,
    pub message_ts: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct GetPermalinkResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub permalink: Option<String>,
}

pub async fn get_permalink<T>(
    client: &T,
    param: &GetPermalinkRequest,
    bot_token: &str,
) -> Result<GetPermalinkResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("chat.getPermalink");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<GetPermalinkResponse>(&result).map_err(Error::SerdeJsonError)
        })
}
