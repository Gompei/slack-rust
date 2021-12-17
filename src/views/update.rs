use crate::error::Error;
use crate::http_client::{get_slack_url, SlackWebAPIClient};
use crate::views::view::View;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct UpdateRequest {
    pub trigger_id: String,
    pub view: View,
    pub external_id: Option<String>,
    pub view_id: Option<String>,
    pub hash: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct UpdateResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub view: Option<View>,
}

pub async fn update<T>(
    client: &T,
    param: &UpdateRequest,
    bot_token: &str,
) -> Result<UpdateResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("views.update");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(&url, &json, bot_token)
        .await
        .and_then(|result| {
            serde_json::from_str::<UpdateResponse>(&result).map_err(Error::SerdeJsonError)
        })
}
