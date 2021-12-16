use crate::error::Error;
use crate::http_client::{get_slack_url, ResponseMetadata, SlackWebAPIClient};
use crate::users::user::User;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct LookupByEmailRequest {
    pub email: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LookupByEmailResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub user: Option<User>,
}

pub async fn lookup_by_email<T>(
    client: &T,
    param: &LookupByEmailRequest,
    bot_token: &str,
) -> Result<LookupByEmailResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("users.lookupByEmail");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<LookupByEmailResponse>(&result).map_err(Error::SerdeJsonError)
        })
}
