use crate::attachment::attachment::Attachment;
use crate::block::blocks::Block;
use crate::chat::message::Message;
use crate::error::Error;
use crate::http_client::{get_slack_url, ResponseMetadata, SlackWebAPIClient};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct PostMessageRequest {
    pub channel: String,
    pub attachments: Option<Vec<Attachment>>,
    pub blocks: Option<Vec<Block>>,
    pub text: Option<String>,
    pub icon_emoji: Option<String>,
    pub icon_url: Option<String>,
    pub link_names: Option<bool>,
    pub mrkdwn: Option<bool>,
    pub parse: Option<String>,
    pub reply_broadcast: Option<bool>,
    pub thread_ts: Option<String>,
    pub unfurl_links: Option<bool>,
    pub unfurl_media: Option<bool>,
    pub username: Option<String>,
}

impl PostMessageRequest {
    pub fn builder(channel: String) -> PostMessageRequestBuilder {
        PostMessageRequestBuilder::new(channel)
    }
}

#[derive(Debug, Default)]
pub struct PostMessageRequestBuilder {
    pub channel: String,
    pub attachments: Option<Vec<Attachment>>,
    pub blocks: Option<Vec<Block>>,
    pub text: Option<String>,
    pub icon_emoji: Option<String>,
    pub icon_url: Option<String>,
    pub link_names: Option<bool>,
    pub mrkdwn: Option<bool>,
    pub parse: Option<String>,
    pub reply_broadcast: Option<bool>,
    pub thread_ts: Option<String>,
    pub unfurl_links: Option<bool>,
    pub unfurl_media: Option<bool>,
    pub username: Option<String>,
}

impl PostMessageRequestBuilder {
    pub fn new(channel: String) -> PostMessageRequestBuilder {
        PostMessageRequestBuilder {
            channel,
            ..Default::default()
        }
    }
    pub fn attachments(mut self, attachments: Vec<Attachment>) -> PostMessageRequestBuilder {
        self.attachments = Some(attachments);
        self
    }
    pub fn blocks(mut self, blocks: Vec<Block>) -> PostMessageRequestBuilder {
        self.blocks = Some(blocks);
        self
    }
    pub fn text(mut self, text: String) -> PostMessageRequestBuilder {
        self.text = Some(text);
        self
    }
    pub fn icon_emoji(mut self, icon_emoji: String) -> PostMessageRequestBuilder {
        self.icon_emoji = Some(icon_emoji);
        self
    }
    pub fn icon_url(mut self, icon_url: String) -> PostMessageRequestBuilder {
        self.icon_url = Some(icon_url);
        self
    }
    pub fn link_names(mut self, link_names: bool) -> PostMessageRequestBuilder {
        self.link_names = Some(link_names);
        self
    }
    pub fn mrkdwn(mut self, mrkdwn: bool) -> PostMessageRequestBuilder {
        self.mrkdwn = Some(mrkdwn);
        self
    }
    pub fn parse(mut self, parse: String) -> PostMessageRequestBuilder {
        self.parse = Some(parse);
        self
    }
    pub fn reply_broadcast(mut self, reply_broadcast: bool) -> PostMessageRequestBuilder {
        self.reply_broadcast = Some(reply_broadcast);
        self
    }
    pub fn thread_ts(mut self, thread_ts: String) -> PostMessageRequestBuilder {
        self.thread_ts = Some(thread_ts);
        self
    }
    pub fn unfurl_links(mut self, unfurl_links: bool) -> PostMessageRequestBuilder {
        self.unfurl_links = Some(unfurl_links);
        self
    }
    pub fn unfurl_media(mut self, unfurl_media: bool) -> PostMessageRequestBuilder {
        self.unfurl_media = Some(unfurl_media);
        self
    }
    pub fn username(mut self, username: String) -> PostMessageRequestBuilder {
        self.username = Some(username);
        self
    }
    pub fn build(self) -> PostMessageRequest {
        PostMessageRequest {
            channel: self.channel,
            attachments: self.attachments,
            blocks: self.blocks,
            text: self.text,
            icon_emoji: self.icon_emoji,
            icon_url: self.icon_url,
            link_names: self.link_names,
            mrkdwn: self.mrkdwn,
            parse: self.parse,
            reply_broadcast: self.reply_broadcast,
            thread_ts: self.thread_ts,
            unfurl_links: self.unfurl_links,
            unfurl_media: self.unfurl_media,
            username: self.username,
        }
    }
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::attachment::attachment::AttachmentField;
    use crate::block::block_actions::ActionBlock;
    use crate::block::block_elements::{BlockElement, ButtonElement, SelectBlockElement};
    use crate::block::block_object::{OptionBlockObject, TextBlockObject, TextBlockType};
    use crate::chat::post_message::PostMessageRequest;
    use crate::http_client::MockSlackWebAPIClient;

