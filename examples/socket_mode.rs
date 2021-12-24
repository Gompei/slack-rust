use async_trait::async_trait;
use slack::http_client::default_client;
use slack::socket::event::HelloEvent;

use slack::socket::socket_mode::{EventHandler, SocketMode};
use slack_rust as slack;
use slack_rust::socket::event::CommonEvent;
use std::env;

#[async_std::main]
async fn main() {
    env_logger::init();

    let slack_app_token =
        env::var("SLACK_APP_TOKEN").unwrap_or_else(|_| panic!("slack app token is not set."));

    let slack_api_client = default_client();

    SocketMode::run(&slack_api_client, &slack_app_token, &mut Handler)
        .await
        .unwrap_or_else(|_| panic!("socket mode run error."));
}

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn on_connect(&mut self) {
        log::info!("start socket mode...");
    }
    async fn on_hello(&mut self, s: &HelloEvent) {
        log::info!("hello event: {:?}", s);
    }
    async fn on_interactive(&mut self, s: &CommonEvent) {
        log::info!("interactive event: {:?}", s);
    }
}
