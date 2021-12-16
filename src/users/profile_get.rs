use crate::error::Error;
use crate::http_client::{get_slack_url, DefaultResponse, SlackWebAPIClient};
use crate::team::profile_get::Profile;
use crate::users::user::User;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct ProfileGetRequest {
    pub include_labels: Option<String>,
    pub user: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ProfileGetResponse {
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
    let url = get_slack_url("users.profile.get");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<ProfileGetResponse>(&result).map_err(Error::SerdeJsonError)
        })
}
