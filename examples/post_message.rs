use slack_rust as slack;
use std::env;

#[async_std::main]
async fn main() {
    let slack_bot_token =
        env::var("SLACK_BOT_TOKEN").unwrap_or_else(|_| panic!("slack bot token is not set."));
    let slack_channel_id =
        env::var("SLACK_CHANNEL_ID").unwrap_or_else(|_| panic!("slack channel id is not set."));

    let slack_api_client = slack::http_client::default_client();
    let param = slack::chat::post_message::PostMessageRequest {
        channel: slack_channel_id,
        text: "Hello World".to_string(),
    };

    let response =
        slack::chat::post_message::post_message(&slack_api_client, &param, &slack_bot_token)
            .await
            .expect("api call error");
    println!("{:?}", response);
}
