use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::http_client::{get_slack_url, ResponseMetadata, SlackWebAPIClient};
use crate::users::user::User;

#[derive(Deserialize, Serialize, Debug)]
pub struct ListRequest {
    pub cursor: Option<String>,
    pub include_locale: Option<bool>,
    pub limit: Option<i32>,
    pub team_id: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ListResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub members: Option<Vec<User>>,
    pub cache_ts: Option<i32>,
    pub response_metadata: Option<ResponseMetadata>,
}

pub async fn list<T>(
    client: &T,
    param: ListRequest,
    bot_token: String,
) -> Result<ListResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("users.list");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(url, json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<ListResponse>(&result).map_err(Error::SerdeJsonError)
        })
}
