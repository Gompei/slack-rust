use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct Reaction {
    pub count: Option<i32>,
    pub name: Option<String>,
    pub users: Option<Vec<String>>,
}
