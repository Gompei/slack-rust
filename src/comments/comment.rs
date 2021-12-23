use crate::reactions::reaction::Reaction;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct Comment {
    pub r#type: Option<String>,
    pub comment: Option<String>,
    pub created: Option<i32>,
    pub id: Option<String>,
    pub reactions: Option<Vec<Reaction>>,
    pub timestamp: Option<i32>,
    pub user: Option<String>,
}
