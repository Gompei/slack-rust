use crate::error::Error;
use crate::http_client::{get_slack_url, ResponseMetadata, SlackWebAPIClient};
use crate::views::view::View;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct PushRequest {
    pub trigger_id: String,
    pub view: View,
}

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct PushResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub view: Option<View>,
    pub response_metadata: Option<ResponseMetadata>,
}

pub async fn push<T>(
    client: &T,
    param: &PushRequest,
    bot_token: &str,
) -> Result<PushResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("views.push");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<PushResponse>(&result).map_err(Error::SerdeJsonError)
        })
}
