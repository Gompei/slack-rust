use crate::block::block_object::TextBlockObject;
use crate::block::blocks::Block;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct HeaderBlock {
    pub r#type: String,
    pub text: Option<TextBlockObject>,
    pub block_id: Option<String>,
}

#[typetag::serde]
impl Block for HeaderBlock {
    fn block_type(&self) -> &String {
        &self.r#type
    }
}
