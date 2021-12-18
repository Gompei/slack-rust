use crate::block::block_elements::MixedElement;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct TextBlockObject {
    pub r#type: String,
    pub text: String,
    pub emoji: Option<bool>,
    pub verbatim: Option<bool>,
}

impl MixedElement for TextBlockObject {
    fn mixed_element_type(&self) -> &String {
        &self.r#type
    }
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
    pub label: Option<TextBlockObject>,
    pub options: Option<Vec<OptionBlockObject>>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ConfirmationBlockObject {
    pub title: TextBlockObject,
    pub text: TextBlockObject,
    pub confirm: TextBlockObject,
    pub deny: TextBlockObject,
    pub style: Option<String>,
}
