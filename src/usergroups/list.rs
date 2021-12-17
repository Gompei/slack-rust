use crate::error::Error;
use crate::http_client::{get_slack_url, SlackWebAPIClient};
use crate::usergroups::usergroup::Usergroup;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ListRequest {
    pub usergroup: String,
    pub include_count: Option<bool>,
    pub include_disabled: Option<bool>,
    pub include_users: Option<bool>,
    pub team_id: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ListResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub usergroups: Option<Vec<Usergroup>>,
}

pub async fn list<T>(
    client: &T,
    param: &ListRequest,
    bot_token: &str,
) -> Result<ListResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("usergroups.list");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<ListResponse>(&result).map_err(Error::SerdeJsonError)
        })
}
