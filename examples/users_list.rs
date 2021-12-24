use slack::http_client::default_client;
use slack::users::list::{list, ListRequest};
use slack_rust as slack;
use std::env;

#[async_std::main]
async fn main() {
    let slack_bot_token =
        env::var("SLACK_BOT_TOKEN").unwrap_or_else(|_| panic!("slack bot token is not set."));

    let slack_api_client = default_client();

    let param = ListRequest {
        ..Default::default()
    };
    let response = list(&slack_api_client, &param, &slack_bot_token)
        .await
        .expect("api call error");
    println!("{:?}", response);
}
