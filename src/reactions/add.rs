use crate::error::Error;
use crate::http_client::{get_slack_url, DefaultResponse, SlackWebAPIClient};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct AddRequest {
    pub channel: String,
    pub name: String,
    pub timestamp: String,
}

pub async fn add<T>(
    client: &T,
    param: &AddRequest,
    bot_token: &str,
) -> Result<DefaultResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("reactions.add");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<DefaultResponse>(&result).map_err(Error::SerdeJsonError)
        })
}
