use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use crate::team::teams::{Team};
use crate::users::user::{User};

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct Invite {
    pub id: String,
    pub date_created: u32,
    pub date_invalid: u32,
    pub inviting_team: Team,
    pub inviting_user: User,
    pub recipient_email: Option<String>,
    pub recipient_user_id: String,
}
