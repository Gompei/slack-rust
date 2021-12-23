use crate::error::Error;
use crate::http_client::{get_slack_url, SlackWebAPIClient};
use crate::usergroups::usergroup::Usergroup;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct EnableRequest {
    pub usergroup: String,
    pub include_count: Option<bool>,
    pub team_id: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct EnableResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub usergroup: Option<Usergroup>,
}

pub async fn enable<T>(
    client: &T,
    param: &EnableRequest,
    bot_token: &str,
) -> Result<EnableResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("usergroups.enable");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<EnableResponse>(&result).map_err(Error::SerdeJsonError)
        })
}
