use crate::error::Error;
use crate::http_client::{get_slack_url, SlackWebAPIClient};
use crate::usergroups::usergroup::Usergroup;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateRequest {
    pub name: String,
    pub channels: Option<Vec<String>>,
    pub description: Option<String>,
    pub handle: Option<String>,
    pub include_count: Option<bool>,
    pub team_id: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub usergroup: Option<Usergroup>,
}

pub async fn create<T>(
    client: &T,
    param: &CreateRequest,
    bot_token: &str,
) -> Result<CreateResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("usergroups.create");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<CreateResponse>(&result).map_err(Error::SerdeJsonError)
        })
}