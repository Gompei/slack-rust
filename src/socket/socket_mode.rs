use crate::apps::connections_open::connections_open;
use crate::error::Error;
use crate::http_client::SlackWebAPIClient;
use crate::socket::event::{
    AcknowledgeMessage, DisconnectEvent, EventsAPI, HelloEvent, InteractiveEvent,
    SlashCommandsEvent, SocketModeEvent,
};
use async_std::net::TcpStream;
use async_tls::client::TlsStream;
use async_tls::TlsConnector;
use async_trait::async_trait;
use async_tungstenite::tungstenite::Message;
use async_tungstenite::{client_async, WebSocketStream};
use futures_util::{SinkExt, StreamExt};
use url::Url;

pub type Stream = WebSocketStream<TlsStream<TcpStream>>;

/// Implement this trait in your code to handle slack events.
#[async_trait]
pub trait EventHandler {
    async fn on_close(&mut self) {
        log::info!("on_close");
    }
    async fn on_connect(&mut self) {
        log::info!("on_connect");
    }
    async fn on_hello(&mut self, e: &HelloEvent) {
        log::info!("on_hello: {:?}", e);
    }
    async fn on_disconnect(&mut self, e: &DisconnectEvent) {
        log::info!("on_disconnect: {:?}", e);
    }
    async fn on_events_api(&mut self, e: &EventsAPI, s: &mut Stream) {
        log::info!("on_events_api: {:?} {:?}", e, s);
    }
    async fn on_interactive(&mut self, e: &InteractiveEvent, s: &mut Stream) {
        log::info!("on_interactive: {:?} {:?}", e, s);
    }
    async fn on_slash_commands(&mut self, e: &SlashCommandsEvent, s: &mut Stream) {
        log::info!("on_slash_commands: {:?} {:?}", e, s);
    }
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
            let message = stream
                .next()
                .await
                .ok_or_else(|| Error::OptionError("web socket stream error".to_string()))?;

            match message? {
                Message::Text(t) => {
                    let event = serde_json::from_str::<SocketModeEvent>(&t)?;
                    match event {
                        SocketModeEvent::HelloEvent(e) => handler.on_hello(&e).await,
                        SocketModeEvent::DisconnectEvent(e) => handler.on_disconnect(&e).await,
                        SocketModeEvent::EventsAPI(e) => {
                            handler.on_events_api(&e, &mut stream).await
                        }
                        SocketModeEvent::InteractiveEvent(e) => {
                            handler.on_interactive(&e, &mut stream).await
                        }
                        SocketModeEvent::SlashCommandsEvent(e) => {
                            handler.on_slash_commands(&e, &mut stream).await
                        }
                    }
                }
                Message::Ping(p) => log::info!("ping: {:?}", p),
                Message::Close(_) => break,
                m => log::warn!("unsupported web socket message: {:?}", m),
            }
        }
        Ok(())
    }
    pub async fn ack(
        envelope_id: &String,
        stream: &mut WebSocketStream<TlsStream<TcpStream>>,
    ) -> Result<(), Error> {
        let json = serde_json::to_string(&AcknowledgeMessage { envelope_id })?;
        stream
            .send(Message::Text(json))
            .await
            .map_err(Error::WebSocketError)
    }
}
