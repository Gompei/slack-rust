use slack::attachment::attachment::Attachment;
use slack::chat::post_message::{post_message, PostMessageRequest};
use slack::http_client::default_client;
use slack_rust as slack;
use std::env;

#[async_std::main]
async fn main() {
    let slack_bot_token =
        env::var("SLACK_BOT_TOKEN").unwrap_or_else(|_| panic!("slack bot token is not set."));
    let slack_channel_id =
        env::var("SLACK_CHANNEL_ID").unwrap_or_else(|_| panic!("slack channel id is not set."));

    let slack_api_client = default_client();

    let attachments = vec![Attachment::builder()
        .color("#36a64f".to_string())
        .author_name("slack-rust".to_string())
        .author_icon("https://2.bp.blogspot.com/-3o7K8_p8NNM/WGCRsl8GiCI/AAAAAAABAoc/XKnspjvc0YIoOiSRK9HW6wXhtlnZvHQ9QCLcB/s800/pyoko_hashiru.png".to_string())
        .title("slack_rust_example".to_string())
        .build()];
    let param = PostMessageRequest::builder(slack_channel_id)
        .text("Hello World!!".to_string())
        .attachments(attachments)
        .build();

    let response = post_message(&slack_api_client, &param, &slack_bot_token)
        .await
        .expect("api call error");
    println!("{:?}", response);
}
