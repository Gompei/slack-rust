use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::http_client::{get_slack_url, SlackWebAPIClient};

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateRequest {
    pub channel_name: String,
    pub name: String,
    pub is_private: Option<bool>,
    pub team_id: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub channel: Option<Channel>,
}

// TODO
#[derive(Deserialize, Serialize, Debug)]
pub struct Channel {
    pub id: Option<String>,
    pub name: Option<String>,
    pub is_channel: Option<bool>,
    pub is_group: Option<bool>,
    pub is_im: Option<bool>,
    pub created: Option<i32>,
    pub creator: Option<String>,
    pub is_archived: Option<bool>,
    pub is_general: Option<bool>,
    pub unlinked: Option<i32>,
    pub name_normalized: Option<String>,
    pub is_shared: Option<bool>,
    pub is_ext_shared: Option<bool>,
    pub is_org_shared: Option<bool>,
    // TODO
    // pub pending_shared: [],
    pub is_pending_ext_shared: Option<bool>,
    pub is_member: Option<bool>,
    pub is_private: Option<bool>,
    pub is_mpim: Option<bool>,
    pub last_read: Option<String>,
    // TODO
    // pub latest: null,
    pub unread_count: Option<i32>,
    pub unread_count_display: Option<i32>,
    pub topic: Option<Topic>,
    pub purpose: Option<Purpose>,
    //pub previous_names: Option,
    pub priority: Option<i32>,
}

// TODO
#[derive(Deserialize, Serialize, Debug)]
pub struct Topic {
    pub value: Option<String>,
    pub creator: Option<String>,
    pub last_set: Option<i32>,
}

// TODO
#[derive(Deserialize, Serialize, Debug)]
pub struct Purpose {
    pub value: Option<String>,
    pub creator: Option<String>,
    pub last_set: Option<i32>,
}

pub async fn create<T>(
    client: &T,
    param: &CreateRequest,
    bot_token: &str,
) -> Result<CreateResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("conversations.create");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<CreateResponse>(&result).map_err(Error::SerdeJsonError)
        })
}
