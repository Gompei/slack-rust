use crate::views::view::View;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct AppMentionEvent {
    pub user: Option<String>,
    pub channel: Option<String>,
    pub event_ts: Option<String>,
    pub tab: Option<String>,
    pub view: Option<View>,
}
