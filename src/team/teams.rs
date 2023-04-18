use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct Team {
    pub id: Option<String>,
    pub name: Option<String>,
    pub date_created: Option<u32>,
    pub domain: Option<String>,
    pub email_domain: Option<String>,
    pub icon: Option<Icon>,
    pub is_verified: Option<bool>,
    pub enterprise_id: Option<String>,
    pub enterprise_name: Option<String>,
}

// TODO
#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct Icon {
    pub image_34: Option<String>,
    pub image_44: Option<String>,
    pub image_68: Option<String>,
    pub image_88: Option<String>,
    pub image_102: Option<String>,
    pub image_132: Option<String>,
    pub image_default: Option<bool>,
}
