use crate::block::block_elements::MixedElement;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct ContextBlock {
    pub block_id: Option<String>,
    pub elements: Option<Vec<MixedElement>>,
}
