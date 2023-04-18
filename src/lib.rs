//! [Slack API Client](https://api.slack.com)
//!
//! It supports [SocketMode](https://api.slack.com/apis/connections/socket), [Event API](https://api.slack.com/apis/connections/events-api), and [WEB API](https://api.slack.com/web).
//!
//! # Usage
//!
//! ## [Web API](https://api.slack.com/methods)
//!
//! ```no_run
//! use slack::chat::post_message::{post_message, PostMessageRequest};
//! use slack::http_client::default_client;
//! use slack_rust as slack;
//! use std::env;
//!
//! #[async_std::main]
//! async fn main() {
//!     let slack_bot_token =
//!         env::var("SLACK_BOT_TOKEN").unwrap_or_else(|_| panic!("slack bot token is not set."));
//!
//!     let slack_api_client = default_client();
//!     let param = PostMessageRequest {
//!         channel: "channel_id".to_string(),
//!         text: Some("Hello world!!".to_string()),
//!         ..Default::default()
//!     };
//!
//!     let response = post_message(&slack_api_client, &param, &slack_bot_token)
//!         .await
//!         .expect("api call error");
//!     println!("{:?}", response);
//! }
//! ```
//!
//! ### Builder
//!
//! ```ignore
//!     let attachments = vec![Attachment::builder()
//!         .color("#36a64f".to_string())
//!         .author_name("slack-rust".to_string())
//!         .author_icon("https://!2.bp.blogspot.com/-3o7K8_p8NNM/WGCRsl8GiCI/AAAAAAABAoc/XKnspjvc0YIoOiSRK9HW6wXhtlnZvHQ9QCLcB/s800/pyoko_hashiru.png".to_string())
//!         .title("slack_rust_example".to_string())
//!         .build()];
//!     let param = PostMessageRequest::builder(slack_channel_id)
//!         .text("Hello World!!".to_string())
//!         .attachments(attachments)
//!         .build();
//! ```
//!
//! ## [Socket Mode](https://api.slack.com/apis/connections/socket-implement)
//!
//! ```no_run
//! use async_trait::async_trait;
//! use slack::chat::post_message::{post_message, PostMessageRequest};
//! use slack::http_client::{default_client, SlackWebAPIClient};
//! use slack::socket::event::{HelloEvent, InteractiveEvent};
//! use slack::socket::socket_mode::{ack, EventHandler, SocketMode, Stream};
//! use slack::views::open::{open, OpenRequest};
//! use slack::views::view::{View, ViewType};
//! use slack_rust as slack;
//! use std::env;
//!
//! #[async_std::main]
//! async fn main() {
//!     env_logger::init();
//!
//!     let slack_app_token =
//!         env::var("SLACK_APP_TOKEN").unwrap_or_else(|_| panic!("slack app token is not set."));
//!     let slack_bot_token =
//!         env::var("SLACK_BOT_TOKEN").unwrap_or_else(|_| panic!("slack bot token is not set."));
//!     let api_client = default_client();
//!
//!     SocketMode::new(api_client, slack_app_token, slack_bot_token)
//!         .option_parameter("SLACK_CHANNEL_ID".to_string(), "channel_id".to_string())
//!         .run(&mut Handler)
//!         .await
//!         .unwrap_or_else(|_| panic!("socket mode run error."));
//! }
//!
//! pub struct Handler;
//!
//! #[allow(unused_variables)]
//! #[async_trait]
//! impl<S> EventHandler<S> for Handler
//!     where
//!         S: SlackWebAPIClient,
//! {
//!     async fn on_hello(&mut self, socket_mode: &SocketMode<S>, e: HelloEvent, s: &mut Stream) {
//!         log::info!("hello event: {:?}", e);
//!     }
//!     async fn on_interactive(
//!         &mut self,
//!         socket_mode: &SocketMode<S>,
//!         e: InteractiveEvent,
//!         s: &mut Stream,
//!     ) {
//!         log::info!("interactive event: {:?}", e);
//!         ack(&e.envelope_id, s)
//!             .await
//!             .expect("socket mode ack error.");
//!
//!         let request = PostMessageRequest {
//!             channel: socket_mode
//!                 .option_parameter
//!                 .get("SLACK_CHANNEL_ID")
//!                 .unwrap()
//!                 .to_string(),
//!             text: Some("Hello World!!".to_string()),
//!             ..Default::default()
//!         };
//!         let response = post_message(&socket_mode.api_client, &request, &socket_mode.bot_token)
//!             .await
//!             .expect("post message api error.");
//!         log::info!("post message api response: {:?}", response);
//!     }
//! }
//! ```

pub mod apps;
pub mod attachment;
pub mod auth;
pub mod block;
pub mod channels;
pub mod chat;
pub mod comments;
pub mod conversations;
pub mod dnd;
pub mod error;
pub mod event_api;
pub mod files;
pub mod http_client;
pub mod invites;
pub mod items;
pub mod payloads;
pub mod profiles;
pub mod reactions;
pub mod reminders;
pub mod socket;
pub mod team;
pub mod usergroups;
pub mod users;
pub mod views;
