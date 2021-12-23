use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::http_client::{get_slack_url, ResponseMetadata, SlackWebAPIClient};

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct TeamsListRequest {
    pub cursor: Option<String>,
    pub include_icon: Option<bool>,
    pub limit: Option<i32>,
}

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct TeamsListResponse {
    pub ok: bool,
    pub error: Option<String>,
    // TODO
    // pub teams: Option<>
    pub response_metadata: Option<ResponseMetadata>,
}

pub async fn teams_list<T>(
    client: &T,
    param: &TeamsListRequest,
    bot_token: &str,
) -> Result<TeamsListResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("auth.teams.list");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<TeamsListResponse>(&result).map_err(Error::SerdeJsonError)
        })
}
