use crate::channels::channel::Channel;
use crate::error::Error;
use crate::http_client::{get_slack_url, ResponseMetadata, SlackWebAPIClient};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct ConversationsRequest {
    pub cursor: Option<String>,
    pub exclude_archived: Option<String>,
    pub limit: Option<i32>,
    pub team_id: Option<String>,
    pub r#types: Option<String>,
    pub user: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct ConversationsResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub channels: Option<Vec<Channel>>,
    pub response_metadata: Option<ResponseMetadata>,
}

pub async fn conversations<T>(
    client: &T,
    param: &ConversationsRequest,
    bot_token: &str,
) -> Result<ConversationsResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("users.conversations");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<ConversationsResponse>(&result).map_err(Error::SerdeJsonError)
        })
}
