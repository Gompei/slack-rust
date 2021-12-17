use async_trait::async_trait;
use slack_rust as slack;
use slack_rust::socket_mode::SocketMessage;
use std::env;

#[async_std::main]
async fn main() {
    env_logger::init();

    let slack_app_token =
        env::var("SLACK_APP_TOKEN").unwrap_or_else(|_| panic!("slack app token is not set."));

    let slack_api_client = slack::http_client::default_client();

    slack::socket_mode::SocketMode::run(&slack_api_client, &slack_app_token, &mut EventHandler)
        .await
        .unwrap_or_else(|_| panic!("socket mode run error."));
}

pub struct EventHandler;

#[async_trait]
impl slack::socket_mode::EventHandler for EventHandler {
    async fn on_hello(&mut self, s: &SocketMessage) {
        println!("{:?}", s);
    }
}
