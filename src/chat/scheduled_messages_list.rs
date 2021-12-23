use crate::error::Error;
use crate::http_client::{get_slack_url, ResponseMetadata, SlackWebAPIClient};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct ScheduledMessagesListRequest {
    pub channel: Option<String>,
    pub cursor: Option<String>,
    pub latest: Option<String>,
    pub limit: Option<i32>,
    pub oldest: Option<String>,
    pub team_id: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct ScheduledMessagesListResponse {
    pub ok: bool,
    pub error: Option<String>,
    // TODO
    //pub scheduled_messages: Option<>,
    pub response_metadata: Option<ResponseMetadata>,
}

pub async fn scheduled_messages_list<T>(
    client: &T,
    param: &ScheduledMessagesListRequest,
    bot_token: &str,
) -> Result<ScheduledMessagesListResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("chat.scheduledMessages.list");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<ScheduledMessagesListResponse>(&result)
                .map_err(Error::SerdeJsonError)
        })
}
