use crate::channels::channel::Channel;
use crate::error::Error;
use crate::http_client::{get_slack_url, SlackWebAPIClient};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct InviteRequest {
    pub channel: String,
    pub include_locale: Option<bool>,
    pub include_num_members: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct InviteResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub channel: Option<Channel>,
}

pub async fn invite<T>(
    client: &T,
    param: &InviteRequest,
    bot_token: &str,
) -> Result<InviteResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("conversations.invite");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<InviteResponse>(&result).map_err(Error::SerdeJsonError)
        })
}
