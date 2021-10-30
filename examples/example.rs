use async_std::net::TcpStream;
use async_tls::client::TlsStream;
use async_trait::async_trait;
use async_tungstenite::tungstenite::Message;
use async_tungstenite::WebSocketStream;
use slack_rust::api::{ApiClient, Query, Token};
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

#[async_trait]
impl SocketModeEventHandler for EventHandler {
    fn on_hello(&mut self, s: &SocketModeMessage) {
        println!("{:?}", s);
    }
    async fn on_interactive(
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
                        let view_modal = r#"{
          "type": "modal",
          "callback_id": "modal-with-inputs",
          "title": {
            "type": "plain_text",
            "text": "Modal with inputs"
          },
          "blocks": [
            {
              "type": "input",
              "block_id": "multiline",
              "label": {
                "type": "plain_text",
                "text": "Enter your value"
              },
              "element": {
                "type": "plain_text_input",
                "multiline": true,
                "action_id": "mlvalue"
              }
            },
            {
              "block_id": "target_channel",
              "type": "input",
              "optional": true,
              "label": {
                "type": "plain_text",
                "text": "Select a channel to post the result on",
              },
              "element": {
                "action_id": "target_select",
                "type": "conversations_select",
                "response_url_enabled": true,
              },
            }
          ],
          "submit": {
            "type": "plain_text",
            "text": "Submit"
          }
        }"#;

                        let query = Query {
                            trigger_id: result.trigger_id.to_string(),
                            view: String::from(view_modal),
                        };

                        let a = client.open_view(query).await.expect("test");
                        println!("{:?}", a);
                    }
                    None => {}
                },
                _ => {}
            },
            _ => {}
        }
    }
}
