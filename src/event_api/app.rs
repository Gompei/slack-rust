use crate::team::teams::Team;
use crate::users::user::User;
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

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct AppRequestedEvent {
    pub app_request: Option<AppRequest>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct AppRequest {
    pub id: Option<String>,
    pub app: Option<App>,
    pub previous_resolution: Option<PreviousResolution>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct App {
    pub id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub help_url: Option<String>,
    pub privacy_policy_url: Option<String>,
    pub app_homepage_url: Option<String>,
    pub app_directory_url: Option<String>,
    pub is_app_directory_approved: Option<bool>,
    pub is_internal: Option<bool>,
    pub additional_info: Option<String>,
    pub user: Option<User>,
    pub team: Option<Team>,
    pub scopes: Option<Vec<Scope>>,
    pub message: Option<String>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct PreviousResolution {
    pub status: Option<String>,
    pub scopes: Option<Vec<Scope>>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Scope {
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_sensitive: Option<bool>,
    pub token_type: Option<String>,
}
