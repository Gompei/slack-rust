use crate::block::block_object::TextBlockObject;
use crate::block::blocks::{Block, BlockAction};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct ViewState {
    pub values: HashMap<String, HashMap<String, BlockAction>>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct View {
    pub id: Option<String>,
    pub team_id: Option<String>,
    #[serde(rename = "type")]
    pub type_filed: Option<String>,
    pub title: Option<TextBlockObject>,
    pub close: Option<TextBlockObject>,
    pub submit: Option<TextBlockObject>,
    pub blocks: Option<Vec<Block>>,
    pub private_metadata: Option<String>,
    pub callback_id: Option<String>,
    pub hash: Option<String>,
    pub clear_on_close: Option<bool>,
    pub notify_on_close: Option<bool>,
    pub root_view_id: Option<String>,
    pub previous_view_id: Option<String>,
    pub app_id: Option<String>,
    pub external_id: Option<String>,
    pub bot_id: Option<String>,
    pub state: Option<ViewState>,
}
