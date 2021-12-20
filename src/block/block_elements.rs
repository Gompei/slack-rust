use crate::block::block_object::{
    ConfirmationBlockObject, DispatchActionConfig, OptionBlockObject, OptionGroupBlockObject,
    TextBlockObject,
};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use std::fmt::Debug;

#[typetag::serde(tag = "type")]
pub trait BlockElement: Debug {}

#[typetag::serde(tag = "type")]
pub trait MixedElement: Debug {}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ButtonElement {
    pub text: TextBlockObject,
    pub action_id: String,
    pub url: Option<String>,
    pub value: Option<String>,
    pub style: Option<String>,
    pub confirm: Option<ConfirmationBlockObject>,
}

#[typetag::serde(name = "button")]
impl BlockElement for ButtonElement {}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct CheckboxGroupsBlockElement {
    pub action_id: String,
    pub options: Vec<OptionBlockObject>,
    pub initial_options: Option<Vec<OptionBlockObject>>,
    pub confirm: Option<ConfirmationBlockObject>,
    pub focus_on_load: Option<bool>,
}

#[typetag::serde(name = "checkboxes")]
impl BlockElement for CheckboxGroupsBlockElement {}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct DatePickerBlockElement {
    pub action_id: String,
    pub placeholder: Option<TextBlockObject>,
    pub initial_date: Option<String>,
    pub confirm: Option<ConfirmationBlockObject>,
    pub focus_on_load: Option<bool>,
}

#[typetag::serde(name = "datepicker")]
impl BlockElement for DatePickerBlockElement {}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ImageBlockElement {
    pub image_url: String,
    pub alt_text: String,
}

#[typetag::serde(name = "image")]
impl BlockElement for ImageBlockElement {}

#[typetag::serde(name = "image")]
impl MixedElement for ImageBlockElement {}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct MultiSelectBlockElement {
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

#[typetag::serde(name = "multi_static_select")]
impl BlockElement for MultiSelectBlockElement {}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct OverflowBlockElement {
    pub action_id: String,
    pub options: Vec<OptionBlockObject>,
    pub confirm: Option<ConfirmationBlockObject>,
}

#[typetag::serde(name = "overflow")]
impl BlockElement for OverflowBlockElement {}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct PlainTextInputBlockElement {
    pub action_id: String,
    pub placeholder: Option<TextBlockObject>,
    pub initial_value: Option<String>,
    pub multiline: Option<bool>,
    pub min_length: Option<i32>,
    pub max_length: Option<i32>,
    pub dispatch_action_config: Option<DispatchActionConfig>,
    pub focus_on_load: Option<bool>,
}

#[typetag::serde(name = "plain_text_input")]
impl BlockElement for PlainTextInputBlockElement {}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct RadioButtonsBlockElement {
    pub action_id: String,
    pub options: Vec<OptionBlockObject>,
    pub initial_option: Option<OptionBlockObject>,
    pub confirm: Option<ConfirmationBlockObject>,
}

#[typetag::serde(name = "radio_buttons")]
impl BlockElement for RadioButtonsBlockElement {}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct SelectBlockElement {
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

#[typetag::serde(name = "static_select")]
impl BlockElement for SelectBlockElement {}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct TimePickerBlockElement {
    pub action_id: String,
    pub placeholder: Option<TextBlockObject>,
    pub initial_time: Option<String>,
    pub confirm: Option<ConfirmationBlockObject>,
}

#[typetag::serde(name = "timepicker")]
impl BlockElement for TimePickerBlockElement {}
