use crate::error::Error;
use crate::http_client::{get_slack_url, SlackWebAPIClient};
use crate::usergroups::usergroup::Usergroup;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct UpdateRequest {
    pub usergroup: String,
    pub channels: Option<String>,
    pub description: Option<String>,
    pub handle: Option<String>,
    pub include_count: Option<bool>,
    pub name: Option<String>,
    pub team_id: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct UpdateResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub usergroups: Option<Vec<Usergroup>>,
}

pub async fn update<T>(
    client: &T,
    param: &UpdateRequest,
    bot_token: &str,
) -> Result<UpdateResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("usergroups.update");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<UpdateResponse>(&result).map_err(Error::SerdeJsonError)
        })
}
