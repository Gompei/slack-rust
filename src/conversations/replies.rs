use crate::chat::message::Message;
use crate::error::Error;
use crate::http_client::{get_slack_url, ResponseMetadata, SlackWebAPIClient};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct RepliesRequest {
    pub channel: String,
    pub ts: String,
    pub cursor: Option<String>,
    pub inclusive: Option<bool>,
    pub latest: Option<String>,
    pub limit: Option<i32>,
    pub oldest: Option<String>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct RepliesResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub response_metadata: Option<ResponseMetadata>,
    pub messages: Option<Vec<Message>>,
    pub has_more: Option<bool>,
}

pub async fn replies<T>(
    client: &T,
    param: &RepliesRequest,
    bot_token: &str,
) -> Result<RepliesResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("conversations.replies");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<RepliesResponse>(&result).map_err(Error::SerdeJsonError)
        })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::http_client::MockSlackWebAPIClient;

    #[test]
    fn convert_request() {
        let request = RepliesRequest {
            channel: "C1234567890".to_string(),
            ts: "1234567890.123456".to_string(),
            cursor: Some("dXNlcjpVMDYxTkZUVDI=".to_string()),
            inclusive: Some(true),
            latest: Some("1234567890.123456".to_string()),
            limit: Some(20),
            oldest: Some("1234567890.123456".to_string()),
        };
        let json = r##"{
  "channel": "C1234567890",
  "ts": "1234567890.123456",
  "cursor": "dXNlcjpVMDYxTkZUVDI=",
  "inclusive": true,
  "latest": "1234567890.123456",
  "limit": 20,
  "oldest": "1234567890.123456"
}"##;

        let j = serde_json::to_string_pretty(&request).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<RepliesRequest>(json).unwrap();
        assert_eq!(request, s);
    }

    #[test]
    fn convert_response() {
        let response = RepliesResponse {
            ok: true,
            messages: Some(vec![
                Message {
                    type_file: Some("message".to_string()),
                    text: Some("island".to_string()),
                    user: Some("U061F7AUR".to_string()),
                    ts: Some("1482960137.003543".to_string()),
                    ..Default::default()
                },
                Message {
                    type_file: Some("message".to_string()),
                    text: Some("island".to_string()),
                    user: Some("U061F7AUR".to_string()),
                    ts: Some("1482960137.003543".to_string()),
                    ..Default::default()
                },
                Message {
                    type_file: Some("message".to_string()),
                    text: Some("island".to_string()),
                    user: Some("U061F7AUR".to_string()),
                    ts: Some("1482960137.003543".to_string()),
                    ..Default::default()
                },
            ]),
            has_more: Some(true),
            ..Default::default()
        };
        let json = r##"{
  "ok": true,
  "messages": [
    {
      "type": "message",
      "text": "island",
      "user": "U061F7AUR",
      "ts": "1482960137.003543"
    },
    {
      "type": "message",
      "text": "island",
      "user": "U061F7AUR",
      "ts": "1482960137.003543"
    },
    {
      "type": "message",
      "text": "island",
      "user": "U061F7AUR",
      "ts": "1482960137.003543"
    }
  ],
  "has_more": true
}"##;

        let j = serde_json::to_string_pretty(&response).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<RepliesResponse>(json).unwrap();
        assert_eq!(response, s);
    }

    #[async_std::test]
    async fn test_replies() {
        let param = RepliesRequest {
            channel: "C1234567890".to_string(),
            ts: "1234567890.123456".to_string(),
            cursor: Some("dXNlcjpVMDYxTkZUVDI=".to_string()),
            inclusive: Some(true),
            latest: Some("1234567890.123456".to_string()),
            limit: Some(20),
            oldest: Some("1234567890.123456".to_string()),
        };
        let mut mock = MockSlackWebAPIClient::new();
        mock.expect_post_json().returning(|_, _, _| {
            Ok(r##"{
  "ok": true,
  "messages": [
    {
      "type": "message",
      "text": "island",
      "user": "U061F7AUR",
      "ts": "1482960137.003543"
    },
    {
      "type": "message",
      "text": "island",
      "user": "U061F7AUR",
      "ts": "1482960137.003543"
    },
    {
      "type": "message",
      "text": "island",
      "user": "U061F7AUR",
      "ts": "1482960137.003543"
    }
  ],
  "has_more": true
}"##
            .to_string())
        });

        let response = replies(&mock, &param, &"test_token".to_string())
            .await
            .unwrap();
        let expect = RepliesResponse {
            ok: true,
            messages: Some(vec![
                Message {
                    type_file: Some("message".to_string()),
                    text: Some("island".to_string()),
                    user: Some("U061F7AUR".to_string()),
                    ts: Some("1482960137.003543".to_string()),
                    ..Default::default()
                },
                Message {
                    type_file: Some("message".to_string()),
                    text: Some("island".to_string()),
                    user: Some("U061F7AUR".to_string()),
                    ts: Some("1482960137.003543".to_string()),
                    ..Default::default()
                },
                Message {
                    type_file: Some("message".to_string()),
                    text: Some("island".to_string()),
                    user: Some("U061F7AUR".to_string()),
                    ts: Some("1482960137.003543".to_string()),
                    ..Default::default()
                },
            ]),
            has_more: Some(true),
            ..Default::default()
        };

        assert_eq!(expect, response);
    }
}
