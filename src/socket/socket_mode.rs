use crate::apps::connections_open::connections_open;
use crate::error::Error;
use crate::http_client::{default_client, Client};
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
    async fn on_close(&mut self, _client: &Client, _token: &SlackToken) {
        log::info!("websocket close");
    }
    async fn on_connect(&mut self, _client: &Client, _token: &SlackToken) {
        log::info!("websocket connect");
    }
    async fn on_hello(&mut self, _client: &Client, _token: &SlackToken, e: &HelloEvent) {
        log::info!("hello event: {:?}", e);
    }
    async fn on_disconnect(&mut self, _client: &Client, _token: &SlackToken, e: &DisconnectEvent) {
        log::info!("disconnect event: {:?}", e);
    }
    async fn on_events_api(
        &mut self,
        _client: &Client,
        _token: &SlackToken,
        e: &EventsAPI,
        _s: &mut Stream,
    ) {
        log::info!("events api event: {:?}", e);
    }
    async fn on_interactive(
        &mut self,
        _client: &Client,
        _token: &SlackToken,
        e: &InteractiveEvent,
        _s: &mut Stream,
    ) {
        log::info!("interactive event: {:?}", e);
    }
    async fn on_slash_commands(
        &mut self,
        _client: &Client,
        _token: &SlackToken,
        e: &SlashCommandsEvent,
        _s: &mut Stream,
    ) {
        log::info!("slash commands event: {:?}", e);
    }
}

/// The socket mode client.
pub struct SocketMode {}

impl SocketMode {
    /// Run slack and websocket communication.
    pub async fn run<T>(token: &SlackToken, handler: &mut T) -> Result<(), Error>
    where
        T: EventHandler + std::marker::Send,
    {
        let client = default_client();
        let wss_url = connections_open(&client, &token.slack_app_token).await?;
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

        handler.on_connect(&client, token).await;

        loop {
            let message = stream
                .next()
                .await
                .ok_or_else(|| Error::OptionError("web socket stream error".to_string()))?;

            match message? {
                Message::Text(t) => {
                    let event = serde_json::from_str::<SocketModeEvent>(&t)?;
                    match event {
                        SocketModeEvent::HelloEvent(e) => {
                            handler.on_hello(&client, token, &e).await
                        }
                        SocketModeEvent::DisconnectEvent(e) => {
                            handler.on_disconnect(&client, token, &e).await
                        }
                        SocketModeEvent::EventsAPI(e) => {
                            handler.on_events_api(&client, token, &e, &mut stream).await
                        }
                        SocketModeEvent::InteractiveEvent(e) => {
                            handler
                                .on_interactive(&client, token, &e, &mut stream)
                                .await
                        }
                        SocketModeEvent::SlashCommandsEvent(e) => {
                            handler
                                .on_slash_commands(&client, token, &e, &mut stream)
                                .await
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
    pub async fn ack(envelope_id: &String, stream: &mut Stream) -> Result<(), Error> {
        let json = serde_json::to_string(&AcknowledgeMessage { envelope_id })?;
        stream
            .send(Message::Text(json))
            .await
            .map_err(Error::WebSocketError)
    }
}

pub struct SlackToken {
    pub slack_app_token: String,
    pub slack_bot_token: String,
}
