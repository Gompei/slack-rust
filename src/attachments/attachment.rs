use crate::block::blocks::Block;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct AttachmentField {
    pub title: Option<String>,
    pub value: Option<String>,
    pub short: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct AttachmentAction {
    pub name: Option<String>,
    pub text: Option<String>,
    pub style: Option<String>,
    pub r#type: Option<String>,
    pub value: Option<String>,
    pub data_source: Option<String>,
    pub min_query_length: Option<i32>,
    pub options: Option<AttachmentActionOption>,
    pub selected_options: Option<AttachmentActionOption>,
    pub option_groups: Option<AttachmentActionOptionGroup>,
    pub confirm: Option<ConfirmationField>,
    pub url: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct AttachmentActionOption {
    pub text: Option<String>,
    pub value: Option<String>,
    pub description: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct AttachmentActionOptionGroup {
    pub text: Option<String>,
    pub options: Option<Vec<AttachmentActionOption>>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ConfirmationField {
    pub title: Option<String>,
    pub text: Option<String>,
    pub ok_text: Option<String>,
    pub dismiss_text: Option<String>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default)]
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
    pub service_name: Option<String>,
    pub service_icon: Option<String>,
    pub from_url: Option<String>,
    pub original_url: Option<String>,
    pub fields: Option<Vec<AttachmentField>>,
    pub actions: Option<Vec<AttachmentAction>>,
    pub mrkdwn_in: Option<Vec<String>>,
    pub blocks: Option<Vec<Block>>,
    pub footer: Option<String>,
    pub footer_icon: Option<String>,
    pub ts: Option<i32>,
}
