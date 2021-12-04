use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct TextBlockObject {
    pub r#type: Option<String>,
    pub text: Option<String>,
    pub emoji: bool,
    pub verbatim: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OptionBlockObject {
    pub text: TextBlockObject,
    pub value: Option<String>,
    pub description: TextBlockObject,
    pub url: Option<String>,
}
