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
use std::collections::HashMap;
use url::Url;

pub type Stream = WebSocketStream<TlsStream<TcpStream>>;

/// Implement this trait in your code to handle slack events.
#[allow(unused_variables)]
#[async_trait]
pub trait EventHandler {
    async fn on_close(&mut self, socket_mode: &SocketMode) {
        log::info!("websocket close");
    }
    async fn on_connect(&mut self, socket_mode: &SocketMode) {
        log::info!("websocket connect");
    }
    async fn on_hello(&mut self, socket_mode: &SocketMode, e: &HelloEvent) {
        log::info!("hello event: {:?}", e);
    }
    async fn on_disconnect(&mut self, socket_mode: &SocketMode, e: &DisconnectEvent) {
        log::info!("disconnect event: {:?}", e);
    }
    async fn on_events_api(&mut self, socket_mode: &SocketMode, e: &EventsAPI, s: &mut Stream) {
        log::info!("events api event: {:?}", e);
    }
    async fn on_interactive(
        &mut self,
        socket_mode: &SocketMode,
        e: &InteractiveEvent,
        s: &mut Stream,
    ) {
        log::info!("interactive event: {:?}", e);
    }
    async fn on_slash_commands(
        &mut self,
        socket_mode: &SocketMode,
        e: &SlashCommandsEvent,
        s: &mut Stream,
    ) {
        log::info!("slash commands event: {:?}", e);
    }
}

/// The socket mode client.
pub struct SocketMode {
    pub api_client: Client,
    pub app_token: String,
    pub bot_token: String,
    pub option_parameter: HashMap<String, String>,
}

impl SocketMode {
    pub fn new(app_token: String, bot_token: String) -> Self {
        SocketMode {
            api_client: default_client(),
            app_token,
            bot_token,
            option_parameter: HashMap::new(),
        }
    }
    pub fn option_parameter(mut self, key: String, value: String) -> Self {
        self.option_parameter.insert(key, value);
        self
    }
    /// Run slack and websocket communication.
    pub async fn run<T>(self, handler: &mut T) -> Result<(), Error>
    where
        T: EventHandler + std::marker::Send,
    {
        let response = connections_open(&self.api_client, &self.app_token).await?;
        let ws_url = response
            .url
            .ok_or_else(|| Error::OptionError("connections open api error".to_string()))?;
        let ws_url_parsed = Url::parse(&ws_url)?;
        let ws_domain = ws_url_parsed
            .domain()
            .ok_or_else(|| Error::OptionError("url doesn't have domain".to_string()))?;

        let tcp_stream = TcpStream::connect((ws_domain, 443)).await?;
        let tls_stream = TlsConnector::default()
            .connect(ws_domain, tcp_stream)
            .await?;
        let (mut ws, _) = client_async(&ws_url, tls_stream).await?;

        handler.on_connect(&self).await;

        loop {
            let message = ws
                .next()
                .await
                .ok_or_else(|| Error::OptionError("web socket stream error".to_string()))?;

            match message? {
                Message::Text(t) => {
                    let event = serde_json::from_str::<SocketModeEvent>(&t)?;
                    match event {
                        SocketModeEvent::HelloEvent(e) => handler.on_hello(&self, &e).await,
                        SocketModeEvent::DisconnectEvent(e) => {
                            handler.on_disconnect(&self, &e).await
                        }
                        SocketModeEvent::EventsAPI(e) => {
                            handler.on_events_api(&self, &e, &mut ws).await
                        }
                        SocketModeEvent::InteractiveEvent(e) => {
                            handler.on_interactive(&self, &e, &mut ws).await
                        }
                        SocketModeEvent::SlashCommandsEvent(e) => {
                            handler.on_slash_commands(&self, &e, &mut ws).await
                        }
                    }
                }
                Message::Ping(p) => log::info!("ping: {:?}", p),
                Message::Close(_) => {
                    handler.on_close(&self).await;
                    break;
                }
                m => log::warn!("unsupported web socket message: {:?}", m),
            }
        }
        Ok(())
    }
}

pub async fn ack(envelope_id: &str, stream: &mut Stream) -> Result<(), Error> {
    let json = serde_json::to_string(&AcknowledgeMessage { envelope_id })?;
    stream
        .send(Message::Text(json))
        .await
        .map_err(Error::WebSocketError)
}
