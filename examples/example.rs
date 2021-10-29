use async_std::net::TcpStream;
use async_tls::client::TlsStream;
use async_tungstenite::tungstenite::Message;
use async_tungstenite::WebSocketStream;
use slack_rust::api::{ApiClient, Token};
use slack_rust::socket_mode::{
    InteractiveType, SocketModeAcknowledgeMessage, SocketModeClient, SocketModeEventHandler,
    SocketModeMessage,
};

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
        &slack_rust::api::ApiClient {
            token: Token { api_key, bot_key },
        },
        &mut EventHandler,
    )
    .await;
}

pub struct EventHandler;
impl SocketModeEventHandler for EventHandler {
    fn on_hello(&mut self, s: &SocketModeMessage) {
        println!("{:?}", s);
    }
    fn on_interactive(
        &mut self,
        s: &SocketModeMessage,
        stream: &mut WebSocketStream<TlsStream<TcpStream>>,
        client: &ApiClient,
    ) {
        match &s.payload {
            Some(result) => match result.message_type {
                InteractiveType::Shortcut => match &s.envelope_id {
                    Some(id) => {
                        self.ack(id, stream);
                        println!("ack")
                    }
                    None => {}
                },
                _ => {}
            },
            _ => {}
        }
    }
}
