use crate::error::Error;
use crate::http_client::{get_slack_url, SlackWebAPIClient};
use crate::profiles::profile::Profile;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct ProfileGetRequest {
    pub visibility: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct ProfileGetResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub profile: Option<Profile>,
}

pub async fn profile_get<T>(
    client: &T,
    param: &ProfileGetRequest,
    bot_token: &str,
) -> Result<ProfileGetResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("team.profile.get");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<ProfileGetResponse>(&result).map_err(Error::SerdeJsonError)
        })
}
