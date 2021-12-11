use crate::apps::connections_open::connections_open;
use crate::error::Error;
use crate::http_client::{Client, SlackWebAPIClient};

use async_std::net::TcpStream;
use async_tls::client::TlsStream;
use async_tls::TlsConnector;
use async_trait::async_trait;
use async_tungstenite::tungstenite::Message;
use async_tungstenite::{client_async, WebSocketStream};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use url::Url;

/// Implement this trait in your code to handle slack events.
#[async_trait]
pub trait EventHandler {
    async fn on_close(&mut self) {
        println!("on_close function is not implemented.")
    }
    async fn on_connect(&mut self) {
        println!("on_connect function is not implemented.")
    }
    async fn on_disconnect(&mut self, _s: &SocketMessage) {
        println!("on_connect function is not implemented.")
    }
    async fn on_events_api(&mut self, _s: &SocketMessage) {
        println!("on_events_api function is not implemented.")
    }
    async fn on_hello(&mut self, _s: &SocketMessage) {
        println!("on_hello function is not implemented.")
    }
    async fn on_interactive(&mut self, _s: &SocketMessage) {
        println!("on_interactive function is not implemented.")
    }
    async fn on_ping(&mut self, _ping: Vec<u8>) {
        println!("on_ping function is not implemented.")
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SocketMessage {
    pub envelope_id: Option<String>,
    #[serde(rename = "type")]
    pub message_type: SocketEventType,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum SocketEventType {
    Hello,
    Disconnect,
    EventApi,
    Interactive,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AcknowledgeMessage {
    pub envelope_id: String,
}

/// The socket mode client.
pub struct SocketMode {}

impl SocketMode {
    /// Run slack and websocket communication.
    pub async fn run<T, S>(client: &S, app_token: String, handler: &mut T) -> Result<(), Error>
    where
        T: EventHandler + std::marker::Send,
        S: SlackWebAPIClient,
    {
        let wss_url = connections_open(client, app_token).await?;
        let url = wss_url
            .url
            .ok_or_else(|| Error::OptionError("connections open error".to_string()))?;
        let wss_parsed = Url::parse(&url)?;
        let wss_domain = wss_parsed
            .domain()
            .ok_or_else(|| Error::OptionError("domain parse error".to_string()))?;

        let tcp_stream = TcpStream::connect(&format!("{}:443", wss_domain)).await?;
        let tls_stream = TlsConnector::default()
            .connect(wss_domain, tcp_stream)
            .await?;

        let (mut stream, _) = client_async(url, tls_stream).await?;

        handler.on_connect();

        loop {
            let next_stream = stream
                .next()
                .await
                .ok_or_else(|| Error::OptionError("web socket stream error".to_string()))?;

            match next_stream? {
                Message::Text(t) => match serde_json::from_str(&t) {
                    Ok(SocketMessage {
                        envelope_id,
                        message_type: socket_event_type,
                        ..
                    }) => match socket_event_type {
                        SocketEventType::Disconnect => {
                            handler
                                .on_disconnect(&SocketMessage {
                                    envelope_id,
                                    message_type: socket_event_type,
                                })
                                .await
                        }
                        SocketEventType::EventApi => {
                            handler
                                .on_events_api(&SocketMessage {
                                    envelope_id,
                                    message_type: socket_event_type,
                                })
                                .await
                        }
                        SocketEventType::Hello => {
                            handler
                                .on_hello(&SocketMessage {
                                    envelope_id,
                                    message_type: socket_event_type,
                                })
                                .await
                        }
                        SocketEventType::Interactive => {
                            handler
                                .on_interactive(&SocketMessage {
                                    envelope_id,
                                    message_type: socket_event_type,
                                })
                                .await
                        }
                    },
                    Err(e) => println!("unknown text frame: {} {:?}", t, e),
                },
                Message::Ping(p) => handler.on_ping(p).await,
                Message::Close(_) => handler.on_close().await,
                m => {
                    println!("unsupported web socket message: {:?}", m);
                }
            }
        }
    }
    pub async fn ack(envelope_id: String, stream: &mut WebSocketStream<TlsStream<TcpStream>>) {
        stream.send(Message::Text(
            serde_json::to_string(&AcknowledgeMessage { envelope_id })
                .expect("send acknowledge message error"),
        ));
    }
}
