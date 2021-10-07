use slack_rust::SocketModeClient;

#[async_std::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let api_key = match args.len() {
        0 | 1 => {
            panic!("No api-key in args! Usage: cargo run --example example -- <api-key>")
        }
        x => args[x - 1].clone(),
    };

    let mut handler = EventHandler;
    SocketModeClient::run(&api_key, &mut handler).await;
}

pub struct EventHandler;
impl slack_rust::SocketModeEventHandler for EventHandler {}
