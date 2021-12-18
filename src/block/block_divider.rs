use crate::block::blocks::Block;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct DividerBlock {
    pub r#type: String,
    pub block_id: Option<String>,
}

#[typetag::serde(name = "divider")]
impl Block for DividerBlock {}
