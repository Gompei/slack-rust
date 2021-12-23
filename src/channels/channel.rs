use serde::{Deserialize, Serialize};

// TODO
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct Channel {
    pub id: Option<String>,
    pub name: Option<String>,
    pub is_channel: Option<bool>,
    pub is_group: Option<bool>,
    pub is_im: Option<bool>,
    pub created: Option<i32>,
    pub creator: Option<String>,
    pub is_archived: Option<bool>,
    pub is_general: Option<bool>,
    pub unlinked: Option<i32>,
    pub name_normalized: Option<String>,
    pub is_shared: Option<bool>,
    pub is_ext_shared: Option<bool>,
    pub is_org_shared: Option<bool>,
    // TODO
    // pub pending_shared: [],
    pub is_pending_ext_shared: Option<bool>,
    pub is_member: Option<bool>,
    pub is_private: Option<bool>,
    pub is_mpim: Option<bool>,
    pub last_read: Option<String>,
    // TODO
    // pub latest: null,
    pub unread_count: Option<i32>,
    pub unread_count_display: Option<i32>,
    pub topic: Option<Topic>,
    pub purpose: Option<Purpose>,
    //pub previous_names: Option,
    pub priority: Option<i32>,
}

// TODO
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct Topic {
    pub value: Option<String>,
    pub creator: Option<String>,
    pub last_set: Option<i32>,
}

// TODO
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct Purpose {
    pub value: Option<String>,
    pub creator: Option<String>,
    pub last_set: Option<i32>,
}
