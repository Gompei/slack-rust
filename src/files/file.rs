use crate::reactions::reaction::Reaction;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct File {
    pub channels: Option<Vec<String>>,
    pub comments_count: Option<i32>,
    pub created: Option<i32>,
    pub groups: Option<Vec<String>>,
    pub id: Option<String>,
    pub ims: Option<Vec<String>>,
    pub name: Option<String>,
    pub reactions: Option<Vec<Reaction>>,
    pub timestamp: Option<i32>,
    pub title: Option<String>,
    pub user: Option<String>,
}
