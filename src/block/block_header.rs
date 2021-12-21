use crate::block::block_object::TextBlockObject;
use crate::block::blocks::BlockType;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct HeaderBlock {
    #[serde(rename = "type")]
    pub type_filed: BlockType,
    pub text: Option<TextBlockObject>,
    pub block_id: Option<String>,
}
