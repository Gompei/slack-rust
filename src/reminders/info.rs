use crate::error::Error;
use crate::http_client::{get_slack_url, SlackWebAPIClient};
use crate::reminders::add::Reminder;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct InfoRequest {
    pub reminder: String,
    pub team_id: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct InfoResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub reminder: Option<Reminder>,
}

pub async fn info<T>(
    client: &T,
    param: &InfoRequest,
    bot_token: &str,
) -> Result<InfoResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("reminders.info");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<InfoResponse>(&result).map_err(Error::SerdeJsonError)
        })
}
