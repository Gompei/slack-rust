use crate::block::block_object::TextBlockObject;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct HeaderBlock {
    pub text: Option<TextBlockObject>,
    pub block_id: Option<String>,
}
