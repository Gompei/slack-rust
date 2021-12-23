use crate::chat::message::Message;
use crate::comments::comment::Comment;
use crate::files::file::File;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct Item {
    pub r#type: Option<String>,
    pub channel: Option<String>,
    pub message: Option<Message>,
    pub file: Option<File>,
    pub comment: Option<Comment>,
    pub timestamp: Option<String>,
}
