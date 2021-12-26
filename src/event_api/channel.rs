use crate::channels::channel::Channel;
use crate::team::teams::Team;
use crate::users::user::User;
use crate::views::view::View;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct ChannelEvent {
    pub channel: Option<String>,
    pub user: Option<String>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct ChannelCreatedEvent {
    pub channel: Option<Channel>,
}
