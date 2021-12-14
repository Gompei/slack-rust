use crate::channels::channel::Channel;
use crate::error::Error;
use crate::http_client::{get_slack_url, DefaultResponse, ResponseMetadata, SlackWebAPIClient};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct KickRequest {
    pub channel: String,
    pub user: String,
}

pub async fn kick<T>(
    client: &T,
    param: &KickRequest,
    bot_token: &str,
) -> Result<DefaultResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("conversations.kick");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<DefaultResponse>(&result).map_err(Error::SerdeJsonError)
        })
}
