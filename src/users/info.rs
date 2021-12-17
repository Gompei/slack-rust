use crate::error::Error;
use crate::http_client::{get_slack_url, SlackWebAPIClient};
use crate::users::user::User;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct InfoRequest {
    pub user: String,
    pub include_locale: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct InfoResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub user: Option<User>,
}

pub async fn info<T>(
    client: &T,
    param: &InfoRequest,
    bot_token: &str,
) -> Result<InfoResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("users.info");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<InfoResponse>(&result).map_err(Error::SerdeJsonError)
        })
}
