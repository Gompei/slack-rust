use crate::block::block_elements::BlockElement;
use crate::block::block_object::TextBlockObject;
use crate::block::blocks::Block;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct SectionBlock {
    pub r#type: String,
    pub block_id: Option<String>,
    pub text: TextBlockObject,
    pub element: Option<Box<dyn BlockElement>>,
    pub fields: Option<Vec<TextBlockObject>>,
    pub accessory: Option<Box<dyn BlockElement>>,
}

#[typetag::serde(name = "context")]
impl Block for SectionBlock {}
