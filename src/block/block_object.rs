use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct TextBlockObject {
    #[serde(rename = "type")]
    pub type_filed: TextBlockType,
    pub text: String,
    pub emoji: Option<bool>,
    pub verbatim: Option<bool>,
}

impl TextBlockObject {
    pub fn builder(text_block_type: TextBlockType, text: String) -> TextBlockObjectBuilder {
        TextBlockObjectBuilder::new(text_block_type, text)
    }
}

#[derive(Debug, Default)]
pub struct TextBlockObjectBuilder {
    pub type_filed: TextBlockType,
    pub text: String,
    pub emoji: Option<bool>,
    pub verbatim: Option<bool>,
}

impl TextBlockObjectBuilder {
    pub fn new(text_block_type: TextBlockType, text: String) -> TextBlockObjectBuilder {
        TextBlockObjectBuilder {
            type_filed: text_block_type,
            text,
            ..Default::default()
        }
    }
    pub fn emoji(mut self, emoji: bool) -> TextBlockObjectBuilder {
        self.emoji = Some(emoji);
        self
    }
    pub fn verbatim(mut self, verbatim: bool) -> TextBlockObjectBuilder {
        self.verbatim = Some(verbatim);
        self
    }
    pub fn build(self) -> TextBlockObject {
        TextBlockObject {
            type_filed: self.type_filed,
            text: self.text,
            emoji: self.emoji,
            verbatim: self.verbatim,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TextBlockType {
    PlainText,
    Mrkdwn,
    #[serde(skip)]
    None,
}

impl Default for TextBlockType {
    fn default() -> Self {
        TextBlockType::None
    }
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct OptionBlockObject {
    pub text: TextBlockObject,
    pub value: Option<String>,
    pub description: Option<TextBlockObject>,
    pub url: Option<String>,
}

impl OptionBlockObject {
    pub fn builder(text: TextBlockObject) -> OptionBlockObjectBuilder {
        OptionBlockObjectBuilder::new(text)
    }
}

#[derive(Debug, Default)]
pub struct OptionBlockObjectBuilder {
    pub text: TextBlockObject,
    pub value: Option<String>,
    pub description: Option<TextBlockObject>,
    pub url: Option<String>,
}

impl OptionBlockObjectBuilder {
    pub fn new(text: TextBlockObject) -> OptionBlockObjectBuilder {
        OptionBlockObjectBuilder {
            text,
            ..Default::default()
        }
    }
    pub fn value(mut self, value: String) -> OptionBlockObjectBuilder {
        self.value = Some(value);
        self
    }
    pub fn description(mut self, description: TextBlockObject) -> OptionBlockObjectBuilder {
        self.description = Some(description);
        self
    }
    pub fn url(mut self, url: String) -> OptionBlockObjectBuilder {
        self.url = Some(url);
        self
    }
    pub fn build(self) -> OptionBlockObject {
        OptionBlockObject {
            text: self.text,
            value: self.value,
            description: self.description,
            url: self.url,
        }
    }
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct OptionGroupBlockObject {
    pub label: Option<TextBlockObject>,
    pub options: Option<Vec<OptionBlockObject>>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct ConfirmationBlockObject {
    pub title: TextBlockObject,
    pub text: TextBlockObject,
    pub confirm: TextBlockObject,
    pub deny: TextBlockObject,
    pub style: Option<String>,
}

impl ConfirmationBlockObject {
    pub fn builder(
        title: TextBlockObject,
        text: TextBlockObject,
        confirm: TextBlockObject,
        deny: TextBlockObject,
    ) -> ConfirmationBlockObjectBuilder {
        ConfirmationBlockObjectBuilder::new(title, text, confirm, deny)
    }
}

#[derive(Debug, Default)]
pub struct ConfirmationBlockObjectBuilder {
    pub title: TextBlockObject,
    pub text: TextBlockObject,
    pub confirm: TextBlockObject,
    pub deny: TextBlockObject,
    pub style: Option<String>,
}

impl ConfirmationBlockObjectBuilder {
    pub fn new(
        title: TextBlockObject,
        text: TextBlockObject,
        confirm: TextBlockObject,
        deny: TextBlockObject,
    ) -> ConfirmationBlockObjectBuilder {
        ConfirmationBlockObjectBuilder {
            title,
            text,
            confirm,
            deny,
            ..Default::default()
        }
    }
    pub fn style(mut self, style: String) -> ConfirmationBlockObjectBuilder {
        self.style = Some(style);
        self
    }
    pub fn build(self) -> ConfirmationBlockObject {
        ConfirmationBlockObject {
            title: self.title,
            text: self.text,
            confirm: self.confirm,
            deny: self.deny,
            style: self.style,
        }
    }
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct DispatchActionConfig {
    pub trigger_actions_on: Option<Vec<String>>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct SelectBlockElementFilter {
    pub include: Option<Vec<String>>,
    pub exclude_external_shared_channel: Option<bool>,
    pub exclude_bot_users: Option<bool>,
}
