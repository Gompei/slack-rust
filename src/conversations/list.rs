use crate::channels::channel::Channel;
use crate::error::Error;
use crate::http_client::{get_slack_url, ResponseMetadata, SlackWebAPIClient};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ListRequest {
    pub channel: String,
    pub cursor: Option<String>,
    pub exclude_archived: Option<bool>,
    pub limit: Option<i32>,
    pub team_id: Option<String>,
    pub types: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ListResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub channels: Option<Vec<Channel>>,
    pub response_metadata: Option<ResponseMetadata>,
}

pub async fn list<T>(
    client: &T,
    param: &ListRequest,
    bot_token: &str,
) -> Result<ListResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("conversations.list");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<ListResponse>(&result).map_err(Error::SerdeJsonError)
        })
}
