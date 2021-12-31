use crate::block::block_elements::BlockElement;
use crate::block::block_object::TextBlockObject;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct SectionBlock {
    pub text: Option<TextBlockObject>,
    pub block_id: Option<String>,
    pub fields: Option<Vec<TextBlockObject>>,
    pub accessory: Option<BlockElement>,
}

impl SectionBlock {
    pub fn builder() -> SectionBlockBuilder {
        SectionBlockBuilder::new()
    }
}

#[derive(Debug, Default)]
pub struct SectionBlockBuilder {
    pub text: Option<TextBlockObject>,
    pub block_id: Option<String>,
    pub fields: Option<Vec<TextBlockObject>>,
    pub accessory: Option<BlockElement>,
}

impl SectionBlockBuilder {
    pub fn new() -> SectionBlockBuilder {
        SectionBlockBuilder {
            ..Default::default()
        }
    }
    pub fn text(mut self, text: TextBlockObject) -> SectionBlockBuilder {
        self.text = Some(text);
        self
    }
    pub fn block_id(mut self, block_id: String) -> SectionBlockBuilder {
        self.block_id = Some(block_id);
        self
    }
    pub fn fields(mut self, fields: Vec<TextBlockObject>) -> SectionBlockBuilder {
        self.fields = Some(fields);
        self
    }
    pub fn accessory(mut self, accessory: BlockElement) -> SectionBlockBuilder {
        self.accessory = Some(accessory);
        self
    }
    pub fn build(self) -> SectionBlock {
        SectionBlock {
            text: self.text,
            block_id: self.block_id,
            fields: self.fields,
            accessory: self.accessory,
        }
    }
}
