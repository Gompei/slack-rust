use crate::block::block_elements::BlockElement;
use crate::block::blocks::Block;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ActionBlock {
    pub elements: Vec<Box<dyn BlockElement>>,
    pub block_id: Option<String>,
}

#[typetag::serde(name = "actions")]
impl Block for ActionBlock {}
