use serde::{Deserialize, Serialize};

use crate::chat::message::Message;
use crate::error::Error;
use crate::http_client::{get_slack_url, ResponseMetadata, SlackWebAPIClient};

#[derive(Deserialize, Serialize, Debug)]
pub struct HistoryRequest {
    pub channel: String,
    pub cursor: Option<String>,
    pub inclusive: Option<bool>,
    pub latest: Option<String>,
    pub limit: Option<i32>,
    pub oldest: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct HistoryResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub messages: Option<Message>,
    pub has_more: Option<bool>,
    pub pin_count: Option<i32>,
    pub response_metadata: Option<ResponseMetadata>,
}

pub async fn history<T>(
    client: &T,
    param: &HistoryRequest,
    bot_token: &str,
) -> Result<HistoryResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("conversations.history");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<HistoryResponse>(&result).map_err(Error::SerdeJsonError)
        })
}
