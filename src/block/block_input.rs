use crate::block::block_elements::BlockElement;
use crate::block::block_object::TextBlockObject;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct InputBlock {
    pub label: TextBlockObject,
    pub element: BlockElement,
    pub dispatch_action: Option<bool>,
    pub block_id: Option<String>,
    pub hint: Option<TextBlockObject>,
    pub optional: Option<bool>,
}

impl InputBlock {
    pub fn builder(label: TextBlockObject, element: BlockElement) -> InputBlockBuilder {
        InputBlockBuilder::new(label, element)
    }
}

#[derive(Debug, Default)]
pub struct InputBlockBuilder {
    pub label: TextBlockObject,
    pub element: BlockElement,
    pub dispatch_action: Option<bool>,
    pub block_id: Option<String>,
    pub hint: Option<TextBlockObject>,
    pub optional: Option<bool>,
}

impl InputBlockBuilder {
    pub fn new(label: TextBlockObject, element: BlockElement) -> InputBlockBuilder {
        InputBlockBuilder {
            label,
            element,
            ..Default::default()
        }
    }
    pub fn dispatch_action(mut self, dispatch_action: bool) -> InputBlockBuilder {
        self.dispatch_action = Some(dispatch_action);
        self
    }
    pub fn block_id(mut self, block_id: String) -> InputBlockBuilder {
        self.block_id = Some(block_id);
        self
    }
    pub fn hint(mut self, hint: TextBlockObject) -> InputBlockBuilder {
        self.hint = Some(hint);
        self
    }
    pub fn optional(mut self, optional: bool) -> InputBlockBuilder {
        self.optional = Some(optional);
        self
    }
    pub fn build(self) -> InputBlock {
        InputBlock {
            label: self.label,
            element: self.element,
            dispatch_action: self.dispatch_action,
            block_id: self.block_id,
            hint: self.hint,
            optional: self.optional,
        }
    }
}
