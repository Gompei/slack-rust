use crate::attachment::attachment::Attachment;
use crate::block::blocks::Block;
use crate::error::Error;
use crate::http_client::{get_slack_url, ResponseMetadata, SlackWebAPIClient};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct PostEphemeralRequest {
    pub channel: String,
    pub text: String,
    pub user: String,
    pub as_user: Option<bool>,
    pub attachments: Option<Vec<Attachment>>,
    pub blocks: Option<Vec<Block>>,
    pub icon_emoji: Option<String>,
    pub icon_url: Option<String>,
    pub link_names: Option<bool>,
    pub parse: Option<String>,
    pub thread_ts: Option<String>,
    pub username: Option<String>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct PostEphemeralResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub response_metadata: Option<ResponseMetadata>,
    pub message_ts: Option<String>,
}

pub async fn post_ephemeral<T>(
    client: &T,
    param: &PostEphemeralRequest,
    bot_token: &str,
) -> Result<PostEphemeralResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("chat.postEphemeral");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<PostEphemeralResponse>(&result).map_err(Error::SerdeJsonError)
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
        let request = PostEphemeralRequest {
            channel: "C1234567890".to_string(),
            text: "Hello world".to_string(),
            user: "U0BPQUNTA".to_string(),
            as_user: Some(true),
            attachments: Some(vec![Attachment {
                pretext: Some("pre-hello".to_string()),
                text: Some("text-world".to_string()),
                ..Default::default()
            }]),
            blocks: Some(vec![Block::SectionBlock(SectionBlock {
                text: TextBlockObject {
                    type_filed: TextBlockType::PlainText,
                    text: "text".to_string(),
                    ..Default::default()
                },
                ..Default::default()
            })]),
            icon_emoji: Some(":chart_with_upwards_trend:".to_string()),
            icon_url: Some("http://lorempixel.com/48/48".to_string()),
            link_names: Some(true),
            parse: Some("full".to_string()),
            thread_ts: Some("1234567890.123456".to_string()),
            username: Some("My Bot".to_string()),
        };
        let json = r##"{
  "channel": "C1234567890",
  "text": "Hello world",
  "user": "U0BPQUNTA",
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
  "icon_emoji": ":chart_with_upwards_trend:",
  "icon_url": "http://lorempixel.com/48/48",
  "link_names": true,
  "parse": "full",
  "thread_ts": "1234567890.123456",
  "username": "My Bot"
}"##;

        let j = serde_json::to_string_pretty(&request).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<PostEphemeralRequest>(json).unwrap();
        assert_eq!(request, s);
    }

    #[test]
    fn convert_response() {
        let response = PostEphemeralResponse {
            ok: true,
            message_ts: Some("1502210682.580145".to_string()),
            ..Default::default()
        };
        let json = r##"{
  "ok": true,
  "message_ts": "1502210682.580145"
}"##;

        let j = serde_json::to_string_pretty(&response).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<PostEphemeralResponse>(json).unwrap();
        assert_eq!(response, s);
    }
    #[async_std::test]
    async fn test_post_ephemeral() {
        let param = PostEphemeralRequest {
            channel: "C1234567890".to_string(),
            text: "Hello world".to_string(),
            user: "U0BPQUNTA".to_string(),
            as_user: Some(true),
            attachments: Some(vec![Attachment {
                pretext: Some("pre-hello".to_string()),
                text: Some("text-world".to_string()),
                ..Default::default()
            }]),
            blocks: Some(vec![Block::SectionBlock(SectionBlock {
                text: TextBlockObject {
                    type_filed: TextBlockType::PlainText,
                    text: "text".to_string(),
                    ..Default::default()
                },
                ..Default::default()
            })]),
            icon_emoji: Some(":chart_with_upwards_trend:".to_string()),
            icon_url: Some("http://lorempixel.com/48/48".to_string()),
            link_names: Some(true),
            parse: Some("full".to_string()),
            thread_ts: Some("1234567890.123456".to_string()),
            username: Some("My Bot".to_string()),
        };

        let mut mock = MockSlackWebAPIClient::new();
        mock.expect_post_json().returning(|_, _, _| {
            Ok(r##"{
  "ok": true,
  "message_ts": "1502210682.580145"
}"##
            .to_string())
        });

        let response = post_ephemeral(&mock, &param, &"test_token".to_string())
            .await
            .unwrap();
        let expect = PostEphemeralResponse {
            ok: true,
            message_ts: Some("1502210682.580145".to_string()),
            ..Default::default()
        };

        assert_eq!(expect, response);
    }
}
