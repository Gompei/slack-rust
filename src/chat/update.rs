use crate::attachments::attachment::Attachment;
use crate::block::blocks::Block;
use crate::chat::message::Message;
use crate::error::Error;
use crate::http_client::{get_slack_url, SlackWebAPIClient};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct UpdateRequest {
    pub channel: String,
    pub ts: String,
    pub as_user: Option<bool>,
    pub attachments: Option<Vec<Attachment>>,
    pub blocks: Option<Vec<Box<dyn Block>>>,
    pub file_ids: Option<Vec<String>>,
    pub link_names: Option<bool>,
    pub parse: Option<String>,
    pub reply_broadcast: Option<String>,
    pub text: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct UpdateResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub channel: Option<String>,
    pub ts: Option<String>,
    pub text: Option<String>,
    pub message: Option<Message>,
}

pub async fn update<T>(
    client: &T,
    param: &UpdateRequest,
    bot_token: &str,
) -> Result<UpdateResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("chat.update");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<UpdateResponse>(&result).map_err(Error::SerdeJsonError)
        })
}
