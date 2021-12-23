use crate::attachments::attachment::Attachment;
use crate::block::blocks::Block;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct Message {
    pub text: Option<String>,
    pub username: Option<String>,
    pub bot_id: Option<String>,
    pub blocks: Option<Vec<Block>>,
    pub attachments: Option<Vec<Attachment>>,
    pub r#type: Option<String>,
    pub subtype: Option<String>,
    pub ts: Option<String>,
}
