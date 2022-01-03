//! [Event API Types](https://api.slack.com/events?filter=Events)

use crate::channels::channel::Channel;
use crate::event_api::app::AppRequest;
use crate::team::teams::Team;
use crate::views::view::View;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// [Event API](https://api.slack.com/events?filter=Events)
#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(tag = "type")]
pub enum Event {
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
    #[serde(rename = "app_mention")]
    AppMention {
        user: String,
        text: String,
        ts: String,
        channel: String,
        event_ts: String,
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
    #[serde(rename = "event_callback")]
    AppUninstalled(CallbackEvent),
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
    #[serde(rename = "event_callback")]
    GridMigrationFinished(CallbackEvent),
    /// An enterprise grid migration has started on this workspace.
    #[serde(rename = "event_callback")]
    GridMigrationStarted(CallbackEvent),
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
    #[serde(rename = "member_joined_channel")]
    Message {
        channel: String,
        user: String,
        channel_type: String,
        text: String,
        ts: String,
    },
    #[serde(skip)]
    None,
}

impl Event {
    pub fn block_type(&self) -> EventType {
        match self {
            Event::AppHomeOpened { .. } => EventType::AppHomeOpened,
            Event::AppMention { .. } => EventType::AppMention,
            Event::AppRateLimited { .. } => EventType::AppRateLimited,
            Event::AppRequested { .. } => EventType::AppRequested,
            Event::AppUninstalled(_) => EventType::AppUninstalled,
            Event::ChannelArchive { .. } => EventType::ChannelArchive,
            Event::ChannelCreated { .. } => EventType::ChannelCreated,
            Event::ChannelDeleted { .. } => EventType::ChannelDeleted,
            Event::ChannelHistoryChanged { .. } => EventType::ChannelHistoryChanged,
            Event::ChannelIDChanged { .. } => EventType::ChannelIDChanged,
            Event::ChannelLeft { .. } => EventType::ChannelLeft,
            Event::ChannelRename { .. } => EventType::ChannelRename,
            Event::ChannelShared { .. } => EventType::ChannelShared,
            Event::ChannelUnarchive { .. } => EventType::ChannelUnarchive,
            Event::ChannelUnshared { .. } => EventType::ChannelUnshared,
            Event::EmojiChanged { .. } => EventType::EmojiChanged,
            Event::GridMigrationFinished(_) => EventType::GridMigrationFinished,
            Event::GridMigrationStarted(_) => EventType::GridMigrationStarted,
            Event::GroupArchive { .. } => EventType::GroupArchive,
            Event::GroupClose { .. } => EventType::GroupClose,
            Event::GroupDeleted { .. } => EventType::GroupDeleted,
            Event::GroupHistoryChanged { .. } => EventType::GroupHistoryChanged,
            Event::GroupLeft { .. } => EventType::GroupLeft,
            Event::GroupOpen { .. } => EventType::GroupOpen,
            Event::GroupRename { .. } => EventType::GroupRename,
            Event::GroupUnarchive { .. } => EventType::GroupUnarchive,
            Event::ImClose { .. } => EventType::ImClose,
            Event::ImCreated { .. } => EventType::ImCreated,
            Event::ImHistoryChanged { .. } => EventType::ImHistoryChanged,
            Event::ImOpen { .. } => EventType::ImOpen,
            Event::InviteRequested { .. } => EventType::InviteRequested,
            Event::LinkShared => EventType::LinkShared,
            Event::MemberJoinedChannel { .. } => EventType::MemberJoinedChannel,
            Event::MemberLeftChannel { .. } => EventType::MemberLeftChannel,
            Event::Message { .. } => EventType::Message,
            Event::None => EventType::None,
        }
    }
}

/// [Event API Type](https://api.slack.com/events?filter=Events)
#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    AppHomeOpened,
    AppMention,
    AppRateLimited,
    AppRequested,
    AppUninstalled,
    ChannelArchive,
    ChannelCreated,
    ChannelDeleted,
    ChannelHistoryChanged,
    ChannelIDChanged,
    ChannelLeft,
    ChannelRename,
    ChannelShared,
    ChannelUnarchive,
    ChannelUnshared,
    EmojiChanged,
    GridMigrationFinished,
    GridMigrationStarted,
    GroupArchive,
    GroupClose,
    GroupDeleted,
    GroupHistoryChanged,
    GroupLeft,
    GroupOpen,
    GroupRename,
    GroupUnarchive,
    ImClose,
    ImCreated,
    ImHistoryChanged,
    ImOpen,
    InviteRequested,
    LinkShared,
    MemberJoinedChannel,
    MemberLeftChannel,
    Message,
    #[serde(skip)]
    None,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct CallbackEvent {
    token: String,
    team_id: String,
    api_app_id: String,
    #[serde(rename = "event")]
    callback_event: CallbackEventInner,
    event_id: String,
    event_time: i32,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct CallbackEventInner {
    #[serde(rename = "type")]
    pub type_filed: CallbackEventType,
    pub enterprise_id: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CallbackEventType {
    AppUninstalled,
    GridMigrationFinished,
    GridMigrationStarted,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn deserialize_app_home_opened_event() {
        let json = r##"{
  "type": "app_home_opened",
  "user": "U061F7AUR",
  "channel": "D0LAN2Q65",
  "event_ts": "1515449522000016",
  "tab": "home",
  "view": {
    "id": "VPASKP233"
  }
}"##;
        let event = serde_json::from_str::<Event>(json).unwrap();
        match event {
            Event::AppHomeOpened {
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
            _ => panic!("Event deserialize into incorrect variant"),
        }
    }
}
