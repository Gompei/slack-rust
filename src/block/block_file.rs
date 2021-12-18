use crate::block::blocks::Block;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct FileBlock {
    pub r#type: String,
    pub block_id: Option<String>,
    pub external_id: Option<String>,
    pub source: Option<String>,
}

#[typetag::serde]
impl Block for FileBlock {
    fn block_type(&self) -> &String {
        &self.r#type
    }
}
