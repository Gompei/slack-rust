use crate::block::block_elements::BlockElement;
use crate::block::block_object::TextBlockObject;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct InputBlock {
    pub block_id: Option<String>,
    pub label: TextBlockObject,
    pub element: Option<BlockElement>,
    pub hint: Option<TextBlockObject>,
    pub optional: Option<bool>,
    pub dispatch_action: Option<bool>,
}
