//! A content divider, like an `<hr>`, to split up different blocks inside of a message.

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A content divider, like an `<hr>`, to split up different blocks inside of a message.    
/// See: <https://api.slack.com/reference/block-kit/blocks#divider>
#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct DividerBlock {
    pub block_id: Option<String>,
}

impl DividerBlock {
    pub fn new(block_id: String) -> DividerBlock {
        DividerBlock {
            block_id: Some(block_id),
        }
    }
}
