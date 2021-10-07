pub mod api;

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
        token: &str,
        handler: &mut T,
    ) -> Result<(), surf::Error> {
        let connection = api::open_connection(token).await?;
        Ok(())
    }
}