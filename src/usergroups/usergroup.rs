use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct Usergroup {
    pub id: Option<String>,
    pub team_id: Option<String>,
    pub is_usergroup: Option<bool>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub handle: Option<String>,
    pub is_external: Option<bool>,
    pub date_create: Option<i32>,
    pub date_update: Option<i32>,
    pub date_delete: Option<i32>,
    pub auto_type: Option<String>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
    pub deleted_by: Option<String>,
    pub prefs: Option<Pref>,
    pub user_count: Option<String>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct Pref {
    pub channels: Option<Vec<String>>,
    pub groups: Option<Vec<String>>,
}
