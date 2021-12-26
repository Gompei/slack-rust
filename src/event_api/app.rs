use crate::views::view::View;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct AppMentionEvent {
    pub user: Option<String>,
    pub text: Option<String>,
    pub ts: Option<String>,
    pub channel: Option<String>,
    pub event_ts: Option<String>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct AppHomeOpenedEvent {
    pub user: Option<String>,
    pub channel: Option<String>,
    pub event_ts: Option<String>,
    pub tab: Option<String>,
    pub view: Option<View>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct AppRateLimitedEvent {
    pub token: Option<String>,
    pub team_id: Option<String>,
    pub minute_rate_limited: Option<i32>,
    pub api_app_id: Option<String>,
}
