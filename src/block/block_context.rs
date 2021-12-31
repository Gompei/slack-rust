//! Displays message context, which can include both images and text.

use crate::block::block_elements::MixedElement;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Displays message context, which can include both images and text.  
/// See: <https://api.slack.com/reference/block-kit/blocks#context>
#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct ContextBlock {
    pub elements: Vec<MixedElement>,
    pub block_id: Option<String>,
}

impl ContextBlock {
    pub fn builder(elements: Vec<MixedElement>) -> ContextBlockBuilder {
        ContextBlockBuilder::new(elements)
    }
}

#[derive(Debug, Default)]
pub struct ContextBlockBuilder {
    pub elements: Vec<MixedElement>,
    pub block_id: Option<String>,
}

impl ContextBlockBuilder {
    pub fn new(elements: Vec<MixedElement>) -> ContextBlockBuilder {
        ContextBlockBuilder {
            elements,
            ..Default::default()
        }
    }
    pub fn block_id(mut self, block_id: String) -> ContextBlockBuilder {
        self.block_id = Some(block_id);
        self
    }
    pub fn build(self) -> ContextBlock {
        ContextBlock {
            elements: self.elements,
            block_id: self.block_id,
        }
    }
}
