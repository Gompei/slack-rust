use crate::event_api::app::{
    AppHomeOpenedEvent, AppMentionEvent, AppRateLimitedEvent, AppRequestedEvent,
};
use crate::event_api::channel::{ChannelCreatedEvent, ChannelEvent};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(tag = "type")]
pub enum Event {
    /// AppMention is an Events API subscribable event
    #[serde(rename = "app_mention")]
    AppMention(AppMentionEvent),
    /// AppHomeOpened Your Slack app home was opened
    #[serde(rename = "app_home_opened")]
    AppHomeOpened(AppHomeOpenedEvent),
    /// AppUninstalled Your Slack app was uninstalled.
    AppUninstalled,
    ///
    #[serde(rename = "app_rate_limited")]
    AppRateLimited(AppRateLimitedEvent),
    ///
    #[serde(rename = "app_requested")]
    AppRequested(AppRequestedEvent),
    /// ChannelCreated is sent when a new channel is created.
    #[serde(rename = "channel_created")]
    ChannelCreated(ChannelCreatedEvent),
    /// ChannelDeleted is sent when a channel is deleted.
    #[serde(rename = "channel_deleted")]
    ChannelDeleted(ChannelEvent),
    /// ChannelArchive is sent when a channel is archived.
    #[serde(rename = "channel_archive")]
    ChannelArchive(ChannelEvent),
    /// ChannelUnarchive is sent when a channel is unarchived.
    ChannelUnarchive,
    /// ChannelLeft is sent when a channel is left.
    ChannelLeft,
    /// ChannelRename is sent when a channel is rename.
    ChannelRename,
    /// ChannelIDChanged is sent when a channel identifier is changed.
    ChannelIDChanged,
    /// GroupDeleted is sent when a group is deleted.
    GroupDeleted,
    /// GroupArchive is sent when a group is archived.
    GroupArchive,
    /// GroupUnarchive is sent when a group is unarchived.
    GroupUnarchive,
    /// GroupLeft is sent when a group is left.
    GroupLeft,
    /// GroupRename is sent when a group is renamed.
    GroupRename,
    /// GridMigrationFinished An enterprise grid migration has finished on this workspace.
    GridMigrationFinished,
    /// GridMigrationStarted An enterprise grid migration has started on this workspace.
    GridMigrationStarted,
    /// LinkShared A message was posted containing one or more links relevant to your application
    LinkShared,
    /// Message A message was posted to a channel, private channel (group), im, or mim
    Message,
    /// Member Joined Channel
    MemberJoinedChannel,
    /// Member Left Channel
    MemberLeftChannel,
    /// PinAdded An item was pinned to a channel
    PinAdded,
    /// PinRemoved An item was unpinned from a channel
    PinRemoved,
    /// ReactionAdded An reaction was added to a message
    ReactionAdded,
    /// ReactionRemoved An reaction was removed from a message
    ReactionRemoved,
    /// TeamJoin A new user joined the workspace
    TeamJoin,
    /// TokensRevoked APP's API tokes are revoked
    TokensRevoked,
    /// EmojiChanged A custom emoji has been added or changed
    EmojiChanged,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    AppMention,
    AppHomeOpened,
    AppUninstalled,
    ChannelCreated,
    ChannelDeleted,
    ChannelArchive,
    ChannelUnarchive,
    ChannelLeft,
    ChannelRename,
    ChannelIDChanged,
    GroupDeleted,
    GroupArchive,
    GroupUnarchive,
    GroupLeft,
    GroupRename,
    GridMigrationFinished,
    GridMigrationStarted,
    LinkShared,
    Message,
    MemberJoinedChannel,
    MemberLeftChannel,
    PinAdded,
    PinRemoved,
    ReactionAdded,
    ReactionRemoved,
    TeamJoin,
    TokensRevoked,
    EmojiChanged,
}
