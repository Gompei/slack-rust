use crate::error::Error;
use crate::http_client::{get_slack_url, SlackWebAPIClient};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct InviteSharedRequest {
    pub channel: String,
    pub emails: Option<Vec<String>>,
    pub external_limited: Option<bool>,
    pub user_ids: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct InviteSharedResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub invite_id: Option<String>,
    pub conf_code: Option<String>,
    pub url: Option<String>,
    pub is_legacy_shared_channel: Option<bool>,
}

pub async fn invite_shared<T>(
    client: &T,
    param: &InviteSharedRequest,
    bot_token: &str,
) -> Result<InviteSharedResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("conversations.inviteShared");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<InviteSharedResponse>(&result).map_err(Error::SerdeJsonError)
        })
}
