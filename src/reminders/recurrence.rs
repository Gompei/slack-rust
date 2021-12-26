use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct Recurrence {
    pub frequency: Option<String>,
    pub weekdays: Option<Vec<String>>,
}
