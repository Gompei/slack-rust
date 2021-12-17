use crate::block::block_object::{OptionBlockObject, TextBlockObject};
use serde::{Deserialize, Serialize};
use std::fmt;

#[typetag::serde]
pub trait Block {
    fn block_type(&self) -> &String;
}

impl fmt::Debug for dyn Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.block_type())
    }
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Blocks(Option<Vec<Box<dyn Block>>>);

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct BlockAction {
    pub action_id: Option<String>,
    pub block_id: Option<String>,
    pub r#type: String,
    pub text: TextBlockObject,
    pub value: Option<String>,
    pub actions_ts: Option<String>,
    pub selected_option: OptionBlockObject,
    pub selected_options: Vec<OptionBlockObject>,
    pub selected_user: Option<String>,
    pub selected_users: Vec<String>,
    pub selected_channel: Option<String>,
    pub selected_channels: Vec<String>,
    pub selected_conversation: Option<String>,
    pub selected_conversations: Vec<String>,
    pub selected_date: Option<String>,
    pub selected_time: Option<String>,
    pub initial_option: OptionBlockObject,
    pub initial_user: Option<String>,
    pub initial_channel: Option<String>,
    pub initial_conversation: Option<String>,
    pub initial_date: Option<String>,
    pub initial_time: Option<String>,
}

#[typetag::serde]
impl Block for BlockAction {
    fn block_type(&self) -> &String {
        &self.r#type
    }
}
