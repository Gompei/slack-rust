use crate::block::block_object::{
    ConfirmationBlockObject, DispatchActionConfig, OptionBlockObject, OptionGroupBlockObject,
    TextBlockObject,
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

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct CheckboxGroupsBlockElement {
    pub r#type: String,
    pub action_id: String,
    pub options: Vec<OptionBlockObject>,
    pub initial_options: Option<Vec<OptionBlockObject>>,
    pub confirm: Option<ConfirmationBlockObject>,
    pub focus_on_load: Option<bool>,
}

#[typetag::serde]
impl BlockElement for CheckboxGroupsBlockElement {
    fn element_type(&self) -> &String {
        &self.r#type
    }
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct DatePickerBlockElement {
    pub r#type: String,
    pub action_id: String,
    pub placeholder: Option<TextBlockObject>,
    pub initial_date: Option<String>,
    pub confirm: Option<ConfirmationBlockObject>,
    pub focus_on_load: Option<bool>,
}

#[typetag::serde]
impl BlockElement for DatePickerBlockElement {
    fn element_type(&self) -> &String {
        &self.r#type
    }
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct MultiSelectBlockElement {
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
impl BlockElement for MultiSelectBlockElemen {
    fn element_type(&self) -> &String {
        &self.r#type
    }
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct OverflowBlockElement {
    pub r#type: String,
    pub action_id: String,
    pub options: Vec<OptionBlockObject>,
    pub confirm: Option<ConfirmationBlockObject>,
}

#[typetag::serde]
impl BlockElement for OverflowBlockElement {
    fn element_type(&self) -> &String {
        &self.r#type
    }
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct PlainTextInputBlockElement {
    pub r#type: String,
    pub action_id: String,
    pub placeholder: Option<TextBlockObject>,
    pub initial_value: Option<String>,
    pub multiline: Option<bool>,
    pub min_length: Option<i32>,
    pub max_length: Option<i32>,
    pub dispatch_action_config: Option<DispatchActionConfig>,
    pub focus_on_load: Option<bool>,
}

#[typetag::serde]
impl BlockElement for PlainTextInputBlockElement {
    fn element_type(&self) -> &String {
        &self.r#type
    }
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct RadioButtonsBlockElement {
    pub r#type: String,
    pub action_id: String,
    pub options: Vec<OptionBlockObject>,
    pub initial_option: Option<OptionBlockObject>,
    pub confirm: Option<ConfirmationBlockObject>,
}

#[typetag::serde]
impl BlockElement for RadioButtonsBlockElement {
    fn element_type(&self) -> &String {
        &self.r#type
    }
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct TimePickerBlockElement {
    pub r#type: String,
    pub action_id: String,
    pub placeholder: Option<TextBlockObject>,
    pub initial_time: Option<String>,
    pub confirm: Option<ConfirmationBlockObject>,
}

#[typetag::serde]
impl BlockElement for TimePickerBlockElement {
    fn element_type(&self) -> &String {
        &self.r#type
    }
}
