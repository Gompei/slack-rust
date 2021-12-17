use crate::block::block_elements::BlockElements;
use crate::block::blocks::Block;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ActionBlock {
    pub r#type: String,
    pub block_id: Option<String>,
    pub elements: Option<BlockElements>,
}

impl Block for ActionBlock {
    fn block_type(&self) -> &String {
        &self.r#type
    }
}
