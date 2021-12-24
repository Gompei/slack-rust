use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct FileBlock {
    pub block_id: Option<String>,
    pub external_id: Option<String>,
    pub source: Option<String>,
}
