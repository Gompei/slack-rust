use async_trait::async_trait;
use slack::socket::event::{HelloEvent, InteractiveEvent};
use slack::socket::socket_mode::{EventHandler, SocketMode, Stream};
use slack::views::open::{open, OpenRequest};
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

        let request = OpenRequest {
            trigger_id: e
                .payload
                .trigger_id
                .as_ref()
                .map_or("".to_string(), |r| r.to_string()),
            ..Default::default()
        };

        open(&socket_mode.client, &request, &socket_mode.token.bot_token)
            .await
            .expect("view open api error.");
    }
}
