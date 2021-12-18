use crate::block::block_elements::MixedElement;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct TextBlockObject {
    pub r#type: String,
    pub text: String,
    pub emoji: Option<bool>,
    pub verbatim: Option<bool>,
}

#[typetag::serde]
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

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct DispatchActionConfig {
    pub trigger_actions_on: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct SelectBlockElementFilter {
    pub include: Option<Vec<String>>,
    pub exclude_external_shared_channel: Option<bool>,
    pub exclude_bot_users: Option<bool>,
}
