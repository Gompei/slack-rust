use crate::api::{ApiClient, Token};
use crate::error;
use async_tungstenite::tungstenite::Message;
use futures_util::{SinkExt, StreamExt};
use url::Url;

/// Implement this trait in your code to handle slack events
pub trait SocketModeEventHandler {
    fn on_hello(&mut self, s: &SocketModeMessage) {
        println!("The on_hello function is not implemented.");
    }
    fn on_events_api(&mut self, s: &SocketModeMessage) {
        println!("The on_events_api function is not implemented.");
    }
    fn on_interactive(&mut self, s: &SocketModeMessage) {
        println!("The on_interactive function is not implemented.")
    }
    fn on_disconnect(&mut self) {
        println!("The on_disconnect function is not implemented.")
    }
}

/// The socket client
pub struct SocketModeClient {}

#[derive(serde::Serialize)]
pub struct SocketModeAcknowledgeMessage<'s> {
    pub envelope_id: &'s str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<&'s str>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "snake_case", tag = "type")]
pub struct SocketModeMessage {
    #[serde(rename = "envelope_id", skip_serializing_if = "Option::is_none")]
    envelope_id: Option<String>,
    #[serde(rename = "type")]
    pub message_type: SocketModeEventType,
    #[serde(rename = "payload", skip_serializing_if = "Option::is_none")]
    pub payload: Option<Payload>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "snake_case", tag = "type")]
pub struct Payload {
    pub trigger_id: String,
    #[serde(rename = "type")]
    pub message_type: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename = "type", rename_all = "snake_case")]
pub enum SocketModeEventType {
    Hello,
    Disconnect,
    EventApi,
    Interactive,
}

impl SocketModeClient {
    pub async fn run<T: SocketModeEventHandler>(
        client: ApiClient,
        handler: &mut T,
    ) -> Result<(), error::Error> {
        let wss_url = client.open_connection().await?;
        let url = wss_url
            .url
            .ok_or_else(|| error::Error::OptionError("Option Error".to_string()))?;
        let wss_parsed = Url::parse(&url)?;
        let wss_domain = wss_parsed
            .domain()
            .ok_or_else(|| error::Error::OptionError("domain parse error".to_string()))?;

        let tcp_stream = async_std::net::TcpStream::connect(&format!("{}:443", wss_domain)).await?;
        let tls_stream = async_tls::TlsConnector::default()
            .connect(wss_domain, tcp_stream)
            .await?;

        let (mut stream, _) = async_tungstenite::client_async(url, tls_stream).await?;
        while let Some(message) = stream.next().await {
            match message? {
                Message::Text(t) => match serde_json::from_str(&t) {
                    Ok(SocketModeMessage {
                        envelope_id,
                        message_type: SocketModeEventType,
                        payload,
                        ..
                    }) => match SocketModeEventType {
                        SocketModeEventType::Hello => handler.on_hello(&SocketModeMessage {
                            envelope_id,
                            message_type: SocketModeEventType,
                            payload,
                        }),
                        SocketModeEventType::EventApi => {
                            handler.on_events_api(&SocketModeMessage {
                                envelope_id,
                                message_type: SocketModeEventType,
                                payload,
                            })
                        }
                        SocketModeEventType::Interactive => {
                            handler.on_interactive(&SocketModeMessage {
                                envelope_id,
                                message_type: SocketModeEventType,
                                payload,
                            })
                        }
                        _ => println!("Unknown Socket Mode Event :{}", t),
                    },
                    Err(e) => {
                        println!("Unknown text frame: {}: {:?}", t, e);
                    }
                },
                Message::Ping(p) => {}
                Message::Close(_) => break,
                _ => {}
            }
        }
        Ok(())
    }
}
