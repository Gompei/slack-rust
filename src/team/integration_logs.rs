use crate::error::Error;
use crate::http_client::{get_slack_url, SlackWebAPIClient};
use crate::team::access_logs::Paging;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct IntegrationLogsRequest {
    pub app_id: Option<String>,
    pub change_type: Option<String>,
    pub count: Option<String>,
    pub page: Option<String>,
    pub service_id: Option<String>,
    pub team_id: Option<String>,
    pub user: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct IntegrationLogsResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub logs: Option<Vec<Log>>,
    pub paging: Option<Paging>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Log {
    pub app_id: Option<String>,
    pub app_type: Option<String>,
    pub service_id: Option<i32>,
    pub service_type: Option<String>,
    pub user_id: Option<String>,
    pub user_name: Option<String>,
    pub channel: Option<String>,
    pub date: Option<String>,
    pub change_type: Option<String>,
    pub scope: Option<String>,
}

pub async fn integration_logs<T>(
    client: &T,
    param: &IntegrationLogsRequest,
    bot_token: &str,
) -> Result<IntegrationLogsResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("team.integrationLogs");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<IntegrationLogsResponse>(&result).map_err(Error::SerdeJsonError)
        })
}
