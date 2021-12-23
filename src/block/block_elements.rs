use crate::block::block_object::{
    ConfirmationBlockObject, DispatchActionConfig, OptionBlockObject, OptionGroupBlockObject,
    TextBlockObject, TextBlockType,
};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum BlockElement {
    ButtonElement(ButtonElement),
    CheckboxGroupsBlockElement(CheckboxGroupsBlockElement),
    DatePickerBlockElement(DatePickerBlockElement),
    ImageBlockElement(ImageBlockElement),
    MultiSelectBlockElement(MultiSelectBlockElement),
    OverflowBlockElement(OverflowBlockElement),
    PlainTextInputBlockElement(PlainTextInputBlockElement),
    RadioButtonsBlockElement(RadioButtonsBlockElement),
    SelectBlockElement(SelectBlockElement),
    TimePickerBlockElement(TimePickerBlockElement),
    None,
}

impl BlockElement {
    pub fn block_type(&self) -> BlockElementType {
        match self {
            BlockElement::ButtonElement(ButtonElement { .. }) => BlockElementType::Button,
            BlockElement::CheckboxGroupsBlockElement(CheckboxGroupsBlockElement { .. }) => {
                BlockElementType::Checkboxes
            }
            BlockElement::DatePickerBlockElement(DatePickerBlockElement { .. }) => {
                BlockElementType::Datepicker
            }
            BlockElement::ImageBlockElement(ImageBlockElement { .. }) => BlockElementType::Image,
            BlockElement::MultiSelectBlockElement(MultiSelectBlockElement { .. }) => {
                BlockElementType::MultiStaticSelect
            }
            BlockElement::OverflowBlockElement(OverflowBlockElement { .. }) => {
                BlockElementType::Overflow
            }
            BlockElement::PlainTextInputBlockElement(PlainTextInputBlockElement { .. }) => {
                BlockElementType::PlainTextInput
            }
            BlockElement::RadioButtonsBlockElement(RadioButtonsBlockElement { .. }) => {
                BlockElementType::RadioButtons
            }
            BlockElement::SelectBlockElement(SelectBlockElement { .. }) => {
                BlockElementType::StaticSelect
            }
            BlockElement::TimePickerBlockElement(TimePickerBlockElement { .. }) => {
                BlockElementType::Timepicker
            }
            BlockElement::None => BlockElementType::None,
        }
    }
}

impl Default for BlockElement {
    fn default() -> Self {
        BlockElement::None
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum MixedElement {
    ImageBlockElement(ImageBlockElement),
    TextBlockObject(TextBlockObject),
    None,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MixedElementType {
    Image,
    PlainText,
    Mrkdwn,
    None,
}

impl MixedElement {
    pub fn block_type(&self) -> MixedElementType {
        match self {
            MixedElement::ImageBlockElement(ImageBlockElement { .. }) => MixedElementType::Image,
            MixedElement::TextBlockObject(TextBlockObject { type_filed, .. }) => match type_filed {
                TextBlockType::PlainText => MixedElementType::PlainText,
                TextBlockType::Mrkdwn => MixedElementType::Mrkdwn,
                TextBlockType::None => MixedElementType::None,
            },
            MixedElement::None => MixedElementType::None,
        }
    }
}

impl Default for MixedElement {
    fn default() -> Self {
        MixedElement::None
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BlockElementType {
    Button,
    Checkboxes,
    Datepicker,
    Image,
    MultiStaticSelect,
    Overflow,
    PlainTextInput,
    RadioButtons,
    StaticSelect,
    Timepicker,
    None,
}

impl Default for BlockElementType {
    fn default() -> Self {
        BlockElementType::None
    }
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct ButtonElement {
    #[serde(rename = "type")]
    pub type_filed: BlockElementType,
    pub text: TextBlockObject,
    pub action_id: String,
    pub url: Option<String>,
    pub value: Option<String>,
    pub style: Option<String>,
    pub confirm: Option<ConfirmationBlockObject>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct CheckboxGroupsBlockElement {
    pub action_id: String,
    pub options: Vec<OptionBlockObject>,
    pub initial_options: Option<Vec<OptionBlockObject>>,
    pub confirm: Option<ConfirmationBlockObject>,
    pub focus_on_load: Option<bool>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct DatePickerBlockElement {
    #[serde(rename = "type")]
    pub type_filed: BlockElementType,
    pub action_id: String,
    pub placeholder: Option<TextBlockObject>,
    pub initial_date: Option<String>,
    pub confirm: Option<ConfirmationBlockObject>,
    pub focus_on_load: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct ImageBlockElement {
    #[serde(rename = "type")]
    pub type_filed: BlockElementType,
    pub image_url: String,
    pub alt_text: String,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct MultiSelectBlockElement {
    #[serde(rename = "type")]
    pub type_filed: BlockElementType,
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

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct OverflowBlockElement {
    #[serde(rename = "type")]
    pub type_filed: BlockElementType,
    pub action_id: String,
    pub options: Vec<OptionBlockObject>,
    pub confirm: Option<ConfirmationBlockObject>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct PlainTextInputBlockElement {
    #[serde(rename = "type")]
    pub type_filed: BlockElementType,
    pub action_id: String,
    pub placeholder: Option<TextBlockObject>,
    pub initial_value: Option<String>,
    pub multiline: Option<bool>,
    pub min_length: Option<i32>,
    pub max_length: Option<i32>,
    pub dispatch_action_config: Option<DispatchActionConfig>,
    pub focus_on_load: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct RadioButtonsBlockElement {
    #[serde(rename = "type")]
    pub type_filed: BlockElementType,
    pub action_id: String,
    pub options: Vec<OptionBlockObject>,
    pub initial_option: Option<OptionBlockObject>,
    pub confirm: Option<ConfirmationBlockObject>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct SelectBlockElement {
    #[serde(rename = "type")]
    pub type_filed: BlockElementType,
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

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct TimePickerBlockElement {
    #[serde(rename = "type")]
    pub type_filed: BlockElementType,
    pub action_id: String,
    pub placeholder: Option<TextBlockObject>,
    pub initial_time: Option<String>,
    pub confirm: Option<ConfirmationBlockObject>,
}
