use crate::error::Error;
use crate::http_client::{get_slack_url, ResponseMetadata, SlackWebAPIClient};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct MembersRequest {
    pub channel: String,
    pub cursor: Option<String>,
    pub limit: Option<i32>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MembersResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub members: Option<Vec<String>>,
    pub response_metadata: Option<ResponseMetadata>,
}

pub async fn members<T>(
    client: &T,
    param: &MembersRequest,
    bot_token: &str,
) -> Result<MembersResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("conversations.members");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<MembersResponse>(&result).map_err(Error::SerdeJsonError)
        })
}
