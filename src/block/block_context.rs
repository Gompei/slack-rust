use crate::block::block_elements::MixedElement;
use crate::block::blocks::Block;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ContextBlock {
    pub block_id: Option<String>,
    pub elements: Option<Vec<Box<dyn MixedElement>>>,
}

#[typetag::serde(name = "context")]
impl Block for ContextBlock {}
