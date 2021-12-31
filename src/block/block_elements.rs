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

impl ImageBlockElement {
    pub fn new(image_url: String, alt_text: String) -> ImageBlockElement {
        ImageBlockElement {
            image_url,
            alt_text,
        }
    }
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

impl MultiSelectBlockElement {
    pub fn builder(
        placeholder: TextBlockObject,
        action_id: String,
        options: Vec<OptionBlockObject>,
    ) -> MultiSelectBlockElementBuilder {
        MultiSelectBlockElementBuilder::new(placeholder, action_id, options)
    }
}

#[derive(Debug, Default)]
pub struct MultiSelectBlockElementBuilder {
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

impl MultiSelectBlockElementBuilder {
    pub fn new(
        placeholder: TextBlockObject,
        action_id: String,
        options: Vec<OptionBlockObject>,
    ) -> MultiSelectBlockElementBuilder {
        MultiSelectBlockElementBuilder {
            placeholder,
            action_id,
            options,
            ..Default::default()
        }
    }
    pub fn option_groups(
        mut self,
        option_groups: Vec<OptionGroupBlockObject>,
    ) -> MultiSelectBlockElementBuilder {
        self.option_groups = Some(option_groups);
        self
    }
    pub fn initial_option(
        mut self,
        initial_option: OptionBlockObject,
    ) -> MultiSelectBlockElementBuilder {
        self.initial_option = Some(initial_option);
        self
    }
    pub fn initial_users(mut self, initial_users: Vec<String>) -> MultiSelectBlockElementBuilder {
        self.initial_users = Some(initial_users);
        self
    }
    pub fn initial_conversations(
        mut self,
        initial_conversations: Vec<String>,
    ) -> MultiSelectBlockElementBuilder {
        self.initial_conversations = Some(initial_conversations);
        self
    }
    pub fn initial_channels(
        mut self,
        initial_channels: Vec<String>,
    ) -> MultiSelectBlockElementBuilder {
        self.initial_channels = Some(initial_channels);
        self
    }
    pub fn confirm(mut self, confirm: ConfirmationBlockObject) -> MultiSelectBlockElementBuilder {
        self.confirm = Some(confirm);
        self
    }
    pub fn min_query_length(mut self, min_query_length: i32) -> MultiSelectBlockElementBuilder {
        self.min_query_length = Some(min_query_length);
        self
    }
    pub fn max_selected_items(mut self, max_selected_items: i32) -> MultiSelectBlockElementBuilder {
        self.max_selected_items = Some(max_selected_items);
        self
    }
    pub fn focus_on_load(mut self, focus_on_load: bool) -> MultiSelectBlockElementBuilder {
        self.focus_on_load = Some(focus_on_load);
        self
    }
    pub fn build(self) -> MultiSelectBlockElementBuilder {
        MultiSelectBlockElementBuilder {
            placeholder: self.placeholder,
            action_id: self.action_id,
            options: self.options,
            option_groups: self.option_groups,
            initial_option: self.initial_option,
            initial_users: self.initial_users,
            initial_conversations: self.initial_conversations,
            initial_channels: self.initial_channels,
            confirm: self.confirm,
            min_query_length: self.min_query_length,
            max_selected_items: self.max_selected_items,
            focus_on_load: self.focus_on_load,
        }
    }
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct OverflowBlockElement {
    pub action_id: String,
    pub options: Vec<OptionBlockObject>,
    pub confirm: Option<ConfirmationBlockObject>,
}

impl OverflowBlockElement {
    pub fn builder(
        action_id: String,
        options: Vec<OptionBlockObject>,
    ) -> OverflowBlockElementBuilder {
        OverflowBlockElementBuilder::new(action_id, options)
    }
}

#[derive(Debug, Default)]
pub struct OverflowBlockElementBuilder {
    pub action_id: String,
    pub options: Vec<OptionBlockObject>,
    pub confirm: Option<ConfirmationBlockObject>,
}

impl OverflowBlockElementBuilder {
    pub fn new(action_id: String, options: Vec<OptionBlockObject>) -> OverflowBlockElementBuilder {
        OverflowBlockElementBuilder {
            action_id,
            options,
            ..Default::default()
        }
    }
    pub fn confirm(mut self, confirm: ConfirmationBlockObject) -> OverflowBlockElementBuilder {
        self.confirm = Some(confirm);
        self
    }
    pub fn build(self) -> OverflowBlockElement {
        OverflowBlockElement {
            action_id: self.action_id,
            options: self.options,
            confirm: self.confirm,
        }
    }
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

impl PlainTextInputBlockElement {
    pub fn builder(action_id: String) -> PlainTextInputBlockElementBuilder {
        PlainTextInputBlockElementBuilder::new(action_id)
    }
}

#[derive(Debug, Default)]
pub struct PlainTextInputBlockElementBuilder {
    pub action_id: String,
    pub placeholder: Option<TextBlockObject>,
    pub initial_value: Option<String>,
    pub multiline: Option<bool>,
    pub min_length: Option<i32>,
    pub max_length: Option<i32>,
    pub dispatch_action_config: Option<DispatchActionConfig>,
    pub focus_on_load: Option<bool>,
}

impl PlainTextInputBlockElementBuilder {
    pub fn new(action_id: String) -> PlainTextInputBlockElementBuilder {
        PlainTextInputBlockElementBuilder {
            action_id,
            ..Default::default()
        }
    }
    pub fn placeholder(
        mut self,
        placeholder: TextBlockObject,
    ) -> PlainTextInputBlockElementBuilder {
        self.placeholder = Some(placeholder);
        self
    }
    pub fn initial_value(mut self, initial_value: String) -> PlainTextInputBlockElementBuilder {
        self.initial_value = Some(initial_value);
        self
    }
    pub fn multiline(mut self, multiline: bool) -> PlainTextInputBlockElementBuilder {
        self.multiline = Some(multiline);
        self
    }
    pub fn min_length(mut self, min_length: i32) -> PlainTextInputBlockElementBuilder {
        self.min_length = Some(min_length);
        self
    }
    pub fn max_length(mut self, max_length: i32) -> PlainTextInputBlockElementBuilder {
        self.max_length = Some(max_length);
        self
    }
    pub fn dispatch_action_config(
        mut self,
        dispatch_action_config: DispatchActionConfig,
    ) -> PlainTextInputBlockElementBuilder {
        self.dispatch_action_config = Some(dispatch_action_config);
        self
    }
    pub fn focus_on_load(mut self, focus_on_load: bool) -> PlainTextInputBlockElementBuilder {
        self.focus_on_load = Some(focus_on_load);
        self
    }
    pub fn build(self) -> PlainTextInputBlockElement {
        PlainTextInputBlockElement {
            action_id: self.action_id,
            placeholder: self.placeholder,
            initial_value: self.initial_value,
            multiline: self.multiline,
            min_length: self.min_length,
            max_length: self.max_length,
            dispatch_action_config: self.dispatch_action_config,
            focus_on_load: self.focus_on_load,
        }
    }
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct RadioButtonsBlockElement {
    pub action_id: String,
    pub options: Vec<OptionBlockObject>,
    pub initial_option: Option<OptionBlockObject>,
    pub confirm: Option<ConfirmationBlockObject>,
}

impl RadioButtonsBlockElement {
    pub fn builder(
        action_id: String,
        options: Vec<OptionBlockObject>,
    ) -> RadioButtonsBlockElementBuilder {
        RadioButtonsBlockElementBuilder::new(action_id, options)
    }
}

#[derive(Debug, Default)]
pub struct RadioButtonsBlockElementBuilder {
    pub action_id: String,
    pub options: Vec<OptionBlockObject>,
    pub initial_option: Option<OptionBlockObject>,
    pub confirm: Option<ConfirmationBlockObject>,
}

impl RadioButtonsBlockElementBuilder {
    pub fn new(
        action_id: String,
        options: Vec<OptionBlockObject>,
    ) -> RadioButtonsBlockElementBuilder {
        RadioButtonsBlockElementBuilder {
            action_id,
            options,
            ..Default::default()
        }
    }
    pub fn initial_option(
        mut self,
        initial_option: OptionBlockObject,
    ) -> RadioButtonsBlockElementBuilder {
        self.initial_option = Some(initial_option);
        self
    }
    pub fn confirm(mut self, confirm: ConfirmationBlockObject) -> RadioButtonsBlockElementBuilder {
        self.confirm = Some(confirm);
        self
    }
    pub fn build(self) -> RadioButtonsBlockElement {
        RadioButtonsBlockElement {
            action_id: self.action_id,
            options: self.options,
            initial_option: self.initial_option,
            confirm: self.confirm,
        }
    }
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

impl SelectBlockElement {
    pub fn builder(
        placeholder: TextBlockObject,
        action_id: String,
        options: Vec<OptionBlockObject>,
    ) -> SelectBlockElementBuilder {
        SelectBlockElementBuilder::new(placeholder, action_id, options)
    }
}

#[derive(Debug, Default)]
pub struct SelectBlockElementBuilder {
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

impl SelectBlockElementBuilder {
    pub fn new(
        placeholder: TextBlockObject,
        action_id: String,
        options: Vec<OptionBlockObject>,
    ) -> SelectBlockElementBuilder {
        SelectBlockElementBuilder {
            placeholder,
            action_id,
            options,
            ..Default::default()
        }
    }
    pub fn option_groups(
        mut self,
        option_groups: Vec<OptionGroupBlockObject>,
    ) -> SelectBlockElementBuilder {
        self.option_groups = Some(option_groups);
        self
    }
    pub fn initial_option(
        mut self,
        initial_option: OptionBlockObject,
    ) -> SelectBlockElementBuilder {
        self.initial_option = Some(initial_option);
        self
    }
    pub fn initial_users(mut self, initial_users: Vec<String>) -> SelectBlockElementBuilder {
        self.initial_users = Some(initial_users);
        self
    }
    pub fn initial_conversations(
        mut self,
        initial_conversations: Vec<String>,
    ) -> SelectBlockElementBuilder {
        self.initial_conversations = Some(initial_conversations);
        self
    }
    pub fn initial_channels(mut self, initial_channels: Vec<String>) -> SelectBlockElementBuilder {
        self.initial_channels = Some(initial_channels);
        self
    }
    pub fn confirm(mut self, confirm: ConfirmationBlockObject) -> SelectBlockElementBuilder {
        self.confirm = Some(confirm);
        self
    }
    pub fn min_query_lengths(mut self, min_query_length: i32) -> SelectBlockElementBuilder {
        self.min_query_length = Some(min_query_length);
        self
    }
    pub fn max_selected_items(mut self, max_selected_items: i32) -> SelectBlockElementBuilder {
        self.max_selected_items = Some(max_selected_items);
        self
    }
    pub fn focus_on_load(mut self, focus_on_load: bool) -> SelectBlockElementBuilder {
        self.focus_on_load = Some(focus_on_load);
        self
    }
    pub fn build(self) -> SelectBlockElement {
        SelectBlockElement {
            placeholder: self.placeholder,
            action_id: self.action_id,
            options: self.options,
            option_groups: self.option_groups,
            initial_option: self.initial_option,
            initial_users: self.initial_users,
            initial_conversations: self.initial_conversations,
            initial_channels: self.initial_channels,
            confirm: self.confirm,
            min_query_length: self.min_query_length,
            max_selected_items: self.max_selected_items,
            focus_on_load: self.focus_on_load,
        }
    }
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct TimePickerBlockElement {
    pub action_id: String,
    pub placeholder: Option<TextBlockObject>,
    pub initial_time: Option<String>,
    pub confirm: Option<ConfirmationBlockObject>,
}

impl TimePickerBlockElement {
    pub fn builder(action_id: String) -> TimePickerBlockElementBuilder {
        TimePickerBlockElementBuilder::new(action_id)
    }
}

#[derive(Debug, Default)]
pub struct TimePickerBlockElementBuilder {
    pub action_id: String,
    pub placeholder: Option<TextBlockObject>,
    pub initial_time: Option<String>,
    pub confirm: Option<ConfirmationBlockObject>,
}

impl TimePickerBlockElementBuilder {
    pub fn new(action_id: String) -> TimePickerBlockElementBuilder {
        TimePickerBlockElementBuilder {
            action_id,
            ..Default::default()
        }
    }
    pub fn placeholder(mut self, placeholder: TextBlockObject) -> TimePickerBlockElementBuilder {
        self.placeholder = Some(placeholder);
        self
    }
    pub fn initial_time(mut self, initial_time: String) -> TimePickerBlockElementBuilder {
        self.initial_time = Some(initial_time);
        self
    }
    pub fn confirm(mut self, confirm: ConfirmationBlockObject) -> TimePickerBlockElementBuilder {
        self.confirm = Some(confirm);
        self
    }
    pub fn build(self) -> TimePickerBlockElementBuilder {
        TimePickerBlockElementBuilder {
            action_id: self.action_id,
            placeholder: self.placeholder,
            initial_time: self.initial_time,
            confirm: self.confirm,
        }
    }
}
