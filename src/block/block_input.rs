use crate::block::block_elements::BlockElement;
use crate::block::block_object::TextBlockObject;
use crate::block::blocks::Block;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct InputBlock {
    pub block_id: Option<String>,
    pub label: TextBlockObject,
    pub element: Option<Box<dyn BlockElement>>,
    pub hint: Option<TextBlockObject>,
    pub optional: Option<bool>,
    pub dispatch_action: Option<bool>,
}

#[typetag::serde(name = "input")]
impl Block for InputBlock {}
