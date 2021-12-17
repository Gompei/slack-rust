use crate::error::Error;
use crate::http_client::{get_slack_url, DefaultResponse, SlackWebAPIClient};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct DeleteRequest {
    pub reminder: String,
    pub team_id: Option<String>,
}

pub async fn delete<T>(
    client: &T,
    param: &DeleteRequest,
    bot_token: &str,
) -> Result<DefaultResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("reminders.delete");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<DefaultResponse>(&result).map_err(Error::SerdeJsonError)
        })
}
