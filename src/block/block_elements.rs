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
    #[serde(skip)]
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

impl ButtonElement {
    pub fn builder(text: TextBlockObject, action_id: String) -> ButtonElementBuilder {
        ButtonElementBuilder::new(text, action_id)
    }
}

#[derive(Debug, Default)]
pub struct ButtonElementBuilder {
    pub text: TextBlockObject,
    pub action_id: String,
    pub url: Option<String>,
    pub value: Option<String>,
    pub style: Option<String>,
    pub confirm: Option<ConfirmationBlockObject>,
}

impl ButtonElementBuilder {
    pub fn new(text: TextBlockObject, action_id: String) -> ButtonElementBuilder {
        ButtonElementBuilder {
            text,
            action_id,
            ..Default::default()
        }
    }
    pub fn url(mut self, url: String) -> ButtonElementBuilder {
        self.url = Some(url);
        self
    }
    pub fn value(mut self, value: String) -> ButtonElementBuilder {
        self.value = Some(value);
        self
    }
    pub fn style(mut self, style: String) -> ButtonElementBuilder {
        self.style = Some(style);
        self
    }
    pub fn confirm(mut self, confirm: ConfirmationBlockObject) -> ButtonElementBuilder {
        self.confirm = Some(confirm);
        self
    }
    pub fn build(self) -> ButtonElement {
        ButtonElement {
            text: self.text,
            action_id: self.action_id,
            url: self.url,
            value: self.value,
            style: self.style,
            confirm: self.confirm,
        }
    }
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

impl CheckboxGroupsBlockElement {
    pub fn builder(
        action_id: String,
        options: Vec<OptionBlockObject>,
    ) -> CheckboxGroupsBlockElementBuilder {
        CheckboxGroupsBlockElementBuilder::new(action_id, options)
    }
}

#[derive(Debug, Default)]
pub struct CheckboxGroupsBlockElementBuilder {
    pub action_id: String,
    pub options: Vec<OptionBlockObject>,
    pub initial_options: Option<Vec<OptionBlockObject>>,
    pub confirm: Option<ConfirmationBlockObject>,
    pub focus_on_load: Option<bool>,
}

impl CheckboxGroupsBlockElementBuilder {
    pub fn new(
        action_id: String,
        options: Vec<OptionBlockObject>,
    ) -> CheckboxGroupsBlockElementBuilder {
        CheckboxGroupsBlockElementBuilder {
            action_id,
            options,
            ..Default::default()
        }
    }
    pub fn initial_options(
        mut self,
        initial_options: Vec<OptionBlockObject>,
    ) -> CheckboxGroupsBlockElementBuilder {
        self.initial_options = Some(initial_options);
        self
    }
    pub fn confirm(
        mut self,
        confirm: ConfirmationBlockObject,
    ) -> CheckboxGroupsBlockElementBuilder {
        self.confirm = Some(confirm);
        self
    }
    pub fn focus_on_load(mut self, focus_on_load: bool) -> CheckboxGroupsBlockElementBuilder {
        self.focus_on_load = Some(focus_on_load);
        self
    }
    pub fn build(self) -> CheckboxGroupsBlockElement {
        CheckboxGroupsBlockElement {
            action_id: self.action_id,
            options: self.options,
            initial_options: self.initial_options,
            confirm: self.confirm,
            focus_on_load: self.focus_on_load,
        }
    }
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

impl DatePickerBlockElement {
    pub fn builder(action_id: String) -> DatePickerBlockElementBuilder {
        DatePickerBlockElementBuilder::new(action_id)
    }
}

#[derive(Debug, Default)]
pub struct DatePickerBlockElementBuilder {
    pub action_id: String,
    pub placeholder: Option<TextBlockObject>,
    pub initial_date: Option<String>,
    pub confirm: Option<ConfirmationBlockObject>,
    pub focus_on_load: Option<bool>,
}

impl DatePickerBlockElementBuilder {
    pub fn new(action_id: String) -> DatePickerBlockElementBuilder {
        DatePickerBlockElementBuilder {
            action_id,
            ..Default::default()
        }
    }
    pub fn placeholder(mut self, placeholder: TextBlockObject) -> DatePickerBlockElementBuilder {
        self.placeholder = Some(placeholder);
        self
    }
    pub fn initial_date(mut self, initial_date: String) -> DatePickerBlockElementBuilder {
        self.initial_date = Some(initial_date);
        self
    }
    pub fn confirm(mut self, confirm: ConfirmationBlockObject) -> DatePickerBlockElementBuilder {
        self.confirm = Some(confirm);
        self
    }
    pub fn focus_on_load(mut self, focus_on_load: bool) -> DatePickerBlockElementBuilder {
        self.focus_on_load = Some(focus_on_load);
        self
    }
    pub fn build(self) -> DatePickerBlockElement {
        DatePickerBlockElement {
            action_id: self.action_id,
            placeholder: self.placeholder,
            initial_date: self.initial_date,
            confirm: self.confirm,
            focus_on_load: self.focus_on_load,
        }
    }
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

#[skip_serializing_none]
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
