use crate::error::Error;

use crate::http_client::{get_slack_url, ResponseMetadata, SlackWebAPIClient};
use crate::items::item::Item;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ListRequest {
    pub count: Option<i32>,
    pub cursor: Option<String>,
    pub full: Option<bool>,
    pub limit: Option<i32>,
    pub page: Option<i32>,
    pub team_id: Option<String>,
    pub user: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ListResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub items: Option<Item>,
    pub response_metadata: Option<ResponseMetadata>,
}

pub async fn list<T>(
    client: &T,
    param: &ListRequest,
    bot_token: &str,
) -> Result<ListResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("reactions.list");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<ListResponse>(&result).map_err(Error::SerdeJsonError)
        })
}
