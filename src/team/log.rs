use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct Login {
    pub user_id: Option<String>,
    pub username: Option<String>,
    pub date_first: Option<i32>,
    pub date_last: Option<i32>,
    pub count: Option<i32>,
    pub ip: Option<String>,
    pub user_agent: Option<String>,
    pub isp: Option<String>,
    pub country: Option<String>,
    pub region: Option<String>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct Paging {
    pub count: Option<i32>,
    pub total: Option<i32>,
    pub page: Option<i32>,
    pub pages: Option<i32>,
}
