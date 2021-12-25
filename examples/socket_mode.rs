use async_trait::async_trait;
use slack::block::block_actions::ActionBlock;
use slack::block::block_elements::{
    BlockElement, ButtonElement, MultiSelectBlockElement, PlainTextInputBlockElement,
};
use slack::block::block_input::InputBlock;
use slack::block::block_object::{OptionBlockObject, TextBlockObject, TextBlockType};
use slack::block::blocks::Block;
use slack::chat::post_message::{post_message, PostMessageRequest};
use slack::payloads::interactive::InteractiveEventType;
use slack::socket::event::{HelloEvent, InteractiveEvent};
use slack::socket::socket_mode::{EventHandler, SocketMode, Stream};
use slack::views::open::{open, OpenRequest};
use slack::views::view::{View, ViewType};
use slack_rust as slack;
use std::env;

#[async_std::main]
async fn main() {
    env_logger::init();

    let slack_app_token =
        env::var("SLACK_APP_TOKEN").unwrap_or_else(|_| panic!("slack app token is not set."));
    let slack_bot_token =
        env::var("SLACK_BOT_TOKEN").unwrap_or_else(|_| panic!("slack bot token is not set."));

    SocketMode::new(slack_app_token, slack_bot_token)
        .run(&mut Handler)
        .await
        .unwrap_or_else(|_| panic!("socket mode run error."));
}

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn on_connect(&mut self, _socket_mode: &SocketMode) {
        log::info!("start socket mode...");
    }
    async fn on_hello(&mut self, _socket_mode: &SocketMode, e: &HelloEvent) {
        log::info!("hello event: {:?}", e);
    }
    async fn on_interactive(
        &mut self,
        socket_mode: &SocketMode,
        e: &InteractiveEvent,
        s: &mut Stream,
    ) {
        log::info!("interactive event: {:?}", e);
        SocketMode::ack(&e.envelope_id, s)
            .await
            .expect("socket mode ack error.");

        match e.payload.type_filed {
            InteractiveEventType::ViewSubmission => {
                let slack_channel_id = env::var("SLACK_CHANNEL_ID")
                    .unwrap_or_else(|_| panic!("slack channel id is not set."));

                let request = PostMessageRequest {
                    channel: slack_channel_id,
                    text: Some("Message received!!".to_string()),
                    ..Default::default()
                };
                let response =
                    post_message(&socket_mode.client, &request, &socket_mode.token.bot_token)
                        .await
                        .expect("post message api error.");
                log::info!("post message api response: {:?}", response);
            }
            InteractiveEventType::Shortcut => {
                // TODO
                match e.payload.callback_id.as_ref().unwrap().as_ref() {
                    "example_shortcut" => {
                        let request =
                            OpenRequest {
                                trigger_id: e
                                    .payload
                                    .trigger_id
                                    .as_ref()
                                    .map_or("".to_string(), |r| r.to_string()),
                                view: View {
                                    type_filed: Some(ViewType::Modal),
                                    title: Some(TextBlockObject {
                                        type_filed: TextBlockType::PlainText,
                                        text: "Slack Rust Example Modal".to_string(),
                                        ..Default::default()
                                    }),
                                    submit: Some(TextBlockObject {
                                        type_filed: TextBlockType::PlainText,
                                        text: "Submit".to_string(),
                                        ..Default::default()
                                    }),
                                    blocks: Some(vec![
                                Block::InputBlock(InputBlock {
                                    label: TextBlockObject {
                                        type_filed: TextBlockType::PlainText,
                                        text: "Title".to_string(),
                                        ..Default::default()
                                    },
                                    element: Some(BlockElement::PlainTextInputBlockElement(
                                        PlainTextInputBlockElement {
                                            action_id: "title".to_string(),
                                            placeholder: Some(TextBlockObject {
                                                type_filed: TextBlockType::PlainText,
                                                text: "What do you want to ask of the world?"
                                                    .to_string(),
                                                ..Default::default()
                                            }),
                                            ..Default::default()
                                        },
                                    )),
                                    ..Default::default()
                                }),
                                Block::InputBlock(InputBlock {
                                    label: TextBlockObject {
                                        type_filed: TextBlockType::PlainText,
                                        text: "Channel(s)".to_string(),
                                        ..Default::default()
                                    },
                                    element: Some(BlockElement::MultiSelectBlockElement(
                                        MultiSelectBlockElement {
                                            action_id: "title".to_string(),
                                            placeholder: TextBlockObject {
                                                type_filed: TextBlockType::PlainText,
                                                text: "Where should the poll be sent?".to_string(),
                                                ..Default::default()
                                            },
                                            options: vec![OptionBlockObject {
                                                text: TextBlockObject {
                                                    type_filed: TextBlockType::PlainText,
                                                    text: "*this is plain_text text*".to_string(),
                                                    ..Default::default()
                                                },
                                                value: Some("value-0".to_string()),
                                                ..Default::default()
                                            }],
                                            ..Default::default()
                                        },
                                    )),
                                    ..Default::default()
                                }),
                                Block::ActionBlock(ActionBlock {
                                    elements: vec![BlockElement::ButtonElement(ButtonElement {
                                        action_id: "add_option".to_string(),
                                        text: TextBlockObject {
                                            type_filed: TextBlockType::PlainText,
                                            text: "Add another option".to_string(),
                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    })],
                                    ..Default::default()
                                }),
                            ]),
                                    ..Default::default()
                                },
                            };
                        let response =
                            open(&socket_mode.client, &request, &socket_mode.token.bot_token)
                                .await
                                .expect("view open api error.");
                        log::info!("view open api response: {:?}", response);
                    }
                    _ => log::info!("unknown shortcuts: {:?}", e.payload.callback_id),
                }
            }
            _ => log::info!("other events: {:?}", e.payload.type_filed),
        }
    }
}
