//! A header is a plain-text block that displays in a larger, bold font.

use crate::block::block_object::TextBlockObject;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A header is a plain-text block that displays in a larger, bold font.    
/// See: <https://api.slack.com/reference/block-kit/blocks#header>
#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct HeaderBlock {
    pub text: TextBlockObject,
    pub block_id: Option<String>,
}

impl HeaderBlock {
    pub fn builder(text: TextBlockObject) -> HeaderBlockBuilder {
        HeaderBlockBuilder::new(text)
    }
}

#[derive(Debug, Default)]
pub struct HeaderBlockBuilder {
    pub text: TextBlockObject,
    pub block_id: Option<String>,
}

impl HeaderBlockBuilder {
    pub fn new(text: TextBlockObject) -> HeaderBlockBuilder {
        HeaderBlockBuilder {
            text,
            ..Default::default()
        }
    }
    pub fn block_id(mut self, block_id: String) -> HeaderBlockBuilder {
        self.block_id = Some(block_id);
        self
    }
    pub fn build(self) -> HeaderBlock {
        HeaderBlock {
            text: self.text,
            block_id: self.block_id,
        }
    }
}
