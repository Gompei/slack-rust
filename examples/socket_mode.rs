use async_trait::async_trait;
use slack::http_client::Client;
use slack::socket::event::{HelloEvent, InteractiveEvent};
use slack::socket::socket_mode::{EventHandler, SocketMode, Stream};
use slack_rust as slack;
use std::env;

#[async_std::main]
async fn main() {
    env_logger::init();

    let slack_app_token =
        env::var("SLACK_APP_TOKEN").unwrap_or_else(|_| panic!("slack app token is not set."));

    SocketMode::run(&slack_app_token, &mut Handler)
        .await
        .unwrap_or_else(|_| panic!("socket mode run error."));
}

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn on_connect(&mut self, _client: &Client) {
        log::info!("start socket mode...");
    }
    async fn on_hello(&mut self, _client: &Client, e: &HelloEvent) {
        log::info!("hello event: {:?}", e);
    }
    async fn on_interactive(&mut self, _client: &Client, e: &InteractiveEvent, s: &mut Stream) {
        log::info!("interactive event: {:?}", e);
        SocketMode::ack(&e.envelope_id, s)
            .await
            .expect("socket mode ack error.");
    }
}
