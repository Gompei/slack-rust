use crate::channels::channel::Channel;
use crate::error::Error;
use crate::http_client::{get_slack_url, ResponseMetadata, SlackWebAPIClient};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct LeaveRequest {
    pub channel: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LeaveResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub not_in_channel: Option<bool>,
}

pub async fn leave<T>(
    client: &T,
    param: &LeaveRequest,
    bot_token: &str,
) -> Result<LeaveResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("conversations.leave");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<LeaveResponse>(&result).map_err(Error::SerdeJsonError)
        })
}
