//! [Event API Types](https://api.slack.com/events?filter=Events)

use crate::channels::channel::Channel;
use crate::event_api::app::AppRequest;
use crate::event_api::messages::{MessageBasic, MessageSubtype};
use crate::team::teams::Team;
use crate::views::view::View;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// [Event API](https://api.slack.com/events?filter=Events)
/// [Event API Type](https://api.slack.com/events?filter=Events)
/// Example of an [event wrapper](https://api.slack.com/types/event)
/// ```
/// {
///         "token": "XXYYZZ",
///         "team_id": "TXXXXXXXX",
///         "api_app_id": "AXXXXXXXXX",
///         "event": {
///                 "type": "name_of_event",
///                 "event_ts": "1234567890.123456",
///                 "user": "UXXXXXXX1"
///         },
///         "type": "event_callback",
///         "authed_users": [
///                 "UXXXXXXX1",
///                 "UXXXXXXX2"
///         ],
///         "event_id": "Ev08MFMKH6",
///         "event_time": 1234567890
/// }
/// ```
#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(tag = "type", rename = "event_callback")]
pub struct Event {
    pub token: String,
    pub team_id: String,
    pub api_app_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authed_users: Option<Vec<String>>,
    pub event_id: String,
    pub event_time: u32,
    pub event: EventType,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[skip_serializing_none]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum EventType {
    /// The list of accounts a user is signed into has changed
    ///
    /// <https://api.slack.com/events/accounts_changed>
    AccountsChanged,
    /// User clicked into your App Home
    #[serde(rename = "app_home_opened")]
    AppHomeOpened {
        user: String,
        channel: String,
        event_ts: String,
        tab: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        view: Option<View>,
    },
    /// Subscribe to only the message events that mention your app or bot
    /// 
    /// <https://api.slack.com/events/app_mention>
    AppMention {
        channel: String,
        event_ts: String,
        text: String,
        thread_ts: Option<String>,
        ts: String,
        user: String,
    },
    /// Indicates your app's event subscriptions are being rate limited
    ///
    /// <https://api.slack.com/events/app_rate_limited>
    AppRateLimited {
        token: String,
        team_id: String,
        minute_rate_limited: i32,
        api_app_id: String,
    },
    /// User requested an app
    ///
    /// <https://api.slack.com/events/app_requested>
    AppRequested {
      app_request: AppRequest
    },
    /// Your Slack app was uninstalled.
    AppUninstalled,
    /// A channel was archived
    #[serde(rename = "channel_archive")]
    ChannelArchive { channel: String, user: String },
    /// A channel was created
    #[serde(rename = "channel_created")]
    ChannelCreated { channel: Channel },
    /// A channel was deleted
    #[serde(rename = "channel_deleted")]
    ChannelDeleted { channel: String },
    /// Bulk updates were made to a channel's history
    #[serde(rename = "channel_history_changed")]
    ChannelHistoryChanged {
        latest: String,
        ts: String,
        event_ts: String,
    },
    /// A channel ID changed
    #[serde(rename = "channel_id_changed")]
    ChannelIDChanged {
        old_channel_id: String,
        new_channel_id: String,
        event_ts: String,
    },
    /// You left a channel
    #[serde(rename = "channel_left")]
    ChannelLeft { channel: String },
    /// A channel was renamed
    #[serde(rename = "channel_rename")]
    ChannelRename { channel: Channel },
    /// A channel has been shared with an external workspace
    #[serde(rename = "channel_shared")]
    ChannelShared {
        connected_team_id: String,
        channel: String,
        event_ts: String,
    },
    /// A channel was unarchived
    #[serde(rename = "channel_unarchive")]
    ChannelUnarchive { channel: String, user: String },
    ///A channel has been unshared with an external workspace
    #[serde(rename = "channel_unshared")]
    ChannelUnshared {
        previously_connected_team_id: String,
        channel: String,
        is_ext_shared: bool,
        event_ts: String,
    },
    /// A custom emoji has been added or changed
    #[serde(rename = "emoji_changed")]
    EmojiChanged {
        subtype: String,
        names: Vec<String>,
        event_ts: String,
    },
    /// An enterprise grid migration has finished on this workspace.
    GridMigrationFinished,
    /// An enterprise grid migration has started on this workspace.
    GridMigrationStarted,
    /// A private channel was archived
    #[serde(rename = "group_archive")]
    GroupArchive { channel: String },
    /// You closed a private channel
    #[serde(rename = "group_close")]
    GroupClose { user: String, channel: String },
    /// A private channel was deleted
    #[serde(rename = "group_deleted")]
    GroupDeleted { channel: String },
    /// A private channel was deleted
    #[serde(rename = "group_history_changed")]
    GroupHistoryChanged {
        latest: String,
        ts: String,
        event_ts: String,
    },
    /// You left a private channel
    #[serde(rename = "group_left")]
    GroupLeft { channel: String },
    /// You created a group DM
    #[serde(rename = "group_open")]
    GroupOpen { user: String, channel: String },
    /// A private channel was renamed
    #[serde(rename = "group_rename")]
    GroupRename { channel: Channel },
    /// A private channel was unarchived
    #[serde(rename = "group_unarchive")]
    GroupUnarchive { channel: String },
    /// You closed a DM
    #[serde(rename = "im_close")]
    ImClose { user: String, channel: String },
    /// A DM was created
    #[serde(rename = "im_created")]
    ImCreated { user: String, channel: Channel },
    /// Bulk updates were made to a DM's history
    #[serde(rename = "im_history_changed")]
    ImHistoryChanged {
        latest: String,
        ts: String,
        event_ts: String,
    },
    /// You opened a DM
    #[serde(rename = "im_open")]
    ImOpen { user: String, channel: String },
    /// User requested an invite
    #[serde(rename = "invite_requested")]
    InviteRequested {
        id: String,
        email: String,
        date_created: i32,
        requester_ids: Vec<String>,
        channel_ids: Vec<String>,
        invite_type: String,
        real_name: String,
        date_expire: i32,
        request_reason: String,
        team: Team,
    },
    // TODO: To be implemented in the future
    /// A message was posted containing one or more links relevant to your application
    #[serde(rename = "link_shared")]
    LinkShared,
    /// A user joined a public or private channel
    #[serde(rename = "member_joined_channel")]
    MemberJoinedChannel {
        user: String,
        channel: String,
        channel_type: String,
        team: String,
        inviter: String,
    },
    /// A user left a public or private channel
    #[serde(rename = "member_left_channel")]
    MemberLeftChannel {
        user: String,
        channel: String,
        channel_type: String,
        team: String,
    },
    /// A message was sent to a channel
    #[serde(rename = "message")]
    Message(MessageBasic),
    #[serde(rename = "message")]
    MessageSubtype(MessageSubtype),
    #[serde(other)]
    Other,
}

#[cfg(test)]
mod test {
    use assert_json_diff::*;
    use crate::event_api::event::{Event, EventType};

