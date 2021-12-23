use slack::attachments::attachment::Attachment;
use slack::attachments::attachment::AttachmentField;
use slack::block::block_actions::ActionBlock;
use slack::block::block_elements::ButtonElement;
use slack::block::block_elements::SelectBlockElement;
use slack::block::block_elements::{BlockElement, BlockElementType};
use slack::block::block_object::OptionBlockObject;
use slack::block::block_object::TextBlockObject;
use slack::block::block_object::TextBlockType;
use slack::block::blocks::{Block, BlockType};
use slack::chat::post_message::{post_message, PostMessageRequest};
use slack_rust as slack;
use std::env;

#[async_std::main]
async fn main() {
    let slack_bot_token =
        env::var("SLACK_BOT_TOKEN").unwrap_or_else(|_| panic!("slack bot token is not set."));
    let slack_channel_id =
        env::var("SLACK_CHANNEL_ID").unwrap_or_else(|_| panic!("slack channel id is not set."));

    let slack_api_client = slack::http_client::default_client();
    let param = PostMessageRequest {
        channel: slack_channel_id,
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

    let response = post_message(&slack_api_client, &param, &slack_bot_token)
        .await
        .expect("api call error");
    println!("{:?}", response);
}
