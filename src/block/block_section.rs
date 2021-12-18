use crate::block::block_elements::BlockElement;
use crate::block::block_object::TextBlockObject;
use crate::block::blocks::Block;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct SectionBlock {
    pub r#type: String,
    pub block_id: Option<String>,
    pub text: TextBlockObject,
    pub element: BlockElement,
    pub fields: Option<Vec<TextBlockObject>>,
    pub accessory: Option<BlockElement>,
}

#[typetag::serde]
impl Block for SectionBlock {
    fn block_type(&self) -> &String {
        &self.r#type
    }
}
