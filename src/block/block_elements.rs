use crate::block::block_object::{
    ConfirmationBlockObject, OptionBlockObject, OptionGroupBlockObject, TextBlockObject,
};
use serde::{Deserialize, Serialize};
use std::fmt;

#[typetag::serde]
pub trait BlockElement {
    fn element_type(&self) -> &String;
}

impl fmt::Debug for dyn BlockElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.element_type())
    }
}

// impl Default for BlockElement {
//     fn default() -> Box<Self> {
//         Box::new(dyn)
//     }
// }

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct BlockElements(Option<Vec<Box<dyn BlockElement>>>);

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct SelectBlockElement {
    pub r#type: String,
    pub placeholder: TextBlockObject,
    pub action_id: String,
    pub options: Vec<OptionBlockObject>,
    pub option_groups: Option<Vec<OptionGroupBlockObject>>,
    pub initial_option: Option<OptionBlockObject>,
    pub initial_users: Option<Vec<String>>,
    pub initial_conversations: Option<Vec<String>>,
    pub initial_channels: Option<Vec<String>>,
    pub confirm: Option<ConfirmationBlockObject>,
    pub min_query_length: Option<i32>,
    pub max_selected_items: Option<i32>,
    pub focus_on_load: Option<bool>,
}

#[typetag::serde]
impl BlockElement for SelectBlockElement {
    fn element_type(&self) -> &String {
        &self.r#type
    }
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ImageBlockElement {
    pub r#type: String,
    pub image_url: String,
    pub alt_text: String,
}

#[typetag::serde]
impl BlockElement for ImageBlockElement {
    fn element_type(&self) -> &String {
        &self.r#type
    }
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ButtonElement {
    pub r#type: String,
    pub text: TextBlockObject,
    pub action_id: String,
    pub url: Option<String>,
    pub value: Option<String>,
    pub style: Option<String>,
    pub confirm: Option<ConfirmationBlockObject>,
}

#[typetag::serde]
impl BlockElement for ButtonElement {
    fn element_type(&self) -> &String {
        &self.r#type
    }
}
