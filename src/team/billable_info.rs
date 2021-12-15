use crate::error::Error;
use crate::http_client::{get_slack_url, SlackWebAPIClient};
use crate::reminders::add::Reminder;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct BillableInfoRequest {
    pub team_id: Option<String>,
    pub user: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct BillableInfoResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub billable_info: Option<BillableInfo>,
}

// TODO
// https://github.com/serde-rs/serde/issues/1387
#[derive(Deserialize, Serialize, Debug)]
pub struct BillableInfo {}

pub async fn billable_info<T>(
    client: &T,
    param: &BillableInfoRequest,
    bot_token: &str,
) -> Result<BillableInfoResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("team.billableInfo");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<BillableInfoResponse>(&result).map_err(Error::SerdeJsonError)
        })
}
