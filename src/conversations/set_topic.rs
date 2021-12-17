use crate::error::Error;
use crate::http_client::{get_slack_url, SlackWebAPIClient};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct SetTopicRequest {
    pub channel: String,
    pub topic: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct SetTopicResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub topic: Option<String>,
}

pub async fn leave<T>(
    client: &T,
    param: &SetTopicRequest,
    bot_token: &str,
) -> Result<SetTopicResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("conversations.setTopic");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<SetTopicResponse>(&result).map_err(Error::SerdeJsonError)
        })
}
