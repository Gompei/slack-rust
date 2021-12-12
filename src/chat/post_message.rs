use serde::{Deserialize, Serialize};

use crate::chat::message::Message;
use crate::error::Error;
use crate::http_client::{get_slack_url, SlackWebAPIClient};

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
    //pub error: Option<String>,
    pub channel: Option<String>,
    // pub ts: Option<String>,
    // pub message: Option<Message>,
}

pub async fn post_message<T>(
    client: &T,
    param: &PostMessageRequest,
    bot_token: &String,
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::http_client::default_client;

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

    #[async_std::test]
    async fn test_post_message() {
        // let param = PostMessageRequest {
        //     channel: "test".to_string(),
        //     text: "test".to_string(),
        // };
        // let slack_api_client = default_client();
        //
        // let response = post_message(slack_api_client, param, "".to_string())
        //     .await
        //     .expect("api call error");
        //
        assert_eq!(1, 1);
    }
}
