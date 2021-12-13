use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::http_client::{get_slack_url, DefaultResponse, SlackWebAPIClient};

#[derive(Deserialize, Serialize, Debug)]
pub struct AddRequest {
    pub text: String,
    pub time: String,
    pub recurrence: Option<Recurrence>,
    pub team_id: Option<String>,
    pub user: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Recurrence {
    // TODO: https://api.slack.com/methods/reminders.add
    pub frequency: Option<String>,
    pub weekdays: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AddResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub reminder: Option<Reminder>,
}

// TODO: https://api.slack.com/methods/reminders.add
#[derive(Deserialize, Serialize, Debug)]
pub struct Reminder {
    pub id: String,
    pub creator: String,
    pub user: String,
    pub text: String,
    pub recurring: bool,
    pub time: i32,
    pub complete_ts: i8,
}

pub async fn add<T>(client: &T, param: &AddRequest, bot_token: &str) -> Result<AddResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("reminders.add");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<AddResponse>(&result).map_err(Error::SerdeJsonError)
        })
}
