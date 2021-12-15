use crate::error::Error;
use crate::http_client::{get_slack_url, SlackWebAPIClient};
use crate::reminders::add::Reminder;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct AccessLogsRequest {
    pub before: Option<String>,
    pub count: Option<String>,
    pub page: Option<String>,
    pub team_id: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AccessLogsResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub logins: Option<Vec<Login>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Login {
    pub user_id: Option<String>,
    pub username: Option<String>,
    pub date_first: Option<i32>,
    pub date_last: Option<i32>,
    pub count: Option<i32>,
    pub ip: Option<String>,
    pub user_agent: Option<String>,
    pub isp: Option<String>,
    pub country: Option<String>,
    pub region: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct paging {
    pub count: Option<i32>,
    pub total: Option<i32>,
    pub page: Option<i32>,
    pub pages: Option<i32>,
}

pub async fn access_logs<T>(
    client: &T,
    param: &AccessLogsRequest,
    bot_token: &str,
) -> Result<AccessLogsResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("team.accessLogs");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<AccessLogsResponse>(&result).map_err(Error::SerdeJsonError)
        })
}
