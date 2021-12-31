//! [Attachment](https://api.slack.com/reference/messaging/attachments)

use crate::block::blocks::Block;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// See: <https://api.slack.com/reference/messaging/attachments#field_objects>
#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct AttachmentField {
    pub title: Option<String>,
    pub value: Option<String>,
    pub short: Option<bool>,
}

impl AttachmentField {
    pub fn builder() -> AttachmentFieldBuilder {
        AttachmentFieldBuilder::new()
    }
}

#[derive(Debug, Default)]
pub struct AttachmentFieldBuilder {
    pub title: Option<String>,
    pub value: Option<String>,
    pub short: Option<bool>,
}

impl AttachmentFieldBuilder {
    pub fn new() -> AttachmentFieldBuilder {
        AttachmentFieldBuilder {
            title: Some("".to_string()),
            ..Default::default()
        }
    }
    pub fn title(mut self, title: String) -> AttachmentFieldBuilder {
        self.title = Some(title);
        self
    }
    pub fn value(mut self, value: String) -> AttachmentFieldBuilder {
        self.value = Some(value);
        self
    }
    pub fn short(mut self, short: bool) -> AttachmentFieldBuilder {
        self.short = Some(short);
        self
    }
    pub fn build(self) -> AttachmentField {
        AttachmentField {
            title: self.title,
            value: self.value,
            short: self.short,
        }
    }
}

/// See: <https://api.slack.com/reference/messaging/attachments#fields>
#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct Attachment {
    pub color: Option<String>,
    pub fallback: Option<String>,
    pub callback_id: Option<String>,
    pub id: Option<i32>,
    pub author_id: Option<String>,
    pub author_name: Option<String>,
    pub author_subname: Option<String>,
    pub author_link: Option<String>,
    pub author_icon: Option<String>,
    pub title: Option<String>,
    pub title_link: Option<String>,
    pub pretext: Option<String>,
    pub text: Option<String>,
    pub image_url: Option<String>,
    pub thumb_url: Option<String>,
    pub thumb_height: Option<i32>,
    pub thumb_width: Option<i32>,
    pub from_url: Option<String>,
    pub original_url: Option<String>,
    pub fields: Option<Vec<AttachmentField>>,
    pub mrkdwn_in: Option<Vec<String>>,
    pub blocks: Option<Vec<Block>>,
    pub footer: Option<String>,
    pub footer_icon: Option<String>,
    pub ts: Option<i32>,
}

impl Attachment {
    pub fn builder() -> AttachmentBuilder {
        AttachmentBuilder::new()
    }
}

#[derive(Debug, Default)]
pub struct AttachmentBuilder {
    pub color: Option<String>,
    pub fallback: Option<String>,
    pub callback_id: Option<String>,
    pub id: Option<i32>,
    pub author_id: Option<String>,
    pub author_name: Option<String>,
    pub author_subname: Option<String>,
    pub author_link: Option<String>,
    pub author_icon: Option<String>,
    pub title: Option<String>,
    pub title_link: Option<String>,
    pub pretext: Option<String>,
    pub text: Option<String>,
    pub image_url: Option<String>,
    pub thumb_url: Option<String>,
    pub thumb_height: Option<i32>,
    pub thumb_width: Option<i32>,
    pub from_url: Option<String>,
    pub original_url: Option<String>,
    pub fields: Option<Vec<AttachmentField>>,
    pub mrkdwn_in: Option<Vec<String>>,
    pub blocks: Option<Vec<Block>>,
    pub footer: Option<String>,
    pub footer_icon: Option<String>,
    pub ts: Option<i32>,
}

impl AttachmentBuilder {
    pub fn new() -> AttachmentBuilder {
        AttachmentBuilder {
            fallback: Some("".to_string()),
            text: Some("".to_string()),
            ..Default::default()
        }
    }
    pub fn color(mut self, color: String) -> AttachmentBuilder {
        self.color = Some(color);
        self
    }
    pub fn fallback(mut self, fallback: String) -> AttachmentBuilder {
        self.fallback = Some(fallback);
        self
    }
    pub fn callback_id(mut self, callback_id: String) -> AttachmentBuilder {
        self.callback_id = Some(callback_id);
        self
    }
    pub fn author_name(mut self, author_name: String) -> AttachmentBuilder {
        self.author_name = Some(author_name);
        self
    }
    pub fn author_subname(mut self, author_subname: String) -> AttachmentBuilder {
        self.author_subname = Some(author_subname);
        self
    }
    pub fn author_link(mut self, author_link: String) -> AttachmentBuilder {
        self.author_link = Some(author_link);
        self
    }
    pub fn author_icon(mut self, author_icon: String) -> AttachmentBuilder {
        self.author_icon = Some(author_icon);
        self
    }
    pub fn title(mut self, title: String) -> AttachmentBuilder {
        self.title = Some(title);
        self
    }
    pub fn title_link(mut self, title_link: String) -> AttachmentBuilder {
        self.title_link = Some(title_link);
        self
    }
    pub fn pretext(mut self, pretext: String) -> AttachmentBuilder {
        self.pretext = Some(pretext);
        self
    }
    pub fn text(mut self, text: String) -> AttachmentBuilder {
        self.text = Some(text);
        self
    }
    pub fn image_url(mut self, image_url: String) -> AttachmentBuilder {
        self.image_url = Some(image_url);
        self
    }
    pub fn thumb_url(mut self, thumb_url: String) -> AttachmentBuilder {
        self.thumb_url = Some(thumb_url);
        self
    }
    pub fn thumb_height(mut self, thumb_height: i32) -> AttachmentBuilder {
        self.thumb_height = Some(thumb_height);
        self
    }
    pub fn thumb_width(mut self, thumb_width: i32) -> AttachmentBuilder {
        self.thumb_width = Some(thumb_width);
        self
    }
    pub fn fields(mut self, fields: Vec<AttachmentField>) -> AttachmentBuilder {
        self.fields = Some(fields);
        self
    }
    pub fn mrkdwn_in(mut self, mrkdwn_in: Vec<String>) -> AttachmentBuilder {
        self.mrkdwn_in = Some(mrkdwn_in);
        self
    }
    pub fn blocks(mut self, blocks: Vec<Block>) -> AttachmentBuilder {
        self.blocks = Some(blocks);
        self
    }
    pub fn footer(mut self, footer: String) -> AttachmentBuilder {
        self.footer = Some(footer);
        self
    }
    pub fn ts(mut self, ts: i32) -> AttachmentBuilder {
        self.ts = Some(ts);
        self
    }
    pub fn build(self) -> Attachment {
        Attachment {
            color: self.color,
            fallback: self.fallback,
            callback_id: self.callback_id,
            id: self.id,
            author_id: self.author_id,
            author_name: self.author_name,
            author_subname: self.author_subname,
            author_link: self.author_link,
            author_icon: self.author_icon,
            title: self.title,
            title_link: self.title_link,
            pretext: self.pretext,
            text: self.text,
            image_url: self.image_url,
            thumb_url: self.thumb_url,
            thumb_height: self.thumb_height,
            thumb_width: self.thumb_width,
            from_url: self.from_url,
            original_url: self.original_url,
            fields: self.fields,
            mrkdwn_in: self.mrkdwn_in,
            blocks: self.blocks,
            footer: self.footer,
            footer_icon: self.footer_icon,
            ts: self.ts,
        }
    }
}
