//! [Event API Types](https://api.slack.com/events?filter=Events)

use crate::channels::channel::Channel;
use crate::event_api::app::AppRequest;
use crate::event_api::messages::{MessageBasic, MessageSubtype, MessageMetadata};
use crate::files::file::{File};
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
    ///
    /// <https://api.slack.com/events/app_uninstalled>
    AppUninstalled,
    /// A call was rejected
    ///
    /// <https://api.slack.com/events/call_rejected>
    CallRejected {
      call_id: String,
      channel_id: String,
      external_unique_id: String,
      user_id: String,
    },
    /// A channel was archived
    ///
    /// <https://api.slack.com/events/channel_archive>
    #[serde(rename = "channel_archive")]
    ChannelArchive {
      channel: String,
      user: String
    },
    /// A channel was created
    ///
    /// <https://api.slack.com/events/channel_created>
    #[serde(rename = "channel_created")]
    ChannelCreated {
      channel: Channel
    },
    /// A channel was deleted
    ///
    /// <https://api.slack.com/events/channel_deleted>
    #[serde(rename = "channel_deleted")]
    ChannelDeleted {
        channel: String
    },
    /// Bulk updates were made to a channel's history
    ///
    /// <https://api.slack.com/events/channel_history_changed>
    #[serde(rename = "channel_history_changed")]
    ChannelHistoryChanged {
        latest: String,
        ts: String,
        event_ts: String,
    },
    /// A channel ID changed
    ///
    /// <https://api.slack.com/events/channel_id_changed>
    #[serde(rename = "channel_id_changed")]
    ChannelIDChanged {
        old_channel_id: String,
        new_channel_id: String,
        event_ts: String,
    },
    /// You left a channel
    ///
    /// <https://api.slack.com/events/channel_left>
    #[serde(rename = "channel_left")]
    ChannelLeft {
        channel: String
    },
    /// A channel was renamed
    ///
    /// <https://api.slack.com/events/channel_rename>
    #[serde(rename = "channel_rename")]
    ChannelRename {
        channel: Channel
    },
    /// A channel has been shared with an external workspace
    ///
    /// <https://api.slack.com/events/channel_shared>
    #[serde(rename = "channel_shared")]
    ChannelShared {
        connected_team_id: String,
        channel: String,
        event_ts: String,
    },
    /// A channel was unarchived
    ///
    /// <https://api.slack.com/events/channel_unarchive>
    #[serde(rename = "channel_unarchive")]
    ChannelUnarchive {
        channel: String,
        user: String
    },
    ///A channel has been unshared with an external workspace
    ///
    /// <https://api.slack.com/events/channel_unshared>
    #[serde(rename = "channel_unshared")]
    ChannelUnshared {
        previously_connected_team_id: String,
        channel: String,
        is_ext_shared: bool,
        event_ts: String,
    },
    /// Do not Disturb settings changed for the current user
    ///
    /// <https://api.slack.com/events/dnd_updated>
    #[serde(rename = "dnd_updated")]
    DoNotDisturbUpdated {
        user: String,
        dnd_status: DoNotDisturbStatus
    },
    /// Do not Disturb settings changed for a member
    ///
    /// <https://api.slack.com/events/dnd_updated_user>
    #[serde(rename = "dnd_updated_user")]
    DoNotDisturbUpdatedUser {
        user: String,
        dnd_status: DoNotDisturbStatus
    },
    /// The workspace email domain has changed
    ///
    /// <https://api.slack.com/events/email_domain_changed>
    EmailDomainChanged {
        email_domain: String,
        event_ts: String,
    },
    /// A custom emoji has been added or changed
    ///
    /// <https://api.slack.com/events/emoji_changed>
    #[serde(rename = "emoji_changed")]
    EmojiChanged(EmojiSubtype),
    /// A file was changed
    ///
    /// <https://api.slack.com/events/file_change>
    FileChange {
        file_id: String,
        file: File,
    },
    /// A file was created
    ///
    /// <https://api.slack.com/events/file_created>
    FileCreated {
        file_id: String,
        file: File,
    },
    /// A file was deleted
    ///
    /// <https://api.slack.com/events/file_deleted>
    FileDeleted {
        file_id: String,
        event_ts: String,
    },
    /// A file was made public
    ///
    /// <https://api.slack.com/events/file_public>
    FilePublic {
        file_id: String,
        file: File,
    },
    /// A file was shared
    ///
    /// <https://api.slack.com/events/file_shared>
    FileShared {
        channel_id: String,
        event_ts: String,
        file_id: String,
        file: File,
        user_id: String,
    },
    /// A file was unshared
    ///
    /// <https://api.slack.com/events/file_unshared>
    FileUnshared {
        file_id: String,
        file: File,
    },
    /// An enterprise grid migration has finished on this workspace.
    ///
    /// <https://api.slack.com/events/grid_migration_finished>
    GridMigrationFinished {
        enterprise_id: String,
    },
    /// An enterprise grid migration has started on this workspace.
    ///
    /// <https://api.slack.com/events/grid_migration_started>
    GridMigrationStarted {
        enterprise_id: String,
    },
    /// A private channel was archived
    ///
    /// <https://api.slack.com/events/group_archive>
    GroupArchive {
       channel: String
    },
    /// You closed a private channel
    ///
    /// <https://api.slack.com/events/group_close>
    GroupClose {
        user: String,
        channel: String
    },
    /// A private channel was deleted
    ///
    /// <https://api.slack.com/events/group_deleted>
    GroupDeleted {
       channel: String
    },
    /// A private channel was deleted
    ///
    /// <https://api.slack.com/events/group_history_changed>
    GroupHistoryChanged {
        latest: String,
        ts: String,
        event_ts: String,
    },
    /// You left a private channel
    ///
    /// <https://api.slack.com/events/group_left>
    GroupLeft {
        channel: String
    },
    /// You created a group DM
    ///
    /// <https://api.slack.com/events/group_open>
    GroupOpen {
        user: String,
        channel: String
    },
    /// A private channel was renamed
    ///
    /// <https://api.slack.com/events/group_rename>
    GroupRename {
        channel: Channel
    },
    /// A private channel was unarchived
    ///
    /// <https://api.slack.com/events/group_unarchive>
    GroupUnarchive {
        channel: String
    },
    /// You closed a DM
    ///
    /// <https://api.slack.com/events/im_close>
    ImClose {
        user: String,
        channel: String
    },
    /// A DM was created
    ///
    /// <https://api.slack.com/events/im_created>
    ImCreated {
        user: String,
        channel: Channel
    },
    /// Bulk updates were made to a DM's history
    ///
    /// <https://api.slack.com/events/im_history_changed>
    ImHistoryChanged {
        latest: String,
        ts: String,
        event_ts: String,
    },
    /// You opened a DM
    ///
    /// <https://api.slack.com/events/im_open>
    ImOpen {
        user: String,
        channel: String
    },
    /// User requested an invite
    ///
    /// <https://api.slack.com/events/invite_requested>
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
    /// A message was posted containing one or more links relevant to your application
    ///
    /// <https://api.slack.com/events/link_shared>
    LinkShared {
        channel: String,
        is_bot_user_member: bool,
        user: String,
        message_ts: String,
        unfurl_id: String,
        thread_ts: String,
        source: LinkSource,
        links: Vec<Link>,
    },
    /// A user joined a public or private channel
    ///
    /// <https://api.slack.com/events/member_joined_channel>
    MemberJoinedChannel {
        user: String,
        channel: String,
        channel_type: ChannelType,
        team: String,
        inviter: String,
    },
    /// A user left a public or private channel
    ///
    /// <https://api.slack.com/events/member_left_channel>
    MemberLeftChannel {
        user: String,
        channel: String,
        channel_type: ChannelType,
        team: String,
    },
    /// A message was sent to a channel
    #[serde(rename = "message")]
    Message(MessageBasic),
    #[serde(rename = "message")]
    MessageSubtype(MessageSubtype),

    /// Message metadata was posted
    ///
    /// <https://api.slack.com/events/message_metadata_posted>
    MessageMetadataPosted {
        app_id: String,
        bot_id: String,
        channel_id: String,
        event_ts: String,
        message_ts: String,
        metadata: MessageMetadata,
        team_id: String,
        user_id: String,
    },
    #[serde(other)]
    Other,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct DoNotDisturbStatus {
    pub dnd_enabled: bool,
    pub next_dnd_start_ts: u32,
    pub next_dnd_end_ts: u32,
    pub snooze_enabled: Option<bool>,
    pub snooze_endtime: Option<u32>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(tag = "subtype", rename_all = "snake_case")]
