use crate::block::block_actions::ActionBlock;
use crate::block::block_context::ContextBlock;
use crate::block::block_divider::DividerBlock;
use crate::block::block_file::FileBlock;
use crate::block::block_header::HeaderBlock;
use crate::block::block_image::ImageBlock;
use crate::block::block_input::InputBlock;
use crate::block::block_object::{OptionBlockObject, TextBlockObject};
use crate::block::block_section::SectionBlock;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum Block {
    ActionBlock(ActionBlock),
    ContextBlock(ContextBlock),
    DividerBlock(DividerBlock),
    FileBlock(FileBlock),
    HeaderBlock(HeaderBlock),
    ImageBlock(ImageBlock),
    InputBlock(InputBlock),
    SectionBlock(SectionBlock),
    None,
}

impl Block {
    pub fn block_type(&self) -> BlockType {
        match self {
            Block::ActionBlock(ActionBlock { .. }) => BlockType::Actions,
            Block::ContextBlock(ContextBlock { .. }) => BlockType::Context,
            Block::DividerBlock(DividerBlock { .. }) => BlockType::Divider,
            Block::FileBlock(FileBlock { .. }) => BlockType::File,
            Block::HeaderBlock(HeaderBlock { .. }) => BlockType::Header,
            Block::ImageBlock(ImageBlock { .. }) => BlockType::Image,
            Block::InputBlock(InputBlock { .. }) => BlockType::Input,
            Block::SectionBlock(SectionBlock { .. }) => BlockType::Section,
            Block::None => BlockType::None,
        }
    }
}

impl Default for Block {
    fn default() -> Self {
        Block::None
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BlockType {
    Actions,
    Context,
    Divider,
    File,
    Header,
    Image,
    Input,
    Section,
    None,
}

impl Default for BlockType {
    fn default() -> Self {
        BlockType::None
    }
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct BlockAction {
    pub action_id: Option<String>,
    pub block_id: Option<String>,
    pub text: TextBlockObject,
    pub value: Option<String>,
    pub actions_ts: Option<String>,
    pub selected_option: OptionBlockObject,
    pub selected_options: Vec<OptionBlockObject>,
    pub selected_user: Option<String>,
    pub selected_users: Vec<String>,
    pub selected_channel: Option<String>,
    pub selected_channels: Vec<String>,
    pub selected_conversation: Option<String>,
    pub selected_conversations: Vec<String>,
    pub selected_date: Option<String>,
    pub selected_time: Option<String>,
    pub initial_option: OptionBlockObject,
    pub initial_user: Option<String>,
    pub initial_channel: Option<String>,
    pub initial_conversation: Option<String>,
    pub initial_date: Option<String>,
    pub initial_time: Option<String>,
}
