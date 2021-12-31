# [Slack API in Rust](https://api.slack.com)

[<img alt="github" src="https://img.shields.io/badge/github-Gompei/slack_rust-6ba5dd?style=for-the-badge&logo=github" height="20">](https://github.com/Gompei/slack-rust)
[<img alt="ci status" src="https://img.shields.io/github/workflow/status/Gompei/slack-rust/ci/main?style=for-the-badge" height="20">](https://github.com/Gompei/slack-rust/actions)

This is a Slack library for Rust that I'm working on, inspired by [slack-go/slack](https://github.com/slack-go/slack).  
It supports [SocketMode](https://api.slack.com/apis/connections/socket), [Event API](https://api.slack.com/apis/connections/events-api), and [WEB API](https://api.slack.com/web).

**This crate is under development. If you have any feature requests, please create them in an issue!** :wave:

## Usage

Add this to your Cargo.toml:

```toml
[dependencies]
slack_rust = "0.1.0-alpha"
```

### [Web API](https://api.slack.com/methods)

```rust
use slack::chat::post_message::{post_message, PostMessageRequest};
use slack::http_client::default_client;
use slack_rust as slack;
use std::env;

#[async_std::main]
async fn main() {
    let slack_bot_token =
        env::var("SLACK_BOT_TOKEN").unwrap_or_else(|_| panic!("slack bot token is not set."));

    let slack_api_client = default_client();
    let param = PostMessageRequest {
        channel: "channel_id".to_string(),
        text: Some("Hello world!!".to_string()),
        ..Default::default()
    };

    let response = post_message(&slack_api_client, &param, &slack_bot_token)
        .await
        .expect("api call error");
    println!("{:?}", response);
}
```


### [Socket Mode](https://api.slack.com/apis/connections/socket-implement)

```rust
use async_trait::async_trait;
use slack::chat::post_message::{post_message, PostMessageRequest};
use slack::http_client::{default_client, SlackWebAPIClient};
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
    let api_client = default_client();

    SocketMode::new(api_client, slack_app_token, slack_bot_token)
        .option_parameter("SLACK_CHANNEL_ID".to_string(), "channel_id".to_string())
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
    async fn on_hello(&mut self, socket_mode: &SocketMode<S>, e: HelloEvent, s: &mut Stream) {
        log::info!("hello event: {:?}", e);
    }
    async fn on_interactive(
        &mut self,
        socket_mode: &SocketMode<S>,
        e: InteractiveEvent,
        s: &mut Stream,
    ) {
        log::info!("interactive event: {:?}", e);
        ack(&e.envelope_id, s)
            .await
            .expect("socket mode ack error.");
            
        let request = PostMessageRequest {
            channel: socket_mode
                .option_parameter
                .get("SLACK_CHANNEL_ID")
                .unwrap()
                .to_string(),
            text: Some("Hello World!!".to_string()),
            ..Default::default()
        };
        let response = post_message(&socket_mode.api_client, &request, &socket_mode.bot_token)
            .await
            .expect("post message api error.");
        log::info!("post message api response: {:?}", response);
    }
}
```

## Other Reference Repository

- [slack-rs/slack-rs](https://github.com/slack-rs/slack-rs)
- [frostly/rust-slack](https://github.com/frostly/rust-slack)
- [Pctg-x8/slack-socket-mode-client](https://github.com/Pctg-x8/slack-socket-mode-client)

## License

This project is licensed under the Apache License Version 2.0.