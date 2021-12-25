use crate::attachment::attachment::Attachment;
use crate::block::blocks::Block;
use crate::error::Error;
use crate::http_client::{get_slack_url, SlackWebAPIClient};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct PostEphemeralRequest {
    pub channel: String,
    pub text: String,
    pub user: String,
    pub as_user: Option<String>,
    pub attachments: Option<Vec<Attachment>>,
    pub blocks: Option<Vec<Block>>,
    pub icon_emoji: Option<String>,
    pub icon_url: Option<String>,
    pub link_names: Option<bool>,
    pub parse: Option<String>,
    pub thread_ts: Option<String>,
    pub username: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct PostEphemeralResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub message_ts: Option<String>,
}

pub async fn post_ephemeral<T>(
    client: &T,
    param: &PostEphemeralRequest,
    bot_token: &str,
) -> Result<PostEphemeralResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("chat.postEphemeral");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<PostEphemeralResponse>(&result).map_err(Error::SerdeJsonError)
        })
}
