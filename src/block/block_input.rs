use crate::block::block_elements::BlockElement;
use crate::block::block_object::TextBlockObject;
use crate::block::blocks::Block;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct InputBlock {
    pub r#type: String,
    pub block_id: Option<String>,
    pub label: TextBlockObject,
    pub element: BlockElement,
    pub hint: Option<TextBlockObject>,
    pub optional: Option<bool>,
    pub dispatch_action: Option<bool>,
}

#[typetag::serde]
impl Block for InputBlock {
    fn block_type(&self) -> &String {
        &self.r#type
    }
}
