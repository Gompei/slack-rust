use crate::block::blocks::Block;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct DividerBlock {
    pub r#type: String,
    pub block_id: Option<String>,
}

#[typetag::serde]
impl Block for DividerBlock {
    fn block_type(&self) -> &String {
        &self.r#type
    }
}
