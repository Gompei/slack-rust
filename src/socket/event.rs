use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(tag = "type")]
pub enum SocketModeEvent {
    #[serde(rename = "hello")]
    HelloEvent(HelloEvent),
    #[serde(rename = "disconnect")]
    DisconnectEvent(DisconnectEvent),
    #[serde(rename = "event_api")]
    APIEvent(CommonEvent),
    #[serde(rename = "interactive")]
    InteractiveEvent(CommonEvent),
}

impl SocketModeEvent {
    pub fn event_type(&self) -> SocketModeEventType {
        match self {
            SocketModeEvent::HelloEvent(HelloEvent { .. }) => SocketModeEventType::Hello,
            SocketModeEvent::DisconnectEvent(DisconnectEvent { .. }) => {
                SocketModeEventType::Disconnect
            }
            SocketModeEvent::APIEvent(CommonEvent { .. }) => SocketModeEventType::EventApi,
            SocketModeEvent::InteractiveEvent(CommonEvent { .. }) => {
                SocketModeEventType::Interactive
            }
        }
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SocketModeEventType {
    Hello,
    Disconnect,
    EventApi,
    Interactive,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct DebugInfo {
    pub host: Option<String>,
    pub started: Option<String>,
    pub build_number: Option<i32>,
    pub approximate_connection_time: Option<i32>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct HelloEvent {
    pub connection_info: Option<ConnectionInfo>,
    pub num_connections: Option<i32>,
    pub debug_info: Option<DebugInfo>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct ConnectionInfo {
    pub app_id: Option<String>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct DisconnectEvent {
    pub reason: DisconnectReason,
    pub debug_info: Option<DebugInfo>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DisconnectReason {
    LinkDisabled,
    Warning,
    RefreshRequested,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct CommonEvent {}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct AcknowledgeMessage {
    pub envelope_id: String,
}
