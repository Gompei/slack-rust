use crate::error::Error;
use crate::http_client::{get_slack_url, SlackWebAPIClient};
use crate::usergroups::usergroup::Usergroup;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct DisableRequest {
    pub usergroup: String,
    pub include_count: Option<bool>,
    pub team_id: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct DisableResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub usergroup: Option<Usergroup>,
}

pub async fn disable<T>(
    client: &T,
    param: &DisableRequest,
    bot_token: &str,
) -> Result<DisableResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("usergroups.disable");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<DisableResponse>(&result).map_err(Error::SerdeJsonError)
        })
}
