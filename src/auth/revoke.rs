use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::http_client::{get_slack_url, SlackWebAPIClient};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct RevokeRequest {
    pub test: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct RevokeResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub revoked: Option<bool>,
}

pub async fn revoke<T>(
    client: &T,
    param: &RevokeRequest,
    bot_token: &str,
) -> Result<RevokeResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("auth.revoke");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<RevokeResponse>(&result).map_err(Error::SerdeJsonError)
        })
}
