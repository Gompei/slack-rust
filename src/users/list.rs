use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::http_client::{get_slack_url, post_json, Client, ResponseMetadata};
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

pub async fn list(
    client: Client,
    param: ListRequest,
    bot_token: String,
) -> Result<ListResponse, Error> {
    let url = get_slack_url("users.list");
    let json = serde_json::to_string(&param)?;

    post_json(client, url, json, bot_token)
        .await?
        .body_json()
        .await
        .map_err(|e| Error::SurfError(e))
}
