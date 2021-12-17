use crate::chat::message::Message;
use crate::error::Error;
use crate::http_client::{get_slack_url, ResponseMetadata, SlackWebAPIClient};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct RepliesRequest {
    pub channel: String,
    pub ts: String,
    pub cursor: Option<String>,
    pub inclusive: Option<bool>,
    pub latest: Option<String>,
    pub limit: Option<i32>,
    pub oldest: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct RepliesResponse {
    pub ok: bool,
    pub error: Option<String>,
    // TODO: 別のディレクトリに切り出しても良いかも
    pub messages: Option<Vec<Message>>,
    pub has_more: Option<bool>,
    pub response_metadata: Option<ResponseMetadata>,
}

pub async fn replies<T>(
    client: &T,
    param: &RepliesRequest,
    bot_token: &str,
) -> Result<RepliesResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("conversations.replies");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<RepliesResponse>(&result).map_err(Error::SerdeJsonError)
        })
}
