use crate::error::Error;
use crate::http_client::{get_slack_url, SlackWebAPIClient};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct GetPresenceRequest {
    pub cursor: Option<String>,
    pub exclude_archived: Option<String>,
    pub limit: Option<i32>,
    pub team_id: Option<String>,
    pub r#types: Option<String>,
    pub user: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct GetPresenceResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub presence: Option<String>,
    pub online: Option<bool>,
    pub auto_away: Option<bool>,
    pub manual_away: Option<bool>,
    pub connection_count: Option<i8>,
    pub last_activity: Option<i32>,
}

pub async fn get_presence<T>(
    client: &T,
    param: &GetPresenceRequest,
    bot_token: &str,
) -> Result<GetPresenceResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("users.getPresence");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<GetPresenceResponse>(&result).map_err(Error::SerdeJsonError)
        })
}
