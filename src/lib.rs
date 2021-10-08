use crate::api::Token;

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
        handler: &mut T,
    ) -> Result<(), error::Error> {
        let connection = token.open_connection().await?;
        Ok(())
    }
}
