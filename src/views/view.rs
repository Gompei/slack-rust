use crate::block::block_object::TextBlockObject;
use crate::block::blocks::BlockAction;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ViewState {
    pub values: HashMap<String, HashMap<String, BlockAction>>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct View {
    pub id: Option<String>,
    pub team_id: Option<String>,
    pub r#type: Option<String>,
    pub title: TextBlockObject,
    pub close: TextBlockObject,
    pub submit: TextBlockObject,
    //pub blocks: Blocks,
    pub private_metadata: Option<String>,
    pub callback_id: Option<String>,
    pub hash: Option<String>,
    pub clear_on_close: bool,
    pub notify_on_close: bool,
    pub root_view_id: Option<String>,
    pub previous_view_id: Option<String>,
    pub app_id: Option<String>,
    pub external_id: Option<String>,
    pub bot_id: Option<String>,
}
