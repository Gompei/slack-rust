use serde::{Deserialize, Serialize};

use crate::attachments::attachment::Attachment;
use crate::block::blocks::Blocks;
use crate::chat::message::Message;
use crate::error::Error;
use crate::http_client::{get_slack_url, SlackWebAPIClient};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ScheduledMessageRequest {
    pub channel: String,
    pub post_at: i32,
    pub text: String,
    pub as_user: Option<bool>,
    pub attachments: Option<Vec<Attachment>>,
    pub blocks: Option<Vec<Blocks>>,
    pub link_names: Option<bool>,
    pub parse: Option<String>,
    pub reply_broadcast: Option<String>,
    pub thread_ts: Option<String>,
    pub unfurl_links: Option<bool>,
    pub unfurl_media: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ScheduledMessageResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub channel: Option<String>,
    pub scheduled_message_id: Option<String>,
    pub post_at: Option<String>,
    pub message: Option<Message>,
}

pub async fn scheduled_message<T>(
    client: &T,
    param: &ScheduledMessageRequest,
    bot_token: &str,
) -> Result<ScheduledMessageResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("chat.scheduleMessage");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<ScheduledMessageResponse>(&result).map_err(Error::SerdeJsonError)
        })
}
