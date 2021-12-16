use crate::error::Error;
use crate::http_client::{get_slack_url, DefaultResponse, SlackWebAPIClient};
use crate::profiles::profile::Profile;
use crate::users::user::User;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct SetRequest {
    pub name: Option<String>,
    pub profile: Option<Profile>,
    pub user: Option<String>,
    pub value: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SetResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub profile: Option<Profile>,
}

pub async fn set<T>(client: &T, param: &SetRequest, bot_token: &str) -> Result<SetResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("users.profile.set");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<GetResponse>(&result).map_err(Error::SerdeJsonError)
        })
}
