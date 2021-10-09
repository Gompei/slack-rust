use crate::api::Token;
use async_tungstenite::tungstenite::Message;
use futures_util::{SinkExt, StreamExt};
use url::Url;

pub mod api;
pub mod error;

/// Implement this trait in your code to handle slack events
pub trait SocketModeEventHandler {
    fn on_hello() {}
    fn on_events_api() {}
    fn on_interactive() {}
    fn on_disconnect() {}
}

/// The socket client
pub struct SocketModeClient {}

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
                Message::Text(t) => {}
                Message::Ping(p) => {}
                Message::Close(_) => break,
                unknown => {}
            }
        }

        Ok(())
    }
}
