//! [Event API Types](https://api.slack.com/events?filter=Events)

use crate::channels::channel::Channel;
use crate::event_api::app::AppRequest;
use crate::team::teams::Team;
use crate::views::view::View;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// [Event API](https://api.slack.com/events?filter=Events)
#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(tag = "type")]
pub struct Event {
    pub token: String,
    pub team_id: String,
    pub api_app_id: String,
    pub event: EventCallback,
    pub event_id: String,
    pub event_time: i32,
    #[serde(rename = "type")]
    pub _type: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[skip_serializing_none]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum EventCallback {
    /// User clicked into your App Home
    #[serde(rename = "app_home_opened")]
    AppHomeOpened {
        user: String,
        channel: String,
        event_ts: String,
        tab: String,
        view: View,
    },
    /// Subscribe to only the message events that mention your app or bot
    AppMention {
        channel: String,
        event_ts: String,
        text: String,
        thread_ts: Option<String>,
        ts: String,
        user: String,
    },
    /// Indicates your app's event subscriptions are being rate limited
    #[serde(rename = "app_rate_limited")]
    AppRateLimited {
        token: String,
        team_id: String,
        minute_rate_limited: i32,
        api_app_id: String,
    },
    /// User requested an app
    #[serde(rename = "app_requested")]
    AppRequested { app_request: AppRequest },
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
    #[serde(rename = "member_joined_channel")]
    MemberLeftChannel {
        user: String,
        channel: String,
        channel_type: String,
        team: String,
    },
    /// A message was sent to a channel
    Message {
        channel_type: String,
        channel: String,
        event_ts: String,
        text: String,
        thread_ts: Option<String>,
        ts: String,
        user: String,
    },
    #[serde(other)]
    Other,
}

#[cfg(test)]
mod test {
    use super::*;

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
            EventCallback::AppHomeOpened {
                user,
                channel,
                event_ts,
                tab,
                view,
            } => {
                assert_eq!(user, "U061F7AUR");
                assert_eq!(channel, "D0LAN2Q65");
                assert_eq!(event_ts, "1515449522000016");
                assert_eq!(tab, "home");
                assert_eq!(view.id.unwrap(), "VPASKP233");
            }
            _ => panic!("Event callback deserialize into incorrect variant"),
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
            EventCallback::Other => assert!(true, "true"),
            _ => panic!("Event callback deserialize into incorrect variant"),
        }
    }
}
