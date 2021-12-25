use crate::block::block_object::TextBlockObject;
use crate::channels::channel::Channel;
use crate::chat::message::Message;
use crate::team::teams::Team;
use crate::users::user::User;
use crate::views::view::View;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct SlashPayload {
    pub token: Option<String>,
    pub team_id: Option<String>,
    pub team_domain: Option<String>,
    pub channel_id: Option<String>,
    pub channel_name: Option<String>,
    pub user_id: Option<String>,
    pub user_name: Option<String>,
    pub command: Option<String>,
    pub text: Option<String>,
    pub response_url: Option<String>,
    pub trigger_id: Option<String>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct InteractivePayload {
    #[serde(rename = "type")]
    pub type_filed: InteractiveEventType,
    pub team: Option<Team>,
    pub user: Option<User>,
    pub api_app_id: Option<String>,
    pub token: Option<String>,
    pub container: Option<Container>,
    pub trigger_id: Option<String>,
    pub channel: Option<Channel>,
    pub message: Option<Message>,
    pub response_url: Option<String>,
    pub actions: Option<Vec<Action>>,
    pub view: Option<View>,
    pub hash: Option<String>,
    pub block_id: Option<String>,
    pub action_id: Option<String>,
    pub value: Option<String>,
    pub is_enterprise_install: Option<bool>,
    pub callback_id: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum InteractiveEventType {
    DialogCancellation,
    DialogSubmission,
    DialogSuggestion,
    InteractionMessage,
    MessageAction,
    BlockActions,
    BlockSuggestion,
    ViewSubmission,
    ViewClosed,
    Shortcut,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Container {
    #[serde(rename = "type")]
    pub type_filed: String,
    pub message_ts: Option<String>,
    pub attachment_id: Option<i32>,
    pub channel_id: Option<String>,
    pub is_ephemeral: Option<bool>,
    pub is_app_unfurl: Option<bool>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Action {
    #[serde(rename = "type")]
    pub type_filed: String,
    pub action_id: Option<String>,
    pub block_id: Option<String>,
    pub text: Option<TextBlockObject>,
    pub value: Option<String>,
    pub action_ts: Option<String>,
}
