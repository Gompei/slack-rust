use crate::attachments::attachment::Attachment;
use crate::block::blocks::Block;
use crate::chat::message::Message;
use crate::error::Error;
use crate::http_client::{get_slack_url, ResponseMetadata, SlackWebAPIClient};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default)]
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

#[derive(Deserialize, Serialize, Debug, Default)]
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
    use crate::attachments::attachment::AttachmentField;
    use crate::block::block_actions::ActionBlock;
    use crate::block::block_elements::{
        BlockElement, BlockElementType, ButtonElement, SelectBlockElement,
    };
    use crate::block::block_object::{OptionBlockObject, TextBlockObject, TextBlockType};
    use crate::block::blocks::BlockType;
    use crate::chat::post_message::PostMessageRequest;

    #[test]
    fn convert_struct_to_json() {
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
                    type_filed: BlockType::Actions,
                    elements: vec![
                        BlockElement::SelectBlockElement(SelectBlockElement{
                            type_filed: BlockElementType::StaticSelect,
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
                            type_filed: BlockElementType::Button,
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
        let json = serde_json::to_string_pretty(&request).unwrap();
        let expected = r##"{
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

        assert_eq!(json, expected);
    }
}
