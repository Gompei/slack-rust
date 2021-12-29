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
use rustls::ClientConfig;
use std::collections::HashMap;
use std::io::Cursor;
use std::sync::Arc;
use url::Url;

pub type Stream = WebSocketStream<TlsStream<TcpStream>>;

/// Implement this trait in your code to handle slack events.
#[allow(unused_variables)]
#[async_trait]
pub trait EventHandler<S>: Send
where
    S: SlackWebAPIClient,
{
    async fn on_close(&mut self, socket_mode: &SocketMode<S>) {
        log::info!("websocket close");
    }
    async fn on_connect(&mut self, socket_mode: &SocketMode<S>) {
        log::info!("websocket connect");
    }
    async fn on_hello(&mut self, socket_mode: &SocketMode<S>, e: HelloEvent, s: &mut Stream) {
        log::info!("hello event: {:?}", e);
    }
    async fn on_disconnect(
        &mut self,
        socket_mode: &SocketMode<S>,
        e: DisconnectEvent,
        s: &mut Stream,
    ) {
        log::info!("disconnect event: {:?}", e);
    }
    async fn on_events_api(&mut self, socket_mode: &SocketMode<S>, e: EventsAPI, s: &mut Stream) {
        log::info!("events api event: {:?}", e);
    }
    async fn on_interactive(
        &mut self,
        socket_mode: &SocketMode<S>,
        e: InteractiveEvent,
        s: &mut Stream,
    ) {
        log::info!("interactive event: {:?}", e);
    }
    async fn on_slash_commands(
        &mut self,
        socket_mode: &SocketMode<S>,
        e: SlashCommandsEvent,
        s: &mut Stream,
    ) {
        log::info!("slash commands event: {:?}", e);
    }
}

/// The socket mode client.
pub struct SocketMode<S>
where
    S: SlackWebAPIClient,
{
    pub api_client: S,
    pub app_token: String,
    pub bot_token: String,
    pub option_parameter: HashMap<String, String>,
    web_socket_port: u16,
    test_ca_file_path: Option<String>,
}

