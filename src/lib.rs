use crate::api::Token;
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

        // TODO: エラー処理が適切ではない
        let wss_parsed = Url::parse(&wss_url.url.expect("open connection api error"));
        println!("{:?}", wss_parsed);
        Ok(())
    }
}
