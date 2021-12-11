use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::http_client::{get_slack_url, SlackWebAPIClient};

#[derive(Deserialize, Serialize, Debug)]
pub struct InfoRequest {
    pub team_id: Option<String>,
    pub user: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct InfoResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub dnd_enabled: Option<bool>,
    pub next_dnd_start_ts: Option<i32>,
    pub next_dnd_end_ts: Option<i32>,
    pub snooze_enabled: Option<bool>,
    pub snooze_endtime: Option<i32>,
    pub snooze_remaining: Option<i16>,
    pub snooze_is_indefinite: Option<bool>,
}

pub async fn info<T>(
    client: &T,
    param: InfoRequest,
    bot_token: String,
) -> Result<InfoResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("dnd.info");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(url, json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<InfoResponse>(&result).map_err(Error::SerdeJsonError)
        })
}
