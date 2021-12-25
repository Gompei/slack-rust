use crate::attachment::attachment::Attachment;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct Channel {
    pub id: Option<String>,
    pub name: Option<String>,
    pub is_channel: Option<bool>,
    pub created: Option<i32>,
    pub creator: Option<String>,
    pub is_archived: Option<bool>,
    pub is_general: Option<bool>,
    pub name_normalized: Option<String>,
    pub is_shared: Option<bool>,
    pub is_org_shared: Option<bool>,
    pub is_member: Option<bool>,
    pub is_private: Option<bool>,
    pub is_mpim: Option<bool>,
    pub last_read: Option<String>,
    pub latest: Option<Latest>,
    pub unread_count: Option<i32>,
    pub unread_count_display: Option<i32>,
    pub members: Option<Vec<String>>,
    pub topic: Option<Topic>,
    pub purpose: Option<Purpose>,
    pub previous_names: Option<Vec<String>>,
    pub priority: Option<i32>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct Latest {
    pub text: Option<String>,
    pub username: Option<String>,
    pub bot_id: Option<String>,
    pub attachments: Option<Vec<Attachment>>,
    #[serde(rename = "type")]
    pub type_filed: Option<String>,
    pub subtype: Option<String>,
    pub ts: Option<String>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct Topic {
    pub value: Option<String>,
    pub creator: Option<String>,
    pub last_set: Option<i32>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct Purpose {
    pub value: Option<String>,
    pub creator: Option<String>,
    pub last_set: Option<i32>,
}