    #[test]
    fn convert_request() {
        let request = PostMessageRequest {
            channel: "test".to_string(),
            text: Some("Hello world".to_string()),
            attachments: Some(vec![Attachment {
                color: Some("#36a64f".to_string()),
                author_name: Some("slack-rust".to_string()),
                author_link: Some("https://www.irasutoya.com/".to_string()),
                author_icon: Some("https://2.bp.blogspot.com/-3o7K8_p8NNM/WGCRsl8GiCI/AAAAAAABAoc/XKnspjvc0YIoOiSRK9HW6wXhtlnZvHQ9QCLcB/s800/pyoko_hashiru.png".to_string()),
                title: Some("title".to_string()),
                title_link: Some("https://www.irasutoya.com/".to_string()),
                pretext: Some("Optional pre-text that appears above the attachment block".to_string()),
                text: Some("Optional `text` that appears within the attachment".to_string()),
                thumb_url: Some("https://2.bp.blogspot.com/-3o7K8_p8NNM/WGCRsl8GiCI/AAAAAAABAoc/XKnspjvc0YIoOiSRK9HW6wXhtlnZvHQ9QCLcB/s800/pyoko_hashiru.png".to_string()),
                fields: Some(vec![
                    AttachmentField {
                        title: Some("A field's title".to_string()),
                        value: Some("This field's value".to_string()),
                        short: Some(false),
                    },
                ]),
                mrkdwn_in: Some(vec!["text".to_string()]),
                footer: Some("footer".to_string()),
                footer_icon: Some("https://1.bp.blogspot.com/-46AF2TCkb-o/VW6ORNeQ3UI/AAAAAAAAt_4/TA4RrGVcw_U/s800/pyoko05_cycling.png".to_string(), ),
                ts: Some(123456789),
                ..Default::default()
            }]),
            blocks: Some(vec![
                Block::ActionBlock(ActionBlock {
                    elements: vec![
                        BlockElement::SelectBlockElement(SelectBlockElement{
                            placeholder: TextBlockObject {
                                type_filed: TextBlockType::PlainText,
                                text: "select".to_string(),
                                ..Default::default()
                            },
                            action_id: "select".to_string(),
                            options: vec![
                                OptionBlockObject{
                                    text: TextBlockObject {
                                        type_filed: TextBlockType::PlainText,
                                        text: "Select1".to_string(),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                },
                                OptionBlockObject{
                                    text: TextBlockObject {
                                        type_filed: TextBlockType::PlainText,
                                        text: "Select2".to_string(),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                },
                            ],
                            ..Default::default()
                        }),
                        BlockElement::ButtonElement(ButtonElement{
                            text: TextBlockObject {
                                type_filed: TextBlockType::PlainText,
                                text: "Submit".to_string(),
                                ..Default::default()
                            },
                            action_id: "button".to_string(),
                            ..Default::default()
                        }),
                    ],
                    ..Default::default()
                }),
            ]),
            ..Default::default()
        };
        let json = r##"{
  "channel": "test",
  "attachments": [
    {
      "color": "#36a64f",
      "author_name": "slack-rust",
      "author_link": "https://www.irasutoya.com/",
      "author_icon": "https://2.bp.blogspot.com/-3o7K8_p8NNM/WGCRsl8GiCI/AAAAAAABAoc/XKnspjvc0YIoOiSRK9HW6wXhtlnZvHQ9QCLcB/s800/pyoko_hashiru.png",
      "title": "title",
      "title_link": "https://www.irasutoya.com/",
      "pretext": "Optional pre-text that appears above the attachment block",
      "text": "Optional `text` that appears within the attachment",
      "thumb_url": "https://2.bp.blogspot.com/-3o7K8_p8NNM/WGCRsl8GiCI/AAAAAAABAoc/XKnspjvc0YIoOiSRK9HW6wXhtlnZvHQ9QCLcB/s800/pyoko_hashiru.png",
      "fields": [
        {
          "title": "A field's title",
          "value": "This field's value",
          "short": false
        }
      ],
      "mrkdwn_in": [
        "text"
      ],
      "footer": "footer",
      "footer_icon": "https://1.bp.blogspot.com/-46AF2TCkb-o/VW6ORNeQ3UI/AAAAAAAAt_4/TA4RrGVcw_U/s800/pyoko05_cycling.png",
      "ts": 123456789
    }
  ],
  "blocks": [
    {
      "type": "actions",
      "elements": [
        {
          "type": "static_select",
          "placeholder": {
            "type": "plain_text",
            "text": "select"
          },
          "action_id": "select",
          "options": [
            {
              "text": {
                "type": "plain_text",
                "text": "Select1"
              }
            },
            {
              "text": {
                "type": "plain_text",
                "text": "Select2"
              }
            }
          ]
        },
        {
          "type": "button",
          "text": {
            "type": "plain_text",
            "text": "Submit"
          },
          "action_id": "button"
        }
      ]
    }
  ],
  "text": "Hello world"
}"##;

        let j = serde_json::to_string_pretty(&request).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<PostMessageRequest>(json).unwrap();
        assert_eq!(request, s);
    }

    #[test]
    fn convert_response() {
        let response = PostMessageResponse {
            ok: true,
            channel: Some("C02H7UK23GB".to_string()),
            ts: Some("1640258472.000200".to_string()),
            message: Some(Message {
                bot_id: Some("B02H2MCBRL6".to_string()),
                type_file: Some("message".to_string()),
                text: Some("Hello world".to_string()),
                user: Some("U02GUNSESDD".to_string()),
                ts: Some("1640258472.000200".to_string()),
                team: Some("T02H7RHQNL9".to_string()),
                blocks: Some(vec![Block::ActionBlock(ActionBlock {
                    block_id: Some("Zf2/".to_string()),
                    elements: vec![
                        BlockElement::SelectBlockElement(SelectBlockElement {
                            action_id: "select".to_string(),
                            placeholder: TextBlockObject {
                                type_filed: TextBlockType::PlainText,
                                text: "select".to_string(),
                                ..Default::default()
                            },
                            options: vec![OptionBlockObject {
                                text: TextBlockObject {
                                    type_filed: TextBlockType::PlainText,
                                    text: "Select1".to_string(),
                                    ..Default::default()
                                },
                                ..Default::default()
                            }],
                            ..Default::default()
                        }),
                        BlockElement::ButtonElement(ButtonElement {
                            text: TextBlockObject {
                                type_filed: TextBlockType::PlainText,
                                text: "Submit".to_string(),
                                ..Default::default()
                            },
                            action_id: "button".to_string(),
                            ..Default::default()
                        }),
                    ],
                })]),
                ..Default::default()
            }),
            ..Default::default()
        };
        let json = r##"{
  "ok": true,
  "channel": "C02H7UK23GB",
  "ts": "1640258472.000200",
  "message": {
    "bot_id": "B02H2MCBRL6",
    "type": "message",
    "text": "Hello world",
    "user": "U02GUNSESDD",
    "ts": "1640258472.000200",
    "team": "T02H7RHQNL9",
    "blocks": [
      {
        "type": "actions",
        "elements": [
          {
            "type": "static_select",
            "placeholder": {
              "type": "plain_text",
              "text": "select"
            },
            "action_id": "select",
            "options": [
              {
                "text": {
                  "type": "plain_text",
                  "text": "Select1"
                }
              }
            ]
          },
          {
            "type": "button",
            "text": {
              "type": "plain_text",
              "text": "Submit"
            },
            "action_id": "button"
          }
        ],
        "block_id": "Zf2/"
      }
    ]
  }
}"##;

        let j = serde_json::to_string_pretty(&response).unwrap();
        assert_eq!(json, j);

        let s = serde_json::from_str::<PostMessageResponse>(json).unwrap();
        assert_eq!(response, s);
    }

    #[async_std::test]
    async fn test_post_message() {
        let param = PostMessageRequest {
            channel: "test".to_string(),
            text: Some("test".to_string()),
            ..Default::default()
        };

        let mut mock = MockSlackWebAPIClient::new();
        mock.expect_post_json().returning(|_, _, _| {
            Ok(r##"{
  "ok": true,
  "channel": "test",
  "message": {
    "text": "test"
  }
}"##
            .to_string())
        });

        let response = post_message(&mock, &param, &"test_token".to_string())
            .await
            .unwrap();
        let expect = PostMessageResponse {
            ok: true,
            channel: Some("test".to_string()),
            message: Some(Message {
                text: Some("test".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        };

        assert_eq!(expect, response);
    }
}