pub enum EmojiSubtype {
    Add {
        name: String,
        value: String,
        event_ts: String,
    },
    Remove {
        names: Vec<String>,
        event_ts: String,
    },
    Rename {
        old_name: String,
        new_name: String,
        value: String,
        event_ts: String,
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
// TODO implement a better type for URL (besides a string)
pub struct Link {
    pub domain: String,
    pub url: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum LinkSource {
    Composer,
    ConversationsHistory,
}

/// See <https://api.slack.com/events/member_joined_channel> for more
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub enum ChannelType {
    #[serde(rename = "C")]
    Public,
    #[serde(rename = "G")]
    Private
}

#[cfg(test)]
mod test {
    use assert_json_diff::*;
    use crate::event_api::event::{Event, EventType, EmojiSubtype};

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
          "token": "12345FVmRUzNDOAu12345h",
          "team_id": "TL1BBBQBD",
          "api_app_id": "BBBU04BB4",
          "event": {
              "type": "accounts_changed"
          },
          "type": "event_callback",
          "event_id": "EvLLACMB6BB",
          "event_time": 1563448153,
          "authed_users": ["UBBB1TYR5"]
        }
        "##;
        let event = serde_json::from_str::<Event>(json).unwrap();
        match event.event {
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
    fn deserializes_call_rejected() {
        let json = r##"
        {
          "token": "12345FVmRUzNDOAu12345h",
          "team_id": "TL1BBBQBD",
          "api_app_id": "BBBU04BB4",
          "event": {
            "type": "call_rejected",
            "call_id": "RL731AVEF",
            "user_id": "ULJS1TYR5",
            "channel_id": "DL5JN9K0T",
            "external_unique_id": "123-456-7890"
          },
          "type": "event_callback",
          "event_id": "EvLLACMB6BB",
          "event_time": 1563448153,
          "authed_users": ["UBBB1TYR5"]
        }
        "##;
        let event = serde_json::from_str::<Event>(json).unwrap();
        println!("{:?}", event.event);
        match event.event {
            EventType::CallRejected{..} => assert!(true),
            _ => panic!("Did not deserialize into expected variant CallRejected"),
        }
    }

    #[test]
    fn deserializes_app_uninstalled() {
        let json = r##"
          {
            "token": "XXYYZZ",
            "team_id": "TXXXXXXXX",
            "api_app_id": "AXXXXXXXXX",
            "event": {
              "type": "app_uninstalled"
            },
            "type": "event_callback",
            "event_id": "EvXXXXXXXX",
            "event_time": 1234567890
          }
        "##;
        let event = serde_json::from_str::<Event>(json).unwrap();
        match event.event {
            EventType::AppUninstalled{..} => assert!(true),
            _ => panic!("Did not deserialize into expected variant"),
        }
    }

    #[test]
    fn deserializes_channel_archive() {
        let json = r##"
          {
            "token": "XXYYZZ",
            "team_id": "TXXXXXXXX",
            "api_app_id": "AXXXXXXXXX",
            "event": {
              "type": "channel_archive",
              "channel": "C024BE91L",
              "user": "U024BE7LH"
            },
            "type": "event_callback",
            "event_id": "EvXXXXXXXX",
            "event_time": 1234567890
          }
        "##;
        let event = serde_json::from_str::<Event>(json).unwrap();
        match event.event {
            EventType::ChannelArchive{..} => assert!(true),
            _ => panic!("Did not deserialize into expected variant"),
        }
    }

    #[test]
    fn deserializes_channel_created() {
        let json = r##"
          {
            "token": "XXYYZZ",
            "team_id": "TXXXXXXXX",
            "api_app_id": "AXXXXXXXXX",
            "event": {
              "type": "channel_created",
              "channel": {
                "id": "C024BE91L",
                "name": "fun",
                "created": 1360782804,
                "creator": "U024BE7LH"
              }
            },
            "type": "event_callback",
            "event_id": "EvXXXXXXXX",
            "event_time": 1234567890
          }
        "##;
        let event = serde_json::from_str::<Event>(json).unwrap();
        match event.event {
            EventType::ChannelCreated{..} => assert!(true),
            _ => panic!("Did not deserialize into expected variant"),
        }
    }

    #[test]
    fn deserializes_channel_deleted() {
        let json = r##"
          {
            "token": "XXYYZZ",
            "team_id": "TXXXXXXXX",
            "api_app_id": "AXXXXXXXXX",
            "event": {
              "type": "channel_deleted",
              "channel": "C024BE91L"
            },
            "type": "event_callback",
            "event_id": "EvXXXXXXXX",
            "event_time": 1234567890
          }
        "##;
        let event = serde_json::from_str::<Event>(json).unwrap();
        match event.event {
            EventType::ChannelDeleted{..} => assert!(true),
            _ => panic!("Did not deserialize into expected variant ChannelDeleted"),
        }
    }

    #[test]
    fn deserializes_do_not_disturb_updated() {
        let json = r##"
          {
            "token": "XXYYZZ",
            "team_id": "TXXXXXXXX",
            "api_app_id": "AXXXXXXXXX",
            "event": {
                "type": "dnd_updated",
                "user": "U1234",
                "dnd_status": {
                    "dnd_enabled": true,
                    "next_dnd_start_ts": 1450387800,
                    "next_dnd_end_ts": 1450423800,
                    "snooze_enabled": true,
                    "snooze_endtime": 1450373897
                }
            },
            "type": "event_callback",
            "event_id": "EvXXXXXXXX",
            "event_time": 1234567890
          }
        "##;
        let event = serde_json::from_str::<Event>(json).unwrap();
        match event.event {
            EventType::DoNotDisturbUpdated{..} => assert!(true),
            _ => panic!("Did not deserialize into expected variant DoNotDisturbUpdated"),
        }
    }

    #[test]
    fn deserializes_do_not_disturb_updated_user() {
        let json = r##"
          {
            "token": "XXYYZZ",
            "team_id": "TXXXXXXXX",
            "api_app_id": "AXXXXXXXXX",
            "event": {
                "type": "dnd_updated_user",
                "user": "U1234",
                "dnd_status": {
                    "dnd_enabled": true,
                    "next_dnd_start_ts": 1450387800,
                    "next_dnd_end_ts": 1450423800
                }
            },
            "type": "event_callback",
            "event_id": "EvXXXXXXXX",
            "event_time": 1234567890
          }
        "##;
        let event = serde_json::from_str::<Event>(json).unwrap();
        match event.event {
            EventType::DoNotDisturbUpdatedUser{..} => assert!(true),
            _ => panic!("Did not deserialize into expected variant DoNotDisturbUpdatedUser"),
        }
    }

    #[test]
    fn deserializes_emoji_changed() {
        let json = r##"
          {
            "token": "XXYYZZ",
            "team_id": "TXXXXXXXX",
            "api_app_id": "AXXXXXXXXX",
            "event": {
                "type": "emoji_changed",
                "subtype": "add",
                "name": "picard_facepalm",
                "value": "https://my.slack.com/emoji/picard_facepalm/db8e287430eaa459.gif",
                "event_ts" : "1361482916.000004"
            },
            "type": "event_callback",
            "event_id": "EvXXXXXXXX",
            "event_time": 1234567890
          }
        "##;
        let event = serde_json::from_str::<Event>(json).unwrap();
        match event.event {
            EventType::EmojiChanged(subtype) => {
                match subtype {
                    EmojiSubtype::Add{..} => assert!(true),
                    _ => panic!("Did not deserialize into expected variant EmojiSubtype::Add")
                }
            },
            _ => panic!("Did not deserialize into expected variant EmojiChanged"),
        }
    }

    #[test]
    fn deserializes_emoji_changed_removed() {
        let json = r##"
          {
            "token": "XXYYZZ",
            "team_id": "TXXXXXXXX",
            "api_app_id": "AXXXXXXXXX",
            "event": {
                "type": "emoji_changed",
                "subtype": "remove",
                "names": ["picard_facepalm"],
                "event_ts" : "1361482916.000004"
            },
            "type": "event_callback",
            "event_id": "EvXXXXXXXX",
            "event_time": 1234567890
          }
        "##;
        let event = serde_json::from_str::<Event>(json).unwrap();
        match event.event {
            EventType::EmojiChanged(subtype) => {
                match subtype {
                    EmojiSubtype::Remove{..} => assert!(true),
                    _ => panic!("Did not deserialize into expected variant EmojiSubtype::Remove")
                }
            },
            _ => panic!("Did not deserialize into expected variant EmojiChanged"),
        }
    }

    #[test]
    fn deserializes_file_change() {
        let json = r##"
          {
            "token": "XXYYZZ",
            "team_id": "TXXXXXXXX",
            "api_app_id": "AXXXXXXXXX",
            "event": {
                "type": "file_change",
                "file_id": "F2147483862",
                "file": {
                    "id": "F2147483862"
                }
            },
            "type": "event_callback",
            "event_id": "EvXXXXXXXX",
            "event_time": 1234567890
          }
        "##;
        let event = serde_json::from_str::<Event>(json).unwrap();
        match event.event {
            EventType::FileChange{..} => assert!(true),
            _ => panic!("Did not deserialize into expected variant EmojiSubtype::Remove")
        }
    }

    #[test]
    fn deserializes_link_shared() {
        let json = r##"
        {
            "token": "XXYYZZ",
            "team_id": "TXXXXXXXX",
            "api_app_id": "AXXXXXXXXX",
            "event": {
                "type": "link_shared",
                "channel": "Cxxxxxx",
                "is_bot_user_member": true,
                "user": "Uxxxxxxx",
                "message_ts": "123456789.9875",
                "unfurl_id": "C123456.123456789.987501.1b90fa1278528ce6e2f6c5c2bfa1abc9a41d57d02b29d173f40399c9ffdecf4b",
                "thread_ts": "123456621.1855",
                "source": "conversations_history",
                "links": [
                    {
                        "domain": "example.com",
                        "url": "https://example.com/12345"
                    },
                    {
                        "domain": "example.com",
                        "url": "https://example.com/67890"
                    },
                    {
                        "domain": "another-example.com",
                        "url": "https://yet.another-example.com/v/abcde"
                    }
                ]
            },
            "type": "event_callback",
            "authed_users": [
                "UXXXXXXX1",
                "UXXXXXXX2"
            ],
            "event_id": "Ev08MFMKH6",
            "event_time": 123456789
        }
        "##;
        let event = serde_json::from_str::<Event>(json).unwrap();
        match event.event {
            EventType::LinkShared{..} => assert!(true),
            _ => panic!("Did not deserialize into expected variant EventType::LinkShared")
        }
    }

    #[test]
    fn deserializes_member_joined_channel() {
        let json = r##"
          {
            "token": "XXYYZZ",
            "team_id": "TXXXXXXXX",
            "api_app_id": "AXXXXXXXXX",
            "event": {
                "type": "member_joined_channel",
                "user": "W06GH7XHN",
                "channel": "C0698JE0H",
                "channel_type": "C",
                "team": "T024BE7LD",
                "inviter": "U123456789"
            },
            "type": "event_callback",
            "event_id": "EvXXXXXXXX",
            "event_time": 1234567890
          }
        "##;
        let event = serde_json::from_str::<Event>(json).unwrap();
        match event.event {
            EventType::MemberJoinedChannel{..} => assert!(true),
            _ => panic!("Did not deserialize into expected variant MemberJoinedChannel")
        }
    }

    #[test]
    fn deserializes_message_metadata_posted() {
        let json = r##"
        {
            "token": "XXYYZZ",
            "team_id": "TXXXXXXXX",
            "api_app_id": "AXXXXXXXXX",
            "event": {
                "type": "message_metadata_posted",
                "app_id": "AQF4F123M",
                "bot_id": "B8241P2B34D",
                "user_id": "UA8829BFL",
                "team_id": "T12F3JCAP",
                "channel_id": "CJN879K8A",
                "metadata":
                {
                    "event_type": "task_created",
                    "event_payload":
                    {
                        "id": "TK-2132",
                        "summary": "New issue with the display of mobile element",
                        "description": "An end user has found a problem with the new mobile container for data entry. It was reproduced in the current version of IOS.",
                        "priority": "HIGH",
                        "resource_type": "TASK"
                    }
                },
                "message_ts": "1658903885.673769",
                "event_ts": "1658903885.673769"
            },
            "type": "event_callback",
            "event_id": "EvXXXXXXXX",
            "event_time": 1234567890
        }
        "##;
        let event = serde_json::from_str::<Event>(json).unwrap();
        match event.event {
            EventType::MessageMetadataPosted{..} => assert!(true),
            _ => panic!("Did not deserialize into expected variant MessageMetadataPosted")
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
