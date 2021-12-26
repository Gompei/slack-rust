use crate::team::teams::Team;
use crate::users::user::User;
use crate::views::view::View;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct ChannelArchiveEvent {
    pub channel: Option<String>,
    pub user: Option<String>,
}
