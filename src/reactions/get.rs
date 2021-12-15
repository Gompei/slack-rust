use crate::error::Error;
use crate::files::file::File;
use crate::http_client::{get_slack_url, SlackWebAPIClient};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct GetRequest {
    pub channel: Option<String>,
    pub file: Option<String>,
    pub file_comment: Option<String>,
    pub full: Option<bool>,
    pub timestamp: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub file: Option<File>,
    pub r#type: Option<String>,
}

pub async fn get<T>(client: &T, param: &GetRequest, bot_token: &str) -> Result<GetResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("reactions.get");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<GetResponse>(&result).map_err(Error::SerdeJsonError)
        })
}
