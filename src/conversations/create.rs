use crate::channels::channel::Channel;
use crate::error::Error;
use crate::http_client::{get_slack_url, SlackWebAPIClient};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct CreateRequest {
    pub channel_name: String,
    pub name: String,
    pub is_private: Option<bool>,
    pub team_id: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct CreateResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub channel: Option<Channel>,
}

pub async fn create<T>(
    client: &T,
    param: &CreateRequest,
    bot_token: &str,
) -> Result<CreateResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("conversations.create");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<CreateResponse>(&result).map_err(Error::SerdeJsonError)
        })
}
