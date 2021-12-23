use crate::block::blocks::BlockType;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct FileBlock {
    #[serde(rename = "type")]
    pub type_filed: BlockType,
    pub block_id: Option<String>,
    pub external_id: Option<String>,
    pub source: Option<String>,
}
