use crate::block::block_elements::{BlockElements, MixedElement};
use crate::block::blocks::Block;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ContextBlock {
    pub r#type: String,
    pub block_id: Option<String>,
    pub elements: Option<ContextElements>,
}

#[typetag::serde]
impl Block for ContextBlock {
    fn block_type(&self) -> &String {
        &self.r#type
    }
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ContextElements(Option<Vec<Box<dyn MixedElement>>>);
