use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct TextBlockObject {
    pub r#type: String,
    pub text: String,
    pub emoji: Option<bool>,
    pub verbatim: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct OptionBlockObject {
    pub text: TextBlockObject,
    pub value: Option<String>,
    pub description: TextBlockObject,
    pub url: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct OptionGroupBlockObject {
    pub label: Option<ioTextBlockObject>,
    pub Options: Option<Vec<OptionBlockObject>>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ConfirmationBlockObject {
    pub title: TextBlockObject,
    pub text: TextBlockObject,
    pub confirm: TextBlockObject,
    pub deny: TextBlockObject,
    pub style: Option<String>,
}
