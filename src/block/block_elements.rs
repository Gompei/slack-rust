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

#[typetag::serde]
pub trait MixedElement {
    fn mixed_element_type(&self) -> &String;
}

impl fmt::Debug for dyn MixedElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.mixed_element_type())
    }
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ImageBlockElement {
    pub r#type: String,
    pub image_url: String,
    pub alt_text: String,
}

impl MixedElement for ImageBlockElement {
    fn mixed_element_type(&self) -> &String {
        &self.r#type
    }
}
