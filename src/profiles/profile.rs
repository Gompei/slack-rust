use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct Profile {
    pub fields: Option<Vec<Field>>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct Field {
    pub id: Option<String>,
    pub ordering: Option<i32>,
    pub label: Option<String>,
    pub hint: Option<String>,
    #[serde(rename = "type")]
    pub type_filed: Option<String>,
    pub possible_values: Option<Vec<String>>,
    pub options: Option<Options>,
    pub is_hidden: Option<bool>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct Options {
    pub is_protected: Option<i32>,
}
