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
