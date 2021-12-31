//! A simple image block, designed to make those cat photos really pop.

use crate::block::block_object::TextBlockObject;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A simple image block, designed to make those cat photos really pop.  
/// See: <https://api.slack.com/reference/block-kit/blocks#image>
#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct ImageBlock {
    pub image_url: String,
    pub alt_text: String,
    pub title: Option<TextBlockObject>,
    pub block_id: Option<String>,
}

impl ImageBlock {
    pub fn builder(image_url: String, alt_text: String) -> ImageBlockBuilder {
        ImageBlockBuilder::new(image_url, alt_text)
    }
}

#[derive(Debug, Default)]
pub struct ImageBlockBuilder {
    pub image_url: String,
    pub alt_text: String,
    pub title: Option<TextBlockObject>,
    pub block_id: Option<String>,
}

impl ImageBlockBuilder {
    pub fn new(image_url: String, alt_text: String) -> ImageBlockBuilder {
        ImageBlockBuilder {
            image_url,
            alt_text,
            ..Default::default()
        }
    }
    pub fn title(mut self, title: TextBlockObject) -> ImageBlockBuilder {
        self.title = Some(title);
        self
    }
    pub fn block_id(mut self, block_id: String) -> ImageBlockBuilder {
        self.block_id = Some(block_id);
        self
    }
    pub fn build(self) -> ImageBlock {
        ImageBlock {
            image_url: self.image_url,
            alt_text: self.alt_text,
            title: self.title,
            block_id: self.block_id,
        }
    }
}
