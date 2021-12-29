use async_trait::async_trait;
use slack::block::block_actions::ActionBlock;
use slack::block::block_elements::{
    BlockElement, ButtonElement, MultiSelectBlockElement, PlainTextInputBlockElement,
};
use slack::block::block_input::InputBlock;
use slack::block::block_object::{OptionBlockObject, TextBlockObject, TextBlockType};
use slack::block::blocks::Block;
use slack::chat::post_message::{post_message, PostMessageRequest};
use slack::http_client::{default_client, SlackWebAPIClient};
use slack::payloads::interactive::InteractiveEventType;
use slack::socket::event::{HelloEvent, InteractiveEvent};
use slack::socket::socket_mode::{ack, EventHandler, SocketMode, Stream};
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
    let slack_channel_id =
        env::var("SLACK_CHANNEL_ID").unwrap_or_else(|_| panic!("slack channel id is not set."));

    let api_client = default_client();

    SocketMode::new(api_client, slack_app_token, slack_bot_token)
        .option_parameter("SLACK_CHANNEL_ID".to_string(), slack_channel_id)
        .run(&mut Handler)
        .await
        .unwrap_or_else(|_| panic!("socket mode run error."));
}

pub struct Handler;

#[allow(unused_variables)]
#[async_trait]
impl<S> EventHandler<S> for Handler
where
    S: SlackWebAPIClient,
{
    async fn on_connect(&mut self, socket_mode: &SocketMode<S>) {
        log::info!("start socket mode...");
    }
    async fn on_hello(&mut self, socket_mode: &SocketMode<S>, e: &HelloEvent, s: &mut Stream) {
        log::info!("hello event: {:?}", e);
    }
    async fn on_interactive(
        &mut self,
        socket_mode: &SocketMode<S>,
        e: &InteractiveEvent,
        s: &mut Stream,
    ) {
        log::info!("interactive event: {:?}", e);
        ack(&e.envelope_id, s)
            .await
            .expect("socket mode ack error.");

        match e.payload.type_filed {
            InteractiveEventType::ViewSubmission => {
                let request = PostMessageRequest {
                    channel: socket_mode
                        .option_parameter
                        .get("SLACK_CHANNEL_ID")
                        .unwrap()
                        .to_string(),
                    text: Some("Message received!!".to_string()),
                    ..Default::default()
                };
                let response =
                    post_message(&socket_mode.api_client, &request, &socket_mode.bot_token)
                        .await
                        .expect("post message api error.");
                log::info!("post message api response: {:?}", response);
            }
            InteractiveEventType::Shortcut => {
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
                            open(&socket_mode.api_client, &request, &socket_mode.bot_token)
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
