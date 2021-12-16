use crate::error::Error;
use crate::http_client::{get_slack_url, ResponseMetadata, SlackWebAPIClient};
use crate::views::view::View;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct PublishRequest {
    pub user_id: String,
    pub view: View,
    pub hash: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PublishResponse {
    pub ok: bool,
    pub view: Option<View>,
    pub response_metadata: Option<ResponseMetadata>,
}

pub async fn publish<T>(
    client: &T,
    param: &PublishRequest,
    bot_token: &str,
) -> Result<PublishResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("views.publish");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<PublishResponse>(&result).map_err(Error::SerdeJsonError)
        })
}
