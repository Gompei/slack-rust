use crate::block::block_elements::BlockElement;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct ActionBlock {
    pub elements: Vec<BlockElement>,
    pub block_id: Option<String>,
}

impl ActionBlock {
    pub fn builder(elements: Vec<BlockElement>) -> ActionBlockBuilder {
        ActionBlockBuilder::new(elements)
    }
}

#[derive(Debug, Default)]
pub struct ActionBlockBuilder {
    pub elements: Vec<BlockElement>,
    pub block_id: Option<String>,
}

impl ActionBlockBuilder {
    pub fn new(elements: Vec<BlockElement>) -> ActionBlockBuilder {
        ActionBlockBuilder {
            elements,
            ..Default::default()
        }
    }
    pub fn block_id(mut self, block_id: String) -> ActionBlockBuilder {
        self.block_id = Some(block_id);
        self
    }
    pub fn build(self) -> ActionBlock {
        ActionBlock {
            elements: self.elements,
            block_id: self.block_id,
        }
    }
}
