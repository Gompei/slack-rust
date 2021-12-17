use crate::attachments::attachment::Attachment;
use crate::block::blocks::Blocks;
use crate::chat::message::Message;
use crate::error::Error;
use crate::http_client::{get_slack_url, ResponseMetadata, SlackWebAPIClient};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct PostMessageRequest {
    pub channel: String,
    pub attachments: Option<Vec<Attachment>>,
    pub blocks: Option<Blocks>,
    // pub text: Option<String>,
    // pub as_user: Option<String>,
    // pub icon_emoji: Option<String>,
    // pub icon_url: Option<String>,
    // pub link_names: Option<i32>,
    // pub mrkdwn: Option<bool>,
    // pub parse: Option<String>,
    // pub reply_broadcast: Option<String>,
    // pub thread_ts: Option<String>,
    // pub unfurl_links: Option<bool>,
    // pub unfurl_media: Option<bool>,
    // pub username: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct PostMessageResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub response_metadata: Option<ResponseMetadata>,
    pub channel: Option<String>,
    pub ts: Option<String>,
    pub message: Option<Message>,
}

pub async fn post_message<T>(
    client: &T,
    param: &PostMessageRequest,
    bot_token: &str,
) -> Result<PostMessageResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("chat.postMessage");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<PostMessageResponse>(&result).map_err(Error::SerdeJsonError)
        })
}
