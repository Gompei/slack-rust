use crate::reactions::reaction::Reaction;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct Comment {
    #[serde(rename = "type")]
    pub type_filed: Option<String>,
    pub comment: Option<String>,
    pub created: Option<i32>,
    pub id: Option<String>,
    pub reactions: Option<Vec<Reaction>>,
    pub timestamp: Option<i32>,
    pub user: Option<String>,
}
