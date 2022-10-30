use std::env;

use async_trait::async_trait;
use slack::chat::post_message::{post_message, PostMessageRequest};
use slack::event_api::event::{Event, EventCallback};
use slack::http_client::{default_client, SlackWebAPIClient};
use slack::socket::event::{EventsAPI, HelloEvent};
use slack::socket::socket_mode::{ack, EventHandler, SocketMode, Stream};
use slack_rust as slack;

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

    let result = SocketMode::new(api_client, slack_app_token, slack_bot_token)
        .option_parameter("SLACK_CHANNEL_ID".to_string(), slack_channel_id)
        .run(&mut Handler)
        .await;

    result.unwrap_or_else(|e| panic!("socket mode run error {:?}.", e));
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
    async fn on_hello(&mut self, socket_mode: &SocketMode<S>, e: HelloEvent, s: &mut Stream) {
        log::info!("hello event: {:?}", e);
    }

    async fn on_events_api(&mut self, socket_mode: &SocketMode<S>, e: EventsAPI, s: &mut Stream) {
        log::info!("event: {:?}", e);
        ack(&e.envelope_id, s)
            .await
            .expect("socket mode ack error.");

        match e.payload.event {
            EventCallback::AppMention {
                text,
                channel,
                ts,
                thread_ts,
                ..
            } => {
                let (reply_thread_ts, reply_text) = if let Some(thread_ts) = thread_ts {
                    (thread_ts, "Hello again!".to_string())
                } else {
                    (ts, "Hello!".to_string())
                };

                let request = PostMessageRequest {
                    channel: socket_mode
                        .option_parameter
                        .get("SLACK_CHANNEL_ID")
                        .unwrap()
                        .to_string(),
                    thread_ts: Some(reply_thread_ts),
                    text: Some(reply_text),
                    ..Default::default()
                };
                let response =
                    post_message(&socket_mode.api_client, &request, &socket_mode.bot_token)
                        .await
                        .expect("post message api error.");
                log::info!("post message api response: {:?}", response);
            }
            _ => {}
        }
    }
}
