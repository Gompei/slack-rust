use crate::reactions::reaction::Reaction;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
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
