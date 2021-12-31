//! Displays a [remote file](https://api.slack.com/messaging/files/remote).

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Displays a remote file.   
/// See: <https://api.slack.com/reference/block-kit/blocks#file>
#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct FileBlock {
    pub external_id: String,
    pub source: String,
    pub block_id: Option<String>,
}

impl FileBlock {
    pub fn builder(external_id: String, source: String) -> FileBlockBuilder {
        FileBlockBuilder::new(external_id, source)
    }
}

#[derive(Debug, Default)]
pub struct FileBlockBuilder {
    pub external_id: String,
    pub source: String,
    pub block_id: Option<String>,
}

impl FileBlockBuilder {
    pub fn new(external_id: String, source: String) -> FileBlockBuilder {
        FileBlockBuilder {
            external_id,
            source,
            ..Default::default()
        }
    }
    pub fn block_id(mut self, block_id: String) -> FileBlockBuilder {
        self.block_id = Some(block_id);
        self
    }
    pub fn build(self) -> FileBlockBuilder {
        FileBlockBuilder {
            external_id: self.external_id,
            source: self.source,
            block_id: self.block_id,
        }
    }
}
