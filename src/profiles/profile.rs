use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Profile {
    pub fields: Option<Vec<Field>>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Field {
    pub id: Option<String>,
    pub ordering: Option<i8>,
    pub label: Option<String>,
    pub hint: Option<String>,
    pub r#type: Option<String>,
    pub possible_values: Option<Vec<String>>,
    pub options: Option<Options>,
    pub is_hidden: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Options {
    pub is_protected: Option<i8>,
}
