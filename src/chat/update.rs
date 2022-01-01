//! Updates a message.  

use crate::attachment::attachment::Attachment;
use crate::block::blocks::Block;
use crate::chat::message::Message;
use crate::error::Error;
use crate::http_client::{get_slack_url, ResponseMetadata, SlackWebAPIClient};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct UpdateRequest {
    pub channel: String,
    pub ts: String,
    pub as_user: Option<bool>,
    pub attachments: Option<Vec<Attachment>>,
    pub blocks: Option<Vec<Block>>,
    pub file_ids: Option<Vec<String>>,
    pub link_names: Option<bool>,
    pub parse: Option<String>,
    pub reply_broadcast: Option<bool>,
    pub text: Option<String>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct UpdateResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub response_metadata: Option<ResponseMetadata>,
    pub channel: Option<String>,
    pub ts: Option<String>,
    pub text: Option<String>,
    pub message: Option<Message>,
}

/// Updates a message.  
/// See: <https://api.slack.com/methods/chat.update>
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::block::block_object::{TextBlockObject, TextBlockType};
    use crate::block::block_section::SectionBlock;
    use crate::http_client::MockSlackWebAPIClient;

    #[test]
    fn convert_request() {
        let request = UpdateRequest {
            channel: "C123456789".to_string(),
            ts: "1405894322.002768".to_string(),
            as_user: Some(true),
            attachments: Some(vec![Attachment {
                pretext: Some("pre-hello".to_string()),
                text: Some("text-world".to_string()),
                ..Default::default()
            }]),
            blocks: Some(vec![Block::SectionBlock(SectionBlock {
                text: Some(TextBlockObject {
                    type_filed: TextBlockType::PlainText,
                    text: "text".to_string(),
                    ..Default::default()
                }),
                ..Default::default()
            })]),
            file_ids: Some(vec!["F013GKY52QK".to_string(), "F013GL22D0T".to_string()]),
            link_names: Some(true),
            parse: Some("none".to_string()),
            reply_broadcast: Some(true),
            text: Some("Hello world".to_string()),
        };
        let json = r##"{
  "channel": "C123456789",
  "ts": "1405894322.002768",
  "as_user": true,
  "attachments": [
    {
      "pretext": "pre-hello",
      "text": "text-world"
    }
  ],
  "blocks": [
    {
      "type": "section",
      "text": {
        "type": "plain_text",
        "text": "text"
      }
    }
  ],
  "file_ids": [
    "F013GKY52QK",
    "F013GL22D0T"
  ],
  "link_names": true,
  "parse": "none",
  "reply_broadcast": true,
  "text": "Hello world"
}"##;

        let j = serde_json::to_string_pretty(&request).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<UpdateRequest>(json).unwrap();
        assert_eq!(request, s);
    }

    #[test]
    fn convert_response() {
        let response = UpdateResponse {
            ok: true,
            channel: Some("C024BE91L".to_string()),
            ts: Some("1401383885.000061".to_string()),
            text: Some("Updated text you carefully authored".to_string()),
            message: Some(Message {
                text: Some("Updated text you carefully authored".to_string()),
                user: Some("U34567890".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        };
        let json = r##"{
  "ok": true,
  "channel": "C024BE91L",
  "ts": "1401383885.000061",
  "text": "Updated text you carefully authored",
  "message": {
    "text": "Updated text you carefully authored",
    "user": "U34567890"
  }
}"##;

        let j = serde_json::to_string_pretty(&response).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<UpdateResponse>(json).unwrap();
        assert_eq!(response, s);
    }

    #[async_std::test]
    async fn test_update() {
        let param = UpdateRequest {
            channel: "C123456789".to_string(),
            ts: "1405894322.002768".to_string(),
            as_user: Some(true),
            attachments: Some(vec![Attachment {
                pretext: Some("pre-hello".to_string()),
                text: Some("text-world".to_string()),
                ..Default::default()
            }]),
            blocks: Some(vec![Block::SectionBlock(SectionBlock {
                text: Some(TextBlockObject {
                    type_filed: TextBlockType::PlainText,
                    text: "text".to_string(),
                    ..Default::default()
                }),
                ..Default::default()
            })]),
            file_ids: Some(vec!["F013GKY52QK".to_string(), "F013GL22D0T".to_string()]),
            link_names: Some(true),
            parse: Some("none".to_string()),
            reply_broadcast: Some(true),
            text: Some("Hello world".to_string()),
        };

        let mut mock = MockSlackWebAPIClient::new();
        mock.expect_post_json().returning(|_, _, _| {
            Ok(r##"{
  "ok": true,
  "channel": "C123456789",
  "ts": "1401383885.000061",
  "text": "Hello world",
  "message": {
    "text": "text",
    "user": "U34567890"
  }
}"##
            .to_string())
        });

        let response = update(&mock, &param, &"test_token".to_string())
            .await
            .unwrap();
        let expect = UpdateResponse {
            ok: true,
            channel: Some("C123456789".to_string()),
            ts: Some("1401383885.000061".to_string()),
            text: Some("Hello world".to_string()),
            message: Some(Message {
                text: Some("text".to_string()),
                user: Some("U34567890".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        };

        assert_eq!(expect, response);
    }
}
