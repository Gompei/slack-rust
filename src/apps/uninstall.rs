use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::http_client::{get_slack_url, DefaultResponse, SlackWebAPIClient};

#[derive(Deserialize, Serialize, Debug)]
pub struct UninstallRequest {
    pub client_id: String,
    pub client_secret: String,
}

pub async fn uninstall<T>(
    client: &T,
    param: UninstallRequest,
    bot_token: String,
) -> Result<DefaultResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("apps.uninstall");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(url, json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<DefaultResponse>(&result).map_err(Error::SerdeJsonError)
        })
}
