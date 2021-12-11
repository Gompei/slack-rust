use slack_rust as slack;
use std::env;

#[async_std::main]
async fn main() {
    let slack_bot_token =
        env::var("SLACK_BOT_TOKEN").unwrap_or_else(|_| panic!("slack bot token is not set."));

    let slack_api_client = slack::http_client::default_client();

    let param = slack::users::list::ListRequest {
        cursor: None,
        include_locale: None,
        limit: None,
        team_id: None,
    };
    let response = slack::users::list::list(&slack_api_client, param, slack_bot_token)
        .await
        .expect("api call error");
    println!("{:?}", response);
}
