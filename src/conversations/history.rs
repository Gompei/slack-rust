//! Fetches a conversation's history of messages and events.
use crate::chat::message::Message;
use crate::error::Error;
use crate::http_client::{get_slack_url, ResponseMetadata, SlackWebAPIClient};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct HistoryRequest {
    pub channel: String,
    pub cursor: Option<String>,
    pub inclusive: Option<bool>,
    pub latest: Option<String>,
    pub limit: Option<i32>,
    pub oldest: Option<String>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct HistoryResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub response_metadata: Option<ResponseMetadata>,
    pub messages: Option<Vec<Message>>,
    pub has_more: Option<bool>,
    pub pin_count: Option<i32>,
}

/// Fetches a conversation's history of messages and events.  
/// See: <https://api.slack.com/methods/conversations.history>
pub async fn history<T>(
    client: &T,
    param: &HistoryRequest,
    bot_token: &str,
) -> Result<HistoryResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("conversations.history");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<HistoryResponse>(&result).map_err(Error::SerdeJsonError)
        })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::http_client::MockSlackWebAPIClient;

    #[test]
    fn convert_request() {
        let request = HistoryRequest {
            channel: "C1234567890".to_string(),
            cursor: Some("dXNlcjpVMDYxTkZUVDI=".to_string()),
            inclusive: Some(true),
            latest: Some("1234567890.123456".to_string()),
            limit: Some(20),
            oldest: Some("1234567890.123456".to_string()),
        };
        let json = r##"{
  "channel": "C1234567890",
  "cursor": "dXNlcjpVMDYxTkZUVDI=",
  "inclusive": true,
  "latest": "1234567890.123456",
  "limit": 20,
  "oldest": "1234567890.123456"
}"##;

        let j = serde_json::to_string_pretty(&request).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<HistoryRequest>(json).unwrap();
        assert_eq!(request, s);
    }

    #[test]
    fn convert_response() {
        let response = HistoryResponse {
            ok: true,
            messages: Some(vec![
                Message {
                    type_file: Some("message".to_string()),
                    text: Some(
                        "I find you punny and would like to smell your nose letter".to_string(),
                    ),
                    user: Some("U012AB3CDE".to_string()),
                    ts: Some("1512085950.000216".to_string()),
                    ..Default::default()
                },
                Message {
                    type_file: Some("message".to_string()),
                    text: Some("What, you want to smell my shoes better?".to_string()),
                    user: Some("U061F7AUR".to_string()),
                    ts: Some("1512104434.000490".to_string()),
                    ..Default::default()
                },
            ]),
            has_more: Some(true),
            pin_count: Some(0),
            ..Default::default()
        };
        let json = r##"{
  "ok": true,
  "messages": [
    {
      "type": "message",
      "text": "I find you punny and would like to smell your nose letter",
      "user": "U012AB3CDE",
      "ts": "1512085950.000216"
    },
    {
      "type": "message",
      "text": "What, you want to smell my shoes better?",
      "user": "U061F7AUR",
      "ts": "1512104434.000490"
    }
  ],
  "has_more": true,
  "pin_count": 0
}"##;

        let j = serde_json::to_string_pretty(&response).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<HistoryResponse>(json).unwrap();
        assert_eq!(response, s);
    }

    #[async_std::test]
    async fn test_history() {
        let param = HistoryRequest {
            channel: "C1234567890".to_string(),
            cursor: Some("dXNlcjpVMDYxTkZUVDI=".to_string()),
            inclusive: Some(true),
            latest: Some("1512104434.000490".to_string()),
            limit: Some(20),
            oldest: Some("1512085950.000216".to_string()),
        };

        let mut mock = MockSlackWebAPIClient::new();
        mock.expect_post_json().returning(|_, _, _| {
            Ok(r##"{
  "ok": true,
  "messages": [
    {
      "type": "message",
      "text": "I find you punny and would like to smell your nose letter",
      "user": "U012AB3CDE",
      "ts": "1512085950.000216"
    },
    {
      "type": "message",
      "text": "What, you want to smell my shoes better?",
      "user": "U061F7AUR",
      "ts": "1512104434.000490"
    }
  ],
  "has_more": true,
  "pin_count": 0
}"##
            .to_string())
        });

        let response = history(&mock, &param, &"test_token".to_string())
            .await
            .unwrap();
        let expect = HistoryResponse {
            ok: true,
            messages: Some(vec![
                Message {
                    type_file: Some("message".to_string()),
                    text: Some(
                        "I find you punny and would like to smell your nose letter".to_string(),
                    ),
                    user: Some("U012AB3CDE".to_string()),
                    ts: Some("1512085950.000216".to_string()),
                    ..Default::default()
                },
                Message {
                    type_file: Some("message".to_string()),
                    text: Some("What, you want to smell my shoes better?".to_string()),
                    user: Some("U061F7AUR".to_string()),
                    ts: Some("1512104434.000490".to_string()),
                    ..Default::default()
                },
            ]),
            has_more: Some(true),
            pin_count: Some(0),
            ..Default::default()
        };

        assert_eq!(expect, response);
    }
}
