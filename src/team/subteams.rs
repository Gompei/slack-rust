use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{Value};

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Subteam {
    pub auto_type: Value,
    pub auto_provision: Option<bool>,
    pub channel_count: Option<u32>,
    pub created_by: Value,
    pub date_create: u32,
    pub date_delete: u32,
    pub date_update: u32,
    pub deleted_by: Value,
    pub description: String,
    pub enterprise_subteam_id: Option<Value>,
    pub handle: String,
    pub id: String,
    pub is_external: bool,
    pub is_usergroup: bool,
    pub is_subteam: Option<bool>,
    pub name: String,
    pub prefs: SubteamPrefs,
    pub team_id: String,
    pub updated_by: Value,
    pub users: Option<Vec<String>>,
    /// Note that the Slack API examples have this as an integer value encoded
    /// as a string
    #[serde(deserialize_with = "Subteam::make_int")]
    pub user_count: u64,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct SubteamPrefs {
    pub channels: Vec<String>,
    pub groups: Vec<String>,
}

impl Subteam {
    /// Implement a custom deserializer to handle both string quoted integer values and unquoted
    /// integer values
    pub fn make_int<'de, D>(deserializer: D) -> Result<u64, D::Error>
        where D: Deserializer<'de>,
    {
        match serde::Deserialize::deserialize(deserializer)? {
            Value::String(s) => s.parse().map_err(serde::de::Error::custom),
            Value::Number(num) => num.as_u64().ok_or_else(|| serde::de::Error::custom(format!("Could not deserialize {} as number", num))),
            _ => return Err(serde::de::Error::custom("invalid type")),
        }
    }
}

