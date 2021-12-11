use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::http_client::{get_slack_url, SlackWebAPIClient};
use crate::views::view::View;

#[derive(Deserialize, Serialize, Debug)]
pub struct ViewOpenRequest {
    pub trigger_id: String,
    pub view: View,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ViewOpenResponse {
    pub ok: bool,
    pub view: Option<View>,
}

pub async fn open<T>(
    client: &T,
    param: ViewOpenRequest,
    bot_token: String,
) -> Result<ViewOpenResponse, Error>
where
    T: SlackWebAPIClient,
{
    let url = get_slack_url("views.open");
    let json = serde_json::to_string(&param)?;

    client
        .post_json(url, json, bot_token)
        .await?
        .body_json()
        .await
        .map_err(Error::SurfError)
}
