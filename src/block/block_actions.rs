use crate::block::block_elements::BlockElement;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct ActionBlock {
    pub elements: Vec<BlockElement>,
    pub block_id: Option<String>,
}
