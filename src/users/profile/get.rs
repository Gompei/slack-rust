use crate::error::Error;
use crate::http_client::{get_slack_url, SlackWebAPIClient};
use crate::profiles::profile::Profile;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct GetRequest {
    pub include_labels: Option<String>,
    pub user: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct GetResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub profile: Option<Profile>,
}

pub async fn get<T>(client: &T, param: &GetRequest, bot_token: &str) -> Result<GetResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("users.profile.get");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<GetResponse>(&result).map_err(Error::SerdeJsonError)
        })
}
