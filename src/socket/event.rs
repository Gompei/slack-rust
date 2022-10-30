use crate::event_api::event::Event;
use crate::payloads::interactive::{InteractivePayload, SlashPayload};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(tag = "type")]
pub enum SocketModeEvent {
    #[serde(rename = "hello")]
    HelloEvent(HelloEvent),
    #[serde(rename = "disconnect")]
    DisconnectEvent(DisconnectEvent),
    #[serde(rename = "events_api")]
    EventsAPI(EventsAPI),
    #[serde(rename = "interactive")]
    InteractiveEvent(InteractiveEvent),
    #[serde(rename = "slash_commands")]
    SlashCommandsEvent(SlashCommandsEvent),
}

impl SocketModeEvent {
    pub fn event_type(&self) -> SocketModeEventType {
        match self {
            SocketModeEvent::HelloEvent(HelloEvent { .. }) => SocketModeEventType::Hello,
            SocketModeEvent::DisconnectEvent(DisconnectEvent { .. }) => {
                SocketModeEventType::Disconnect
            }
            SocketModeEvent::EventsAPI(EventsAPI { .. }) => SocketModeEventType::EventsAPI,
            SocketModeEvent::InteractiveEvent(InteractiveEvent { .. }) => {
                SocketModeEventType::Interactive
            }
            SocketModeEvent::SlashCommandsEvent(SlashCommandsEvent { .. }) => {
                SocketModeEventType::SlashCommands
            }
        }
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SocketModeEventType {
    Hello,
    Disconnect,
    EventsAPI,
    Interactive,
    SlashCommands,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
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
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
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
pub struct EventsAPI {
    pub envelope_id: String,
    pub accepts_response_payload: bool,
    pub payload: Event,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct InteractiveEvent {
    pub envelope_id: String,
    pub accepts_response_payload: bool,
    pub payload: InteractivePayload,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct SlashCommandsEvent {
    pub envelope_id: String,
    pub accepts_response_payload: bool,
    pub payload: SlashPayload,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct AcknowledgeMessage<'s> {
    pub envelope_id: &'s str,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::event_api::event::*;
    use crate::payloads::interactive::InteractiveEventType;

    #[test]
    fn deserialize_hello_event() {
        let json = r##"{
  "type": "hello",
  "connection_info": {
    "app_id": "app_id"
  },
  "num_connections": 1,
  "debug_info": {
    "host": "host"
  }
}"##;
        let event = serde_json::from_str::<SocketModeEvent>(json).unwrap();
        match event {
            SocketModeEvent::HelloEvent(HelloEvent {
                connection_info,
                num_connections,
                debug_info,
            }) => {
                assert_eq!(connection_info.unwrap().app_id.unwrap(), "app_id");
                assert_eq!(num_connections.unwrap(), 1);
                assert_eq!(debug_info.unwrap().host.unwrap(), "host");
            }
            _ => panic!("Event deserialize into incorrect variant"),
        }
    }

    #[test]
    fn deserialize_disconnect_event() {
        let json = r##"{
  "type": "disconnect",
  "reason": "link_disabled",
  "debug_info": {
    "host": "wss-111.slack.com"
  }
}"##;
        let event = serde_json::from_str::<SocketModeEvent>(json).unwrap();
        match event {
            SocketModeEvent::DisconnectEvent(DisconnectEvent { reason, debug_info }) => {
                assert_eq!(reason, DisconnectReason::LinkDisabled);
                assert_eq!(debug_info.unwrap().host.unwrap(), "wss-111.slack.com");
            }
            _ => panic!("Event deserialize into incorrect variant"),
        }
    }

    #[test]
    fn deserialize_events_api() {
        let json = r##"{
  "type": "events_api",
  "envelope_id": "dbdd0ef3-1543-4f94-bfb4-133d0e6c1545",
  "accepts_response_payload": false,
  "payload": {
    "token": "bHKJ2n9AW6Ju3MjciOHfbA1b",
    "team_id": "T1234567890",
    "api_app_id": "A0000000000",
    "event_id": "Ev0000000000",
    "event_time": 1600000000,
    "type": "event_callback",
    "event": {
      "type": "app_home_opened",
      "user": "U061F7AUR",
      "channel": "D0LAN2Q65",
      "event_ts": "1515449522000016",
      "tab": "home",
      "view": {
        "id": "VPASKP233"
      }
    }
  }
}"##;
        let event = serde_json::from_str::<SocketModeEvent>(json).unwrap();
        match event {
            SocketModeEvent::EventsAPI(EventsAPI {
                envelope_id,
                accepts_response_payload,
                payload,
            }) => {
                assert_eq!(envelope_id, "dbdd0ef3-1543-4f94-bfb4-133d0e6c1545");
                assert!(!accepts_response_payload, "false");

                match payload.event {
                    EventCallback::AppHomeOpened { user, .. } => {
                        assert_eq!(user, "U061F7AUR");
                    }
                    _ => panic!("Event callback deserialize into incorrect variant"),
                }
            }
            _ => panic!("Event deserialize into incorrect variant"),
        }
    }

    #[test]
    fn deserialize_interactive_event() {
        let json = r##"{
  "type": "interactive",
  "envelope_id": "dbdd0ef3-1543-4f94-bfb4-133d0e6c1545",
  "accepts_response_payload": true,    
  "payload": {
    "type": "view_submission"
  }
}"##;
        let event = serde_json::from_str::<SocketModeEvent>(json).unwrap();
        match event {
            SocketModeEvent::InteractiveEvent(InteractiveEvent {
                envelope_id,
                accepts_response_payload,
                payload,
            }) => {
                assert_eq!(envelope_id, "dbdd0ef3-1543-4f94-bfb4-133d0e6c1545");
                assert!(accepts_response_payload, "true");
                assert_eq!(payload.type_filed, InteractiveEventType::ViewSubmission);
            }
            _ => panic!("Event deserialize into incorrect variant"),
        }
    }

    #[test]
    fn deserialize_slash_commands_event() {
        let json = r##"{
  "type": "slash_commands",
  "envelope_id": "dbdd0ef3-1543-4f94-bfb4-133d0e6c1545",
  "accepts_response_payload": true,
  "payload": {
    "token": "bHKJ2n9AW6Ju3MjciOHfbA1b"
  }
}"##;
        let event = serde_json::from_str::<SocketModeEvent>(json).unwrap();
        match event {
            SocketModeEvent::SlashCommandsEvent(SlashCommandsEvent {
                envelope_id,
                accepts_response_payload,
                payload,
            }) => {
                assert_eq!(envelope_id, "dbdd0ef3-1543-4f94-bfb4-133d0e6c1545");
                assert!(accepts_response_payload, "true");
                assert_eq!(payload.token.unwrap(), "bHKJ2n9AW6Ju3MjciOHfbA1b");
            }
            _ => panic!("Event deserialize into incorrect variant"),
        }
    }

    #[test]
    fn serialize_acknowledge_message() {
        let json = serde_json::to_string_pretty(&AcknowledgeMessage {
            envelope_id: "xxxxxxxxxxxxxxxxxxxxx",
        })
        .unwrap();

        let expect = r##"{
  "envelope_id": "xxxxxxxxxxxxxxxxxxxxx"
}"##;

        assert_eq!(expect, json);
    }
}
