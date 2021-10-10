use slack_rust::SocketModeClient;

#[async_std::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let api_key = match args.len() {
        0 | 1 => {
            panic!("No api-key in args! Usage: cargo run --example example -- <api-key> <bot-key>")
        }
        x => args[x - 2].clone(),
    };
    let bot_key = match args.len() {
        0 | 1 => {
            panic!("No bot-key in args! Usage: cargo run --example example -- <api-key> <bot-key>")
        }
        x => args[x - 1].clone(),
    };

    SocketModeClient::run(
        slack_rust::api::Token { api_key, bot_key },
        &mut EventHandler,
    )
    .await;
}

pub struct EventHandler;
impl slack_rust::SocketModeEventHandler for EventHandler {}
