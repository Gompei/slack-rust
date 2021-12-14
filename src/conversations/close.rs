use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::http_client::{get_slack_url, SlackWebAPIClient};

#[derive(Deserialize, Serialize, Debug)]
pub struct CloseRequest {
    pub channel_name: String,
    pub channel_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CloseResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub no_op: Option<bool>,
    pub already_closed: Option<bool>,
}

pub async fn close<T>(
    client: &T,
    param: &CloseRequest,
    bot_token: &str,
) -> Result<CloseResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("conversations.close");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<CloseResponse>(&result).map_err(Error::SerdeJsonError)
        })
}
