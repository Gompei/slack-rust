use crate::error::Error;

use crate::http_client::{get_slack_url, DefaultResponse, SlackWebAPIClient};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct RemoveRequest {
    pub name: String,
    pub channel: Option<String>,
    pub file: Option<String>,
    pub file_comment: Option<String>,
    pub timestamp: Option<String>,
}

pub async fn remove<T>(
    client: &T,
    param: &RemoveRequest,
    bot_token: &str,
) -> Result<DefaultResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("reactions.remove");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<DefaultResponse>(&result).map_err(Error::SerdeJsonError)
        })
}
