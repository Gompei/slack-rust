use crate::block::block_object::TextBlockObject;
use crate::block::blocks::Block;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ImageBlock {
    pub r#type: String,
    pub image_url: String,
    pub alt_text: String,
    pub block_id: Option<String>,
    pub text: Option<TextBlockObject>,
}

#[typetag::serde]
impl Block for ImageBlock {
    fn block_type(&self) -> &String {
        &self.r#type
    }
}
