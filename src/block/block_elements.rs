use crate::block::block_object::{
    ConfirmationBlockObject, DispatchActionConfig, OptionBlockObject, OptionGroupBlockObject,
    TextBlockObject,
};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(tag = "type")]
pub enum BlockElement {
    #[serde(rename = "button")]
    ButtonElement(ButtonElement),
    #[serde(rename = "checkboxes")]
    CheckboxGroupsBlockElement(CheckboxGroupsBlockElement),
    #[serde(rename = "datepicker")]
    DatePickerBlockElement(DatePickerBlockElement),
    #[serde(rename = "image")]
    ImageBlockElement(ImageBlockElement),
    #[serde(rename = "multi_static_select")]
    MultiSelectBlockElement(MultiSelectBlockElement),
    #[serde(rename = "overflow")]
    OverflowBlockElement(OverflowBlockElement),
    #[serde(rename = "plain_text_input")]
    PlainTextInputBlockElement(PlainTextInputBlockElement),
    #[serde(rename = "radio_buttons")]
    RadioButtonsBlockElement(RadioButtonsBlockElement),
    #[serde(rename = "static_select")]
    SelectBlockElement(SelectBlockElement),
    #[serde(rename = "timepicker")]
    TimePickerBlockElement(TimePickerBlockElement),
    #[serde(skip)]
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
#[serde(tag = "type")]
pub enum MixedElement {
    #[serde(rename = "image")]
    ImageBlockElement(ImageBlockElement),
    #[serde(rename = "plain_text")]
    PlainTextBlockObject {
        text: String,
        emoji: Option<bool>,
        verbatim: Option<bool>,
    },
    #[serde(rename = "mrkdwn")]
    MarkdownBlockObject {
        text: String,
        emoji: Option<bool>,
        verbatim: Option<bool>,
    },
    #[serde(skip)]
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
            MixedElement::PlainTextBlockObject { .. } => MixedElementType::PlainText,
            MixedElement::MarkdownBlockObject { .. } => MixedElementType::Mrkdwn,
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
    #[serde(skip)]
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
    pub action_id: String,
    pub placeholder: Option<TextBlockObject>,
    pub initial_date: Option<String>,
    pub confirm: Option<ConfirmationBlockObject>,
    pub focus_on_load: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct ImageBlockElement {
    pub image_url: String,
    pub alt_text: String,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
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

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct OverflowBlockElement {
    pub action_id: String,
    pub options: Vec<OptionBlockObject>,
    pub confirm: Option<ConfirmationBlockObject>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
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

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct RadioButtonsBlockElement {
    pub action_id: String,
    pub options: Vec<OptionBlockObject>,
    pub initial_option: Option<OptionBlockObject>,
    pub confirm: Option<ConfirmationBlockObject>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
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

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct TimePickerBlockElement {
    pub action_id: String,
    pub placeholder: Option<TextBlockObject>,
    pub initial_time: Option<String>,
    pub confirm: Option<ConfirmationBlockObject>,
}
