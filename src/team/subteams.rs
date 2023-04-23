use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{Value};

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Subteam {
    pub id: String,
    pub team_id: String,
    pub is_usergroup: bool,
    pub name: String,
    pub description: String,
    pub handle: String,
    pub is_external: bool,
    pub date_create: u32,
    pub date_update: u32,
    pub date_delete: u32,
    pub auto_type: Value,
    pub created_by: Value,
    pub updated_by: Value,
    pub deleted_by: Value,
    pub prefs: SubteamPrefs,
    /// Note that the Slack API examples have this as an integer value encoded
    /// as a string
    #[serde(deserialize_with = "Subteam::make_int")]
    pub user_count: u32,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct SubteamPrefs {
    pub channels: Vec<String>,
    pub groups: Vec<String>,
}

impl Subteam {
    fn make_int<'de, D>(deserializer: D) -> Result<u32, D::Error>
        where D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

