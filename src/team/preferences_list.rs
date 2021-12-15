use crate::error::Error;
use crate::http_client::{get_slack_url, SlackWebAPIClient};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct PreferencesListResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub allow_message_deletion: Option<bool>,
    pub display_real_names: Option<bool>,
    pub disable_file_uploads: Option<String>,
    pub msg_edit_window_mins: Option<i32>,
    pub who_can_post_general: Option<String>,
}

pub async fn preferences_list<T>(
    client: &T,
    bot_token: &str,
) -> Result<PreferencesListResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("team.preferences.list");

    client.post(&url, bot_token).await.and_then(|result| {
        serde_json::from_str::<PreferencesListResponse>(&result).map_err(Error::SerdeJsonError)
    })
}