impl<S> SocketMode<S>
where
    S: SlackWebAPIClient,
{
    pub fn new(api_client: S, app_token: String, bot_token: String) -> Self {
        SocketMode {
            api_client,
            app_token,
            bot_token,
            option_parameter: HashMap::new(),
            web_socket_port: 443,
            test_ca_file_path: None,
        }
    }
    pub fn option_parameter(mut self, key: String, value: String) -> Self {
        self.option_parameter.insert(key, value);
        self
    }
    fn web_socket_port(mut self, port: u16) -> Self {
        self.web_socket_port = port;
        self
    }
    fn test_ca_file_path(mut self, test_ca_file_path: String) -> Self {
        self.test_ca_file_path = Some(test_ca_file_path);
        self
    }
    /// Run slack and websocket communication.
    pub async fn run<T>(self, handler: &mut T) -> Result<(), Error>
    where
        T: EventHandler<S>,
    {
        let response = connections_open(&self.api_client, &self.app_token).await?;
        let ws_url = response
            .url
            .ok_or_else(|| Error::OptionError("connections open api error".to_string()))?;
        let ws_url_parsed = Url::parse(&ws_url)?;
        let ws_domain = ws_url_parsed
            .domain()
            .ok_or_else(|| Error::OptionError("url doesn't have domain".to_string()))?;

        let tcp_stream = TcpStream::connect((ws_domain, self.web_socket_port)).await?;
        let connector = if let Some(test_ca_file_path) = &self.test_ca_file_path {
            connector_for_ca_file(test_ca_file_path).await?
        } else {
            TlsConnector::default()
        };
        let tls_stream = connector.connect(ws_domain, tcp_stream).await?;

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
                        SocketModeEvent::HelloEvent(e) => handler.on_hello(&self, e, &mut ws).await,
                        SocketModeEvent::DisconnectEvent(e) => {
                            handler.on_disconnect(&self, e, &mut ws).await
                        }
                        SocketModeEvent::EventsAPI(e) => {
                            handler.on_events_api(&self, e, &mut ws).await
                        }
                        SocketModeEvent::InteractiveEvent(e) => {
                            handler.on_interactive(&self, e, &mut ws).await
                        }
                        SocketModeEvent::SlashCommandsEvent(e) => {
                            handler.on_slash_commands(&self, e, &mut ws).await
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

async fn connector_for_ca_file(ca_file_path: &str) -> Result<TlsConnector, Error> {
    let mut config = ClientConfig::new();
    let file = async_std::fs::read(ca_file_path).await?;
    let mut pem = Cursor::new(file);
    config.root_store.add_pem_file(&mut pem);
    Ok(TlsConnector::from(Arc::new(config)))
}

#[cfg(test)]
mod test {
    use crate::event_api::app::AppHomeOpenedEvent;
    use crate::event_api::event::Event;
    use crate::http_client::{MockSlackWebAPIClient, SlackWebAPIClient};
    use crate::payloads::interactive::InteractiveEventType;
    use crate::socket::event::{
        DisconnectEvent, DisconnectReason, EventsAPI, HelloEvent, InteractiveEvent,
        SlashCommandsEvent, SocketModeEvent,
    };
    use crate::socket::socket_mode::{ack, EventHandler, SocketMode, Stream};
    use async_std::net::TcpListener;
    use async_std::task;
    use async_tls::TlsAcceptor;
    use async_trait::async_trait;
    use async_tungstenite::tungstenite::Message;
    use futures_util::{SinkExt, StreamExt};
    use rustls::internal::pemfile::{certs, pkcs8_private_keys};
    use rustls::{Certificate, NoClientAuth, PrivateKey, ServerConfig};
    use std::error::Error;
    use std::fs::File;
    use std::io;
    use std::io::BufReader;
    use std::sync::Arc;

    pub struct Handler;

    #[allow(unused_variables)]
    #[async_trait]
    impl<S> EventHandler<S> for Handler
    where
        S: SlackWebAPIClient,
    {
        async fn on_close(&mut self, socket_mode: &SocketMode<S>) {
            assert!(true, "always true");
            log::info!("success on_close test");
        }
        async fn on_connect(&mut self, socket_mode: &SocketMode<S>) {
            assert!(true, "always true");
            log::info!("success on_connect test");
        }
        async fn on_hello(&mut self, socket_mode: &SocketMode<S>, e: HelloEvent, s: &mut Stream) {
            assert_eq!(e.connection_info.unwrap().app_id.unwrap(), "app_id");
            assert_eq!(e.num_connections.unwrap(), 1);
            assert_eq!(e.debug_info.unwrap().host.unwrap(), "host");
            log::info!("success on_hello test");
        }
        async fn on_disconnect(
            &mut self,
            socket_mode: &SocketMode<S>,
            e: DisconnectEvent,
            s: &mut Stream,
        ) {
            assert_eq!(e.reason, DisconnectReason::LinkDisabled);
            assert_eq!(e.debug_info.unwrap().host.unwrap(), "wss-111.slack.com");
            log::info!("success on_disconnect test");
        }
        async fn on_events_api(
            &mut self,
            socket_mode: &SocketMode<S>,
            e: EventsAPI,
            s: &mut Stream,
        ) {
            assert_eq!(e.envelope_id, "dbdd0ef3-1543-4f94-bfb4-133d0e6c1545");
            assert_eq!(e.accepts_response_payload, false);

            match e.payload {
                Event::AppHomeOpened(AppHomeOpenedEvent { user, .. }) => {
                    assert_eq!(user.unwrap(), "U061F7AUR");
                }
                _ => panic!("Payload deserialize into incorrect variant"),
            }
            log::info!("success on_events_api test");
        }
        async fn on_interactive(
            &mut self,
            socket_mode: &SocketMode<S>,
            e: InteractiveEvent,
            s: &mut Stream,
        ) {
            assert_eq!(e.envelope_id, "dbdd0ef3-1543-4f94-bfb4-133d0e6c1545");
            assert_eq!(e.accepts_response_payload, true);
            assert_eq!(e.payload.type_filed, InteractiveEventType::ViewSubmission);
            log::info!("success on_interactive test")
        }
        async fn on_slash_commands(
            &mut self,
            socket_mode: &SocketMode<S>,
            e: SlashCommandsEvent,
            s: &mut Stream,
        ) {
            assert_eq!(e.envelope_id, "dbdd0ef3-1543-4f94-bfb4-133d0e6c1545");
            assert_eq!(e.accepts_response_payload, true);
            assert_eq!(e.payload.token.unwrap(), "bHKJ2n9AW6Ju3MjciOHfbA1b");
            log::info!("success on_slash_commands test");
        }
    }

    #[async_std::test]
    async fn test_socket_mode() {
        env_logger::init();

        let mut event = vec![
            r##"{
  "type": "hello",
  "connection_info": {
    "app_id": "app_id"
  },
  "num_connections": 1,
  "debug_info": {
    "host": "host"
  }
}"##
            .to_string(),
            r##"{
  "type": "disconnect",
  "reason": "link_disabled",
  "debug_info": {
    "host": "wss-111.slack.com"
  }
}"##
            .to_string(),
            r##"{
  "type": "events_api",
  "envelope_id": "dbdd0ef3-1543-4f94-bfb4-133d0e6c1545",
  "accepts_response_payload": false,
  "payload": {
    "type": "app_home_opened",
    "user": "U061F7AUR"
  }
}"##
            .to_string(),
            r##"{
  "type": "interactive",
  "envelope_id": "dbdd0ef3-1543-4f94-bfb4-133d0e6c1545",
  "accepts_response_payload": true,
  "payload": {
    "type": "view_submission"
  }
}"##
            .to_string(),
            r##"{
  "type": "slash_commands",
  "envelope_id": "dbdd0ef3-1543-4f94-bfb4-133d0e6c1545",
  "accepts_response_payload": true,
  "payload": {
    "token": "bHKJ2n9AW6Ju3MjciOHfbA1b"
  }
}"##
            .to_string(),
        ];

        let mut mock = MockSlackWebAPIClient::new();
        mock.expect_post().times(1).returning(|_, _| {
            Ok(r##"{
                  "ok": true,
                  "url": "wss://localhost"
                }"##
            .to_string())
        });

        let port = mock_web_socket(event).await.unwrap();
        SocketMode::new(
            mock,
            "slack_app_token".to_string(),
            "slack_bot_token".to_string(),
        )
        .web_socket_port(port)
        .option_parameter(
            "SLACK_CHANNEL_ID".to_string(),
            "slack_channel_id".to_string(),
        )
        .test_ca_file_path("rootCA.pem".to_string())
        .run(&mut Handler)
        .await
        .unwrap_or_else(|_| panic!("socket mode run error."));
    }

    async fn mock_web_socket(mut event: Vec<String>) -> Result<u16, Box<dyn Error>> {
        let listener = TcpListener::bind("localhost:0").await?;
        let port = listener.local_addr()?.port();

        task::spawn(async move {
            web_socket_handler(&listener, event).await;
        });

        Ok(port)
    }

    async fn web_socket_handler(listener: &TcpListener, mut event: Vec<String>) {
        let config = load_config("localhost.pem", "localhost-key.pem").unwrap();
        // TODO: async-tungstenite latest version Crate depends on rustls v.0.19
        let acceptor = TlsAcceptor::from(Arc::new(config));

        let mut incoming = listener.incoming();

        while let Some(stream) = incoming.next().await {
            let acceptor = acceptor.clone();
            let tcp_stream = stream.unwrap();
            let tls_stream = acceptor.accept(tcp_stream).await.unwrap();
            let mut ws = async_tungstenite::accept_async(tls_stream).await.unwrap();

            let m = event.clone();

            for e in m {
                ws.send(Message::Text(e.to_string())).await.unwrap();
            }

            ws.close(None).await.unwrap();
        }
    }

    fn load_config(certs_path: &str, key_path: &str) -> io::Result<ServerConfig> {
        let certs = load_certs(certs_path).unwrap();
        let mut private_key = load_key(key_path).unwrap();

        let mut config = ServerConfig::new(NoClientAuth::new());
        config
            .set_single_cert(certs, private_key.remove(0))
            .unwrap();

        Ok(config)
    }

    fn load_certs(path: &str) -> io::Result<Vec<Certificate>> {
        certs(&mut BufReader::new(File::open(path)?))
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid cert"))
    }

    fn load_key(path: &str) -> io::Result<Vec<PrivateKey>> {
        pkcs8_private_keys(&mut BufReader::new(File::open(path)?))
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid key"))
    }
}
