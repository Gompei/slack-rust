use crate::apps::connections_open::connections_open;
use crate::error::Error;
use crate::http_client::SlackWebAPIClient;
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
        log::info!("on_close");
    }
    async fn on_connect(&mut self) {
        log::info!("on_connect");
    }
    async fn on_disconnect(&mut self, s: &SocketMessage) {
        log::info!("on_disconnect: {:?}", s);
    }
    async fn on_events_api(&mut self, s: &SocketMessage) {
        log::info!("on_events_api: {:?}", s);
    }
    async fn on_hello(&mut self, s: &SocketMessage) {
        log::info!("on_hello: {:?}", s);
    }
    async fn on_interactive(&mut self, s: &SocketMessage) {
        log::info!("on_interactive: {:?}", s);
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct SocketMessage {
    pub envelope_id: Option<String>,
    #[serde(rename = "type")]
    pub message_type: SocketEventType,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SocketEventType {
    Hello,
    Disconnect,
    EventApi,
    Interactive,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct AcknowledgeMessage {
    pub envelope_id: String,
}

/// The socket mode client.
pub struct SocketMode {}

impl SocketMode {
    /// Run slack and websocket communication.
    pub async fn run<T, S>(client: &S, app_token: &str, handler: &mut T) -> Result<(), Error>
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

        handler.on_connect().await;

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
                    Err(e) => log::error!("unknown text frame: {} {:?}", t, e),
                },
                Message::Ping(p) => log::info!("ping: {:?}", p),
                Message::Close(_) => break,
                m => log::warn!("unsupported web socket message: {:?}", m),
            }
        }
        Ok(())
    }
    pub async fn ack(
        envelope_id: String,
        stream: &mut WebSocketStream<TlsStream<TcpStream>>,
    ) -> Result<(), Error> {
        let json = serde_json::to_string(&AcknowledgeMessage { envelope_id })?;
        stream
            .send(Message::Text(json))
            .await
            .map_err(Error::WebSocketError)
    }
}
