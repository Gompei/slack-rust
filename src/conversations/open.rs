use crate::channels::channel::Channel;
use crate::error::Error;
use crate::http_client::{get_slack_url, ResponseMetadata, SlackWebAPIClient};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct OpenRequest {
    pub channel: Option<String>,
    pub prevent_creation: Option<bool>,
    pub return_im: Option<bool>,
    pub users: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OpenResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub no_op: Option<bool>,
    pub already_open: Option<bool>,
    pub channel: Option<Channel>,
}

pub async fn open<T>(
    client: &T,
    param: &OpenRequest,
    bot_token: &str,
) -> Result<OpenResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("conversations.open");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<OpenResponse>(&result).map_err(Error::SerdeJsonError)
        })
}
