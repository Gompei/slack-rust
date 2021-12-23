use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::http_client::{get_slack_url, DefaultResponse, SlackWebAPIClient};

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct ArchiveRequest {
    pub channel: String,
}

pub async fn archive<T>(
    client: &T,
    param: &ArchiveRequest,
    bot_token: &str,
) -> Result<DefaultResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("conversations.archive");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<DefaultResponse>(&result).map_err(Error::SerdeJsonError)
        })
}
