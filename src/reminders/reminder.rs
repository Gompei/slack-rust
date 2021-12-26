use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct Reminder {
    pub id: Option<String>,
    pub creator: Option<String>,
    pub user: Option<String>,
    pub text: Option<String>,
    pub recurring: Option<bool>,
    pub time: Option<i32>,
    pub complete_ts: Option<i32>,
}
