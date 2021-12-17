use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::http_client::{get_slack_url, SlackWebAPIClient};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct DeleteRequest {
    pub channel: String,
    pub ts: String,
    pub as_user: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct DeleteResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub ts: Option<String>,
}

pub async fn delete<T>(
    client: &T,
    param: &DeleteRequest,
    bot_token: &str,
) -> Result<DeleteResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("chat.delete");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<DeleteResponse>(&result).map_err(Error::SerdeJsonError)
        })
}
