use crate::chat::message::Message;
use crate::comments::comment::Comment;
use crate::files::file::File;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct Item {
    #[serde(rename = "type")]
    pub type_filed: Option<String>,
    pub channel: Option<String>,
    pub message: Option<Message>,
    pub file: Option<File>,
    pub comment: Option<Comment>,
    pub timestamp: Option<String>,
}
