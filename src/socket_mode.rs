use crate::api::Token;
use crate::error;
use async_tungstenite::tungstenite::Message;
use futures_util::{SinkExt, StreamExt};
use url::Url;

/// Implement this trait in your code to handle slack events
pub trait SocketModeEventHandler {
    fn on_hello() {}
    fn on_events_api() {}
    fn on_interactive() {}
    fn on_disconnect() {}
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
    pub message_type: String,
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

impl SocketModeClient {
    pub async fn run<T: SocketModeEventHandler>(
        token: Token,
        _handler: &mut T,
    ) -> Result<(), error::Error> {
        let wss_url = token.open_connection().await?;
        // TODO: NoneError エラー処理が適切ではない
        let url = wss_url.url.expect("url does not exist");

        let wss_parsed = Url::parse(&url)?;

        // TODO: NoneError エラー処理が適切ではない
        let wss_domain = wss_parsed.domain().expect("domain parse error");

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
                        message_type,
                        payload,
                        ..
                    }) => match &*message_type {
                        // TODO: Enumにしたい
                        "hello" => {}
                        "event_api" => {}
                        "interactive" => match payload {
                            Some(result) => match &*result.message_type {
                                "shortcut" => match envelope_id {
                                    Some(id) => {
                                        stream.send(Message::Text(serde_json::to_string(
                                            &SocketModeAcknowledgeMessage {
                                                envelope_id: &id,
                                                payload: None,
                                            },
                                        )?));
                                        token.open_view(result.trigger_id).await?;
                                    }
                                    None => {}
                                },
                                _ => {}
                            },
                            None => {}
                        },
                        _ => println!("Unknown Socket Mode Event :{}", t),
                    },
                    Err(e) => {
                        println!("Unknown text frame: {}: {:?}", t, e);
                    }
                },
                Message::Ping(p) => {}
                Message::Close(_) => break,
                unknown => {}
            }
        }

        Ok(())
    }
}
