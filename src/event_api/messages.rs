use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(tag = "subtype")]
pub enum MessageSubtype {
    #[serde(rename = "message_updated")]
    MessageUpdate,
    #[serde(rename = "message_replied")]
    MessageReplied {
        #[serde(rename = "type", default = "MessageSubtype::default_type")]
        _type: String,
        channel: String,
        event_ts: String,
        hidden: bool,
        message: MessageReply,
        ts: String,
    }
}

impl MessageSubtype {
    fn default_type() -> String {
        "message".to_string()
    }

}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[skip_serializing_none]
pub struct MessageBasic {
    pub channel: String,
    pub channel_type: String,
    pub edited: Option<MessageEdit>,
    pub event_ts: String,
    pub text: String,
    pub thread_ts: Option<String>,
    pub ts: String,
    pub user: String,
}

// TODO - this could probably be merged with MessageBasic
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct MessageReply {
    pub replies: Option<Vec<MessageEdit>>,
    pub reply_count: u8,
    pub text: String,
    pub thread_ts: String,
    pub ts: String,
    pub user: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[skip_serializing_none]
pub struct MessageEdit {
    pub user: String,
    pub ts: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[skip_serializing_none]
pub struct MessageUpdate {
    pub edited: MessageEdit,
    pub subtype: String,
    pub text: String,
    pub ts: String,
    pub user: String,
}

#[cfg(test)]
mod tests {
    use crate::event_api::messages::*;
    use serde_json::Value;

    #[test]
    fn serializes_message_replied_correctly() {
        let expected = r##"
        {
        "type": "message",
        "message": {
          "type": "message",
          "user": "U1111111",
          "text": "Was there was there was there what was there was there what was there was there there was there.",
          "thread_ts": "1482960137.003543",
          "reply_count": 1,
          "replies": [
            {
              "user": "U2222222",
              "ts": "1483037603.017503"
            }
          ],
          "ts": "1482960137.003543"
        },
        "subtype": "message_replied",
        "hidden": true,
        "channel": "C12345678",
        "event_ts": "1483037604.017506",
        "ts": "1483037604.017506"
        }"##;

        let deserialized = serde_json::from_str::<MessageSubtype>(&expected).unwrap();
        // Reserialize this for later assertions
        let serialized = serde_json::to_string(&deserialized).unwrap();
        match deserialized {
            MessageSubtype::MessageReplied{message, _type, ..} => {
                assert_eq!(message.user, "U1111111");
                assert_eq!(_type, "message".to_string());
            },
            _ => panic!("Event callback deserialize into incorrect variant"),
        }
        // Validate that reserialized JSON contains expected fields
        let json: Value = serde_json::from_str(&serialized).unwrap();
        assert_eq!(json["subtype"], "message_replied".to_string());
        assert_eq!(json["type"], "message".to_string());
    }
}
