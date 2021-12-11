use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::http_client::{get_slack_url, post_json, Client};

#[derive(Deserialize, Serialize, Debug)]
pub struct PostMessageRequest {
    pub channel: String,
    pub text: String,
    // pub as_user: Option<String>,
    // pub username: Option<String>,
    // pub parse: Option<String>,
    // pub thread_ts: Option<String>,
    // pub reply_broadcast: Option<String>,
    // pub link_names: Option<i32>,
    // pub unfurl_links: Option<bool>,
    // pub unfurl_media: Option<bool>,
    // pub icon_url: Option<String>,
    // pub icon_emoji: Option<String>,
    // pub mrkdwn: Option<bool>,
    // pub escape_text: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PostMessageResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub channel: Option<String>,
}

pub async fn post_message(
    client: Client,
    param: PostMessageRequest,
    bot_token: String,
) -> Result<PostMessageResponse, Error> {
    let url = get_slack_url("chat.postMessage");
    let json = serde_json::to_string(&param)?;

    post_json(client, url, json, bot_token)
        .await?
        .body_json()
        .await
        .map_err(Error::SurfError)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::chat::post_message::PostMessageRequest;

    #[test]
    fn convert_json_request() {
        let request = PostMessageRequest {
            channel: "test channel".to_string(),
            text: "test text".to_string(),
        };
        let json = serde_json::to_string_pretty(&request).unwrap();
        let expected = r#"{
  "channel": "test channel",
  "text": "test text"
}"#;

        assert_eq!(json, expected);
    }

    #[test]
    fn convert_json_response() {
        let response = PostMessageResponse {
            ok: false,
            channel: Some("test channel".to_string()),
        };
        let json = serde_json::to_string_pretty(&response).unwrap();
        let expected = r#"{
  "ok": false,
  "channel": "test channel"
}"#;

        assert_eq!(json, expected);
    }
}
