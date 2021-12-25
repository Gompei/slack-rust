use crate::block::block_elements::BlockElement;
use crate::block::block_object::TextBlockObject;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct SectionBlock {
    pub block_id: Option<String>,
    pub text: TextBlockObject,
    pub element: Option<BlockElement>,
    pub fields: Option<Vec<TextBlockObject>>,
    pub accessory: Option<BlockElement>,
}
