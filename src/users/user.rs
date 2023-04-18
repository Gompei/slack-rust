use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct UserProfile {
    pub avatar_hash: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub real_name: Option<String>,
    pub real_name_normalized: Option<String>,
    pub display_name: Option<String>,
    pub display_name_normalized: Option<String>,
    pub email: Option<String>,
    pub skype: Option<String>,
    pub phone: Option<String>,
    pub image_24: Option<String>,
    pub image_32: Option<String>,
    pub image_48: Option<String>,
    pub image_72: Option<String>,
    pub image_192: Option<String>,
    pub image_512: Option<String>,
    pub image_original: Option<String>,
    pub title: Option<String>,
    pub bot_id: Option<String>,
    pub api_app_id: Option<String>,
    pub status_text: Option<String>,
    pub status_emoji: Option<String>,
    pub status_expiration: Option<i32>,
    pub team: Option<String>,
    pub always_active: Option<bool>,
    pub status_text_canonical: Option<String>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct User {
    pub id: Option<String>,
    pub team_id: Option<String>,
    pub name: Option<String>,
    pub deleted: Option<bool>,
    pub email: Option<String>,
    pub color: Option<String>,
    pub real_name: Option<String>,
    pub tz: Option<String>,
    pub tz_label: Option<String>,
    pub tz_offset: Option<i32>,
    pub profile: Option<UserProfile>,
    pub is_bot: Option<bool>,
    pub is_admin: Option<bool>,
    pub is_owner: Option<bool>,
    pub is_primary_owner: Option<bool>,
    pub is_restricted: Option<bool>,
    pub is_ultra_restricted: Option<bool>,
    pub is_stranger: Option<bool>,
    pub is_app_user: Option<bool>,
    pub is_invited_user: Option<bool>,
    pub updated: Option<i32>,
    pub is_email_confirmed: Option<bool>,
    pub who_can_share_contact_card: Option<String>,
}
