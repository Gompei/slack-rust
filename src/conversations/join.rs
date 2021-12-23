use crate::channels::channel::Channel;
use crate::error::Error;
use crate::http_client::{get_slack_url, ResponseMetadata, SlackWebAPIClient};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct JoinRequest {
    pub channel: String,
}

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct JoinResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub channel: Option<Channel>,
    pub warning: Option<String>,
    pub response_metadata: Option<ResponseMetadata>,
}

pub async fn join<T>(
    client: &T,
    param: &JoinRequest,
    bot_token: &str,
) -> Result<JoinResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("conversations.join");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<JoinResponse>(&result).map_err(Error::SerdeJsonError)
        })
}
