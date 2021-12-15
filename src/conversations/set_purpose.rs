use crate::error::Error;
use crate::http_client::{get_slack_url, SlackWebAPIClient};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct SetPurposeRequest {
    pub channel: String,
    pub purpose: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SetPurposeResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub purpose: Option<String>,
}

pub async fn set_purpose<T>(
    client: &T,
    param: &SetPurposeRequest,
    bot_token: &str,
) -> Result<SetPurposeResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("conversations.setPurpose");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<SetPurposeResponse>(&result).map_err(Error::SerdeJsonError)
        })
}
