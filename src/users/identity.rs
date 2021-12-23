use crate::error::Error;
use crate::http_client::{get_slack_url, SlackWebAPIClient};
use crate::team::info::Team;
use crate::users::user::User;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct IdentityResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub user: Option<User>,
    pub team: Option<Team>,
}

pub async fn identity<T>(client: &T, bot_token: &str) -> Result<IdentityResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("users.identity");

    client.post(&url, bot_token).await.and_then(|result| {
        serde_json::from_str::<IdentityResponse>(&result).map_err(Error::SerdeJsonError)
    })
}
