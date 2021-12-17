use serde::{Deserialize, Serialize};
use std::fmt;

#[typetag::serde]
pub trait BlockElement {
    fn element_type(&self) -> &String;
}

impl fmt::Debug for dyn BlockElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.element_type())
    }
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct BlockElements(Option<Vec<Box<dyn BlockElement>>>);

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct SelectBlockElement {}

#[typetag::serde]
impl BlockElement for SelectBlockElement {
    fn element_type(&self) -> &String {
        &self.r#type
    }
}
