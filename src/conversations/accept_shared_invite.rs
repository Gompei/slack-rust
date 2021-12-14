use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::http_client::{get_slack_url, SlackWebAPIClient};

#[derive(Deserialize, Serialize, Debug)]
pub struct AcceptSharedInviteRequest {
    pub channel_name: String,
    pub channel_id: Option<String>,
    pub free_trial_accepted: Option<String>,
    pub invite_id: Option<String>,
    pub is_private: Option<String>,
    pub team_id: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AcceptSharedInviteResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub implicit_approval: Option<bool>,
    pub channel_id: Option<String>,
    pub invite_id: Option<String>,
}

pub async fn accept_shared_invite<T>(
    client: &T,
    param: &AcceptSharedInviteRequest,
    bot_token: &str,
) -> Result<AcceptSharedInviteResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("conversations.acceptSharedInvite");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<AcceptSharedInviteResponse>(&result)
                .map_err(Error::SerdeJsonError)
        })
}
