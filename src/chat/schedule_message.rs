//! Schedules a message to be sent to a channel.  
//! See: <https://api.slack.com/methods/chat.scheduleMessage>

use crate::attachment::attachment::Attachment;
use crate::block::blocks::Block;
use crate::chat::message::Message;
use crate::error::Error;
use crate::http_client::{get_slack_url, ResponseMetadata, SlackWebAPIClient};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct ScheduledMessageRequest {
    pub channel: String,
    pub post_at: i32,
    pub text: String,
    pub as_user: Option<bool>,
    pub attachments: Option<Vec<Attachment>>,
    pub blocks: Option<Vec<Block>>,
    pub link_names: Option<bool>,
    pub parse: Option<String>,
    pub reply_broadcast: Option<bool>,
    pub thread_ts: Option<String>,
    pub unfurl_links: Option<bool>,
    pub unfurl_media: Option<bool>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct ScheduledMessageResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub response_metadata: Option<ResponseMetadata>,
    pub channel: Option<String>,
    pub scheduled_message_id: Option<String>,
    pub post_at: Option<String>,
    pub message: Option<Message>,
}

/// Schedules a message to be sent to a channel.  
/// See: <https://api.slack.com/methods/chat.scheduleMessage>
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::block::block_object::{TextBlockObject, TextBlockType};
    use crate::block::block_section::SectionBlock;
    use crate::http_client::MockSlackWebAPIClient;

    #[test]
    fn convert_request() {
        let request = ScheduledMessageRequest {
            channel: "C1234567890".to_string(),
            post_at: 299876400,
            text: "Hello world".to_string(),
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
            link_names: Some(true),
            parse: Some("full".to_string()),
            reply_broadcast: Some(true),
            thread_ts: Some("1234567890.123456".to_string()),
            unfurl_links: Some(true),
            unfurl_media: Some(true),
        };
        let json = r##"{
  "channel": "C1234567890",
  "post_at": 299876400,
  "text": "Hello world",
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
  "link_names": true,
  "parse": "full",
  "reply_broadcast": true,
  "thread_ts": "1234567890.123456",
  "unfurl_links": true,
  "unfurl_media": true
}"##;

        let j = serde_json::to_string_pretty(&request).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<ScheduledMessageRequest>(json).unwrap();
        assert_eq!(request, s);
    }

    #[test]
    fn convert_response() {
        let response = ScheduledMessageResponse {
            ok: true,
            channel: Some("C1H9RESGL".to_string()),
            scheduled_message_id: Some("Q1298393284".to_string()),
            post_at: Some("1562180400".to_string()),
            message: Some(Message {
                bot_id: Some("B19LU7CSY".to_string()),
                type_file: Some("delayed_message".to_string()),
                text: Some("Here's a message for you in the future".to_string()),
                user: Some("ecto1".to_string()),
                attachments: Some(vec![Attachment {
                    fallback: Some("This is an attachment's fallback".to_string()),
                    id: Some(1),
                    text: Some("This is an attachment".to_string()),
                    ..Default::default()
                }]),
                subtype: Some("bot_message".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        };
        let json = r##"{
  "ok": true,
  "channel": "C1H9RESGL",
  "scheduled_message_id": "Q1298393284",
  "post_at": "1562180400",
  "message": {
    "bot_id": "B19LU7CSY",
    "type": "delayed_message",
    "text": "Here's a message for you in the future",
    "user": "ecto1",
    "attachments": [
      {
        "fallback": "This is an attachment's fallback",
        "id": 1,
        "text": "This is an attachment"
      }
    ],
    "subtype": "bot_message"
  }
}"##;

        let j = serde_json::to_string_pretty(&response).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<ScheduledMessageResponse>(json).unwrap();
        assert_eq!(response, s);
    }

    #[async_std::test]
    async fn test_scheduled_message() {
        let param = ScheduledMessageRequest {
            channel: "C1234567890".to_string(),
            post_at: 299876400,
            text: "Hello world".to_string(),
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
            link_names: Some(true),
            parse: Some("full".to_string()),
            reply_broadcast: Some(true),
            thread_ts: Some("1234567890.123456".to_string()),
            unfurl_links: Some(true),
            unfurl_media: Some(true),
        };

        let mut mock = MockSlackWebAPIClient::new();
        mock.expect_post_json().returning(|_, _, _| {
            Ok(r##"{
  "ok": true,
  "channel": "C1H9RESGL",
  "scheduled_message_id": "Q1298393284",
  "post_at": "1562180400",
  "message": {
    "bot_id": "B19LU7CSY",
    "type": "delayed_message",
    "text": "Here's a message for you in the future",
    "user": "ecto1",
    "attachments": [
      {
        "fallback": "This is an attachment's fallback",
        "id": 1,
        "text": "This is an attachment"
      }
    ],
    "subtype": "bot_message"
  }
}"##
            .to_string())
        });

        let response = scheduled_message(&mock, &param, &"test_token".to_string())
            .await
            .unwrap();
        let expect = ScheduledMessageResponse {
            ok: true,
            channel: Some("C1H9RESGL".to_string()),
            scheduled_message_id: Some("Q1298393284".to_string()),
            post_at: Some("1562180400".to_string()),
            message: Some(Message {
                bot_id: Some("B19LU7CSY".to_string()),
                type_file: Some("delayed_message".to_string()),
                text: Some("Here's a message for you in the future".to_string()),
                user: Some("ecto1".to_string()),
                attachments: Some(vec![Attachment {
                    fallback: Some("This is an attachment's fallback".to_string()),
                    id: Some(1),
                    text: Some("This is an attachment".to_string()),
                    ..Default::default()
                }]),
                subtype: Some("bot_message".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        };

        assert_eq!(expect, response);
    }
}
