use crate::error::Error;
use crate::http_client::{get_slack_url, DefaultResponse, SlackWebAPIClient};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct UnfurlRequest {
    pub channel: String,
    pub ts: String,
    // TODO
    pub unfurls: String,
    pub source: Option<String>,
    pub unfurl_id: Option<String>,
    // TODO
    //pub user_auth_blocks:
    //pub user_auth_message
    pub user_auth_required: Option<bool>,
    //pub user_auth_url:
}

pub async fn unfurl<T>(
    client: &T,
    param: &UnfurlRequest,
    bot_token: &str,
) -> Result<DefaultResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("chat.unfurl");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<DefaultResponse>(&result).map_err(Error::SerdeJsonError)
        })
}
