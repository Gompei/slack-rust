use crate::channels::channel::Channel;
use crate::error::Error;
use crate::http_client::{get_slack_url, SlackWebAPIClient};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct RenameRequest {
    pub channel: String,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct RenameResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub channel: Option<Channel>,
}

pub async fn rename<T>(
    client: &T,
    param: &RenameRequest,
    bot_token: &str,
) -> Result<RenameResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("conversations.rename");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<RenameResponse>(&result).map_err(Error::SerdeJsonError)
        })
}
