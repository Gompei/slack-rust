use crate::error::Error;
use crate::http_client::{get_slack_url, SlackWebAPIClient};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct MeMessageRequest {
    pub channel: String,
    pub text: String,
}

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct MeMessageResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub ts: Option<String>,
}

pub async fn me_message<T>(
    client: &T,
    param: &MeMessageRequest,
    bot_token: &str,
) -> Result<MeMessageResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("chat.meMessage");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<MeMessageResponse>(&result).map_err(Error::SerdeJsonError)
        })
}