    #[test]
    fn deserialize_app_home_opened_event() {
        let json = r##"{
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
}"##;
        let event = serde_json::from_str::<Event>(json).unwrap();
        match event.event {
            EventType::AppHomeOpened{..} => assert!(true, "true"),
            _ => panic!("Event callback deserialize into incorrect variant"),
        }
    }

    #[test]
    fn it_serializes_event_from_a_struct() {
      let json = r##"{
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
          "tab": "home"
        }
      }"##;

      let struct_thing = Event {
        token: "bHKJ2n9AW6Ju3MjciOHfbA1b".to_string(),
        team_id: "T1234567890".to_string(),
        api_app_id: "A0000000000".to_string(),
        event_id: "Ev0000000000".to_string(),
        event_time: 1600000000,
        authed_users: None,
        event: EventType::AppHomeOpened {
          user: "U061F7AUR".to_string(),
          channel: "D0LAN2Q65".to_string(),
          event_ts: "1515449522000016".to_string(),
          tab: "home".to_string(),
          view: None,
        }
      };

      let serialized_json = serde_json::to_string(&struct_thing).unwrap();
      let deserialized = serde_json::from_str::<Event>(&serialized_json).unwrap();
      let expected = serde_json::from_str::<Event>(&json).unwrap();
      assert_json_eq!(deserialized, expected);
    }

    #[test]
    fn deserializes_accounts_changed() {
        let json = r##"
        {
          "type": "accounts_changed"
        }"##;
        let event = serde_json::from_str::<EventType>(json).unwrap();
        match event {
            EventType::AccountsChanged{..} => assert!(true),
            _ => panic!("Did not deserialize into expected variant"),
        }
    }

    #[test]
    fn deserializes_app_rate_limited() {
        let json = r##"
        {
          "token": "Jhj5dZrVaK7ZwHHjRyZWjbDl",
          "type": "app_rate_limited",
          "team_id": "T123456",
          "minute_rate_limited": 1518467820,
          "api_app_id": "A123456"
        }"##;
        let event = serde_json::from_str::<EventType>(json).unwrap();
        match event {
            EventType::AppRateLimited{..} => assert!(true),
            _ => panic!("Did not deserialize into expected variant"),
        }
    }

    #[test]
    fn deserializes_app_mention() {
        let json = r##"
        {
            "type": "app_mention",
            "user": "U061F7AUR",
            "text": "<@U0LAN0Z89> is it everything a river should be?",
            "ts": "1515449522.000016",
            "channel": "C0LAN2Q65",
            "event_ts": "1515449522000016"
        }"##;
        let event = serde_json::from_str::<EventType>(json).unwrap();
        match event {
            EventType::AppMention{..} => assert!(true),
            _ => panic!("Did not deserialize into expected variant"),
        }
    }

    #[test]
    fn deserialize_unknown_event() {
        let json = r##"{
  "token": "bHKJ2n9AW6Ju3MjciOHfbA1b",
  "team_id": "T1234567890",
  "api_app_id": "A0000000000",
  "event_id": "Ev0000000000",
  "event_time": 1600000000,
  "type": "event_callback",
  "event": {
    "type": "other"
  }
}"##;

        let event = serde_json::from_str::<Event>(json).unwrap();
        match event.event {
          EventType::Other => assert!(true, "true"),
          _ => panic!("Event callback deserialize into incorrect variant"),
        }
    }
}
