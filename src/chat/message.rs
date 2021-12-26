use crate::attachment::attachment::Attachment;
use crate::block::blocks::Block;
use crate::reactions::reaction::Reaction;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct Message {
    pub bot_id: Option<String>,
    #[serde(rename = "type")]
    pub type_file: Option<String>,
    pub text: Option<String>,
    pub user: Option<String>,
    pub username: Option<String>,
    pub ts: Option<String>,
    pub team: Option<String>,
    pub attachments: Option<Vec<Attachment>>,
    pub blocks: Option<Vec<Block>>,
    pub subtype: Option<String>,
    pub reactions: Option<Vec<Reaction>>,
}
