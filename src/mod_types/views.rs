use std::collections::HashMap;

pub struct View {
    id: String,
    team_id: String,
    r#type: String,
    title: TextBlockObject,
    close: TextBlockObject,
    submit: TextBlockObject,
    blocks: Blocks,
    private_metadata: String,
    callback_id: String,
    // State           *ViewState       `json:"state"`
    hash: String,
    clear_on_close: bool,
    notify_on_close: bool,
    root_view_id: String,
    previous_view_id: String,
    app_id: String,
    external_id: String,
    bot_id: String,
}

pub struct TextBlockObject {
    r#type: String,
    text: String,
    emoji: bool,
    verbatim: bool,
}

pub struct Blocks {
    block_set: Vec<Block>,
}

pub trait Block {
    fn block_type(&self) -> String;
}

pub struct BlockAction {
    action_id: String,
    block_id: String,
    action_type: String,
}
